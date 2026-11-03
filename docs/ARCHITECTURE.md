# Architecture

Document de référence technique. En cas de contradiction : `PROMPT.md` fait foi, puis ce document.

## 1. Sources officielles du format de session

- Arrêté du 21 mars 2023 modifié relatif aux épreuves des concours d'entrée à l'INSP (JORF du 22 mars 2023).
- Page officielle du concours externe : https://insp.gouv.fr/concours-externe (consultée le 2026-10-30).
  Épreuves, durées, coefficients, thèmes 2026-2027 et calendrier 2027 y sont recoupés.
- Le texte consolidé Légifrance n'a pas pu être lu automatiquement : le programme détaillé
  reste à confirmer (voir PROGRESS.md, points à vérifier).

## 2. Arborescence

Voir `CLAUDE.md` (section Structure). Principe : le code (`backend/`, `frontend/`) ne contient
aucune donnée de concours ; tout vient de `content/` (importé en base au démarrage) ou de `.env`.

## 3. Backend (Rust, Axum, sqlx)

Modules : `auth` (isolé, trait `Authenticator`, implémentation pseudo), `session_config`,
`fiches`, `flashcards` (SM-2 pur dans `srs.rs`), `references`, `annales`, `copies`,
`correction` (client Claude derrière un trait, `batch.rs` pour le suivi des lots),
`budget`, `veille`, `planning`, `signalements`. Une tâche de fond tokio démarre avec le serveur.

SQL : types portables uniquement (TEXT, INTEGER, REAL, horodatages en TEXT ISO 8601 UTC,
identifiants TEXT UUID, booléens INTEGER 0/1), pas de syntaxe propre à SQLite hors migrations.

## 4. Schéma de base de données (cible, par migrations)

- `users(id, pseudo UNIQUE, majeure, mineure, active_session_id, created_at)`
- `sessions(id, label, config_json, active)` : importé depuis `content/sessions/*.json`
- `fiches(id, slug UNIQUE, titre, matiere, epreuves_json, updated_at, statut, sources_json, body_md)`
- `flashcards(id, fiche_id, type, recto, verso, source_ref)`
- `card_states(user_id, card_id, ease, interval_days, repetitions, due_at, last_reviewed)`
- `signalements(id, user_id, cible_type, cible_id, message, created_at, status)`
- `references_docs(id, titre, auteurs, editeur, edition, annee, matiere, acces, present, chemin)`
- `annales(id, annee, concours, epreuve, format, url_source, chemin)`
- `grilles(id, epreuve_code, session_id, criteres_json)`
- `copies(id, user_id, sujet_id, mode, statut, pdf_path, created_at)`
- `corrections(id, copie_id, passe, batch_id, statut, resultat_json, note, tokens_json, cout_eur)`
- `ai_usage(id, user_id, mois, cout_eur)` : le budget restant se calcule par somme du mois
- `notifications(id, user_id, message, lu, created_at)`
- `veille_items(id, source, titre, url UNIQUE, date, resume, categorie)` et `veille_liens(item_id, fiche_id)`
- `plans(id, user_id, config_json, generated_at)`, `work_log(id, user_id, kind, minutes, at)`

## 5. Endpoints (REST JSON, préfixe `/api`)

`POST /login`, `GET/PUT /me`, `GET /sessions`, `PUT /me/session`, `GET /fiches`, `GET /fiches/:slug`,
`GET /flashcards/due`, `POST /flashcards/:id/review`, `POST /signalements`, `GET /signalements`,
`GET /references`, `GET /annales`, `GET /grilles/:epreuve`, `POST /copies`, `GET /copies`,
`POST /copies/:id/estimate`, `POST /copies/:id/correct`, `GET /budget`, `GET /notifications`,
`GET /veille`, `GET/POST /planning`, `GET /dashboard`.

## 6. Formats

- Fiche : fichier Markdown dans `content/fiches/`, front matter YAML : `slug, titre, matiere,
  epreuves, maj, statut, sources[]` ; sections obligatoires : notions clés, enjeux d'action publique,
  chiffres clés datés, débats et positions, références, pour aller plus loin, flashcards liées.
- Session : JSON dans `content/sessions/` (voir `2027.json`).
- Grille : JSON dans `content/grilles/` : critères, points, éléments attendus, erreurs fréquentes, source (rapport du jury).

### Cours et fiches

