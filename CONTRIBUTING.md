# Contribuer à Wardstone

## Mise en place

```bash
npm install
npm run tauri dev
```

Prérequis : Node.js ≥ 20, Rust stable ≥ 1.77, [prérequis Tauri v2](https://v2.tauri.app/start/prerequisites/).

## Structure du projet

Voir [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

- Frontend : `src/features/<feature>` — un module par fonctionnalité (composants, hooks,
  store local, types). Le code partagé va dans `src/shared`.
- Backend : `src-tauri/src` — Clean Architecture (`domain` → `application` → `infrastructure`),
  commandes Tauri fines dans `commands/`.

## Style de code

- TypeScript : ESLint + Prettier (`npm run lint`, `npm run format`).
- Rust : `cargo fmt` + `cargo clippy -- -D warnings`.
- Les hooks Husky (`pre-commit`) lancent lint-staged automatiquement ; ne pas contourner
  avec `--no-verify` sauf urgence documentée dans le message de commit.

## Tests

```bash
npm run test          # Vitest (frontend)
cargo test             # depuis src-tauri, tests unitaires + intégration Rust
npm run test:e2e       # Playwright (lance automatiquement `npm run dev` ; frontend seul, hors shell Tauri)
```

Toute nouvelle fonctionnalité doit être accompagnée de tests unitaires couvrant au minimum
la logique métier (application/domain côté Rust, hooks/logique côté frontend).

## Commits

Convention [Conventional Commits](https://www.conventionalcommits.org/) :

```
feat(champion-select): recommandation de runes par elo
fix(lcu-connector): reconnexion après veille du client
refactor(cache): factoriser le TTL cache générique
test(riot-api): couverture du rate limiter
docs(readme): mise à jour de la stack technique
chore(ci): ajout du job clippy
```

Scope conseillé : nom du module/feature concerné (`auth`, `api`, `champion-select`,
`profile`, `database`, `overlay`, `cache`, `stats-engine`...).

## Conformité Riot

Toute contribution qui automatise des actions in-game, contourne l'authentification du
client, scrape un site tiers, ou viole les
[CGU de l'API Riot Games](https://developer.riotgames.com/policies/general) sera refusée.

## Pull Requests

- Une PR = un sujet cohérent (une feature, un fix, un refactor).
- Mettre à jour `docs/ROADMAP.md` si le statut d'une tâche change.
- CI (lint + tests + build) doit passer avant merge.
