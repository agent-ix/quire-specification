---
id: TC-949
title: "Extension definition grammar and typed-node schema members"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-467
    type: verifies
---

# TC-949: Extension definition grammar and typed-node schema members

## Description

Validate extension definitions that carry a grammar schema and a typed-node schema, then mutate each member. Scope: FR-467-AC-1 to FR-467-AC-7.

## Test Procedure

1. Build the `retention` extension of FR-467-AC-1 and validate it alone (positive).
2. Remove, in turn, the grammar schema, the typed-node schema, the node-kind entry of the production and the subnode type of the `days` clause.
3. Change the leading keyword to `function`, then give a second selected extension the keyword `retention`.
4. Give the node kind the spelling of a core node kind, then the spelling of the second extension's node kind.
5. Make the `retention` extension and a second extension depend on each other.
6. Combine a missing member with a keyword collision, and a collision with a cycle.
7. Parse a source using `retention` as a parameter name with and without the extension selected.

## Expected Results

The positive case validates. Step 2 refuses `missing-member` naming the definition and member; steps 3 and 4 refuse `conflicting-definition` naming both definitions and the keyword or node kind; step 5 refuses `definition-cycle` naming both in path order; step 6 reports only the earlier cause in the order of FR-467; step 7 yields identical parses. No refusal emits an approximated or partial extension.
