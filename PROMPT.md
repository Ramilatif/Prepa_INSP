# Contexte

Je prépare le concours externe de la voie générale de l'INSP (Institut national du service
public). La session n'est pas encore fixée. Je veux une application web de révision et
d'entraînement, utilisée par moi et quelques amis qui préparent aussi le concours.
L'appli couvre d'abord les épreuves écrites ; les oraux viendront plus tard (voir « Évolutions »).
Design et contenus originaux ou issus de sources officielles citées.

# Stack et contraintes techniques

- Backend : Rust (Axum), sqlx avec migrations
- Base de données : SQLite, mais schéma et requêtes compatibles PostgreSQL pour une migration future
- Frontend : React (TypeScript), responsive PC et mobile
- Lancement local via Docker Compose (`docker compose up`) ; je suis sous Arch Linux
- Mise en ligne prévue plus tard : configuration par variables d'environnement (.env),
  aucun secret en dur, README avec instructions de lancement
- Les appels à l'API Claude se font uniquement côté serveur : la clé API n'est jamais
  exposée au navigateur
- Code commenté, tests unitaires sur la logique métier (planning, répétition espacée,
  calcul et plafonnement du budget IA, suivi des lots de correction, import des contenus)

# Utilisateurs

- Connexion par pseudo simple, sans mot de passe
- Chaque utilisateur a ses propres données (copies, corrections, flashcards, progression,
  planning, consommation IA)
- Chaque utilisateur choisit sa matière majeure et sa mineure (droit public ou économie),
  modifiable à tout moment ; tant que ce n'est pas fait, les deux sont traitées à égalité
- Garde l'authentification isolée dans un module pour ajouter un mot de passe avant la mise en ligne

# Design

- Style sobre et institutionnel, très lisible, pensé pour de longues sessions de travail

# Configuration par session

- Tout ce qui dépend de la session est défini en données, jamais codé en dur : épreuves,
  durées, coefficients, thèmes de la note de réflexion (renouvelés tous les deux ans),
  programme, dates. Une « session active » est sélectionnable dans l'appli
