use sysinfo::{ProcessRefreshKind, RefreshKind, System};

/// Identifiants de connexion au LCU, extraits de la ligne de commande du
/// processus `LeagueClientUx`. Cette approche fonctionne quel que soit le
/// chemin d'installation du client (contrairement au parsing du lockfile
/// seul) et est celle utilisee par la plupart des outils communautaires LCU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LcuCredentials {
    pub port: u16,
    pub auth_token: String,
}

const PROCESS_NAMES: [&str; 2] = ["LeagueClientUx.exe", "LeagueClientUx"];

/// Cherche le processus du client League of Legends et en extrait le port
/// et le token d'authentification. Retourne `None` si le client n'est pas
/// lance (ce qui correspond a la phase `ClientClosed`).
pub fn discover_lcu_credentials() -> Option<LcuCredentials> {
    let mut system = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    for process in system.processes().values() {
        let name = process.name().to_string_lossy();
        if !PROCESS_NAMES
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            continue;
        }

        let args: Vec<String> = process
            .cmd()
            .iter()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect();

        if let Some(credentials) = parse_credentials(&args) {
            return Some(credentials);
        }
    }

    None
}

fn parse_credentials(args: &[String]) -> Option<LcuCredentials> {
    let mut port: Option<u16> = None;
    let mut auth_token: Option<String> = None;

    for arg in args {
        if let Some(value) = arg.strip_prefix("--app-port=") {
            port = value.parse().ok();
        } else if let Some(value) = arg.strip_prefix("--remoting-auth-token=") {
            auth_token = Some(value.to_string());
        }
    }

    match (port, auth_token) {
        (Some(port), Some(auth_token)) => Some(LcuCredentials { port, auth_token }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_port_and_token_from_argv() {
        let args = vec![
            "LeagueClientUx.exe".to_string(),
            "--riotclient-app-port=51234".to_string(),
            "--app-port=54321".to_string(),
            "--remoting-auth-token=abc123XYZ".to_string(),
            "--locale=fr_FR".to_string(),
        ];

        let creds = parse_credentials(&args).expect("credentials should be parsed");
        assert_eq!(creds.port, 54321);
        assert_eq!(creds.auth_token, "abc123XYZ");
    }

    #[test]
    fn returns_none_when_arguments_are_missing() {
        let args = vec![
            "LeagueClientUx.exe".to_string(),
            "--locale=fr_FR".to_string(),
        ];
        assert!(parse_credentials(&args).is_none());
    }
}
