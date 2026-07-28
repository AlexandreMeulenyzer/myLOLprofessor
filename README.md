# Wardstone

**Wardstone** est un compagnon de bureau moderne, rapide et entièrement modulaire pour les
joueurs de League of Legends. Il s'inspire des usages popularisés par Porofessor, OP.GG
Desktop, Blitz, Mobalytics et U.GG, sans en reprendre l'interface ni les méthodes : toutes
les données proviennent de l'API officielle Riot Games, du Data Dragon et de Community
Dragon, ou sont calculées localement à partir de ces données.

> Statut : projet en développement actif. Voir [`docs/ROADMAP.md`](docs/ROADMAP.md) pour
> l'état détaillé de chaque fonctionnalité (livré / en cours / planifié).

## Pourquoi Wardstone ?

| | Porofessor / OP.GG / Blitz / Mobalytics / U.GG | Wardstone |
|---|---|---|
| Source des statistiques | Scraping / API propriétaires fermées | Riot API officielle + agrégation locale, 100% conforme aux CGU Riot |
| Overlay in-game | Fixe, peu personnalisable | Fenêtre flottante déplaçable, redimensionnable, modulaire par widgets |
| Interface | Dense, figée | Fluent Design / glassmorphism léger, thèmes, disposition libre |
| Détection du client | Souvent lente | Connecteur LCU temps réel (WebSocket + polling de secours) |
| Coaching | Générique | Comparaison aux références de rôle/elo calculées sur vos propres données |
| Vie privée | Données envoyées à des tiers | Base SQLite locale, clé API stockée dans le trousseau système |

## Fonctionnalités

- **Détection automatique** du client (lancement, connexion, lobby, file, champion select,
  chargement, partie en cours, fin de partie) via le connecteur LCU.
- **Profil** : rang, LP, winrate, historique, niveau, icône, champion principal, MMR estimé.
- **Assistant Champion Select** : runes, build, ordre des compétences, sorts, objets, contres,
  synergies, winrate/pickrate/banrate, difficulté, astuces — filtré par elo, région, patch.
- **Analyse d'équipe** en sélection : rang, OTP, autofill estimé, forme récente, score de
  force, composition (scaling, teamfight, splitpush, CC, frontline).
- **Overlay in-game** discret : timers d'objectifs (dragon/héraut/baron/void), or estimé,
  power spikes, conseils contextuels — alimenté par la Live Client Data API officielle.
- **Historique** filtrable, graphiques de progression LP/WR, pool de champions.
- **Dashboard** moderne avec résumé, objectifs et dernières parties.
- **Objectifs personnels** suivis automatiquement.
- **Coaching post-partie** basé sur des références calculées sur vos données.
- **Comparaison** joueurs / champions / builds / historiques.
- **Recherche globale** (invocateurs, champions, objets, runes, sorts, patch).
- **Multi-comptes** Riot avec synchronisation.
- **Personnalisation** complète (thèmes, widgets, disposition, raccourcis).
- **Notifications** natives (file trouvée, champion select, victoire/défaite, promotion...).

## Stack technique

| Domaine | Choix | Raison |
|---|---|---|
| Desktop shell | **Tauri v2** | Binaire natif léger (~10-20 Mo vs Electron), sandbox process séparé, WebView OS native |
| Frontend | React 18 + TypeScript + Vite | Écosystème mature, DX rapide, typage strict |
| UI | Tailwind CSS + design system maison | Cohérence visuelle, thèmes dark/OLED/light, glassmorphism léger |
| State client | Zustand | Store minimal, sans boilerplate, parfait pour état UI/overlay |
| Data fetching | TanStack Query | Cache, retry, invalidation, synchronisation avec le backend Rust |
| Backend local | **Rust** (dans `src-tauri`) | Perf, sécurité mémoire, faible empreinte CPU/RAM, accès natif au système (lockfile LCU) |
| Persistance | SQLite (`rusqlite` + migrations) | Zéro dépendance externe, requêtes locales rapides |
| API jeu | Riot Games API officielle (Account-v1, Summoner-v4/v5, League-v4, Match-v5,
Champion-Mastery-v4, Spectator-v5) | Seule source de données conforme aux CGU Riot |
| Données statiques | Data Dragon + Community Dragon | Champions, objets, runes, sorts, splash arts |
| Client local | League Client Update (LCU) API + Live Client Data API | Détection de phase, session de champion select, données in-game |

Wardstone **n'utilise jamais** de méthode contraire aux
[conditions d'utilisation de l'API Riot Games](https://developer.riotgames.com/policies/general) :
aucune automatisation du jeu, aucune donnée obtenue par scraping de sites tiers, aucun
contournement d'authentification du client.

## Architecture

Voir [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) pour le détail (Clean Architecture,
SOLID, DDD, feature-based, diagrammes Mermaid).

```
/src            Frontend React (feature-based)
/src-tauri      Backend Rust (Clean Architecture : domain / application / infrastructure)
/docs           Documentation technique, produit et diagrammes
/assets         Icônes, images, ressources de design
/scripts        Scripts de build, dev, release
/tests          Tests end-to-end (Playwright / WebDriver)
/.github        Workflows CI/CD
```

## Prérequis de développement

- Node.js ≥ 20, npm ≥ 10
- Rust stable ≥ 1.77 (toolchain gérée par `rustup`)
- Dépendances système Tauri v2 : voir la
  [doc officielle Tauri](https://v2.tauri.app/start/prerequisites/) selon votre OS
  (WebView2 sous Windows, cible de distribution principale de ce projet)

## Démarrage

```bash
npm install
npm run tauri dev      # lance l'app en mode développement (frontend + backend Rust)
npm run build           # build de production du frontend
npm run tauri build     # génère l'installeur Windows (.msi / .exe NSIS)
```

## Configuration d'une clé API Riot

Wardstone est un logiciel **client-side** : chaque utilisateur doit renseigner sa propre
clé API Riot Games (obtenue gratuitement sur le
[Riot Developer Portal](https://developer.riotgames.com/)) lors du premier lancement. La
clé est chiffrée et stockée dans le trousseau sécurisé du système d'exploitation (jamais en
clair, jamais journalisée, jamais transmise à un serveur tiers).

## Qualité & CI

- Lint/format : ESLint, Prettier, Clippy, rustfmt
- Tests : Vitest (frontend), `cargo test` (backend), Playwright (E2E)
- Hooks pre-commit : Husky + lint-staged
- Intégration continue : GitHub Actions (lint, tests, build multi-plateforme)
- Release : build et publication automatisés via `tauri-action`

## Documentation

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — architecture technique détaillée
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — découpage Epics / Features / Tâches et statut
- [`docs/API.md`](docs/API.md) — commandes Tauri exposées au frontend
- [`docs/USER_GUIDE.md`](docs/USER_GUIDE.md) — guide utilisateur
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — guide de contribution

## Licence

MIT — voir [`LICENSE`](LICENSE). Wardstone n'est pas approuvé par ni affilié à Riot Games.
League of Legends est une marque de Riot Games, Inc.
