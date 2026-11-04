# Vérification des fiches et cours : désaccords entre sources

Règle (PROMPT.md) : si l'ouvrage et la source officielle divergent, la source officielle la plus récente l'emporte
lorsque l'écart s'explique par une évolution des textes ou des chiffres ; sinon on ne tranche pas seul et on consigne
ici et dans PROGRESS.md.

| Date | Fiche / cours | Point | Ouvrage | Source officielle | Décision |
|---|---|---|---|---|---|
| 2026-11-04 | cours-cedh-cour-de-strasbourg | Date d'adoption du Protocole n° 16 | de Ravel d'Esclapon, p. 27 : « adopté le 2 octobre 2018 » (alors qu'il entre en vigueur le 1er août 2018) | Denizeau p. 57 : « ouvert à la signature et à la ratification le 2 octobre 2013 » | 2013 retenu ; coquille évidente du livre de 2024. |
| 2026-11-04 | cours-protection-juridictionnelle-libertes | Date de la décision *Tliba* (vie familiale normale) | Denizeau, p. 189 : « 30 novembre 2001 » | Conseil d'État, dossier « Le juge des référés » : décision du **30 octobre 2001** (Ministre de l'intérieur c/ Mme T.) | Date officielle retenue ; erreur probable de l'ouvrage. |
| 2026-11-04 | cours-protection-juridictionnelle-libertes | Numéro de l'ordonnance du Conseil d'État du 20 septembre 2022 (droit à un environnement équilibré, liberté fondamentale) | Denizeau, p. 190 : « n° 451291 » | Conseil d'État / Légifrance : **n° 451129** | Numéro officiel retenu ; l'écart est une coquille probable de l'ouvrage (transposition de chiffres). |
| 2026-11-03 | cours-responsabilite-administrative | Date de la loi en cause dans l'arrêt La Fleurette | Degoffe, p. 646 : « 9 juin 1934 » | Page du Conseil d'État sur l'arrêt : « loi du **29 juin 1934** » | Date de la page du Conseil d'État retenue ; écart non expliqué par une évolution des textes (erreur probable de l'ouvrage). À confirmer par le texte de la loi sur Légifrance si possible. |
| 2026-11-03 | cours-hierarchie-des-normes | Date de l'arrêt de la CJUE dans l'affaire Arcelor (C-127/07) | Degoffe, *Droit administratif*, p. 131 : « 16 novembre 2008 » | EUR-Lex (ECLI:EU:C:2008:728) : arrêt de la grande chambre du **16 décembre 2008** | Date officielle retenue ; l'écart ne s'explique pas par une évolution des textes mais par une erreur probable de l'ouvrage (ou de son extraction). À signaler à l'utilisateur. |

## Vérification affirmation par affirmation : droit constitutionnel (2026-11-03)

Méthode : chaque affirmation importante des trois cours (`cours-hierarchie-des-normes`, `cours-controle-de-constitutionnalite`,
`cours-executif-parlement-procedure-legislative`) a été confrontée (1) à une source officielle et (2) aux trois manuels de droit constitutionnel
déposés (Rouvillois tome 1, 2025 ; Rouvillois tome 2, 5e éd. 2016 ; Caporal-Greco et al., 4e éd., 2025 selon l'utilisateur), avec l'outil
`tools/references/verify_claims.py` (listes d'affirmations : `claims_constitutionnel.json` et `claims_constitutionnel_2.json`).
Le numéro de section est celui du fichier indexé (pas la page du livre imprimé). Légende : ✓ confirmé ; ◐ confirmé en partie ; ✗ non couvert
par les manuels (la source officielle seule demeure).

### Cours « Hiérarchie des normes »

| Id | Affirmation | Officiel | Rouvillois t.1 / t.2 | Caporal-Greco | Verdict |
|---|---|---|---|---|---|
| H1 | Bloc de constitutionnalité (1958, 1789, 1946, Charte 2004) | ✓ CC | t.2 s.32 | s.22 | ✓ |
| H2 | CC 71-44 DC, liberté d'association | ✓ CC | t.2 s.73 | s.23 | ✓ |
| H3 | Art. 55 : conditions, réciprocité | ✓ CC | t.2 s.33 | s.26 | ✓ |
| H4 | Costa c/ ENEL, 15 juillet 1964 | ✓ EUR-Lex | t.2 s.51 | s.41 | ✓ |
| H5 | Art. 34 / 37 | ✓ | t.2 s.23 | s.25 | ✓ |
| H6 | Aramu 1945, premiers principes généraux du droit | ✓ CE | Degoffe p. 221 | non couvert | ✓ (Degoffe) |
| H7 | Dame Lamotte 1950 | ✓ CE | non couvert | non couvert | ✗ |
| H8 | CC 74-54 DC, pas de contrôle de conventionnalité | ✓ CC | — | s.26 | ✓ |
| H9 | Jacques Vabre, 24 mai 1975 | ✓ (via CC) | t.2 s.75 | s.26 | ✓ |
| H10 | Nicolo, 20 octobre 1989 (Rec. p. 190) | ✓ CE | t.2 s.75 | s.26 | ✓ (corrige l'hésitation 190 / 290) |
| H11 | Sarran, 30 octobre 1998 (n° 200286 et 200287) | ✓ CE | t.2 s.33 | s.27, s.41 | ✓ |
| H12 | Arcelor, 8 février 2007 : méthode en deux temps | ✓ CE | — | s.41 | ✓ |
| H13 | CC 2004-496 DC, art. 88-1 ; 2006-540 DC (identité constitutionnelle) | ✓ CC (2004) | — | s.26 | ✓ ; 2006-540 DC apportée par le manuel, non relue sur le site du CC |
| H14 | Boisdet (24 sept. 1990), directives (28 févr. 1992, Rothmans) | ✓ CE | t.2 s.75 | s.26 | ✓ |
| H15 | Fraisse (2 juin 2000), CC 2004-505 DC (19 nov. 2004) | ✓ CE | t.2 s.15 et 74 | s.26, s.41 | ✓ |
| H15' | Rouquette (1999), SNIP (2001) | ✓ CE | non couvert | non couvert | ✗ (source officielle seule) |
| H16 | Art. 54 | ✓ CC | t.2 s.33 | s.27 | ✓ |
| H17 | Deprez (CE, 5 janvier 2005) : priorité d'application | non relue | — | s.41 | ◐ (ouvrage seul) |

### Cours « Le Conseil constitutionnel (DC et QPC) »

| Id | Affirmation | Officiel | Manuels | Verdict |
|---|---|---|---|---|
| C1 | 9 membres, 9 ans non renouvelable, tiers, anciens Présidents de droit | ✓ CC (art. 56) | t.2 s.28 ; CG s.35 | ✓ |
| C2 | Nominations 3 / 3 / 3 ; veto aux trois cinquièmes (révision 2008) | ✓ (art. 56) | t.2 s.28 ; CG s.35 | ✓ (précisions apportées par les manuels) |
| C3 | Saisine a priori : PR, PM, présidents d'assemblées, 60 parlementaires | ✓ | t.2 s.68 | ◐ (CG non trouvé) |
| C4 | Délai d'un mois / huit jours ; suspension de la promulgation | ✓ | t.2 s.28 ; CG s.35 | ✓ |
| C5 | Lois organiques obligatoirement soumises (art. 61 al. 1) | ✓ | t.2 s.29 | ✓ |
| C6 | QPC : art. 61-1, renvoi par le CE ou la Cour de cassation | ✓ | CG s.35 | ✓ |
| C7 | Conditions du filtre (ordonnance, art. 23-2) | ✓ Légifrance (via résumé) | CG s.35 renvoie à un passage non lu | ◐ |
| C8 | Trois mois pour le CE / la Cour de cassation | ✓ (art. 23-4) | CG s.35 | ✓ |
| C9 | Art. 62 : effets a priori / a posteriori | ✓ | t.2 s.34 (cite l'article) | ✓ |
| C10 | Réserves d'interprétation, abrogation différée, législateur négatif | non relue | CG s.35 | ◐ (ouvrage seul) |
| C11 | Le CC statue en trois mois en QPC | non relue (art. 23-10 de l'ordonnance) | t.2 s.28 | ◐ (ouvrage seul) |
| C12 | CC 2010-14/22 QPC (garde à vue), 790 000 mesures en 2009 | ✓ CC (communiqué) | non couvert | ✗ |
| C13 | Origine précise de la QPC dans la révision de 2008 | — | non trouvée | ✗ (reste « à compléter ») |

### Cours « Président, Gouvernement, Parlement, procédure législative »

| Id | Affirmation | Officiel | Manuels | Verdict |
|---|---|---|---|---|
| E1 | Définition du régime parlementaire | — | t.1 p. 282 (lu) | ✓ |
| E2 | Parlementarisme rationalisé, échec, rôle des partis | — | t.1 p. 316 et 334 (lus) | ✓ |
| E3 | 5 ans, suffrage universel direct, deux mandats consécutifs | ✓ | t.2 s.18 ; CG s.31 | ✓ |
| E4 | Septennat → quinquennat (octobre 2000) ; révision de juillet 2008 | — | t.2 s.18 ; CG s.31 | ✓ |
| E5 | Art. 8 (al. 1 et 2) | ✓ | t.2 s.22 (cite l'article) ; CG s.31 | ✓ |
| E6 | Art. 12 : consultation, 20 à 40 jours, pas de nouvelle dissolution dans l'année | ✓ | non trouvé | ✗ (source officielle seule) |
| E7 | Art. 16 : conditions, trente jours | ✓ | t.2 s.19 | ◐ (le délai de soixante jours n'est pas dans l'extrait lu) |
| E8 | Art. 19 : pouvoirs propres et liste | ✓ | t.2 s.19 ; CG s.31 | ✓ |
| E9 | Art. 24 : 577 députés, 348 sénateurs | ✓ | t.2 s.22 ; CG s.33 | ✓ |
| E10 | Art. 45 : navette, CMP (7 députés et 7 sénateurs), dernier mot à l'AN | ✓ | t.2 s.23 | ✓ |
| E11 | Art. 49 al. 2 : un dixième ; majorité des membres composant l'Assemblée | ✓ | t.2 s.23 | ✓ |
| E12 | Art. 49 al. 3 : un seul texte par session (hors budget) ; loi Macron 2015 | ✓ | t.2 s.23 | ✓ |
| E13 | Art. 47 : 40 / 70 jours, ordonnance | ✓ | t.2 s.23 | ✓ |
| E14 | Art. 38 : ordonnances, ratification | ✓ | t.2 s.65 ; CG s.24 | ◐ |
| E15 | Art. 39 et 44 : initiative, amendement | ✓ | t.2 s.23 ; CG s.33 | ◐ |
| E16 | Fait majoritaire, cohabitations depuis 1986, primauté présidentielle | — | t.2 s.21 ; CG s.31 | ✓ |
| E17 | Art. 20 et 21 | ✓ | t.2 s.60 ; CG s.39 | ✓ |
| E18 | Art. 5 | ✓ | t.2 s.19 ; CG s.31 | ✓ |
| E19 | Législatives 2022 ; dissolution du 9 juin 2024 ; Barnier, Bayrou | ✓ (Intérieur, Légifrance, CC) | CG s.31 (2024) | ✓ pour 2024 ; les chiffres de 2022 divergent selon les sources |

### Écarts relevés entre ouvrages et sources

| Point | Ouvrage | Autre source | Décision |
|---|---|---|---|
| Nomination de Lionel Jospin comme Premier ministre | Caporal-Greco s.31 : « en 1993 » (avec la mention 1997-2002) | Même phrase, contradictoire en interne | Coquille de l'ouvrage ; 1997 retenu (dissolution de 1997) |
| Réserve de l'identité constitutionnelle | Attribuée par ma première rédaction à la décision de 2004 | Caporal-Greco : décision n° 2006-540 DC du 27 juillet 2006 | Le cours a été corrigé |

### Points restant sans appui d'un manuel (sources officielles seules ou ouvrage seul)

- Lamotte (1950), Rouquette (1999), SNIP (2001) ; la décision 2010-14/22 QPC ; l'article 12 (détails) ; le délai de soixante jours de l'article 16.
- Ouvrage seul, à relire à la source officielle : Deprez (2005), 2006-540 DC, 2021-940 QPC, réserves d'interprétation, délai de trois mois du Conseil en QPC.
- Origine de la QPC dans la révision de 2008 : non retrouvée dans les manuels.

## Vérification affirmation par affirmation : droit administratif (2026-11-03)

Même méthode (outil `verify_claims.py --books Degoffe`, liste `claims_administratif.json`). Seul ouvrage de droit administratif déposé :
Degoffe, *Droit administratif* (734 pages, numéros de page du fichier PDF). Légende : ✓ confirmé ; ◐ en partie ; ✗ non couvert par l'ouvrage.

### Cours « Actes administratifs et recours pour excès de pouvoir »

| Id | Affirmation | Officiel | Degoffe | Verdict |
|---|---|---|---|---|
| A1 | Lamotte (1950), REP sans texte | ✓ CE | non couvert | ✗ |
| A2 | Duvignères (2002), circulaires impératives, fin de Notre-Dame du Kreisker | ✓ CE | p. 392 | ✓ |
| A3 | REP : deux mois, délai franc (exemple 4 août → 5 octobre) | ✓ Légifrance | p. 425 | ✓ |
| A4 | Absence de publicité, connaissance acquise, tiers | — | p. 425 et 437 | ✓ (ouvrage seul) |
| A5 | Mention des voies de recours, délai raisonnable d'un an | ✓ CE (n° 419220) | p. 416 | ✓ |
| A6 | Danthony (2011) : influence sur la décision ou garantie | ✓ CE | p. 482 (effets de la jurisprudence ; le principe n'y est pas détaillé) | ◐ |
| A7 | Détournement de pouvoir ; Pariset (1875) | ✓ CE (principe) | p. 482 (faits) | ✓ |
| A8 | Exception d'illégalité sans délai | — | p. 69-70, 440, 455 | ✓ (ouvrage seul) |
| A9 | Légalité externe / interne | — | p. 482-483 | ✓ (ouvrage seul) |
| A10 | Permis de construire : pas un acte d'application du document d'urbanisme | — | p. 69-70 | ✓ (ouvrage seul) |
| A11 | Ne pas appliquer un règlement illégal ; appliquer l'acte individuel | — | p. 440 | ✓ (ouvrage seul) |

### Cours « Responsabilité de la puissance publique »

| Id | Affirmation | Officiel | Degoffe | Verdict |
|---|---|---|---|---|
| R1 | Blanco (8 février 1873), loi des 16-24 août 1790 | ✓ CE | p. 53-54 | ✓ |
| R2 | Pelletier (30 juillet 1873), faute personnelle / de service | ✓ CE | p. 683 (note : GAJA n° 2) | ✓ |
| R3 | Faute personnelle détachable du service | ✓ CE | p. 678 | ✓ |
| R4 | Anguet (1911), cumul de fautes, faits (bureau de poste) | ✓ CE (principe) | p. 669 | ✓ (faits : ouvrage seul) |
| R5 | Lemonnier (26 juillet 1918), cumul de responsabilités | ✓ CE | p. 684 (note), table des matières | ✓ |
| R6 | Cames (21 juin 1895), risque | ✓ CE | non couvert | ✗ |
| R7 | Couitéas (30 novembre 1923) | ✓ CE | p. 655 | ✓ |
| R8 | Trois fondements de la responsabilité sans faute | — | p. 636 | ✓ (ouvrage seul) |
| R9 | Préjudice anormal et spécial | ✓ CE | p. 645 et 657 | ✓ |
| R10 | La Fleurette (14 janvier 1938) | ✓ CE | p. 646 | ✓, avec l'écart de date ci-dessous |
| R11 | Ax'ion (2 novembre 2005) | ✓ (cité par la page La Fleurette) | p. 646 et 663 (note) | ✓ |
| R12 | Faute lourde : recul depuis les années 1990, exemple hospitalier | ✓ (étude annuelle 2005, via extrait) | p. 621 | ✓ |
| R13 | TC, 22 janvier 1921 (services publics industriels et commerciaux) | ✓ CE | p. 101 (note) | ✓ |
| R14 | Loi du 9 juillet 1991, art. 16 (consécration législative de Couitéas) | ✓ CE | p. 655 | ✓ |
| R15 | Loi du 9 avril 1898 (accidents du travail) ; collaborateur occasionnel | ✓ CE | non couvert | ✗ |
| R16 | Gardedieu (lois contraires aux engagements internationaux) | ✓ CE (principe, page La Fleurette) | p. 650 | ✓ (détail et date : ouvrage seul / non relevée) |

### Écarts relevés
- Date de la loi de La Fleurette : voir tableau principal (Degoffe : 9 juin 1934 ; Conseil d'État : 29 juin 1934).
- Numéro d'affaire d'Ax'ion : la note de Degoffe (p. 663) donne un numéro apparemment tronqué (« n° 26564 ») ; non repris.

### Restent sans appui d'un manuel
- Lamotte, Cames, loi du 9 avril 1898 (sources officielles seules) ; le principe de Danthony n'est qu'évoqué par l'ouvrage ; les définitions et exemples sur les délais, l'exception d'illégalité et les moyens sont soutenus par l'ouvrage seul (pas de texte officiel relu pour tous).


## Droit des libertés fondamentales : protection juridictionnelle en droit interne (2026-11-04)

Cours `cours-protection-juridictionnelle-libertes` (statut brouillon : première rédaction, vérification affirmation par affirmation à faire).
Ouvrage de base : Denizeau, *Droit des libertés fondamentales* (684 pages, édition 2023 selon l'utilisateur). Premier état des recoupements :

| Affirmation | Officiel | Denizeau | État |
|---|---|---|---|
| Art. 66 (détention arbitraire, autorité judiciaire gardienne) | ✓ CC (texte intégral) | p. 168, 173 | ✓ |
| Référé-liberté : 48 heures, « toutes mesures nécessaires », atteinte grave et manifestement illégale | ✓ Légifrance (L. 521-2, via résumé) | p. 191 | ✓ |
| Conditions détaillées, pouvoirs, appel en quinze jours, faible taux de réussite (9 sur 10 rejetées) | non relue (page du glossaire du CE illisible) | p. 190-192 | ◐ ouvrage seul |
| Droit à un environnement équilibré, liberté fondamentale (20 sept. 2022) | ✓ CE n° 451129 | p. 190 (numéro erroné) | ✓ avec correction |
| Même formule reprise le 18 oct. 2024 (n° 498433) | ✓ Légifrance | — | ✓ (postérieur au manuel) |
| Référé-suspension (L. 521-1), loi du 30 juin 2000 | non relue | Degoffe p. 33 | ◐ ouvrage seul |
| Déféré-laïcité (loi n° 2021-1109, art. 5 ; L. 2131-6 CGCT ; L. 554-3 CJA ; 48 h) | ✓ Légifrance (via résumé) | p. 192 | ✓ |
| CE 21 juin 2022, n° 464648 (Grenoble, laïcité et non neutralité) | ✓ CE (via résumé) | — | ✓ |
| Voie de fait : trois conditions, formule du Tribunal des conflits, déclin (Eucat, Grizivatz) | non relue | p. 174-175 | ◐ ouvrage seul |
| Liste d'ordonnances « liberté fondamentale » (Desperthes, Hyacinthe, Karamoko, etc.) | non relues (sauf l'existence de la décision Ville de Paris, n° 353172) | p. 188-189 | ◐ ouvrage seul |
| CEDH, *Gebremedhin*, 26 avril 2007 : référé-liberté sans recours effectif pour l'asile | non relue | p. 192 | ◐ ouvrage seul |


### Vérification affirmation par affirmation : cours « Protection juridictionnelle des libertés fondamentales » (2026-11-04)

Cette vérification remplace le « premier état » ci-dessus. Légende : ✓ confirmé par une source officielle ; ◐ confirmé en partie ; ✗ ouvrage seul (non retrouvé sur une source officielle).

| Id | Affirmation | Officiel | Ouvrage(s) | Verdict |
|---|---|---|---|---|
| L1 | Art. 66 : détention arbitraire, autorité judiciaire gardienne | ✓ CC | Denizeau p. 168, 173 | ✓ |
| L2 | Principe de 1987 : juge administratif sauf matières réservées à l'autorité judiciaire (86-224 DC, cons. 15) | ✓ CC | Denizeau p. 168 | ✓ ; l'origine « loi du 24 mai 1872 » n'est donnée que par l'ouvrage (le lien avec la loi de 1790, ajouté par erreur dans la première rédaction, a été retiré) |
| L3 | Propriété immobilière : rôle de l'autorité judiciaire, PFRLR (89-256 DC) | ✓ CC (cons. 23) | Denizeau p. 168, 174 | ✓ |
| L4 | Voie de fait : définition actuelle en deux cas (TC, 17 juin 2013, *Bergoend*, n° C3911) | ✓ CE / Légifrance | Denizeau p. 174-176 (approche plus ancienne, « vision étroite ») | ✓ ; **le manuel ne donne pas la formule de 2013**, la définition du cours a été corrigée |
| L5 | Liberté d'aller et venir hors de la liberté individuelle pour la voie de fait (TC, 12 février 2018, C4110) | ✓ Légifrance (via résumé) | — | ◐ |
| L6 | Voie de fait : Eucat (1986), Grizivatz (1987), TC 15 déc. 2008, TC 19 nov. 2007 | ✗ | Denizeau p. 175-176 | ✗ |
| L7 | Affaire Lambert : Cass. ass. plén., 28 juin 2019, pas de voie de fait ni art. 66 | ✗ | Denizeau p. 176 | ✗ |
| L8 | Référé-liberté (L. 521-2) : demande justifiée par l'urgence, mesures nécessaires, atteinte grave et manifestement illégale, « dans l'exercice d'un de ses pouvoirs », 48 heures, loi du 30 juin 2000 | ✓ Légifrance (texte lu) | Denizeau p. 190-191 | ✓ |
| L9 | Appel du référé-liberté : quinze jours, Conseil d'État, 48 heures, sans avocat obligatoire (L. 523-1, R. 523-3) | ✓ Légifrance + CE | Denizeau p. 192 | ✓ |
| L10 | Référé-suspension (L. 521-1) : texte, urgence et doute sérieux ; dernier ressort (L. 523-1) | ✓ Légifrance (texte lu) | Degoffe p. 33 | ✓ |
| L11 | Faible taux de réussite du référé-liberté (9 actions sur 10 rejetées) | ✗ | Denizeau p. 190 | ✗ (chiffre non daté) |
| L12 | Ville de Paris et SEM PariSeine (CE, Sect., 16 nov. 2011, n° 353172 et 353173) : droit à la vie | ✓ CE / Légifrance | Denizeau p. 189 | ✓ |
| L13 | *Tliba* : 30 octobre 2001 (et non novembre) | ✓ CE (dossier « juge des référés ») | Denizeau p. 189 (date erronée) | ✓ avec correction |
| L14 | Desperthes (9 janv. 2001), Hyacinthe (12 janv. 2001) | ◐ dates des décisions anonymisées retrouvées, noms non confirmés | Denizeau p. 188-189 | ◐ |
| L15 | Merzouk (10 avril 2001), Karamoko (10 février 2012), Ceyrat, dignité 2009, etc. | ✗ | Denizeau p. 188-192 | ✗ |
| L16 | *Gebremedhin c. France* (CEDH, 26 avril 2007, n° 25389/05) : violation des art. 13 et 3, pas de recours suspensif de plein droit | ✓ HUDOC (via résumé) | Denizeau p. 192 | ✓ |
| L17 | Environnement équilibré, liberté fondamentale (CE, ord., 20 sept. 2022, n° 451129) ; reprise le 18 oct. 2024 (n° 498433) | ✓ CE / Légifrance | Denizeau p. 190 (numéro erroné) | ✓ avec correction |
| L18 | Déféré-laïcité : loi n° 2021-1109, art. 5 ; L. 2131-6 CGCT ; L. 554-3 CJA ; 48 heures ; CE, 21 juin 2022, n° 464648 | ✓ Légifrance + CE (via résumé) | Denizeau p. 192 | ✓ |

Résumé : le cours conserve trois erreurs d'ouvrage corrigées (numéro 451129, date de *Tliba*, définition de la voie de fait postérieure au manuel). Restent sans source officielle lue : L6, L7, L11, L15 (références de jurisprudence et chiffres du manuel), signalés dans le texte.


### Vérification : cours « La Convention européenne des droits de l'homme et la Cour de Strasbourg » (2026-11-04)

Légende : ✓ confirmé par une source officielle ; ◐ en partie ; ✗ ouvrage seul. Le texte officiel de la Convention (version incluant le Protocole n° 15) a été lu directement (PDF de la Cour, extrait avec PyMuPDF).

| Id | Affirmation | Officiel | Ouvrage(s) | Verdict |
|---|---|---|---|---|
| E1 | Préambule : subsidiarité et marge d'appréciation, sous le contrôle de la Cour | ✓ Convention | — | ✓ |
| E2 | Art. 1, 19, 20, 22, 23 : institution, nombre de juges, élection (liste de trois), neuf ans non rééligible, révocation aux deux tiers | ✓ Convention | — | ✓ |
| E3 | Art. 26 : juge unique, comités de 3, chambres de 7, Grande Chambre de 17 ; juge national membre de droit | ✓ Convention | — | ✓ |
| E4 | Art. 27 : le juge unique déclare irrecevable ou raye ; art. 33, 34 : requêtes interétatiques et individuelles | ✓ Convention | — | ✓ |
| E5 | Art. 35 : épuisement des voies internes, quatre mois, cas d'irrecevabilité, préjudice important | ✓ Convention | de Ravel p. 257 ; Denizeau p. 218 | ✓ |
| E6 | Protocole n° 15 : en vigueur le 1er août 2021, quatre mois pour les décisions internes à partir du 1er février 2022 | ✓ Conseil de l'Europe / greffe (via résumé) | de Ravel p. 27 ; Denizeau p. 57 | ✓ |
| E7 | Art. 41 satisfaction équitable ; art. 43-44 renvoi devant la Grande Chambre (trois mois, collège de cinq juges) | ✓ Convention | Denizeau p. 228 | ✓ |
| E8 | Art. 46 : force obligatoire, Comité des ministres, saisine pour interprétation et manquement (deux tiers) ; art. 47 : avis consultatifs | ✓ Convention | de Ravel p. 243, 249 ; Denizeau p. 239 | ✓ |
| E9 | Protocole n° 16 : en vigueur depuis le 1er août 2018 pour les États qui l'ont ratifié ; la France figure parmi eux | ✓ communiqués du greffier 2018 (12 avril, 1er août), via résumé : 10 États dont la France ; ouverture à la signature le 2 octobre 2013 | de Ravel p. 27 ; Denizeau p. 57 | ◐ ouvrages (19 États au 1er juillet 2023 selon Denizeau, 21 selon le livre de 2024) |
| E10 | Interprétation autonome, évolutive, globale (*Golder*, *Demir et Baykara*) | ✓ Golder 1975, Tyrer 1978 § 31, Demir et Baykara GC 12 nov. 2008 (documents de la Cour, via résumé) | de Ravel p. 41, 45 | ✗ |
| E11 | Marge d'appréciation, exemple de la GPA, grille de l'article 8 § 2 | ◐ liste des buts de l'art. 8 § 2 lue dans le texte de la Convention (PDF) ; marge d'appréciation et GPA non recoupées | de Ravel p. 133 | ◐ |
| E12 | Quatre réunions annuelles du Comité des ministres pour surveiller l'exécution | ✓ Conseil de l'Europe (via résumé) | de Ravel p. 249 | ✗ |
| E13 | Moreira de Azevedo (1991) : trois mois pour payer ; restitutio in integrum | ✗ | Denizeau p. 228 | ✗ |
| E14 | KlimaSeniorinnen (GC, 9 avril 2024) : violation des art. 8 (16 voix contre 1) et 6 § 1, association admise, requérantes individuelles irrecevables | ✓ HUDOC (communiqué, via résumé) | de Ravel p. 45 (citation) | ✓ |
| E15 | Russie : cesse d'être partie le 16 septembre 2022 | ✓ Conseil de l'Europe, résolution de la Cour du 22 mars 2022 (via résumé) | Denizeau p. 58 | ✗ |
| E16 | Ratification française le 3 mai 1974 ; caractéristiques (sans réciprocité, applicabilité directe, primauté, recours individuel) | ✗ | Denizeau (plan du manuel seul) | ✗ |
