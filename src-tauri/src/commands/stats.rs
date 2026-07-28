use tauri::State;

use crate::infrastructure::db::Database;
use crate::stats_engine::{self, ChampionRoleStats};

/// Statistiques de champion calculees a partir des matchs deja synchronises
/// localement (voir `stats_engine`). Retourne `None` si aucune donnee n'a
/// encore ete collectee pour ce champion/role/patch — la couverture grandit
/// avec l'usage du logiciel (synchronisation d'historique, analyse
/// d'equipe...).
#[tauri::command]
pub fn get_champion_role_stats(
    db: State<'_, Database>,
    champion_id: i64,
    role: String,
    patch: String,
) -> Result<Option<ChampionRoleStats>, String> {
    stats_engine::compute_champion_role_stats(&db.lock(), champion_id, &role, &patch)
        .map_err(|err| err.to_string())
}
