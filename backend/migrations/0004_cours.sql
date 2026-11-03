-- Les contenus rédigés sont de deux types : « fiche » (synthèse) et « cours » (chapitre développé).
-- Un cours appartient à un parcours (ex. droit-public) et s'y classe par ordre.
ALTER TABLE fiches ADD COLUMN type TEXT NOT NULL DEFAULT 'fiche';
ALTER TABLE fiches ADD COLUMN parcours TEXT;
ALTER TABLE fiches ADD COLUMN ordre INTEGER;
