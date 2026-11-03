# Suivi d'avancement — appli INSP

## État actuel

- Phase en cours : 4 — correction IA différée (Batch, deux passes) et budget (phases 2 et 3 terminées ; phase 1 : MVP fonctionnel mais fiches très incomplètes)
- Dernière mise à jour : 2026-10-30
- Prochaine action : phase 4. Vérifier d'abord en ligne, dans la documentation Anthropic : tarifs Batch d'Opus 5.5, limites PDF (taille/pages), disponibilité de l'extended thinking et de l'outil de recherche web en Batch, mise en cache de prompt en Batch. Ne rien figer de mémoire.

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
- [x] 3. Sujets chronométrés, dépôt PDF, auto-évaluation
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
| Droit public                   | —       | 5        | 2                 | 0                    | 0             |
| Économie                       | —       | 1        | 1                 | 0                    | 0             |
| Finances publiques             | —       | 1        | 1                 | 0                    | 0             |
| Questions sociales             | —       | 0        | 0                 | 0                    | 0             |
| Europe et international        | —       | 0        | 0                 | 0                    | 0             |
| Transitions                    | —       | 0        | 0                 | 0                    | 0             |
| Thèmes de la note de réflexion | —       | 0        | 0                 | 0                    | 0             |

Désaccords consignés dans docs/VERIFICATION.md : 1 (tranché par la source officielle)
Cours rédigés (type « cours ») : 1 (droit public : hiérarchie des normes), statut brouillon

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

- 2026-10-30 — Chronomètre : démarré et tenu côté serveur (heure de début en base) ; la durée vient de la session (`duree_minutes`), jamais codée en dur ; les épreuves de l'ancien format (sans durée dans la session) ne peuvent pas être chronométrées. Dépôt hors délai accepté mais marqué « hors délai » (tolérance de 120 s).
- 2026-10-30 — Limites du PDF : 32 Mo et 100 pages par défaut (`MAX_PDF_MB`, `MAX_PDF_PAGES`). Ces valeurs viennent de ma connaissance de l'API Claude et n'ont PAS été revérifiées en ligne : à confirmer en phase 4. Un PDF chiffré est refusé. Texte tapé et scan manuscrit sont tous deux acceptés (aucune détection de scan à ce stade).
- 2026-10-30 — Auto-évaluation : l'utilisateur note chaque critère de la grille (0 à son poids) ; la note est arrondie au demi-point et toujours présentée comme indicative.

- 2026-11-03 — Retour de l'utilisateur : il veut de vrais cours (chapitres rédigés), pas des fiches de mémorisation avec flashcards. Décision : nouveau type de contenu « cours » (voir ARCHITECTURE.md), page « Cours » dans l'appli, flashcards reléguées en fin de chapitre. Un premier chapitre modèle est écrit (hiérarchie des normes, ~3 400 mots) pour valider le format avant d'en écrire d'autres. Les fiches existantes (synthèses) sont conservées.

## Problèmes rencontrés

<!-- Format : date — problème — solution ou statut -->
- 2026-10-30 — sqlx 0.9 refuse le SQL construit à l'exécution (`format!`) : requêtes assemblées à la compilation avec `concat!` (macro `copie_select!`).
- 2026-10-30 — Pas de test automatisé de l'interface (seuls les utilitaires sont testés) : le parcours chronomètre → dépôt → auto-évaluation a été testé par l'API (Docker) mais pas dans un navigateur.
- 2026-10-30 — Rapport du jury 2025 : les moyennes de plusieurs épreuves sont identiques d'une épreuve à l'autre (ex. 8,65 pour les trois QRC externes) ; recopiées telles quelles et signalées dans la grille QRC.
- 2026-10-30 — Rapport 2025, annexe « Cas pratique » : lue seulement jusqu'à la rubrique « Sur la forme » ; la suite (recommandations) n'est pas exploitée. Les rapports 2024 et les sujets 2024-2026 n'ont pas encore été exploités pour affiner les grilles (extraction des erreurs fréquentes par épreuve à compléter).

## Points à vérifier par l'utilisateur

- Cours « La hiérarchie des normes et le principe de légalité » (statut brouillon) : décisions lues sur les pages du Conseil d'État, du Conseil constitutionnel, d'EUR-Lex et de la Cour de cassation via un outil de résumé (pas en texte intégral). Éléments à documenter ou à reconfirmer : numéro de pourvoi Jacques Vabre (73-13556, relevé dans une page du Conseil constitutionnel) ; réserve de l'identité constitutionnelle (décisions postérieures à 2004, non vérifiées, signalées dans le cours) ; décisions Rouquette 1999, SNIP 2001, Fraisse 2000, Boisdet 1990, Trompier-Gravier 1944 (citées d'après les pages du Conseil d'État, sans lecture de leur texte) ; liste détaillée du bloc de constitutionnalité (principes à valeur constitutionnelle, objectifs de valeur constitutionnelle) non fournie par la source.
- Désaccord ouvert/clos dans docs/VERIFICATION.md : date de l'arrêt de la CJUE Arcelor (Degoffe : 16 novembre 2008 ; EUR-Lex : 16 décembre 2008 → date officielle retenue).

- Lot de fiches du 2026-11-03 (4 fiches : recours pour excès de pouvoir / Dame Lamotte, responsabilité de la puissance publique, déficit et dette publics 2025, politique monétaire de la BCE), statut « sources officielles vérifiées » :
  - Décisions (Lamotte, Blanco, Cames, Couitéas) lues sur les pages « Les grandes décisions depuis 1873 » du Conseil d'État ; chiffres INSEE sur la page de l'Insee Première n° 2106 (29 mai 2026) ; taux et projections sur le communiqué de la BCE du 10 septembre 2026 ; chiffres de la Cour des comptes dans le PDF local. Ces pages ont été lues via un outil de résumé, pas en texte intégral : relire les chiffres clés avant usage en copie.
  - Concordance avec les ouvrages seulement partielle : Degoffe confirme Blanco (p. 53-54), les trois fondements (p. 636), Couitéas (p. 655), préjudice anormal et spécial (p. 645) ; Lamotte et Cames n'y ont pas été retrouvés. Les numéros de page sont ceux du fichier PDF, pas forcément ceux du livre imprimé.
  - Dans la fiche sur la dette, la définition de l'effet « boule de neige » et la décomposition structurel/conjoncturel de 2024 s'appuient uniquement sur les ouvrages déposés (pas de source officielle citée pour ces deux points).
  - Les taux de la BCE changent à chaque réunion (prochaine à revérifier). La décision « inchangés le 29 octobre 2026 » vient d'un résultat de recherche non relu sur la page.
  - Article du TFUE fondant le mandat de la BCE non cité (EUR-Lex illisible automatiquement).
  - Le numéro 86949 de l'arrêt Lamotte vient de la base ArianeWeb (titre de la page), pas de la page « grandes décisions ».
  - Toujours pas de relecture critique indépendante de ce lot ni du précédent.
  - Une erreur de syntaxe YAML (titre contenant « : » non entre guillemets) a empêché l'import de 3 fiches avant correction : toujours mettre les titres entre guillemets.

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
