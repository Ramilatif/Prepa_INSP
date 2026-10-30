-- Schéma initial. Types portables SQLite/PostgreSQL : TEXT, INTEGER, REAL.
-- Horodatages en TEXT ISO 8601 UTC, identifiants en TEXT.

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    pseudo TEXT NOT NULL UNIQUE,
    majeure TEXT,            -- 'droit_public' | 'economie' | NULL (égalité)
    mineure TEXT,
    active_session_id TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    config_json TEXT NOT NULL
);

CREATE TABLE fiches (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    titre TEXT NOT NULL,
    matiere TEXT NOT NULL,
    epreuves_json TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    statut TEXT NOT NULL,
    sources_json TEXT NOT NULL,
    body_md TEXT NOT NULL,
    a_verifier INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE flashcards (
    id TEXT PRIMARY KEY,
    fiche_id TEXT NOT NULL REFERENCES fiches(id),
    kind TEXT NOT NULL,
    recto TEXT NOT NULL,
    verso TEXT NOT NULL,
    source_ref TEXT NOT NULL
);

CREATE TABLE card_states (
    user_id TEXT NOT NULL REFERENCES users(id),
    card_id TEXT NOT NULL REFERENCES flashcards(id),
    ease REAL NOT NULL,
    interval_days INTEGER NOT NULL,
    repetitions INTEGER NOT NULL,
    due_at TEXT NOT NULL,
    last_reviewed TEXT NOT NULL,
    PRIMARY KEY (user_id, card_id)
);

CREATE TABLE signalements (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    cible_type TEXT NOT NULL,   -- 'fiche' | 'flashcard' | 'corrige'
    cible_id TEXT NOT NULL,
    message TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'ouvert',
    created_at TEXT NOT NULL
);
