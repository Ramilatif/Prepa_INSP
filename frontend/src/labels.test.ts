import { describe, expect, it } from "vitest";
import { formatDuration, matiereLabel, statutLevel } from "./labels";

describe("labels", () => {
  it("traduit une matière connue et laisse passer une inconnue", () => {
    expect(matiereLabel("droit_public")).toBe("Droit public");
    expect(matiereLabel("autre")).toBe("autre");
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
