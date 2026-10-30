# Suivi d'avancement — appli INSP

## État actuel

- Phase en cours : 1 — MVP (reste à faire : lot de fiches de droit constitutionnel, téléchargement des publications officielles, compléter le catalogue)
- Dernière mise à jour : 2026-10-30
- Prochaine action : rédiger les fiches suivantes de droit constitutionnel (sources officielles, vérification croisée), puis finaliser la phase 1 et passer à la phase 2

Fait en phase 1 : backend Axum + migrations + SM-2 + import des fiches + signalements, frontend React
(fiches, flashcards, réglages, signalements), Docker Compose fonctionnel, pipeline d'indexation des
références, catalogue de la bibliographie INSP (121 ouvrages), 1 fiche.

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
| Publications officielles | —        | 2        | 2                   | Cour des comptes : communiqué sur le budget de l'État 2025 (22/04/2026) et synthèse « situation et perspectives des finances publiques » (juin 2026). Rapport complet du budget de l'État 2025 non téléchargé (PDF principal introuvable automatiquement). |
| Ouvrages déposés         | —        | 3        | 3                   | 2 PDF + 1 EPUB, tous avec texte (pas de scan) |

Publications officielles introuvables : — (téléchargement pas encore lancé)
Éditions plus récentes trouvées : —

## Fiches

| Matière                        | Prévues | Rédigées | Sources vérifiées | Concordance ouvrages | Relues humain |
| ------------------------------ | ------- | -------- | ----------------- | -------------------- | ------------- |
| Droit public                   | —       | 3        | 0                 | 0                    | 0             |
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
- Catalogue de la bibliographie : 10 notices sur 121 marquées `a_verifier` (édition, éditeur ou année mal analysés) ; les listes « Zoom » des rubriques droit public et économie n'ont pas été capturées ; le champ `depose_id` (rapprochement avec les ouvrages déposés) est à renseigner à la main.
- Fiche `controle-de-constitutionnalite` : statut « brouillon ». Vérifiés en ligne le 2026-10-30 : texte des art. 61, 61-1, 62 (Légifrance / Conseil constitutionnel, résumé), décisions 71-44 DC et 2010-14/22 QPC. Non vérifié : la loi organique n° 2009-1523 du 10 décembre 2009 (Légifrance illisible automatiquement), volontairement non citée. Aucun ouvrage déposé ne couvre le droit constitutionnel (le Brunet-Hamon-Troper n'est pas déposé) : la concordance avec les ouvrages ne peut pas être vérifiée.
- Relecture critique du lot de 3 fiches de droit constitutionnel (faite par moi, dans la même session : ce n'est pas la relecture indépendante demandée par PROMPT.md, à refaire dans une session dédiée) : art. 61 complété (propositions de loi de l'article 11), art. 62 confirmé sur Légifrance. Les sections « Débats et positions » des 3 fiches sont des amorces sans source : elles sont signalées en italique dans les fiches et gardent le statut « brouillon ». Texte intégral des art. 34, 37, 38, 39, 45 lu seulement par extraits/résumé du site du Conseil constitutionnel : à reconfronter au texte complet.
- Constitution : version consolidée « à jour au 8 mars 2024 » selon le Conseil constitutionnel ; à revérifier si une révision plus récente existe.
- Le jeton de connexion est l'identifiant utilisateur (UUID) : suffisant sans mot de passe, à remplacer par une vraie session avec mot de passe avant la mise en ligne.

## Travail en cours (en cas d'interruption)

<!-- Ce qui était en train d'être fait, pour reprendre proprement -->
