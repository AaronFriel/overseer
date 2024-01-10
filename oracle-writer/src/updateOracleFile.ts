import * as fs from "node:fs/promises";
import { escapeRegExp } from "./escapeRegExp";

export interface Card {
  readonly name: string;
  readonly oracleText?: string;
  readonly legalities?: Record<string, "legal" | "not_legal" | "restricted">;

  // Added by us, immutable
  rulesText: string;
  line: number;

  tested?: boolean;
}

export async function updateOracleFile() {
  let cardFile = await fs.readFile("data/oracle.json", "utf-8");

  let cards = (JSON.parse(cardFile) as any[]).flatMap((x): Card | [] => {
    const legalities = x.legalities ?? {};
    const isLegal = (x: string) => x === "legal" || x === "restricted";

    if (
      !(
        isLegal(legalities.legacy) ||
        isLegal(legalities.vintage) ||
        isLegal(legalities.modern) ||
        isLegal(legalities.historic) ||
        isLegal(legalities.standard)
      )
    ) {
      return [];
    }

    return {
      name: x.name,
      oracleText: x.oracle_text,
      rulesText: x.oracle_text ?? "",
      legalities: x.legalities ?? {},
      line: 0,
      tested: false,
    };
  });

  // Let's sort the cards by length of oracle text, so that we can start with the shortest ones.
  for (const card of cards) {
    const regexEscapedName = escapeRegExp(card.name);
    const nameRegexp = new RegExp(`\\b${regexEscapedName}\\b`, "g");
    if (card.oracleText?.match(nameRegexp)) {
      card.rulesText = card.oracleText.replaceAll(nameRegexp, "~");
    }

    if (card.name.includes(",")) {
      const [name, _] = card.name.split(",");
      const nameRegexp = new RegExp(`\\b${regexEscapedName}\\b`, "g");
      if (card.oracleText?.match(nameRegexp)) {
        card.rulesText = card.oracleText.replaceAll(nameRegexp, "~");
      }
    }

    // then, remove any text in parentheses
    card.rulesText = card.rulesText.replace(/\([^)]*\)/g, "");

    // then we'll split the text by lines, and remove any lines that are empty.
    card.rulesText = card.rulesText
      .split("\n")
      .map((x) => x.trim())
      .filter((x) => x)
      .join("\n\n");
  }

  // Create a distinct 'card' for each line of rules text
  cards = cards.flatMap((card) =>
    card.rulesText
      .split("\n")
      .filter((x) => x.length > 0)
      .map((x, idx) => ({
        ...card,
        line: idx,
        rulesText: x,
        legalities: undefined,
      }))
  );

  // Then, to make things simpler, we'll remove all duplicates by rules text
  const rulesTexts = new Set<string>();
  cards = cards.filter((x) => {
    if (rulesTexts.has(x.rulesText)) {
      return false;
    }

    rulesTexts.add(x.rulesText);
    return true;
  });

  cards = cards.sort(
    (a, b) => (a.oracleText?.length ?? 0) - (b.oracleText?.length ?? 0)
  );

  for (const iterator of cards.slice(0, 100)) {
    console.log(iterator.name);
    console.log(iterator.rulesText);
  }

  await fs.writeFile(
    "data/oracle-simplified.json",
    JSON.stringify(cards, null, 2)
  );
}