- Deux types de contenu, même fichier Markdown et même pipeline d'import (`content/fiches/`) : la **fiche**
  (synthèse à réviser, flashcards) et le **cours** (chapitre développé, `type: cours`).
- Front matter additionnel pour un cours : `type: cours`, `parcours` (ex. `droit-public`), `domaine` (ex. `droit-constitutionnel`,
  `droit-administratif`, `libertes-fondamentales`, `droit-union-europeenne`), `ordre` (rang dans le domaine).
  Colonnes `type`, `parcours`, `ordre` (migration 0004) et `domaine` (migration 0005) dans `fiches`.
  La page « Cours » les range par parcours, puis par domaine (ordre déclaré dans `frontend/src/labels.ts`). `GET /api/fiches?type=cours|fiche`.
- Structure d'un cours : encadré « à quoi sert ce chapitre », problématique, plan annoncé, parties développées
  avec exemples et tableaux, application à l'épreuve (méthode, pièges, ce que le jury sanctionne), « pour réviser »,
  « pour aller plus loin », flashcards en fin de chapitre (secondaires).
- Règle de rédaction : les cours sont rédigés à partir des seules sources officielles autorisées ; les ouvrages
  déposés servent à la vérification (concordance) et ne sont ni recopiés ni reformulés de près.

### Annales et grilles (phase 2)

- `content/annales/index.json` (généré par `tools/annales/fetch_annales.py`) : une entrée par document
  `{annee, concours, type, epreuve, epreuve_code, format, url, fichier, sha256}` ; `format` vaut `reforme` ou
  `ancien`. Source : https://insp.gouv.fr/concours/ressources-utiles/sujets-meilleures-copies-et-rapports-des-jurys/voie-generale
  (relevé du 2026-10-30). Les PDF (`content/annales/files/`) ne sont pas versionnés.
- `content/grilles/<epreuve>.json` : `{epreuve_code, nom, total, criteres[{id, categorie, libelle, poids,
  poids_officiel, attendus[]}], erreurs_frequentes[], conseils[], source}` ; l'import refuse une grille dont la
  somme des poids diffère de `total`.
- Tables `annales` et `grilles` (migration 0002) ; endpoints `GET /api/annales?format=&epreuve=&type=` et
  `GET /api/grilles/{code}`.

### Copies (phase 3)

- Tables `copies` (statut `en_cours` → `deposee` → `auto_evaluee`, durée, début, délai, PDF) et `auto_evaluations`
  (migration 0003). Le PDF est rangé dans `DATA_DIR/copies/<user>/<copie>.pdf` (nom jamais fourni par le client).
- `POST /api/copies/start`, `GET /api/copies`, `GET /api/copies/{id}`, `POST /api/copies/{id}/pdf` (corps brut
  `application/pdf`), `POST /api/copies/{id}/autoeval` (`{scores: {critere: points}}`).
- Logique pure dans `backend/src/copies.rs` : validation du PDF, chronomètre, note (testés).

## 7. Pipeline des références (`tools/references/`)

Python (venv local) : extraction PDF/EPUB page par page (PyMuPDF, ebooklib), écriture dans
`references/index/` (jamais versionné), index plein texte SQLite FTS5 avec numéro de page.
`search.py "requête"` renvoie les passages avec pages. Les trois ouvrages déposés contiennent
du texte (pas de scan), l'OCR n'est donc pas nécessaire à ce stade.
Le texte des ouvrages ne quitte jamais la machine et n'est jamais envoyé à l'API.

## 8. Budget IA

`coût = Σ tokens × tarif Batch` (entrée, sortie, lecture et écriture de cache, recherches web),
tarifs lus dans `.env`. Estimation avant envoi sur la base de la taille du PDF et du prompt ;
envoi bloqué si `dépense du mois + estimation > plafond`. Mois civil UTC.

## 9. Suivi des lots

Machine à états par correction : `en_attente → passe1_en_cours → passe2_en_cours → terminee`
ou `echec(raison)`. Tâche de fond : sonde périodique des lots en base, lance la passe 2 quand la
passe 1 est finie, stocke les `batch_id` pour reprise après redémarrage. Un renvoi réutilise
les passes réussies (pas de refacturation).

## 10. Évolutions prévues

Les oraux s'ajouteront comme nouveaux types d'épreuves dans la configuration de session
et nouveaux modules, sans toucher au schéma existant.
