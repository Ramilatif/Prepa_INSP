# Appli de préparation au concours externe de l'INSP

Application web de révision et d'entraînement aux écrits du concours externe de l'INSP
(voie générale), pour moi et quelques amis. Oraux prévus plus tard, hors périmètre actuel.

## Documents de référence

- Instructions complètes du projet : @PROMPT.md
- Avancement, décisions et points à vérifier : @PROGRESS.md
- Architecture validée : @docs/ARCHITECTURE.md
- Rapport de vérification des fiches : docs/VERIFICATION.md

En cas de contradiction : PROMPT.md fait foi, puis ARCHITECTURE.md.

## Stack

- Backend : Rust (Axum), sqlx, SQLite (requêtes compatibles PostgreSQL)
- Frontend : React + TypeScript, responsive
- Correction IA : API Claude via l'API Batch, côté serveur uniquement
- Environnement : Docker Compose, poste de dev sous Arch Linux

## Structure

- backend/ : API Rust (dont la tâche de fond de suivi des lots de correction)
- backend/migrations/ : migrations sqlx
- frontend/ : application React
- content/sessions/ : configuration par session (épreuves, coefficients, thèmes, dates)
- content/fiches/ : fiches de cours (Markdown + métadonnées)
- content/annales/ : sujets et rapports du jury, avec métadonnées
- content/grilles/ : grilles d'évaluation par épreuve
- references/catalogue.json : catalogue de la bibliographie INSP et des documents déposés
- references/officiels/ : publications officielles en accès libre
- references/ouvrages/ : mes ouvrages en PDF (exclu de git)
- references/index/ : texte extrait et index (exclu de git)
- tools/references/ : scripts d'extraction et d'indexation
- docs/ : documentation technique

## Commandes

- Lancer l'appli : docker compose up --build
- Tests backend : cd backend && cargo test
- Lint backend : cd backend && cargo clippy -- -D warnings && cargo fmt --check
- Tests frontend : cd frontend && npm test
- Lint frontend : cd frontend && npm run lint
- Nouvelle migration : cd backend && sqlx migrate add <nom>
- Extraction et indexation des références : à documenter dans tools/references/README.md

## Règles de travail

- Exécution autonome : enchaîner les phases sans attendre de validation,
  sauf blocage réel (accès ou installation système imprévu, erreur insoluble)
- Ne rien installer au niveau système et ne rien modifier hors du dossier du projet
- Une phase n'est terminée que si tout compile, les lints passent et les tests sont verts
- Au moins un commit par phase, messages clairs (ex. "phase 4: correction IA différée")
- Mettre à jour PROGRESS.md à la fin de chaque phase et à chaque décision non prévue par PROMPT.md
- Aucun secret en dur : clé API et tarifs dans .env (exclu de git), .env.example versionné

## Règles sur les fiches

- Rédaction uniquement à partir des sources autorisées, en respectant la hiérarchie
  (textes et décisions > statistiques officielles > rapports > centres de recherche pour les débats)
- Jamais de rédaction de mémoire pour ce qui évolue vite : vérifier en ligne
- Chaque affirmation importante a une référence précise et datée
- Vérification croisée avec references/officiels/ et references/ouvrages/ ; désaccords
  consignés dans docs/VERIFICATION.md, jamais tranchés seul sauf évolution datée des textes
- Une affirmation sans appui est retirée
- Le texte des ouvrages n'est jamais recopié ni reformulé de près, ni envoyé à l'API par l'appli
- Statut de chaque fiche : brouillon, sources officielles vérifiées, concordance avec
  les ouvrages vérifiée, relue par un humain
- Relecture critique séparée après chaque lot

## Règles sur la correction IA

- Uniquement en différé via l'API Batch ; pas de correction immédiate
- Opus 5.5 par défaut (configurable), réflexion approfondie, deux passes
- Plafond de budget par utilisateur vérifié avant chaque envoi
- Les lots en cours doivent survivre à un redémarrage du serveur
- Note toujours présentée comme indicative

## Règles générales

- Tout ce qui dépend de la session est en données dans content/sessions/, jamais en dur
- Annales de l'ancien format clairement distinguées du format réformé
- Ne jamais télécharger un ouvrage payant depuis une source non officielle

## Modules à garder interchangeables

- Authentification (pseudo aujourd'hui, mot de passe avant la mise en ligne)
- Client de l'API Claude (modèle et tarifs configurables)
- Sources de veille (ajout ou retrait d'un flux sans toucher au reste)

## Reprise après interruption

En début de session, si le projet est déjà entamé :

1. Lire PROGRESS.md
2. Vérifier l'état du dépôt (git log --oneline -10, git status)
3. Terminer ou annuler proprement tout travail laissé à moitié
4. Reprendre à la phase indiquée dans PROGRESS.md