- Format actuel connu, à vérifier contre les textes officiels (arrêté du 21 mars 2023 fixant
  les épreuves, site de l'INSP) avant de le saisir, en citant la source dans docs/ARCHITECTURE.md :
  - Note de réflexion sur une question contemporaine (5 h, coef. 2), sur deux thèmes
    annoncés à l'avance
  - Épreuve opérationnelle d'économie et épreuve opérationnelle de droit public (5 h chacune),
    l'une en majeure (coef. 4), l'autre en mineure (coef. 2) : identifier un problème,
    établir un diagnostic, proposer des solutions réalistes
  - Questions à réponses courtes (5 h, coef. 3) : finances publiques, questions sociales,
    questions européennes et internationales, avec documents possibles
  - Cas pratique sur les transitions écologique ou numérique (4 h, coef. 2), à partir
    d'un dossier documentaire

# Contenu : fiches de cours

- Tu rédiges les fiches à partir des sources autorisées (voir « Sources pour la rédaction
  des fiches »)
- Couverture alignée sur le programme officiel : droit public (constitutionnel, administratif,
  libertés), économie et politiques économiques, finances publiques, questions sociales,
  questions européennes et internationales, transitions écologique et numérique,
  et les thèmes de la note de réflexion de la session active
- Structure de chaque fiche : notions clés, enjeux d'action publique, chiffres clés datés,
  débats et positions, références précises, « Pour aller plus loin » (renvois à la
  bibliographie), liens vers les flashcards et sujets associés
- Fiches en Markdown avec métadonnées (matière, épreuves concernées, date de mise à jour,
  sources, statut), importées en base

# Sources pour la rédaction des fiches

## Sources autorisées, par matière

- Droit public : Légifrance, Conseil constitutionnel (décisions et commentaires officiels),
  Conseil d'État (décisions, études annuelles, dossiers thématiques), vie-publique.fr
- Économie : INSEE, DG Trésor, Banque de France, Conseil d'analyse économique,
  France Stratégie, Eurostat, OCDE, FMI
- Finances publiques : documents budgétaires de l'État (PLF, rapports annuels de performance),
  Cour des comptes, Haut Conseil des finances publiques, rapports budgétaires
  de l'Assemblée nationale et du Sénat
- Questions sociales : DREES, DARES, Conseil d'orientation des retraites, Haut Conseil
  du financement de la protection sociale, Cour des comptes (Sécurité sociale)
- Europe et international : EUR-Lex (traités, actes, jurisprudence de la CJUE), Commission
  européenne, Conseil de l'UE, ministère de l'Europe et des Affaires étrangères, SGAE
- Transitions écologique et numérique : Haut Conseil pour le climat, ADEME, ministère
  de la Transition écologique, CNIL, ARCEP, ANSSI
- Méthode et attentes : rapports du jury de l'INSP
- Ressources web listées dans les encadrés « Zoom » de la bibliographie de l'INSP

## Hiérarchie

1. Textes normatifs et décisions de justice (source primaire, toujours privilégiée)
2. Statistiques et évaluations officielles
3. Rapports parlementaires et d'institutions consultatives
4. Pour les débats uniquement : centres de recherche reconnus (ex. OFCE, Institut des
   politiques publiques), présentés comme des analyses parmi d'autres, en confrontant
   plusieurs points de vue

## Exclusions

- Pas de manuels, cours de prépa ou contenus payants comme sources de rédaction
- Sites grand public et Wikipédia : point de départ possible, jamais source citée
- Si une information n'est disponible que dans une source non autorisée, signale-la
  dans PROGRESS.md au lieu de l'intégrer

# Bibliographie de l'INSP

- Je fournis la bibliographie « Ouvrages de référence » du centre de documentation
  de l'INSP (references/bibliographie_insp.pdf). Importe-la dans references/catalogue.json :
  pour chaque entrée, titre, auteurs, éditeur, édition, année, matière, et un champ
  « accès » (libre / payant)
- Publications officielles en accès libre : j'en ai déjà déposé une partie dans
  references/officiels/. Vérifie ce qui est présent, télécharge les manquantes depuis le site
  officiel de l'institution qui les publie, et vérifie à chaque fois s'il existe une édition
  plus récente que celle de la bibliographie
- Ouvrages payants : ne les télécharge jamais depuis une source non officielle ;
  ils restent des références bibliographiques jusqu'à ce que je dépose moi-même
  un exemplaire dans references/ouvrages/

# Ouvrages de référence pour la vérification

- Mes ouvrages en PDF (references/ouvrages/, exclu de git) et les publications officielles
  (references/officiels/) servent de base de vérification, pas de source de rédaction
- Pipeline local : extraction du texte (avec numéros de page), indexation, et recherche
  des passages pertinents pour chaque fiche, sans relire les documents en entier à chaque fois
- Métadonnées de chaque document dans references/catalogue.json
- Vérification de chaque fiche :
  - Chaque affirmation importante (définition, mécanisme, règle, chiffre, date) est
    confrontée aux sources officielles ET aux ouvrages pertinents
  - Si l'ouvrage concorde : la fiche peut le citer en référence bibliographique
    (auteur, titre, édition, pages), en plus de la source officielle
  - En cas de désaccord : la source officielle la plus récente l'emporte si l'écart
    s'explique par une évolution des textes ou des chiffres depuis l'édition de l'ouvrage ;
    sinon, ne tranche pas seul et consigne le désaccord dans docs/VERIFICATION.md
    et dans PROGRESS.md (points à vérifier)
  - Une affirmation qui ne trouve d'appui ni dans une source officielle ni dans un ouvrage
    est retirée de la fiche
- Le texte des ouvrages n'est jamais recopié ni reformulé de près dans les fiches,
  et n'est pas envoyé à l'API Claude par l'appli
- Pour les autres utilisateurs, les ouvrages n'apparaissent que comme références
  bibliographiques
- Si un PDF est un scan (images de pages), signale-le : l'extraction demandera une
  reconnaissance de caractères

# Garde-fous sur la véracité du contenu

- Ne rédige jamais de mémoire ce qui évolue vite (chiffres, textes récents, jurisprudence,
  réformes) : vérifie en ligne au moment de la rédaction
- Chaque affirmation importante d'une fiche renvoie à une référence précise (article de loi,
  décision avec date et numéro, rapport, statistique avec source et date)
- Chaque fiche porte une date de mise à jour et un statut : brouillon, sources officielles
  vérifiées, concordance avec les ouvrages vérifiée, relue par un humain
- Après chaque lot de fiches, effectue une relecture critique distincte de la rédaction
  (idéalement dans une session ou un sous-agent dédié) : exactitude juridique et économique,
  références, chiffres datés, absence de simplification fausse
