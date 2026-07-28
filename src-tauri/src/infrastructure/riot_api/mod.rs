pub mod client;
pub mod dto;
pub mod rate_limiter;
pub mod regions;

#[allow(unused_imports)] // surface d'API complete du module ; RiotApiError et
// RegionalRoute seront consommes cote frontend/commands des les Epics 4/5.
pub use client::{RiotApiClient, RiotApiError};
#[allow(unused_imports)]
pub use regions::{Platform, RegionalRoute};
