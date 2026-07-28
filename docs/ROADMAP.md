# Roadmap

Découpage Epic → Feature → Tâche, avec statut. Légende : ✅ Livré · 🔄 En cours · ⏳ Planifié.

Ce document est mis à jour à chaque phase de développement, en même temps que le code —
un statut ✅ ne doit être posé qu'après implémentation réelle et vérifiée.

## Epic 0 — Fondations & qualité

| Feature         | Tâche                                                                                    | Statut                                                                     |
| --------------- | ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Scaffold projet | Structure monorepo (`src`, `src-tauri`, `docs`, `assets`, `scripts`, `tests`, `.github`) | ✅                                                                         |
| Scaffold projet | Frontend Vite + React + TypeScript + Tailwind                                            | ✅                                                                         |
| Scaffold projet | Shell Tauri v2 (fenêtre principale + fenêtre overlay)                                    | ✅                                                                         |
| Qualité         | ESLint + Prettier + Husky + lint-staged                                                  | ✅                                                                         |
| Qualité         | Tests unitaires frontend (Vitest)                                                        | ✅ (couverture initiale, a etoffer au fil des features)                    |
| Qualité         | Tests unitaires backend (`cargo test`)                                                   | ✅ (couverture initiale, a etoffer au fil des features)                    |
| Qualité         | Tests E2E (Playwright)                                                                   | ⏳                                                                         |
| CI/CD           | Pipeline GitHub Actions (lint, test, build)                                              | ✅                                                                         |
| CI/CD           | Release automatisée (tauri-action, artefacts Windows/macOS/Linux)                        | ✅ (build en brouillon a chaque tag `v*`, a valider en conditions reelles) |
| Documentation   | README, ARCHITECTURE, ROADMAP, CONTRIBUTING                                              | ✅                                                                         |

## Epic 1 — Détection & état du client

| Feature          | Tâche                                                                           | Statut |
| ---------------- | ------------------------------------------------------------------------------- | ------ |
| Connecteur LCU   | Découverte du processus client (port + token, multi-plateforme)                 | ✅     |
| Connecteur LCU   | Client REST authentifié (TLS auto-signé)                                        | ✅     |
| Connecteur LCU   | Flux d'événements gameflow (WebSocket WAMP)                                     | ✅     |
| Connecteur LCU   | Fallback polling adaptatif si WebSocket indisponible                            | ✅     |
| Machine à états  | Modélisation des phases (Lobby → EndOfGame)                                     | ✅     |
| Frontend         | Adaptation automatique de l'UI selon la phase (badge de phase, sync temps réel) | ✅     |
| Live Client Data | Poller `localhost:2999` pendant la partie                                       | ✅     |

## Epic 2 — Comptes & profil

| Feature           | Tâche                                         | Statut                                                  |
| ----------------- | --------------------------------------------- | ------------------------------------------------------- |
| Onboarding        | Saisie Riot ID + région + clé API personnelle | ✅                                                      |
| Onboarding        | Validation via `account-v1`                   | ✅                                                      |
| Stockage sécurisé | Clé API dans le trousseau OS                  | ✅                                                      |
| Multi-comptes     | Ajout / suppression / bascule de compte       | ✅                                                      |
| Multi-comptes     | Synchronisation périodique des comptes liés   | ⏳                                                      |
| Profil            | Rang, LP, WR, niveau, icône                   | ✅ (icône : identifiant récupéré, rendu visuel à faire) |
| Profil            | Champion principal, statistiques agrégées     | ✅                                                      |
| Profil            | MMR estimé (heuristique documentée)           | ✅                                                      |

## Epic 3 — Champion Select Assistant

| Feature         | Tâche                                                          | Statut                                                                                  |
| --------------- | -------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Moteur de stats | Ingestion de matchs (`match-v5`) en tâche de fond              | ✅ (déclenchée manuellement depuis Historique pour l'instant)                           |
| Moteur de stats | Agrégation winrate/pickrate/banrate par champion/rôle/patch    | ✅ (bucket elo unique "toutes parties collectées" — voir backlog)                       |
| Moteur de stats | Agrégation runes (keystone)/items/sorts d'invocateur           | ✅                                                                                      |
| Moteur de stats | Ordre des compétences (skill order)                            | ❌ non disponible via l'API Riot publique (match-v5 n'expose pas le level-up des sorts) |
| Moteur de stats | Filtrage par elo, région                                       | ⏳ (nécessiterait un lookup de rang par participant, coûteux en rate-limit)             |
| Assistant       | Détection automatique de l'entrée en champion select           | ✅                                                                                      |
| Assistant       | Recommandations (runes/sorts/objets)                           | ✅                                                                                      |
| Assistant       | Contres, synergies, difficulté, astuces (fallback Data Dragon) | 🔄 (difficulté/tags/tips ✅, contres/synergies calculés ⏳)                             |
| Assistant       | Temps moyen de partie par champion                             | ✅                                                                                      |

## Epic 4 — Analyse d'équipe

