import { describe, expect, it } from "vitest";
import remarkCallouts, { calloutClass } from "./remarkCallouts";

const quote = (label: string) => ({
  type: "blockquote",
  children: [{ type: "paragraph", children: [{ type: "strong", children: [{ type: "text", value: label }] }] }],
});

describe("encadrés annotés", () => {
  it("reconnaît les libellés, avec accents, casse et ponctuation tolérés", () => {
    expect(calloutClass(quote("À retenir"))).toBe("retenir");
    expect(calloutClass(quote("Piège :"))).toBe("piege");
    expect(calloutClass(quote("pour la copie"))).toBe("copie");
  });
  it("laisse une citation ordinaire telle quelle", () => {
    expect(calloutClass(quote("Autre chose"))).toBeNull();
    expect(calloutClass({ type: "blockquote", children: [{ type: "paragraph", children: [{ type: "text", value: "x" }] }] })).toBeNull();
  });
  it("annote l'arbre en profondeur", () => {
    const tree = { type: "root", children: [quote("Exemple")] };
    remarkCallouts()(tree);
    expect(tree.children[0]).toMatchObject({ data: { hProperties: { className: "callout exemple" } } });
  });
});
