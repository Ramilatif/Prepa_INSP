// Plugin remark : transforme une citation Markdown dont la première phrase commence par un libellé
// en gras (« **À retenir** », « **Piège** »…) en encadré annoté, stylé par la classe CSS associée.

/** Libellé (en minuscules) → classe CSS de l'encadré. */
export const CALLOUTS: Record<string, string> = {
  "à retenir": "retenir",
  piège: "piege",
  exemple: "exemple",
  "pour la copie": "copie",
  note: "note",
  définition: "definition",
};

// Nœud minimal de l'arbre Markdown (mdast) : on évite une dépendance de types pour si peu.
interface Node {
  type: string;
  value?: string;
  children?: Node[];
  data?: { hProperties?: Record<string, unknown> };
}

/** Renvoie la classe de l'encadré si le nœud `blockquote` commence par un libellé connu. */
export function calloutClass(blockquote: Node): string | null {
  const paragraph = blockquote.children?.[0];
  const first = paragraph?.type === "paragraph" ? paragraph.children?.[0] : undefined;
  if (first?.type !== "strong") return null;
  const label = (first.children?.[0]?.value ?? "").trim().replace(/[\s:.]+$/, "").toLowerCase();
  return CALLOUTS[label] ?? null;
}

function walk(node: Node): void {
  if (node.type === "blockquote") {
    const cls = calloutClass(node);
    if (cls) node.data = { ...node.data, hProperties: { className: `callout ${cls}` } };
  }
  node.children?.forEach(walk);
}

export default function remarkCallouts() {
  return (tree: Node) => walk(tree);
}
