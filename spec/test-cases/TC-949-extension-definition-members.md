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

Validate extension definitions that carry a grammar schema and a typed-node schema, then mutate each member. Scope: FR-467-AC-1 to FR-467-AC-10.

## Test Procedure

1. Build the `retention` extension of FR-467-AC-1 and validate it alone (positive).
2. Remove, in turn, the grammar schema, the typed-node schema, the node-kind entry of the production and the subnode type of the `days` clause.
3. Change the leading keyword to `function`, then give a second selected extension the keyword `retention`.
4. Give the node kind the spelling of a core node kind, then the spelling of the second extension's node kind.
5. Make the `retention` extension and a second extension depend on each other.
6. Combine a missing member with a keyword collision, and a collision with a cycle.
7. Parse a source using `retention` as an identifier, including as a member declaration name inside a model body, with and without the extension selected.
8. Give one definition two productions with one leading keyword, then two node entries with one node kind.
9. Parse `retention Audit { subject: "audit", days: 30 }`, then the forms without a declared name, without braces and with a trailing separator, then forms with clauses out of order, an unknown clause and a repeated clause.
10. Give a node entry a `keyword` matching no production, then two node entries one `keyword`.

## Expected Results

The positive case validates. Step 2 (including a clause with no entry and a clause with no type) refuses `missing-member` naming the definition and member; steps 3 and 4 refuse `conflicting-definition` naming the keyword or node kind and the definitions involved (the extension and the core grammar for a core keyword or node kind, both extensions otherwise); step 5 refuses `definition-cycle` naming both in path order; step 6 reports only the earlier cause in the order of FR-467; step 7 yields identical parses; step 8 refuses `conflicting-definition` naming the definition and both indices; step 9 accepts the first and refuses the others `invalid_syntax`/`unexpected-token` or `unexpected-end` at the source; step 10 refuses `missing-member` for the unmatched keyword and `conflicting-definition` for the shared one. Every refusal of steps 1 to 8 and 10 is `invalid_package`. No refusal emits an approximated or partial extension.
