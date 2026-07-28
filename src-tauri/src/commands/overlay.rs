use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const OVERLAY_LABEL: &str = "overlay";

/// Cree la fenetre overlay si elle n'existe pas deja : transparente, sans
/// decorations, toujours au premier plan. Utilisee a la fois par le
/// basculement manuel (parametres) et par l'ouverture automatique en
/// debut de partie (voir `lib.rs`).
pub fn show_overlay_window(app: &AppHandle) -> Result<(), String> {
    if app.get_webview_window(OVERLAY_LABEL).is_some() {
        return Ok(());
    }

    WebviewWindowBuilder::new(
        app,
        OVERLAY_LABEL,
        WebviewUrl::App("index.html#/overlay".into()),
    )
    .title("Wardstone Overlay")
    .inner_size(420.0, 260.0)
    .min_inner_size(280.0, 160.0)
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .shadow(false)
    .resizable(true)
    .build()
    .map_err(|err| err.to_string())?;

    Ok(())
}

pub fn hide_overlay_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(OVERLAY_LABEL) {
        window.close().map_err(|err| err.to_string())?;
    }
    Ok(())
}

/// Bascule manuel utilise par la page Parametres (test/debug de l'overlay
/// en dehors d'une partie).
#[tauri::command]
pub fn toggle_overlay_window(app: AppHandle) -> Result<bool, String> {
    if app.get_webview_window(OVERLAY_LABEL).is_some() {
        hide_overlay_window(&app)?;
        Ok(false)
    } else {
        show_overlay_window(&app)?;
        Ok(true)
    }
}
