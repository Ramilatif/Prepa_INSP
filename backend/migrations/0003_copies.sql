-- Copies déposées par les utilisateurs et leur auto-évaluation.

CREATE TABLE copies (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    epreuve_code TEXT NOT NULL,
    sujet_url TEXT NOT NULL,            -- annale (sujet) travaillée
    duree_minutes INTEGER NOT NULL,     -- durée réelle de l'épreuve, issue de la session
    started_at TEXT NOT NULL,
    submitted_at TEXT,
    elapsed_seconds INTEGER,
    hors_delai INTEGER NOT NULL DEFAULT 0,
    statut TEXT NOT NULL,               -- 'en_cours' | 'deposee' | 'auto_evaluee'
    pdf_path TEXT,
    pdf_pages INTEGER,
    pdf_bytes INTEGER
);

CREATE TABLE auto_evaluations (
    copie_id TEXT PRIMARY KEY REFERENCES copies(id),
    scores_json TEXT NOT NULL,          -- {critere_id: points}
    note REAL NOT NULL,                 -- sur 20
    created_at TEXT NOT NULL
);
