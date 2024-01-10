import * as fs from "node:fs/promises";
import * as openai from "openai";
import { ChatCompletionMessageParam } from "openai/resources/index.mjs";
import { FunctionDefinition } from "openai/resources/shared";
import Parser from "tree-sitter";
import { $ } from "zx";
// @ts-ignore
import JavaScript from "tree-sitter-javascript";
import { Card, updateOracleFile } from "./src/updateOracleFile";
import { escapeRegExp } from "./src/escapeRegExp";
import { applyPatchHunk, parsePatch } from "./src/patch";

if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch((error) => {
    console.error(error);
    process.exit(1);
  });
}

const corpusFilename = "../oracle-sitter/corpus/current-test.txt";
const OracleInstructions = `\
You are a helpful, intelligent programming assistant for writing a Tree-Sitter grammar for Magic: The Gathering oracle text.

You are an expert in writing parsers and grammars for natural and programming languages.

You always:
- Deeply understand the domain-specific context, including the function and sequence of keywords and abilities.
- Reuse and extend existing grammar rules before creating new ones; additions should be strictly necessary.
- Start with simple and generic constructs, introducing specificity only as required for correct rule engine functionality.
- Ensure each rule directly contributes to the machine interpretability of the text, without extraneous natural language fluidity.
- Maintain grammar consistency and predictability with uniform rule patterns for easier comprehension and debugging.
- Structure the grammar to be modular and scalable, allowing seamless updates and extensions as the domain evolves.
- Write hierarchically named production rules for readability, ensuring that each node in the syntax tree is interpretable by machines and humans alike.
- Design the grammar to parse qualifiers irrespective of order, ensuring machine interpretability of rule text without imposing human linguistic preferences, unless order impacts game mechanics.
- Write complete code that will run on the first attempt, without requiring the user to fill in any "TODOs" or implementations.

## Tool Formatting

When using tools, emit whitespace between every token that isn't part of a primitive value, e.g.:

\`\`\`json
{ "foo" : "bar" , "baz" : [ 1 , 2 , 3 ] , "quux" : { "thwomp" : [ "a" , "b" , "c" ] } }
\`\`\`
`;

