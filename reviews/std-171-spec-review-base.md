---
id: SR-12822
title: "Base review of PR #279: state-graph graph-predicate production"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@5193b7d306a98ac4eac8950974af9c188db1c899; PR #279 diff against origin/main 1da546f: proposals/quire-v1/shared-grammar.md (state-graph EBNF, recognition rules, six fragment rows), FR-390 (Admission, FR-390-AC-1, FR-390-AC-2), TC-330 (SG-R09..SG-R13); context: quire-spec-language FR-165 and ADR-022 at origin/main 49bcf9a, quire-spec-language PR #968"
review_set: subset
---
# Base review of PR #279 (STD-171)

## Summary

Ticket: STD-171. The PR replaces the plain `block` / `expr` predicate
positions of the three state-graph clauses with new `graph-body` and
`graph-predicate` productions. These admit unary and binary temporal
operators with an optional `interval`, and `{ fairness }` at the start of a
body. FR-390 now says both constructs are parsed and then refused at
admission. TC-330 gains vectors SG-R09 to SG-R13.

Checked and clean:

- Every nonterminal in the new EBNF is defined: `comparison`, `expr`,
  `parameter`, `ident` (core), `interval` and `fairness` (complete temporal
  section). There is no left recursion: `graph-implication` is
  right-recursive, and the other productions use repetition.
- `and`, `or` and `implies` are not ambiguous between `graph-*` and `expr`.
  `comparison` stops below `conjunction`, so a top-level connective can only
  come from `graph-conjunction` / `graph-disjunction` / `graph-implication`.
- Author point (2): the `interval` used is the complete-section redefinition
  `[uint, (uint | '*')]`, the right one for a complete facet. It matches
  QSL's `P::Interval`.
- Author point (3): keeping `let`/`if` with plain-`expr` conditions and
  bindings, so temporal operators appear only in result positions, is a
  sound design. It needs the spec to say so; see SR-12824 FND-003.
- Author point (4): a parenthesised group is a real EBNF ambiguity.
  `(c.ok)` derives both as `comparison` → `primary '(' expr ')'` and as
  `'(' graph-predicate ')'`. The two trees mean the same thing, so the
  prose tie-break is enough for parentheses. It does not cover `not`
  (FND-001).
- TC-330 oracle strength: SG-R09 expects "parses ... not a syntax error",
  so a grammar that still rejects `eventually` at S1 fails it. SG-R10 and
  SG-R11 cover an interval, a parenthesised temporal operand, a binary
  operator, and the `from` and `to` positions.
- Vector and AC ids do not collide with open PRs #273, #278 (STD-172) or
  #280 (STD-173). None of them touches TC-330 or FR-390, and #278's
  shared-grammar hunk (around line 450) does not overlap this one.
- No hash, digest or pin is introduced.

## Verdict

Not mergeable as is: FND-001 is a real grammar ambiguity that changes
the meaning of existing, temporal-free predicates. FND-002 is an oracle a
correct parser fails. FND-003 is low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `not` is ambiguous between the two `not` productions, and the two trees differ in meaning. `graph-unary`'s first alternative `'not', graph-unary`, and its `comparison` alternative via `comparison` → `sum` → `product` → `unary = 'not', unary` both derive a leading `not`. For `not c.n = 2` one tree is `not (c.n = 2)` (graph-unary `not` over `comparison`). The other is `(not c.n) = 2` (the core `unary` reading, which is how the same text parsed before this PR as `expr`). The first tree type-checks and the second is ill-typed, so two conforming parsers admit or refuse the same source. The paren rule at line 724 resolves only parenthesised groups. QSL #968 resolves the question by trying a temporal-free predicate as `Expression` first. Fix: replace the paren sentence with a general rule: a `graph-predicate` (or parenthesised group) that derives as an `expr` is read as that `expr`, and the `graph-*` alternatives apply only where it does not. That also covers author point (4). | proposals/quire-v1/shared-grammar.md:596-600; proposals/quire-v1/shared-grammar.md:724-726; proposals/quire-v1/shared-grammar.md:130-133 |
| FND-002 | medium | The SG-R13 locus is wrong. In `possible P using v { c.done; fair weak M::Cfg::step }`, `graph-body` has parsed `c.done` as a complete `graph-predicate` and expects `}` (or an operator). The first token it cannot accept is the `;` after `c.done`, not `fair`. A correct parser reports `unexpected-token` at `;`, and the vector's "syntax error at `fair`" fails it. QSL #968 (`StateGraphBody`) also stops at `;`. Fix: expect the error at `;`, or say "a syntax error" without a locus. The fragment row at shared-grammar.md:751 states no locus and is fine. | spec/test-cases/TC-330-state-graph-claim-syntax-and-routing.md:39; proposals/quire-v1/shared-grammar.md:584 |
| FND-003 | low | `[a,*]` is admitted in every `graph-*` interval position, but line 760 admits `[a,*]` only on a clause that selects `quire.temporal.infinite-trace/v1`, and a state-graph claim selects no temporal profile. It is unstated which refusal applies: the profile refusal or FR-390's `expression-form` at the operator. No vector covers `[a,*]`. Fix: state that FR-390's operator refusal applies whatever the interval, or add an `eventually[0,*]` vector. | proposals/quire-v1/shared-grammar.md:595; proposals/quire-v1/shared-grammar.md:598; proposals/quire-v1/shared-grammar.md:760-762 |

## Dispositions

Round 1, reviewed at fad2cd29132c763e218b918230ed71c48a210a0c.

The code lead asked whether a prose rule is enough, with the EBNF itself
left ambiguous. It is. The rule at shared-grammar.md:724-731 is total and
deterministic: read the `expr` derivation wherever one exists, and use
`graph-unary`'s `not`, `(` and temporal alternatives only for a phrase that
has no `expr` derivation. It applies phrase by phrase, so it also fixes
`eventually c.x and not c.n = 2`: its `not c.n = 2` operand derives as an
`expr`. The file already settles syntax this way elsewhere (the `case`
scrutinee rule). A disambiguated production would need `graph-unary` to
exclude every operand that derives as an `expr`, which EBNF cannot state
compactly. SG-R16's oracle names the tree, `(not c.n) = 2`, so it
distinguishes the two readings.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
