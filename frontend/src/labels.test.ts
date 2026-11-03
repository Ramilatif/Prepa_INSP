import { describe, expect, it } from "vitest";
import { formatDuration, groupByDomaine, matiereLabel, statutLevel } from "./labels";

describe("labels", () => {
  it("traduit une matière connue et laisse passer une inconnue", () => {
    expect(matiereLabel("droit_public")).toBe("Droit public");
    expect(matiereLabel("autre")).toBe("autre");
  });
  it("range les cours par domaine, dans l'ordre déclaré, domaines vides omis", () => {
    const items = [
      { id: 1, domaine: "droit-administratif" },
      { id: 2, domaine: "droit-constitutionnel" },
      { id: 3, domaine: null },
      { id: 4, domaine: "droit-administratif" },
    ];
    const groups = groupByDomaine("droit-public", items);
    expect(groups.map((g) => g.id)).toEqual(["droit-constitutionnel", "droit-administratif", "autres"]);
    expect(groups[1].items.map((c) => c.id)).toEqual([1, 4]);
  });
  it("formate une durée", () => {
    expect(formatDuration(4 * 3600 + 5 * 60 + 9)).toBe("4 h 05 min 09 s");
    expect(formatDuration(-3)).toBe("0 h 00 min 00 s");
  });
  it("classe les statuts du moins au plus fiable", () => {
    expect(statutLevel("brouillon")).toBe(0);
    expect(statutLevel("relue par un humain")).toBe(3);
    expect(statutLevel("inconnu")).toBe(0);
  });
});
