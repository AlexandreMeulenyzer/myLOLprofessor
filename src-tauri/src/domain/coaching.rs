use serde::Serialize;

use crate::stats_engine::ChampionRoleStats;

/// Statistiques d'un participant pour une partie donnee (extraites de
/// `MatchParticipant`), independantes de l'API Riot pour rester testables.
pub struct MatchPerformance {
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub cs_per_min: f64,
    pub gold_per_min: f64,
    pub win: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoachingReport {
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub tips: Vec<String>,
}

/// Marge au-dela de laquelle un ecart par rapport a la reference locale
/// (stats_engine) est considere significatif plutot que du bruit normal.
const SIGNIFICANT_MARGIN: f64 = 0.15;

/// Genere un rapport de coaching en comparant la performance d'une partie a
/// la reference locale du meme champion/role/patch (calculee par
/// `stats_engine`, elle-meme issue des matchs deja synchronises — voir
/// docs/ROADMAP.md). Sans reference disponible, retourne des conseils
/// generiques bases sur des seuils absolus prudents plutot que de
/// pretendre a une comparaison precise.
pub fn generate_report(
    performance: &MatchPerformance,
    baseline: Option<&ChampionRoleStats>,
) -> CoachingReport {
    let mut strengths = Vec::new();
    let mut weaknesses = Vec::new();

    let own_kda_ratio = kda_ratio(performance.kills, performance.deaths, performance.assists);

    match baseline {
        Some(baseline) => {
            let baseline_kda_ratio = kda_ratio(
                baseline.avg_kills.round() as i64,
                baseline.avg_deaths.round() as i64,
                baseline.avg_assists.round() as i64,
            );

            compare_metric(
                own_kda_ratio,
                baseline_kda_ratio,
                "Ratio KDA nettement supérieur à votre moyenne sur ce champion.",
                "Ratio KDA en retrait par rapport à votre moyenne sur ce champion : soignez le positionnement en combat.",
                &mut strengths,
                &mut weaknesses,
            );

            compare_metric(
                performance.cs_per_min,
                baseline.avg_cs_per_min,
                "Farm au-dessus de votre moyenne habituelle sur ce champion.",
                "Farm en retrait par rapport à votre moyenne : privilégiez le last-hit en early game.",
                &mut strengths,
                &mut weaknesses,
            );

            compare_metric(
                performance.gold_per_min,
                baseline.avg_gold_per_min,
                "Génération d'or supérieure à votre moyenne (farm + kills/objectifs efficaces).",
                "Génération d'or en retrait : cherchez plus d'opportunités de farm ou d'objectifs.",
                &mut strengths,
                &mut weaknesses,
            );
        }
        None => {
            if performance.cs_per_min >= 7.0 {
                strengths.push("Bon rythme de farm (≥7 CS/min).".to_string());
            } else if performance.cs_per_min < 5.0 {
                weaknesses.push(
                    "Farm faible (<5 CS/min) : priorisez le last-hit en laning phase.".to_string(),
                );
            }

            if performance.deaths >= 8 {
                weaknesses.push(
                    "Nombre de morts élevé : revoyez le positionnement et la prise de risque."
                        .to_string(),
                );
            }
        }
    }

    if weaknesses.is_empty() && strengths.is_empty() {
        strengths.push(
            "Performance globalement dans la moyenne, rien de notable à signaler.".to_string(),
        );
    }

    let mut tips: Vec<String> = weaknesses
        .iter()
        .map(|weakness| weakness_to_tip(weakness))
        .collect();

    if !performance.win && weaknesses.is_empty() {
        tips.push(
            "Défaite malgré une performance individuelle correcte : la partie s'est probablement jouée sur la macro ou le jeu d'équipe plutôt que sur votre lane."
                .to_string(),
        );
    }

    if tips.is_empty() {
        tips.push(
            "Continuez sur cette lancée : aucune faiblesse majeure détectée cette partie."
                .to_string(),
        );
    }

    CoachingReport {
        strengths,
        weaknesses,
        tips,
    }
}

fn kda_ratio(kills: i64, deaths: i64, assists: i64) -> f64 {
    (kills + assists) as f64 / deaths.max(1) as f64
}

fn compare_metric(
    own: f64,
    baseline: f64,
    strength_message: &str,
    weakness_message: &str,
    strengths: &mut Vec<String>,
    weaknesses: &mut Vec<String>,
) {
    if baseline <= 0.0 {
        return;
    }

    let ratio = own / baseline;
    if ratio >= 1.0 + SIGNIFICANT_MARGIN {
        strengths.push(strength_message.to_string());
    } else if ratio <= 1.0 - SIGNIFICANT_MARGIN {
        weaknesses.push(weakness_message.to_string());
    }
}

fn weakness_to_tip(weakness: &str) -> String {
    // Les messages de faiblesse contiennent deja un conseil actionnable
    // apres le ":" — reutilise tel quel pour eviter la duplication.
    weakness
        .split_once(':')
        .map(|(_, tip)| tip.trim().to_string())
        .unwrap_or_else(|| weakness.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline() -> ChampionRoleStats {
        ChampionRoleStats {
            champion_id: 103,
            role: "MIDDLE".to_string(),
            patch: "14.1".to_string(),
            games: 10,
            wins: 6,
            winrate_percent: 60.0,
            pickrate_percent: Some(20.0),
            banrate_percent: Some(5.0),
            avg_kills: 6.0,
            avg_deaths: 4.0,
            avg_assists: 6.0,
            avg_game_duration_seconds: 1800.0,
            avg_cs_per_min: 7.0,
            avg_gold_per_min: 400.0,
            common_items: vec![],
            common_summoner_spells: vec![],
            common_keystones: vec![],
        }
    }

    #[test]
    fn flags_strength_when_significantly_above_baseline() {
        let performance = MatchPerformance {
            kills: 12,
            deaths: 2,
            assists: 8,
            cs_per_min: 9.0,
            gold_per_min: 500.0,
            win: true,
        };
        let report = generate_report(&performance, Some(&baseline()));
        assert!(!report.strengths.is_empty());
        assert!(report.weaknesses.is_empty());
    }

    #[test]
    fn flags_weakness_when_significantly_below_baseline() {
        let performance = MatchPerformance {
            kills: 1,
            deaths: 9,
            assists: 2,
            cs_per_min: 4.0,
            gold_per_min: 250.0,
            win: false,
        };
        let report = generate_report(&performance, Some(&baseline()));
        assert!(!report.weaknesses.is_empty());
        assert!(!report.tips.is_empty());
    }

    #[test]
    fn falls_back_to_generic_thresholds_without_baseline() {
        let performance = MatchPerformance {
            kills: 3,
            deaths: 9,
            assists: 4,
            cs_per_min: 4.0,
            gold_per_min: 250.0,
            win: false,
        };
        let report = generate_report(&performance, None);
        assert!(report.weaknesses.iter().any(|w| w.contains("Farm")));
        assert!(report.weaknesses.iter().any(|w| w.contains("morts")));
    }

    #[test]
    fn never_returns_empty_summaries() {
        let performance = MatchPerformance {
            kills: 6,
            deaths: 4,
            assists: 6,
            cs_per_min: 7.0,
            gold_per_min: 400.0,
            win: true,
        };
        let report = generate_report(&performance, Some(&baseline()));
        assert!(!report.strengths.is_empty());
        assert!(!report.tips.is_empty());
    }
}
