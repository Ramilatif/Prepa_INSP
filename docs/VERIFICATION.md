# Vérification des fiches et cours : désaccords entre sources

Règle (PROMPT.md) : si l'ouvrage et la source officielle divergent, la source officielle la plus récente l'emporte
lorsque l'écart s'explique par une évolution des textes ou des chiffres ; sinon on ne tranche pas seul et on consigne
ici et dans PROGRESS.md.

| Date | Fiche / cours | Point | Ouvrage | Source officielle | Décision |
|---|---|---|---|---|---|
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
