use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const OVERLAY_LABEL: &str = "overlay";

/// Cree (ou ferme si deja ouverte) la fenetre overlay : transparente, sans
/// decorations, toujours au premier plan. Le contenu reel (timers,
/// objectifs, or estime...) est ajoute par l'Epic 5 — voir docs/ROADMAP.md ;
/// cette commande pose la mecanique de fenetre pour le shell desktop.
#[tauri::command]
pub fn toggle_overlay_window(app: AppHandle) -> Result<bool, String> {
    if let Some(window) = app.get_webview_window(OVERLAY_LABEL) {
        window.close().map_err(|err| err.to_string())?;
        return Ok(false);
    }

    WebviewWindowBuilder::new(
        &app,
        OVERLAY_LABEL,
        WebviewUrl::App("index.html#/overlay".into()),
    )
    .title("Wardstone Overlay")
    .inner_size(420.0, 180.0)
    .min_inner_size(240.0, 120.0)
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .shadow(false)
    .resizable(true)
    .build()
    .map_err(|err| err.to_string())?;

    Ok(true)
}