const UpdateGrammarFunction: FunctionDefinition = {
  name: "update-code",
  description: `\
# Update Code

Use this tool to update the code - the grammar or otherwise - of the parser to handle new rules or fix problems.

If there are multiple issues, tackle the most important one. Do not use this tool multiple times.

## Using this tool

You are an automated assistant, and the tool provides a means for you to ideate a solution before implementing it.

Use the \`discussion\` key to role play a conversation between two engineers, and the \`conclusion\` key to describe the patches to implement the change.
`,
  parameters: {
    type: "object",
    properties: {
      discussion: {
        description: `\
A conversation back and forth between two engineers pair programming. They are discussing the problem and the solution until both agree, one is the engineer working on the issue, the other is the code reviewer.

# Values

Each message has a \`role\` and a \`message\`. The \`role\` is either \`engineer\` or \`reviewer\`. The \`message\` is a markdown formatted message.

The first message must be from the engineer. The two engineers must alternate messages.

The last message must be from the reviewer, when the reviewer approves the proposed patch(es). If the reviewer does not agree, the engineer must continue to discuss the problem and propose a new patch(es).

## engineer

The first message discussion item must be a description of the problem from the engineer and their initial thoughts.

The engineer proposes specific edits including any proposed patches in code blocks.

The engineer knows the reviewer's standards, described below, and is receptive to feedback.

The engineer writes the proposed patch(es) in code blocks in their messages.

## Reviewer

The reviewer enforces the patch rules to the letter.
The reviewer knows all patch(es) must be appliable by an automated tool, and patch(es) that are invalid diffs will not be merged, wasting time.
The engineer's contributions must **improve** the codebase, and the reviewer goes above and beyond to provide feedback to the engineer to ensure this.
The proposed patch must contain context sufficient to **uniquely** identify the code.
The reviewer never approves patch(es) that contains only additions, as the automated tool will not know where to insert the new code.

# Conclusion

The discussion continues until the reviewer writes in a message that they approve the proposed patch(es).

Do not continue to the next property of the tool until the reviewer has seen and approved the patch(es).
`,
        type: "array",
        items: {
          type: "object",
          properties: {
            role: {
              type: "string",
              enum: ["engineer", "reviewer"],
            },
            message: {
              type: "string",
              description: `A markdown formatted message.`,
            },
          },
          required: ["role", "message"],
        },
        minItems: 3,
      },
      conclusion: {
        description: `The conclusion of the discussion, repeat the accepted changes here as patches.`,
        type: "object",
        properties: {
          patches: {
            description: `An array of diffs to apply as patches. Be concise, do not add unnecessary comments. Each of these is a diff to apply as a patch.

# Patch Rules

* Hunks conform to standard \`diff\` format, starting with "@@" and including "+", "-", or " " lines.
* Hunks are contiguous, non-overlapping sequences of lines.
* Group consecutive line changes together, deletions first, for readability.
* Deletions and context lines must match the source document's exact whitespace and indentation:
  * This ensures deletions reflect the exact content to be removed.
  * Context lines must accurately represent the surrounding code for correct patch placement.
* Use a unique context line or a deletion for each hunk to pinpoint the patch location.
  * Take great care to produce exact deletions and context. If you don't, the diff will not apply and you will waste the user's time.
* Include up to 5 context lines per hunk unless combining adjacent hunks is justified for efficiency.
* Include several context line or deletion in each hunk, to place the change in context and ensure you write valid syntax for your edit.
  * Consider how an automated tool would interpret your patch. If it contains only additions, where will it insert the new code? The answer: the tool will fail.
* When adding or changing rules, add or update a comment by the rule with an example of the text that should be parsed.

I repeat: do not waste the user's time by giving them incomplete patches. Consider this in your discussion.
    `,
            type: "array",
            items: {
              type: "object",
              properties: {
                filename: {
                  description: `The name of the file to patch. Use this instead of \`---\` and \`+++\` lines in the patch file.`,
                  type: "string",
                },
                patch: {
                  description: `
When fixing compilation errors, fix the error *and* add a comment describing what to do or not to do.

Write code as if you were an engineer skilled in writing compilers and parsers.
When changing code, you write correct syntax the first time. You make sure to include all the necessary context lines and deletions to locate the code.
You write code that will run on the first attempt, without requiring the user to fill in any "TODOs" or implementations.
You are expected to correctly change all lines necessary to make the code work - if changing code requires adjusting the surrounding code to make it work.
You are expected to write code that is correct, or the user will be frustrated and you will waste their time.

## Patch Format

A typical diff format:

\`\`\`diff
@@ (hunk location, not used) @@
  context line
-line to delete
+line to add
  another context line
\`\`\`

### Example:

\`\`\`diff
@@ ... @@
  function foo() {
-  console.log('foo');
+  console.log('bar');
  }
\`\`\`

Take care to note that a deletion serves a similar role as a context line and each must exactly match the source document's text, or else the hunk will not apply.
    `,
                  type: "string",
                },
              },
            },
          },
        },
        required: ["patches"],
      },
    },
    required: ["discussion", "conclusion"],
  },
};

const ReportTextError: FunctionDefinition = {
  name: "report-text-error",
  description: `\
Whenever the Oracle text for a card would contain the card's name, it should be replaced by "~".

Use this tool to report when the text should contain "~" but does not, e.g.: If the card's name is Telim'Tor and you get the rules text:

\`\`\`
Telim'Tor deals 1 damage to any target.
\`\`\`

Use this tool to report the error. The Oracle text should be:

\`\`\`
~ deals 1 damage to any target.
\`\`\`
`,
  parameters: {
    type: "object",
    properties: {
      nameInOracleText: {
        type: "string",
        description: `The string that should be replaced by "~" in the oracle text.`,
      },
    },
    required: ["nameInOracleText"],
  },
};

const SubmitGrammarFunction: FunctionDefinition = {
  name: "accept-grammar",
  description: `If the parse tree is acceptable, submit the grammar for review.

You use this tool to indicate that no further changes are needed to the grammar, and that the grammar should be submitted for human review.`,
  parameters: {
    type: "object",
    properties: {},
  },
};

