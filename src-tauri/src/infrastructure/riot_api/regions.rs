use serde::{Deserialize, Serialize};

/// Plateforme (serveur de jeu) au sens Riot API — utilisee pour les
/// endpoints summoner/league/champion-mastery/spectator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Na1,
    Euw1,
    Eun1,
    Kr,
    Jp1,
    Br1,
    La1,
    La2,
    Oc1,
    Tr1,
    Ru,
    Ph2,
    Sg2,
    Th2,
    Tw2,
    Vn2,
}

impl Platform {
    pub fn host(self) -> &'static str {
        match self {
            Platform::Na1 => "na1.api.riotgames.com",
            Platform::Euw1 => "euw1.api.riotgames.com",
            Platform::Eun1 => "eun1.api.riotgames.com",
            Platform::Kr => "kr.api.riotgames.com",
            Platform::Jp1 => "jp1.api.riotgames.com",
            Platform::Br1 => "br1.api.riotgames.com",
            Platform::La1 => "la1.api.riotgames.com",
            Platform::La2 => "la2.api.riotgames.com",
            Platform::Oc1 => "oc1.api.riotgames.com",
            Platform::Tr1 => "tr1.api.riotgames.com",
            Platform::Ru => "ru.api.riotgames.com",
            Platform::Ph2 => "ph2.api.riotgames.com",
            Platform::Sg2 => "sg2.api.riotgames.com",
            Platform::Th2 => "th2.api.riotgames.com",
            Platform::Tw2 => "tw2.api.riotgames.com",
            Platform::Vn2 => "vn2.api.riotgames.com",
        }
    }

    /// Route regionale associee (utilisee par account-v1 et match-v5).
    pub fn regional_route(self) -> RegionalRoute {
        match self {
            Platform::Na1 | Platform::Br1 | Platform::La1 | Platform::La2 | Platform::Oc1 => {
                RegionalRoute::Americas
            }
            Platform::Euw1 | Platform::Eun1 | Platform::Tr1 | Platform::Ru => RegionalRoute::Europe,
            Platform::Kr | Platform::Jp1 => RegionalRoute::Asia,
            Platform::Ph2 | Platform::Sg2 | Platform::Th2 | Platform::Tw2 | Platform::Vn2 => {
                RegionalRoute::Sea
            }
        }
    }

    pub fn from_str_loose(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "na1" => Some(Platform::Na1),
            "euw1" => Some(Platform::Euw1),
            "eun1" => Some(Platform::Eun1),
            "kr" => Some(Platform::Kr),
            "jp1" => Some(Platform::Jp1),
            "br1" => Some(Platform::Br1),
            "la1" => Some(Platform::La1),
            "la2" => Some(Platform::La2),
            "oc1" => Some(Platform::Oc1),
            "tr1" => Some(Platform::Tr1),
            "ru" => Some(Platform::Ru),
            "ph2" => Some(Platform::Ph2),
            "sg2" => Some(Platform::Sg2),
            "th2" => Some(Platform::Th2),
            "tw2" => Some(Platform::Tw2),
            "vn2" => Some(Platform::Vn2),
            _ => None,
        }
    }
}

/// Route regionale (regroupement continental) utilisee par les endpoints
/// account-v1 et match-v5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegionalRoute {
    Americas,
    Europe,
    Asia,
    Sea,
}

impl RegionalRoute {
    pub fn host(self) -> &'static str {
        match self {
            RegionalRoute::Americas => "americas.api.riotgames.com",
            RegionalRoute::Europe => "europe.api.riotgames.com",
            RegionalRoute::Asia => "asia.api.riotgames.com",
            RegionalRoute::Sea => "sea.api.riotgames.com",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_platform_to_expected_regional_route() {
        assert_eq!(Platform::Euw1.regional_route(), RegionalRoute::Europe);
        assert_eq!(Platform::Na1.regional_route(), RegionalRoute::Americas);
        assert_eq!(Platform::Kr.regional_route(), RegionalRoute::Asia);
        assert_eq!(Platform::Sg2.regional_route(), RegionalRoute::Sea);
    }

    #[test]
    fn parses_platform_case_insensitively() {
        assert_eq!(Platform::from_str_loose("EUW1"), Some(Platform::Euw1));
        assert_eq!(Platform::from_str_loose("kr"), Some(Platform::Kr));
        assert_eq!(Platform::from_str_loose("atlantis"), None);
    }
}
