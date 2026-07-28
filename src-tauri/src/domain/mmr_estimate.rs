/// Riot Games ne publie pas de MMR interne. Cette heuristique produit un
/// score comparable (documente ici, jamais presente comme une valeur
/// officielle) a partir du rang classe : palier de tier + palier de
/// division + LP. Utilise uniquement pour comparer deux joueurs entre eux
/// dans l'analyse d'equipe et le profil (toujours affiche avec la mention
/// "estime" cote UI).
pub fn estimate_mmr(tier: &str, rank: &str, league_points: i64) -> Option<i64> {
    let tier_base = match tier.to_ascii_uppercase().as_str() {
        "IRON" => 0,
        "BRONZE" => 400,
        "SILVER" => 800,
        "GOLD" => 1200,
        "PLATINUM" => 1600,
        "EMERALD" => 2000,
        "DIAMOND" => 2400,
        "MASTER" | "GRANDMASTER" | "CHALLENGER" => 2800,
        _ => return None,
    };

    let rank_offset = match rank.to_ascii_uppercase().as_str() {
        "IV" => 0,
        "III" => 100,
        "II" => 200,
        "I" => 300,
        _ => 0,
    };

    Some(tier_base + rank_offset + league_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimates_higher_score_for_higher_tiers() {
        let iron = estimate_mmr("IRON", "IV", 0).unwrap();
        let gold = estimate_mmr("GOLD", "IV", 0).unwrap();
        let diamond = estimate_mmr("DIAMOND", "IV", 0).unwrap();
        assert!(iron < gold);
        assert!(gold < diamond);
    }

    #[test]
    fn division_and_league_points_increase_the_score_within_a_tier() {
        let low = estimate_mmr("GOLD", "IV", 0).unwrap();
        let higher_division = estimate_mmr("GOLD", "I", 0).unwrap();
        let higher_lp = estimate_mmr("GOLD", "IV", 50).unwrap();
        assert!(higher_division > low);
        assert!(higher_lp > low);
    }

    #[test]
    fn returns_none_for_an_unknown_tier() {
        assert_eq!(estimate_mmr("ATLANTIS", "IV", 0), None);
    }
}