async function main() {
  // await updateOracleFile();

  // return;

  const oracleFile = JSON.parse(
    await fs.readFile("data/oracle-simplified.json", "utf-8")
  ) as Card[];

  const percentile = 0.01;

  const untestedCards = oracleFile.filter((x) => !x.tested);
  const card =
    untestedCards[
      Math.floor(Math.random() * untestedCards.length * percentile)
    ];

  // const card = oracleFile.find((x) => x.name === "Serum Visions")!;

  console.log(`Starting with ${card.name}`);

  const client = new openai.OpenAI();

  let thread: ChatCompletionMessageParam[] = [];
  let threadPrefix: ChatCompletionMessageParam[] = [];

  const quotedName = `"${card.name}"`;

  // Let's write the corpus content:
  let corpusContent = `\
========
${quotedName}
========

${card.rulesText}

---

`;

  // And we'll write the file to disk.

  await fs.writeFile(corpusFilename, corpusContent, "utf-8");

  thread = [];

  const parser = new Parser();
  parser.setLanguage(JavaScript);

  const grammarFileName = "../oracle-sitter/grammar.js";
  while (true) {
    const currentGrammarFile = await fs.readFile(grammarFileName, "utf-8");

    // Read the file, are there any errors?
    const { parseOutput } = await getParseOutput(quotedName);

    console.log("💥", parseOutput);

    if (parseOutput.trim().length === 0) {
      throw new Error("No parse output");
    }

    threadPrefix = [
      {
        role: "system",
        content: OracleInstructions,
      },
      {
        role: "assistant",
        content: `\
The current grammar.js file is:

\`\`\`
${currentGrammarFile}
\`\`\`
`,
      },
      {
        role: "user",
        content: `\
Let's implement the grammar rules needed to parse ${card.name}. Here's the rules text:

\`\`\`
${card.rulesText}
\`\`\`

Here's how it currently parses using tree-sitter, note carefully any errors or ambiguities:

\`\`\`
${parseOutput}
\`\`\`

If there are any errors, we should fix those, and update the grammar.js file to handle this.

If the syntax tree looks correct, we can accept the current grammar.
`,
      },
    ];

    const output = await client.chat.completions.create({
      messages: [...threadPrefix, ...thread],
      model: "gpt-4-1106-preview",
      temperature: 0.3,
      max_tokens: 2048,
      tools: [
        {
          type: "function",
          function: UpdateGrammarFunction,
        },
        {
          type: "function",
          function: ReportTextError,
        },
        {
          type: "function",
          function: SubmitGrammarFunction,
        },
      ],
    });

    let newGrammarFile = currentGrammarFile;

    // find the list of rules
    for (const choice of output.choices) {
      for (const toolCall of choice.message?.tool_calls ?? []) {
        if (toolCall.function.name === UpdateGrammarFunction.name) {
          const {
            conclusion,
            discussion,
            ...rest
          }: {
            discussion?: {
              role: string;
              message: string;
            }[];
            conclusion?: {
              patches?: { patch: string }[];
            };
            [key: string]: any;
          } = JSON.parse(toolCall.function.arguments);

          for (const { role, message } of discussion ?? []) {
            console.log(`👾 ${role}: ${message}`);
          }
          for (const { patch } of conclusion?.patches ?? []) {
            console.log("👾", patch);
            const hunks = parsePatch(patch);
            for (const hunk of hunks) {
              newGrammarFile = applyPatchHunk(newGrammarFile, hunk);
            }
          }
          console.log("👾", JSON.stringify(rest, null, 2));
        }
      }
    }

    await fs.writeFile(grammarFileName, newGrammarFile, "utf-8");

    break;
  }
}

async function getParseOutput(quotedName: string) {
  $.cwd = "../oracle-sitter";
  const generateOutput = await $`./node_modules/.bin/tree-sitter generate`
    .quiet()
    .nothrow();

  if (generateOutput.exitCode !== 0) {
    return {
      parseOutput:
        "Stdout:\n" +
        generateOutput.stdout +
        "\n\nStderr:\n" +
        generateOutput.stderr +
        `

The error above was a result of calling \`tree-sitter generate\`.
If the error is a rule conflict, often the issue is that we have two similar overlapping rules, and we need to think about a more systematic fix to the grammar, usually involving deleting one of the rules and refactoring the complexity.
If the error is a JavaScript syntax or type error with a stack trace, in your discussion, the engineer should repeat the block of code surrounding the relevant lines, then analyze it and fix it. Remember that the error is often not due to the specific line in the stack trace, but may be contextual.

Additionally, we currently have a number of "precedence" levels - we think these are a mistake and we need to think about simplifying the grammar.

`,
    };
  }

  const updateOutput =
    await $`./node_modules/.bin/tree-sitter test --update --filter ${quotedName}`
      .quiet()
      .nothrow();

  if (updateOutput.exitCode !== 0) {
    return {
      parseOutput:
        "Stdout:\n" +
        updateOutput.stdout +
        "\n\nStderr:\n" +
        updateOutput.stderr,
    };
  }

  let corpusContent = await fs.readFile(corpusFilename, "utf-8");

  corpusContent = corpusContent.replace(/===(=+)/g, "===");
  corpusContent = corpusContent.replace(/---(-+)/g, "---");

  let parseOutput = corpusContent.split("---")[1].trim();

  return { parseOutput };
}
