# Suivi d'avancement — appli INSP

## État actuel

- Phase en cours : 3 — sujets chronométrés, dépôt PDF, auto-évaluation (phase 1 : MVP fonctionnel mais fiches très incomplètes, voir plus bas ; phase 2 terminée)
- Dernière mise à jour : 2026-10-30
- Prochaine action : phase 3 (chronomètre, dépôt de copies en PDF, auto-évaluation avec les grilles) ; compléter les fiches en parallèle

Fait en phase 1 : backend Axum + migrations + SM-2 + import des fiches + signalements, frontend React
(fiches, flashcards, réglages, signalements), Docker Compose fonctionnel, pipeline d'indexation des
références, catalogue de la bibliographie INSP (121 ouvrages), 1 fiche.

## Phases

- [x] 0. Prérequis confirmés
- [x] Architecture documentée (docs/ARCHITECTURE.md)
- [ ] 1. MVP : pseudo, session, majeure/mineure, fiches avec sources et statut,
     catalogue bibliographie, pipeline des références, premier lot de fiches vérifiées,
     flashcards SM-2, signalement
- [x] 2. Annales et rapports du jury, grilles d'évaluation (voir « Annales » ci-dessous)
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

- Sujets importés (format réformé / ancien format) : 52 documents au total (concours externe) : sujets 2024-2026 + sujets fictifs V0 + meilleures copies 2024-2025 + 2 rapports en format réformé ; sujets, copies et rapports 2021-2023 en ancien format (23)
- Rapports du jury importés : 5 (2021 à 2025) ; seuls 2024 et 2025 sont du format réformé
- Grilles d'évaluation construites : 5 / 5 épreuves (note de réflexion, économie, droit public, QRC, cas pratique), d'après le rapport 2025

## Journal des décisions

- 2026-10-30 — Développement sous Windows + Docker Desktop (la spec visait Arch) — la machine est sous Windows ; les images sont Linux, donc `docker compose up` reste identique sous Arch.
- 2026-10-30 — Extraction de références en Python dans un venv local (PyMuPDF, ebooklib), sans OCR — les 3 ouvrages et la bibliographie contiennent du texte ; EPUB accepté.
- 2026-10-30 — Session 2027 saisie dans content/sessions/2027.json à partir de la page officielle insp.gouv.fr/concours-externe (thèmes 2026-2027 : « Puissance et impuissances publiques » ; « L'apport de la science et des technologies aux politiques publiques ») — le texte Légifrance n'est pas lisible automatiquement.

- 2026-10-30 — Annales : concours externe uniquement, téléchargées par tools/annales/fetch_annales.py (liste relevée sur insp.gouv.fr), PDF dans content/annales/files/ exclus de git (re-téléchargeables), manifeste index.json versionné. Format « réformé » = 2024 et après (d'après la page de l'INSP) ; 2022-2023 = « ancien ».
- 2026-10-30 — Grilles : seul le barème de la forme de la note de réflexion (5/20) est officiel ; tous les autres poids sont des choix de l'appli (champ `poids_officiel: false`, affiché comme « indicatif »). Le total de chaque grille est vérifié à l'import (somme des poids = 20).

## Problèmes rencontrés

<!-- Format : date — problème — solution ou statut -->
- 2026-10-30 — Rapport du jury 2025 : les moyennes de plusieurs épreuves sont identiques d'une épreuve à l'autre (ex. 8,65 pour les trois QRC externes) ; recopiées telles quelles et signalées dans la grille QRC.
- 2026-10-30 — Rapport 2025, annexe « Cas pratique » : lue seulement jusqu'à la rubrique « Sur la forme » ; la suite (recommandations) n'est pas exploitée. Les rapports 2024 et les sujets 2024-2026 n'ont pas encore été exploités pour affiner les grilles (extraction des erreurs fréquentes par épreuve à compléter).

## Points à vérifier par l'utilisateur

- Programme détaillé de chaque épreuve : à confirmer sur le texte consolidé de l'arrêté du 21 mars 2023 (Légifrance), non lisible automatiquement.
- Deux ouvrages déposés portent « z-library » dans leur nom : utilisés en vérification locale uniquement, exclus de git.
- Catalogue de la bibliographie : 10 notices sur 121 marquées `a_verifier` (édition, éditeur ou année mal analysés) ; les listes « Zoom » des rubriques droit public et économie n'ont pas été capturées ; le champ `depose_id` (rapprochement avec les ouvrages déposés) est à renseigner à la main.
- Fiche `controle-de-constitutionnalite` : statut « brouillon ». Vérifiés en ligne le 2026-10-30 : texte des art. 61, 61-1, 62 (Légifrance / Conseil constitutionnel, résumé), décisions 71-44 DC et 2010-14/22 QPC. Non vérifié : la loi organique n° 2009-1523 du 10 décembre 2009 (Légifrance illisible automatiquement), volontairement non citée. Aucun ouvrage déposé ne couvre le droit constitutionnel (le Brunet-Hamon-Troper n'est pas déposé) : la concordance avec les ouvrages ne peut pas être vérifiée.
- Relecture critique du lot de 3 fiches de droit constitutionnel (faite par moi, dans la même session : ce n'est pas la relecture indépendante demandée par PROMPT.md, à refaire dans une session dédiée) : art. 61 complété (propositions de loi de l'article 11), art. 62 confirmé sur Légifrance. Les sections « Débats et positions » des 3 fiches sont des amorces sans source : elles sont signalées en italique dans les fiches et gardent le statut « brouillon ». Texte intégral des art. 34, 37, 38, 39, 45 lu seulement par extraits/résumé du site du Conseil constitutionnel : à reconfronter au texte complet.
- Ouvrages déposés le 2026-10-30 : Rouvillois, *Droit constitutionnel 2. La Ve République*, 5e éd. (le fichier ne fait que 45 pages : extrait d'introduction, inutilisable pour vérifier les fiches de droit constitutionnel, aucune des notions vérifiées n'y figure) ; Zarka, *Droit international public* (237 pages, indexé, utile pour la matière Europe et international). Il faut un exemplaire complet d'un manuel de droit constitutionnel pour passer les 3 fiches au statut « concordance vérifiée ».
- Rouvillois, *Droit constitutionnel*, tome 1 « Fondements et pratiques » déposé et indexé (821 pages, texte complet). Le fichier du tome 2 (« La Ve République ») est toujours l'extrait de 45 pages. Le tome 1 est théorique : il ne traite des art. 5, 37 et 38 que par renvois (p. 95, p. 163) et ne couvre pas la QPC, l'art. 49-3 ni les décisions citées. Statut des 3 fiches inchangé (« brouillon »), en attente du tome 2 complet.
- Constitution : version consolidée « à jour au 8 mars 2024 » selon le Conseil constitutionnel ; à revérifier si une révision plus récente existe.
- Le jeton de connexion est l'identifiant utilisateur (UUID) : suffisant sans mot de passe, à remplacer par une vraie session avec mot de passe avant la mise en ligne.

## Travail en cours (en cas d'interruption)

<!-- Ce qui était en train d'être fait, pour reprendre proprement -->
