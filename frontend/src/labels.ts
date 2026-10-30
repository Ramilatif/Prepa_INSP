// Libellés lisibles pour les identifiants de matière et les statuts de fiche.

const MATIERES: Record<string, string> = {
  droit_public: "Droit public",
  economie: "Économie",
  finances_publiques: "Finances publiques",
  questions_sociales: "Questions sociales",
  europe_international: "Europe et international",
  transitions: "Transitions écologique et numérique",
  note_reflexion: "Note de réflexion",
};

export const matiereLabel = (id: string): string => MATIERES[id] ?? id;

/** Niveau de fiabilité de 0 (brouillon) à 3 (relue par un humain). */
export const STATUTS = [
  "brouillon",
  "sources officielles vérifiées",
  "concordance avec les ouvrages vérifiée",
  "relue par un humain",
];
export const statutLevel = (s: string): number => Math.max(0, STATUTS.indexOf(s));