- Note dans PROGRESS.md tout point dont tu n'es pas certain
- Bouton « signaler une erreur » sur chaque fiche, flashcard et corrigé, avec une page
  qui liste les signalements

# Annales et rapports du jury

- Télécharge les sujets et rapports du jury publiés sur insp.gouv.fr, stockés avec leurs
  métadonnées (année, concours, épreuve, lien source)
- Distingue clairement les annales du format réformé et celles de l'ancien format :
  les secondes servent de matière thématique, pas de modèle de format
- Extrais des rapports du jury les attentes et erreurs fréquentes par épreuve, pour construire
  les grilles d'évaluation et les conseils de méthode

# Modes de révision

- Fiches de cours consultables, avec recherche et filtres par matière et épreuve
- Flashcards et quiz de connaissances avec répétition espacée (algorithme SM-2),
  générés à partir des fiches (définitions, dates, chiffres, jurisprudences, institutions)
- Entraînement méthodo en exercices courts : formuler une problématique, construire un plan,
  rédiger un diagnostic, proposer des solutions, répondre à une question courte
- Sujets complets chronométrés à la durée réelle de l'épreuve (annales ou sujets inédits
  que tu rédiges dans le format réformé)

# Dépôt et correction des copies

- Je rends mes copies en PDF : texte tapé ou scan d'une copie manuscrite. Gère les deux cas
  et vérifie les limites de taille et de pages de l'API
- Deux modes de correction, au choix pour chaque copie :
  - Auto-évaluation : grille de critères issue des rapports du jury, éléments attendus
    et corrigé type ou plan indicatif
  - Correction par IA (optionnelle) via l'API Claude, en privilégiant la qualité au coût
- Configuration de la correction IA :
  - Modèle par défaut : Claude Opus 5.5 (configurable dans .env), avec la réflexion
    approfondie (extended thinking) activée
  - Sources : la grille de l'épreuve, les extraits pertinents des rapports du jury,
    les fiches de l'appli liées au sujet, et l'outil de recherche web de l'API autorisé
    quand la copie aborde un point absent des fiches ou récent
  - Correction en deux passes : la première produit la correction, la seconde la vérifie
    (chaque erreur de fond signalée est-elle réellement une erreur, sources à l'appui ?
    la note est-elle cohérente avec la grille ?) et corrige la correction si besoin
- Contenu de la correction : note indicative sur 20, évaluation critère par critère selon
  la grille, points forts, faiblesses, erreurs de fond avec la source qui permet de
  les vérifier, conseils concrets et prioritaires pour progresser, éléments attendus
  que la copie n'a pas traités
- Toute correction IA affiche clairement que la note est indicative
- Avant le premier envoi, informer l'utilisateur que sa copie est transmise à l'API d'Anthropic
- Historique des copies et corrections par utilisateur
- Fonctionnement différé :
  - À l'envoi, la copie passe au statut « en attente de correction », avec l'information
    que le résultat peut prendre jusqu'à 24 heures par passe
  - Une tâche de fond côté serveur suit l'avancement des lots, lance la seconde passe dès
    que la première est terminée, puis enregistre la correction finale
  - Le traitement continue même si l'utilisateur ferme l'appli ; les lots en cours sont
    repris après un redémarrage du serveur
  - Notification dans l'appli quand la correction est prête
  - En cas d'échec d'un lot (erreur, expiration), la copie passe au statut « échec »
    avec la raison, et peut être renvoyée sans être facturée deux fois pour la partie réussie

# Budget IA

- Plafond de 5 € par mois et par utilisateur, configurable dans .env
- Toutes les corrections IA passent obligatoirement par l'API Batch d'Anthropic (traitement
  différé, moitié prix) : pas de mode de correction immédiate
- Coût de chaque correction calculé à partir des tokens consommés (deux passes, réflexion
  et recherches web comprises) et des tarifs Batch configurés dans .env
- Avant un envoi : estimation du coût ; si le plafond serait dépassé, l'envoi est bloqué
  avec un message clair
- Affichage du budget restant ; remise à zéro mensuelle
- Cache de prompt pour les éléments identiques d'une copie à l'autre (grilles,
  rapports du jury, consignes)

# Veille d'actualité

