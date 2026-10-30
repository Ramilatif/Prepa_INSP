-- Annales / rapports du jury et grilles d'évaluation par épreuve.

CREATE TABLE annales (
    id TEXT PRIMARY KEY,
    annee INTEGER,                 -- NULL pour les sujets fictifs
    concours TEXT NOT NULL,
    type TEXT NOT NULL,            -- 'sujet' | 'meilleure_copie' | 'rapport_jury' | 'sujet_fictif'
    epreuve TEXT NOT NULL,
    epreuve_code TEXT,
    format TEXT NOT NULL,          -- 'reforme' | 'ancien' (l'ancien format sert de matière thématique)
    url_source TEXT NOT NULL,
    fichier TEXT
);

CREATE TABLE grilles (
    epreuve_code TEXT PRIMARY KEY,
    nom TEXT NOT NULL,
    config_json TEXT NOT NULL
);
