import test from "ava";
import { applyPatchHunk, parsePatch } from "./patch";
import { readFileSync } from "node:fs";

test("Applies simple patches", (t) => {
  const document = `\
function foo() {
  console.log('foo');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@
 function foo() {
-  console.log('foo');
+  console.log('bar');
 }
`;

  const expected = `\
function foo() {
  console.log('bar');
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 1);

  const result = applyPatchHunk(document, hunks[0]);

  t.deepEqual(result, expected);
});

test("Applies patches with only an addition", (t) => {
  const document = `\
function foo() {
  console.log('foo');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@
   console.log('foo');
+  console.log('bar');
 }
`;

  const expected = `\
function foo() {
  console.log('foo');
  console.log('bar');
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 1);

  const result = applyPatchHunk(document, hunks[0]);

  t.deepEqual(result, expected);
});

test("Applies patches with only a deletion", (t) => {
  const document = `\
function foo() {
  console.log('foo');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@
 function foo() {
-  console.log('foo');
 }
`;

  const expected = `\
function foo() {
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 1);

  const result = applyPatchHunk(document, hunks[0]);

  t.deepEqual(result, expected);
});

test("Applies patches with multiple additions", (t) => {
  const document = `\
function foo() {
  console.log('foo');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@
 function foo() {
   console.log('foo');
+  console.log('bar');
+  console.log('baz');
 }
`;

  const expected = `\
function foo() {
  console.log('foo');
  console.log('bar');
  console.log('baz');
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 1);

  const result = applyPatchHunk(document, hunks[0]);
  t.deepEqual(result, expected);
});

test("Applies patches with multiple deletions", (t) => {
  const document = `\
function foo() {
  console.log('foo');
  console.log('bar');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@
 function foo() {
-  console.log('foo');
-  console.log('bar');
+  return 42;
 }
`;

  const expected = `\
function foo() {
  return 42;
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 1);

  const result = applyPatchHunk(document, hunks[0]);
  t.deepEqual(result, expected);
});

test("Applies patches with multiple additions and deletions", (t) => {
  const document = `\
function foo() {
  console.log('1');
  console.log('2');
  console.log('3');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@
 function foo() {
-  console.log('1');
+  console.log('Hello,');
   console.log('2');
+  console.log('World!');
-  console.log('3');
 }
`;

  const expected = `\
function foo() {
  console.log('Hello,');
  console.log('2');
  console.log('World!');
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 1);

  const result = applyPatchHunk(document, hunks[0]);
  t.deepEqual(result, expected);
});

test("Handles patches with multiple hunks", (t) => {
  const document = `\
function foo() {
  console.log('1');
  console.log('2');
  console.log('3');
}
function bar() {
  console.log('a');
  console.log('b');
  console.log('c');
}
  `;
  const patch = `\
@@ 0,1 1,2 @@ -- irrelevant
 function foo() {
   console.log('1');
-  console.log('2');
+  console.log('Hello,');
   console.log('3');
 }
@@ 10,11 12,13 @@ -- irrelevant
 function bar() {
   console.log('a');
+  console.log('World!');
-  console.log('b');
   console.log('c');
 }
`;

  const expected = `\
function foo() {
  console.log('1');
  console.log('Hello,');
  console.log('3');
}
function bar() {
  console.log('a');
  console.log('World!');
  console.log('c');
}
  `;

  const hunks = parsePatch(patch);
  t.deepEqual(hunks.length, 2);

  let result = document;
  for (const hunk of hunks) {
    result = applyPatchHunk(result, hunk);
  }
  t.deepEqual(result, expected);
});

test("Fix hang", (t) => {
  const document = readFileSync(
    "/home/friel/c/overseer/oracle-sitter/grammar.js",
    "utf-8"
  );

  const patches = [
    `\
@@ -1,1 +1,1 @@
 cost: ($) =>
+action_cost: ($) => seq($.action_verb, $.subject),
     ),
`,
    `\
@@ ... @@
 `,
  ];

  let result = document;
  for (const patch of patches) {
    const hunks = parsePatch(patch);
    for (const hunk of hunks) {
      result = applyPatchHunk(result, hunk);
    }
  }

  t.snapshot(result);
});
