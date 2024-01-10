type Line = {
  type: "context" | "addition" | "deletion";
  text: string;
};
type Hunk = Line[];

export function parsePatch(patchText: string): Hunk[] {
  // split on lines
  const lines = patchText.split("\n");
  // find each hunk
  const hunks: Hunk[] = [];
  for (const line of lines) {
    if (
      line.startsWith("diff --git") ||
      line.startsWith("+++") ||
      line.startsWith("---")
    ) {
      continue;
    }
    switch (line[0]) {
      case "@": {
        hunks.push([]);
        break;
      }
      case "+": {
        hunks[hunks.length - 1].push({
          type: "addition",
          text: line.slice(1),
        });
        break;
      }
      case "-": {
        hunks[hunks.length - 1].push({
          type: "deletion",
          text: line.slice(1),
        });
        break;
      }
      default:
        hunks[hunks.length - 1].push({
          type: "context",
          text: line.slice(1),
        });
    }
  }

  return hunks;
}

export function applyPatchHunk(document: string, hunk: Readonly<Hunk>) {
  // if the hunk starts with context or deletion, use that to find the location to patch
  let documentLines = document.split("\n");
  let reversed = false;

  // Either the first line must give us context or the last line must give us context, if the last line gives us context we reverse the document and the hunk and then apply the patch, then reverse the document again.
  if (hunk[0].type !== "context" && hunk[0].type !== "deletion") {
    hunk = hunk.slice().reverse(); // immutable, must slice and reverse
    documentLines = documentLines.reverse();
    reversed = true;
  }

  if (hunk[0].type !== "context" && hunk[0].type !== "deletion") {
    throw new Error(
      "Hunk must contain context - either context lines or deleted lines to locate the patch"
    );
  }

  docLoop: for (const [docLineNumber] of documentLines.entries()) {
    const hunkContext = hunk.filter((x) => x.type !== "addition");

    if (hunkContext.length === 0) {
      // We have a major issue, we can't find the context of the patch.
      throw new Error("Hunk must contain context lines to locate the patch");
    }
    if (docLineNumber + hunkContext.length > documentLines.length) {
      // We've reached the end of the document and we don't have enough lines to match the hunk
      continue docLoop;
    }

    // find a section of the document that allows us to consume all of the next context & deletion lines
    for (const [hunkLineNumber, hunkLine] of hunkContext.entries()) {
      const docLine = documentLines[docLineNumber + hunkLineNumber];
      if (!docLine.includes(hunkLine.text.trim())) {
        continue docLoop;
      }
    }

    // we found a matching section of the document, let's apply the patch
    let patchOffset = docLineNumber;
    for (const [hunkLineNumber, hunkLine] of hunk.entries()) {
      if (hunkLine.type === "addition") {
        documentLines.splice(patchOffset + hunkLineNumber, 0, hunkLine.text);
      } else if (hunkLine.type === "deletion") {
        documentLines.splice(patchOffset + hunkLineNumber, 1);
        patchOffset -= 1;
      }
    }
  }

  if (reversed) {
    documentLines = documentLines.reverse();
  }
  return documentLines.join("\n");
}
