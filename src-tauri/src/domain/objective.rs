use serde::{Deserialize, Serialize};

use super::mmr_estimate;

/// Type d'objectif personnel defini par l'utilisateur. Serialise en JSON
/// dans `objectives.target_json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ObjectiveKind {
    ReachRank {
        queue_type: String,
        tier: String,
        rank: String,
    },
    WinrateTarget {
        queue_type: String,
        percent: f64,
        min_games: i64,
    },
    GamesPlayed {
        count: i64,
    },
}

/// Etat courant necessaire pour evaluer la progression d'un objectif.
/// Rassemble ce que les commandes savent deja (profil courant, historique
/// local) sans que le domaine n'ait besoin de connaitre l'API Riot.
pub struct ObjectiveContext {
    pub current_tier: Option<String>,
    pub current_rank: Option<String>,
    pub current_league_points: i64,
    pub current_wins: i64,
    pub current_losses: i64,
    pub games_played_since_creation: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveProgress {
    pub percent: f64,
    pub achieved: bool,
    pub summary: String,
}

impl ObjectiveKind {
    pub fn tag(&self) -> &'static str {
        match self {
            ObjectiveKind::ReachRank { .. } => "reachRank",
            ObjectiveKind::WinrateTarget { .. } => "winrateTarget",
            ObjectiveKind::GamesPlayed { .. } => "gamesPlayed",
        }
    }
}

pub fn evaluate_progress(kind: &ObjectiveKind, context: &ObjectiveContext) -> ObjectiveProgress {
    match kind {
        ObjectiveKind::ReachRank { tier, rank, .. } => evaluate_reach_rank(tier, rank, context),
        ObjectiveKind::WinrateTarget {
            percent, min_games, ..
        } => evaluate_winrate_target(*percent, *min_games, context),
        ObjectiveKind::GamesPlayed { count } => evaluate_games_played(*count, context),
    }
}

fn evaluate_reach_rank(
    target_tier: &str,
    target_rank: &str,
    context: &ObjectiveContext,
) -> ObjectiveProgress {
    let target_score = mmr_estimate::estimate_mmr(target_tier, target_rank, 0).unwrap_or(0);

    let current_score = match (&context.current_tier, &context.current_rank) {
        (Some(tier), Some(rank)) => {
            mmr_estimate::estimate_mmr(tier, rank, context.current_league_points).unwrap_or(0)
        }
        _ => 0,
    };

    let achieved = target_score > 0 && current_score >= target_score;
    let percent = if target_score <= 0 {
        0.0
    } else {
        ((current_score as f64 / target_score as f64) * 100.0).clamp(0.0, 100.0)
    };

    let summary = match (&context.current_tier, &context.current_rank) {
        (Some(tier), Some(rank)) => format!("Rang actuel : {tier} {rank}"),
        _ => "Non classé pour l'instant".to_string(),
    };

    ObjectiveProgress {
        percent,
        achieved,
        summary,
    }
}

fn evaluate_winrate_target(
    target_percent: f64,
    min_games: i64,
    context: &ObjectiveContext,
) -> ObjectiveProgress {
    let games = context.current_wins + context.current_losses;

    if games < min_games {
        let percent = if min_games <= 0 {
            100.0
        } else {
            (games as f64 / min_games as f64) * 100.0
        };
        return ObjectiveProgress {
            percent: percent.min(100.0),
            achieved: false,
            summary: format!(
                "{games}/{min_games} partie(s) requise(s) avant évaluation du winrate"
            ),
        };
    }

    let winrate = if games > 0 {
        (context.current_wins as f64 / games as f64) * 100.0
    } else {
        0.0
    };
    let achieved = winrate >= target_percent;
    let percent = if target_percent <= 0.0 {
        100.0
    } else {
        (winrate / target_percent * 100.0).min(100.0)
    };

    ObjectiveProgress {
        percent,
        achieved,
        summary: format!("Winrate actuel : {:.1}% sur {games} partie(s)", winrate),
    }
}

fn evaluate_games_played(target_count: i64, context: &ObjectiveContext) -> ObjectiveProgress {
    let played = context.games_played_since_creation;
    let achieved = target_count > 0 && played >= target_count;
    let percent = if target_count <= 0 {
        100.0
    } else {
        (played as f64 / target_count as f64 * 100.0).min(100.0)
    };

    ObjectiveProgress {
        percent,
        achieved,
        summary: format!("{played}/{target_count} partie(s) jouée(s)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(
        tier: Option<&str>,
        rank: Option<&str>,
        lp: i64,
        wins: i64,
        losses: i64,
    ) -> ObjectiveContext {
        ObjectiveContext {
            current_tier: tier.map(String::from),
            current_rank: rank.map(String::from),
            current_league_points: lp,
            current_wins: wins,
            current_losses: losses,
            games_played_since_creation: 0,
        }
    }

    #[test]
    fn reach_rank_not_achieved_when_below_target() {
        let kind = ObjectiveKind::ReachRank {
            queue_type: "RANKED_SOLO_5x5".to_string(),
            tier: "GOLD".to_string(),
            rank: "IV".to_string(),
        };
        let progress = evaluate_progress(&kind, &context(Some("SILVER"), Some("I"), 20, 5, 5));
        assert!(!progress.achieved);
        assert!(progress.percent < 100.0);
    }

    #[test]
    fn reach_rank_achieved_when_at_or_above_target() {
        let kind = ObjectiveKind::ReachRank {
            queue_type: "RANKED_SOLO_5x5".to_string(),
            tier: "GOLD".to_string(),
            rank: "IV".to_string(),
        };
        let progress = evaluate_progress(&kind, &context(Some("GOLD"), Some("II"), 10, 5, 5));
        assert!(progress.achieved);
        assert_eq!(progress.percent, 100.0);
    }

    #[test]
    fn winrate_target_requires_minimum_games_first() {
        let kind = ObjectiveKind::WinrateTarget {
            queue_type: "RANKED_SOLO_5x5".to_string(),
            percent: 60.0,
            min_games: 20,
        };
        let progress = evaluate_progress(&kind, &context(Some("GOLD"), Some("IV"), 10, 5, 3));
        assert!(!progress.achieved);
        assert!(progress.summary.contains("requise"));
    }

    #[test]
    fn winrate_target_achieved_once_threshold_met() {
        let kind = ObjectiveKind::WinrateTarget {
            queue_type: "RANKED_SOLO_5x5".to_string(),
            percent: 60.0,
            min_games: 10,
        };
        let progress = evaluate_progress(&kind, &context(Some("GOLD"), Some("IV"), 10, 13, 7));
        assert!(progress.achieved);
    }

    #[test]
    fn games_played_progress_scales_linearly() {
        let kind = ObjectiveKind::GamesPlayed { count: 50 };
        let mut ctx = context(None, None, 0, 0, 0);
        ctx.games_played_since_creation = 25;
        let progress = evaluate_progress(&kind, &ctx);
        assert_eq!(progress.percent, 50.0);
        assert!(!progress.achieved);
    }
}
