# Suivi d'avancement — appli INSP

## État actuel

- Phase en cours : 1 — MVP (squelette backend/frontend)
- Dernière mise à jour : 2026-10-30
- Prochaine action : créer backend/ (Axum, sqlx, migrations), frontend/ (Vite React TS), docker-compose.yml, .env.example

## Phases

- [x] 0. Prérequis confirmés
- [x] Architecture documentée (docs/ARCHITECTURE.md)
- [ ] 1. MVP : pseudo, session, majeure/mineure, fiches avec sources et statut,
     catalogue bibliographie, pipeline des références, premier lot de fiches vérifiées,
     flashcards SM-2, signalement
- [ ] 2. Annales et rapports du jury, grilles d'évaluation
- [ ] 3. Sujets chronométrés, dépôt PDF, auto-évaluation
- [ ] 4. Correction IA différée (Batch, deux passes) et budget
- [ ] 5. Entraînement méthodo et questions à réponses courtes
- [ ] 6. Veille d'actualité
- [ ] 7. Planning, tableau de bord, quiz
- [ ] 8. Complétion des fiches

## Choix techniques clés

- Format de session vérifié contre : — (texte officiel et date)
- Modèle de correction et tarifs configurés : —
- Outils d'extraction et d'indexation des références : —
- Sources de veille retenues : —

## Références

| Type                     | Attendus | Présents | Extraits et indexés | Remarques |
| ------------------------ | -------- | -------- | ------------------- | --------- |
| Publications officielles | —        | 0        | 0                   |           |
| Ouvrages déposés         | —        | 0        | 0                   |           |

Publications officielles introuvables : —
Éditions plus récentes trouvées : —

## Fiches

| Matière                        | Prévues | Rédigées | Sources vérifiées | Concordance ouvrages | Relues humain |
| ------------------------------ | ------- | -------- | ----------------- | -------------------- | ------------- |
| Droit public                   | —       | 0        | 0                 | 0                    | 0             |
| Économie                       | —       | 0        | 0                 | 0                    | 0             |
| Finances publiques             | —       | 0        | 0                 | 0                    | 0             |
| Questions sociales             | —       | 0        | 0                 | 0                    | 0             |
| Europe et international        | —       | 0        | 0                 | 0                    | 0             |
| Transitions                    | —       | 0        | 0                 | 0                    | 0             |
| Thèmes de la note de réflexion | —       | 0        | 0                 | 0                    | 0             |

Désaccords ouverts dans docs/VERIFICATION.md : 0

## Annales

- Sujets importés (format réformé / ancien format) : 0 / 0
- Rapports du jury importés : 0
- Grilles d'évaluation construites : 0 / 5 épreuves

## Journal des décisions

- 2026-10-30 — Développement sous Windows + Docker Desktop (la spec visait Arch) — la machine est sous Windows ; les images sont Linux, donc `docker compose up` reste identique sous Arch.
- 2026-10-30 — Extraction de références en Python dans un venv local (PyMuPDF, ebooklib), sans OCR — les 3 ouvrages et la bibliographie contiennent du texte ; EPUB accepté.
- 2026-10-30 — Session 2027 saisie dans content/sessions/2027.json à partir de la page officielle insp.gouv.fr/concours-externe (thèmes 2026-2027 : « Puissance et impuissances publiques » ; « L'apport de la science et des technologies aux politiques publiques ») — le texte Légifrance n'est pas lisible automatiquement.

## Problèmes rencontrés

<!-- Format : date — problème — solution ou statut -->

## Points à vérifier par l'utilisateur

- Programme détaillé de chaque épreuve : à confirmer sur le texte consolidé de l'arrêté du 21 mars 2023 (Légifrance), non lisible automatiquement.
- Deux ouvrages déposés portent « z-library » dans leur nom : utilisés en vérification locale uniquement, exclus de git.

## Travail en cours (en cas d'interruption)

<!-- Ce qui était en train d'être fait, pour reprendre proprement -->