| Feature          | Tâche                                                                      | Statut                                                                                      |
| ---------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| Scan des joueurs | Récupération des puuids via session LCU (alliés + adversaires si visibles) | ✅                                                                                          |
| Scan des joueurs | Rang / WR (saison) / maîtrises principales par joueur                      | ✅                                                                                          |
| Scan des joueurs | Forme récente (N dernières games) et pool de champions détaillé            | ⏳ (nécessiterait de synchroniser l'historique de chaque joueur, coûteux en rate-limit)     |
| Heuristiques     | Détection d'autofill estimée                                               | ❌ non implémenté (nécessiterait un modèle rôle-par-champion fiable, risque de désinformer) |
| Heuristiques     | Score de force (MMR estimé moyen par équipe)                               | ✅                                                                                          |
| Heuristiques     | Risque, menaces, avantages détaillés                                       | ⏳                                                                                          |
| Composition      | Répartition par tags de champion (Tank/Mage/Marksman...)                   | ✅                                                                                          |
| Composition      | Scaling early/mid/late, teamfight/splitpush, CC, frontline, AP/AD          | ⏳ (backlog — nécessite un modèle de composition plus riche)                                |

## Epic 5 — Partie en cours

| Feature | Tâche                                                                       | Statut                                                                                                           |
| ------- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Overlay | Fenêtre transparente, déplaçable, redimensionnable                          | ✅ (ouverture/fermeture automatique selon la phase InProgress)                                                   |
| Overlay | Timers objectifs (dragon/baron/héraut)                                      | ✅ (calculés depuis le flux d'événements Live Client Data + constantes documentées ; void grubs non implémentés) |
| Overlay | Or (donnée exacte, pas une estimation — fournie par l'API Live Client Data) | ✅                                                                                                               |
| Overlay | Power spikes détaillés par champion                                         | ⏳ (nécessiterait de croiser stats_engine et courbe de puissance par champion)                                   |
| Overlay | Conseils contextuels                                                        | ✅ (règles génériques liées au temps de jeu/objectifs, pas encore par champion)                                  |
| Overlay | Widgets activables/désactivables individuellement                           | ⏳                                                                                                               |

## Epic 6 — Historique & progression

| Feature    | Tâche                                          | Statut                      |
| ---------- | ---------------------------------------------- | --------------------------- |
| Historique | Liste des parties, filtres (champion/résultat) | ✅ (filtre date à ajouter)  |
| Historique | Recherche                                      | ✅ (recherche par champion) |
| Graphiques | Progression LP, progression WR                 | ⏳                          |
| Graphiques | Champion préféré, heatmap d'activité           | ⏳                          |
| Dashboard  | Résumé, dernières parties, top champions       | ✅                          |

## Epic 7 — Objectifs & coaching

| Feature   | Tâche                                                | Statut |
| --------- | ---------------------------------------------------- | ------ |
| Objectifs | Définition (rang cible, WR cible, nombre de parties) | ⏳     |
| Objectifs | Suivi automatique de la progression                  | ⏳     |
| Coaching  | Analyse post-partie automatique                      | ⏳     |
| Coaching  | Points forts / faibles / conseils / priorités        | ⏳     |

## Epic 8 — Comparaison & recherche

| Feature           | Tâche                                               | Statut |
| ----------------- | --------------------------------------------------- | ------ |
| Comparaison       | Joueurs, champions, builds, historiques             | ⏳     |
| Recherche globale | Invocateurs, champions, objets, runes, sorts, patch | ⏳     |

## Epic 9 — Personnalisation & notifications

| Feature       | Tâche                                                                      | Statut |
| ------------- | -------------------------------------------------------------------------- | ------ |
| Thèmes        | Dark, OLED, Light, couleurs d'accent                                       | ⏳     |
| Disposition   | Widgets, fenêtres, raccourcis clavier                                      | ⏳     |
| Notifications | Partie trouvée, champion select, patch, promo, victoire/défaite, objectifs | ⏳     |

## Backlog / améliorations futures

- Support multi-langue (i18n) de l'interface.
- Mode spectateur enrichi (spectator-v5) pour scouter en direct.
- Export des statistiques (CSV/JSON).
- Widgets overlay tiers-parties via SDK de plugin (architecture modulaire déjà prête).
- Portail de données partagées optionnel (opt-in, anonymisé) pour accélérer l'agrégation
  communautaire des statistiques de builds sans dépendre de services tiers.

## Pourquoi pas de scraping OP.GG/U.GG/Blitz/Mobalytics ?

Ces services n'exposent pas d'API publique stable et leurs CGU interdisent généralement le
scraping. Riot Games ne fournit pas non plus d'API d'agrégats communautaires (winrate/
pickrate/banrate par champion). Wardstone reconstruit ces statistiques **localement**, à
partir des matchs réellement consultés via l'API officielle `match-v5` (vos parties, celles
de votre historique, et celles des joueurs analysés en champion select). La couverture
grandit avec l'usage réel du logiciel — c'est un choix délibéré pour rester 100% conforme
aux CGU Riot, quitte à avoir une base de données plus modeste qu'un service tiers au tout
début.
