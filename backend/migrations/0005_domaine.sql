-- Un cours appartient à un domaine à l'intérieur de son parcours
-- (ex. parcours « droit-public », domaine « droit-administratif »).
ALTER TABLE fiches ADD COLUMN domaine TEXT;
