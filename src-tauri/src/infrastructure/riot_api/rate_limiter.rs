use std::time::{Duration, Instant};

use tokio::sync::Mutex;

/// Une fenetre de rate-limit Riot (ex: 20 requetes / 1s, 100 requetes / 2min).
#[derive(Debug, Clone, Copy)]
pub struct RateWindow {
    pub max_requests: usize,
    pub window: Duration,
}

impl RateWindow {
    pub const fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }
}

/// Limiteur de debit respectant plusieurs fenetres simultanement (limites
/// Riot par defaut d'une cle de developpement : 20 req/1s ET 100 req/2min).
/// Bloque (sleep) plutot que de rejeter, ce qui suffit pour un usage client
/// desktop mono-utilisateur.
pub struct RateLimiter {
    windows: Vec<RateWindow>,
    history: Mutex<Vec<Instant>>,
}

impl RateLimiter {
    pub fn new(windows: Vec<RateWindow>) -> Self {
        Self {
            windows,
            history: Mutex::new(Vec::new()),
        }
    }

    /// Limites par defaut d'une cle de developpement Riot Games.
    pub fn development_defaults() -> Self {
        Self::new(vec![
            RateWindow::new(20, Duration::from_secs(1)),
            RateWindow::new(100, Duration::from_secs(120)),
        ])
    }

    /// Attend, si necessaire, qu'un emplacement soit disponible dans toutes
    /// les fenetres, puis enregistre la requete.
    pub async fn acquire(&self) {
        loop {
            let wait = {
                let mut history = self.history.lock().await;
                let now = Instant::now();

                let longest_window = self
                    .windows
                    .iter()
                    .map(|w| w.window)
                    .max()
                    .unwrap_or_default();
                history.retain(|instant| now.duration_since(*instant) < longest_window);

                let mut required_wait: Option<Duration> = None;
                for rate_window in &self.windows {
                    let count_in_window = history
                        .iter()
                        .filter(|instant| now.duration_since(**instant) < rate_window.window)
                        .count();

                    if count_in_window >= rate_window.max_requests {
                        if let Some(oldest) = history
                            .iter()
                            .filter(|instant| now.duration_since(**instant) < rate_window.window)
                            .min()
                        {
                            let remaining = rate_window.window - now.duration_since(*oldest);
                            required_wait =
                                Some(required_wait.map_or(remaining, |w| w.max(remaining)));
                        }
                    }
                }

                if required_wait.is_none() {
                    history.push(now);
                }
                required_wait
            };

            match wait {
                Some(duration) => tokio::time::sleep(duration).await,
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn allows_requests_within_the_limit_without_delay() {
        let limiter = RateLimiter::new(vec![RateWindow::new(5, Duration::from_millis(50))]);

        let start = Instant::now();
        for _ in 0..5 {
            limiter.acquire().await;
        }
        assert!(start.elapsed() < Duration::from_millis(30));
    }

    #[tokio::test]
    async fn delays_requests_beyond_the_limit() {
        let limiter = RateLimiter::new(vec![RateWindow::new(2, Duration::from_millis(100))]);

        limiter.acquire().await;
        limiter.acquire().await;

        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() >= Duration::from_millis(80));
    }
}