- Récupération automatique et périodique depuis des sources officielles (flux RSS ou API
  publiques : ex. vie-publique.fr, Journal officiel, Conseil constitutionnel, Conseil d'État,
  Cour des comptes, INSEE, institutions européennes)
- Chaque élément : titre, date, source, lien, résumé issu du flux, catégorie
- Liens automatiques vers les fiches concernées ; quand une actualité touche le sujet d'une
  fiche, celle-ci est marquée « à vérifier, actualité récente »
- Pas d'appel IA pour la veille

# Planning et suivi

- Dès que l'utilisateur saisit une date de concours et son temps disponible, l'appli génère
  un programme (fiches, flashcards, exercices méthodo, sujets complets) et le réajuste
  selon ses résultats et points faibles
- Tableau de bord : progression par matière, notes par épreuve (auto-évaluation et IA),
  évolution dans le temps, temps de travail, flashcards dues

# Évolutions (hors périmètre pour l'instant)

- Préparation des oraux : entretien avec le jury (fiche de renseignements, questions types,
  simulation), mise en situation collective, anglais
- Conçois l'architecture pour pouvoir les ajouter sans refonte, mais ne les développe pas

# Phases

1. MVP : pseudo, configuration par session, choix majeure/mineure, structure des fiches
   avec sources et statut, catalogue de la bibliographie, pipeline d'extraction et
   d'indexation des références, premier lot de fiches vérifiées (droit constitutionnel),
   flashcards SM-2 de base, signalement d'erreurs
2. Annales et rapports du jury : import, métadonnées, grilles d'évaluation par épreuve
3. Sujets chronométrés, dépôt PDF, auto-évaluation
4. Correction par IA en différé (Batch, deux passes) et budget
5. Entraînement méthodo et questions à réponses courtes
6. Veille d'actualité
7. Planning, tableau de bord, quiz de connaissances
8. Complétion des fiches sur tout le programme

# Méthode

## Étape 0 — Prérequis (seul moment où tu attends ma réponse)

Avant d'écrire du code, fais-moi en une seule fois la liste complète de ce dont tu as besoin :

- Outils et dépendances système à installer (avec la commande exacte pour Arch Linux, pacman
  ou AUR, et la version minimale requise)
- Accès ou permissions nécessaires (dossiers, droits Docker, ports utilisés, etc.)
- Comptes et clés : clé API Anthropic (pour la correction IA), et tout compte nécessaire
  aux sources officielles (ex. API Légifrance), en précisant ce qui est indispensable
  ou optionnel
- Les sources de veille et d'annales retenues, avec leur mode d'accès
- Le dépôt de mes ouvrages dans references/ouvrages/, la liste des publications officielles
  manquantes dans references/officiels/, et les outils nécessaires à l'extraction du texte
- Ce que tu vas installer toi-même dans le projet (crates, paquets npm) sans action de ma part
  Vérifie ce qui est déjà présent sur ma machine avant de me demander de l'installer.
  Anticipe tous les besoins de toutes les phases, pour ne plus avoir à m'interrompre ensuite.
  N'installe rien au niveau système et ne modifie rien hors du dossier du projet sans mon accord.

## Ensuite — Exécution autonome

Une fois les prérequis confirmés, enchaîne toutes les phases sans attendre de validation :

- Commence par documenter l'architecture (arborescence, schéma de base de données, endpoints,
  format des fiches et des sujets, configuration de session avec ses sources officielles,
  grilles d'évaluation, pipeline des références, calcul du budget IA, suivi des lots)
  dans docs/ARCHITECTURE.md, puis respecte-la
- Initialise un dépôt git et fais au moins un commit par phase, avec un message clair
- Ne passe à la phase suivante que si le projet compile et que les tests passent
- Tiens à jour PROGRESS.md (le modèle existe déjà à la racine) : phases terminées, choix
  techniques faits et pourquoi, problèmes rencontrés, points à vérifier par moi
- Si les commandes ou la structure du projet changent, mets à jour CLAUDE.md en conséquence
- Face à un choix non précisé dans ce prompt, prends la décision la plus raisonnable,
  note-la dans PROGRESS.md et continue
- Ne t'arrête que si tu es réellement bloqué (accès ou installation système imprévu,
  erreur que tu ne peux pas résoudre) : dans ce cas, explique précisément ce qu'il te faut
- À la fin, fais-moi un résumé global : ce qui fonctionne, comment lancer l'appli,
  et ce qui reste à améliorer
