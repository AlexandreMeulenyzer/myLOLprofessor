use std::time::Duration;

use base64::{engine::general_purpose::STANDARD, Engine};
use futures_util::{SinkExt, StreamExt};
use native_tls::TlsConnector as NativeTlsConnector;
use reqwest::header::HeaderValue;
use serde_json::Value;
use thiserror::Error;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, Message},
    Connector,
};

use super::process_discovery::LcuCredentials;

#[derive(Debug, Error)]
pub enum LcuWebSocketError {
    #[error("impossible d'etablir la connexion WebSocket au LCU: {0}")]
    Connect(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("impossible de construire le connecteur TLS: {0}")]
    Tls(#[from] native_tls::Error),
    #[error("en-tete d'authentification invalide")]
    InvalidHeader,
}

/// Type d'evenement du protocole RTM (WAMP-like) expose par le LCU.
/// Reference (comportement documente par la communaute des outils LCU,
/// notamment Rift Explorer / lcu-driver) :
///   5 = SUBSCRIBE, 6 = UNSUBSCRIBE, 8 = EVENT.
const OPCODE_SUBSCRIBE: u8 = 5;
const OPCODE_EVENT: u8 = 8;

/// Evenement LCU deja filtre/parsee, pret a etre consomme par le watcher.
pub struct LcuEvent {
    pub uri: String,
    pub data: Value,
}

/// Ouvre une connexion WebSocket au LCU et s'abonne au flux `OnJsonApiEvent`
/// (tous les evenements de l'API LCU). Retourne un flux d'evenements deja
/// parses ; l'appelant est responsable de filtrer par `uri`.
pub async fn subscribe_events(
    credentials: &LcuCredentials,
) -> Result<impl futures_util::Stream<Item = Result<LcuEvent, LcuWebSocketError>>, LcuWebSocketError>
{
    let url = format!("wss://127.0.0.1:{}/", credentials.port);
    let mut request = url.into_client_request()?;

    let auth_value = format!(
        "Basic {}",
        STANDARD.encode(format!("riot:{}", credentials.auth_token))
    );
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&auth_value).map_err(|_| LcuWebSocketError::InvalidHeader)?,
    );

    let tls_connector = NativeTlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build()?;

    let (mut socket, _response) = connect_async_tls_with_config(
        request,
        None,
        false,
        Some(Connector::NativeTls(tls_connector)),
    )
    .await?;

    let subscribe_frame = serde_json::json!([OPCODE_SUBSCRIBE, "OnJsonApiEvent"]).to_string();
    socket
        .send(Message::Text(subscribe_frame))
        .await
        .map_err(LcuWebSocketError::Connect)?;

    let stream = socket.filter_map(|message| async move {
        let message = match message {
            Ok(message) => message,
            Err(err) => return Some(Err(LcuWebSocketError::Connect(err))),
        };

        let text = match message {
            Message::Text(text) => text,
            _ => return None,
        };

        parse_event_frame(&text).map(Ok)
    });

    Ok(stream)
}

fn parse_event_frame(text: &str) -> Option<LcuEvent> {
    let frame: Value = serde_json::from_str(text).ok()?;
    let array = frame.as_array()?;

    let opcode = array.first()?.as_u64()? as u8;
    if opcode != OPCODE_EVENT {
        return None;
    }

    let payload = array.get(2)?;
    let uri = payload.get("uri")?.as_str()?.to_string();
    let data = payload.get("data").cloned().unwrap_or(Value::Null);

    Some(LcuEvent { uri, data })
}

/// Delai avant nouvelle tentative de connexion WebSocket apres un echec.
pub const RECONNECT_DELAY: Duration = Duration::from_secs(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_gameflow_event_frame() {
        let raw = r#"[8, "OnJsonApiEvent", {"uri": "/lol-gameflow/v1/gameflow-phase", "eventType": "Update", "data": "ChampSelect"}]"#;
        let event = parse_event_frame(raw).expect("should parse");
        assert_eq!(event.uri, "/lol-gameflow/v1/gameflow-phase");
        assert_eq!(event.data, Value::String("ChampSelect".to_string()));
    }

    #[test]
    fn ignores_non_event_opcodes() {
        let raw = r#"[5, "OnJsonApiEvent"]"#;
        assert!(parse_event_frame(raw).is_none());
    }

    #[test]
    fn ignores_malformed_frames() {
        assert!(parse_event_frame("not json").is_none());
        assert!(parse_event_frame("{}").is_none());
    }
}
