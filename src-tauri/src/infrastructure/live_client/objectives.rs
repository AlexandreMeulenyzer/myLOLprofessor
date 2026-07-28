use serde::Serialize;

use super::dto::GameEvent;

/// Constantes de respawn des objectifs neutres. Ce sont des valeurs de
/// reference largement stables sur plusieurs saisons, mais Riot les ajuste
/// parfois en cours de saison : a valider/ajuster si les temps affiches
/// divergent du jeu reel. Riot n'expose pas ces constantes via une API —
/// c'est une estimation documentee, jamais une donnee officielle.
const DRAGON_FIRST_SPAWN_SECONDS: f64 = 5.0 * 60.0;
const DRAGON_RESPAWN_SECONDS: f64 = 5.0 * 60.0;
const BARON_FIRST_SPAWN_SECONDS: f64 = 20.0 * 60.0;
const BARON_RESPAWN_SECONDS: f64 = 6.0 * 60.0;
const HERALD_AVAILABLE_FROM_SECONDS: f64 = 8.0 * 60.0;
const HERALD_AVAILABLE_UNTIL_SECONDS: f64 = 20.0 * 60.0;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveTimers {
    /// Secondes avant le prochain dragon (0 si deja disponible).
    pub next_dragon_seconds: f64,
    /// Secondes avant le prochain Baron (0 si deja disponible).
    pub next_baron_seconds: f64,
    /// Le Heraut est dans sa fenetre de disponibilite et n'a pas encore ete tue.
    pub herald_available: bool,
}

pub fn compute_objective_timers(events: &[GameEvent], game_time: f64) -> ObjectiveTimers {
    let last_dragon_kill = last_event_time(events, "DragonKill");
    let next_dragon_absolute = match last_dragon_kill {
        Some(last_kill) => last_kill + DRAGON_RESPAWN_SECONDS,
        None => DRAGON_FIRST_SPAWN_SECONDS,
    };

    let last_baron_kill = last_event_time(events, "BaronKill");
    let next_baron_absolute = match last_baron_kill {
        Some(last_kill) => (last_kill + BARON_RESPAWN_SECONDS).max(BARON_FIRST_SPAWN_SECONDS),
        None => BARON_FIRST_SPAWN_SECONDS,
    };

    let herald_killed = last_event_time(events, "HeraldKill").is_some();
    let herald_window = HERALD_AVAILABLE_FROM_SECONDS..HERALD_AVAILABLE_UNTIL_SECONDS;
    let herald_available = !herald_killed && herald_window.contains(&game_time);

    ObjectiveTimers {
        next_dragon_seconds: (next_dragon_absolute - game_time).max(0.0),
        next_baron_seconds: (next_baron_absolute - game_time).max(0.0),
        herald_available,
    }
}

fn last_event_time(events: &[GameEvent], name: &str) -> Option<f64> {
    events
        .iter()
        .filter(|event| event.event_name == name)
        .map(|event| event.event_time)
        .fold(None, |max, time| {
            Some(max.map_or(time, |m: f64| m.max(time)))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(name: &str, time: f64) -> GameEvent {
        GameEvent {
            event_name: name.to_string(),
            event_time: time,
        }
    }

    #[test]
    fn dragon_defaults_to_first_spawn_when_never_killed() {
        let timers = compute_objective_timers(&[], 60.0);
        assert_eq!(
            timers.next_dragon_seconds,
            DRAGON_FIRST_SPAWN_SECONDS - 60.0
        );
    }

    #[test]
    fn dragon_respawns_five_minutes_after_last_kill() {
        let events = vec![event("DragonKill", 400.0)];
        let timers = compute_objective_timers(&events, 500.0);
        assert_eq!(
            timers.next_dragon_seconds,
            400.0 + DRAGON_RESPAWN_SECONDS - 500.0
        );
    }

    #[test]
    fn timer_clamps_to_zero_when_objective_is_overdue() {
        let events = vec![event("DragonKill", 100.0)];
        let timers = compute_objective_timers(&events, 10_000.0);
        assert_eq!(timers.next_dragon_seconds, 0.0);
    }

    #[test]
    fn baron_never_available_before_twenty_minutes_even_if_killed_early() {
        // Ne devrait pas arriver en jeu reel, mais protege contre une donnee
        // d'evenement aberrante.
        let events = vec![event("BaronKill", 100.0)];
        let timers = compute_objective_timers(&events, 200.0);
        assert_eq!(timers.next_baron_seconds, BARON_FIRST_SPAWN_SECONDS - 200.0);
    }

    #[test]
    fn herald_available_within_its_window() {
        let before = compute_objective_timers(&[], 400.0);
        assert!(!before.herald_available);

        let during = compute_objective_timers(&[], 600.0);
        assert!(during.herald_available);

        let after = compute_objective_timers(&[], 1300.0);
        assert!(!after.herald_available);
    }

    #[test]
    fn herald_unavailable_once_killed() {
        let events = vec![event("HeraldKill", 500.0)];
        let timers = compute_objective_timers(&events, 600.0);
        assert!(!timers.herald_available);
    }
}
