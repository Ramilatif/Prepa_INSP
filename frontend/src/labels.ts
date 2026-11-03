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

/** Intitulés des parcours de cours. */
export const PARCOURS: Record<string, string> = {
  "droit-public": "Droit public",
  economie: "Économie et politiques économiques",
};

/** Domaines d'un parcours, dans l'ordre d'affichage (identifiant → intitulé). */
export const DOMAINES: Record<string, [string, string][]> = {
  "droit-public": [
    ["droit-constitutionnel", "Droit constitutionnel"],
    ["droit-administratif", "Droit administratif"],
    ["libertes-fondamentales", "Libertés et droits fondamentaux"],
    ["droit-union-europeenne", "Droit de l'Union européenne"],
  ],
};

/** Regroupe des cours d'un parcours par domaine, dans l'ordre déclaré ; les domaines inconnus passent à la fin. */
export function groupByDomaine<T extends { domaine: string | null }>(
  parcours: string,
  items: T[],
): { id: string; label: string; items: T[] }[] {
  const known = DOMAINES[parcours] ?? [];
  const groups = known.map(([id, label]) => ({ id, label, items: items.filter((c) => c.domaine === id) }));
  const others = items.filter((c) => !known.some(([id]) => id === c.domaine));
  if (others.length > 0) groups.push({ id: "autres", label: "Autres", items: others });
  return groups.filter((g) => g.items.length > 0);
}

export const TYPE_ANNALE: Record<string, string> = {
  sujet: "Sujet",
  meilleure_copie: "Meilleure copie",
  rapport_jury: "Rapport du jury",
  sujet_fictif: "Sujet fictif de l'INSP",
};

/** Épreuves du format réformé, pour lesquelles une grille existe. */
export const EPREUVES = [
  ["note_reflexion", "Note de réflexion"],
  ["op_economie", "Épreuve opérationnelle d'économie"],
  ["op_droit_public", "Épreuve opérationnelle de droit public"],
  ["qrc", "Questions à réponses courtes"],
  ["cas_pratique", "Cas pratique"],
] as const;

export const matiereLabel = (id: string): string => MATIERES[id] ?? id;

/** Niveau de fiabilité de 0 (brouillon) à 3 (relue par un humain). */
export const STATUTS = [
  "brouillon",
  "sources officielles vérifiées",
  "concordance avec les ouvrages vérifiée",
  "relue par un humain",
];
/** Formate des secondes en « 4 h 05 min 09 s ». */
export function formatDuration(totalSeconds: number): string {
  const s = Math.max(0, Math.floor(totalSeconds));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${h} h ${pad(m)} min ${pad(s % 60)} s`;
}

export const statutLevel = (s: string): number => Math.max(0, STATUTS.indexOf(s));
