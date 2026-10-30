import { describe, expect, it } from "vitest";
import { matiereLabel, statutLevel } from "./labels";

describe("labels", () => {
  it("traduit une matière connue et laisse passer une inconnue", () => {
    expect(matiereLabel("droit_public")).toBe("Droit public");
    expect(matiereLabel("autre")).toBe("autre");
  });
  it("classe les statuts du moins au plus fiable", () => {
    expect(statutLevel("brouillon")).toBe(0);
    expect(statutLevel("relue par un humain")).toBe(3);
    expect(statutLevel("inconnu")).toBe(0);
  });
});
