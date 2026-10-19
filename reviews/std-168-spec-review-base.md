---
id: SR-12820
title: "Base review of PR #275: FR-375 history-row literal production"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@5f88120f77768e54c518426c6a2396a3bf006875; PR #275 diff against origin/main fd7a16b: FR-375 (surface syntax, history rows, FR-375-AC-9), TC-310 (mutations 10 and 11); context: proposals/quire-v1/shared-grammar.md, FR-151, FR-378, open PR #273 (FR-149), quire-spec-language FR-138 and ADR-020 AX-1"
review_set: subset
---
# Base review of PR #275 (STD-168)

## Summary

Ticket: STD-168. The PR replaces the undefined `literal` nonterminal in
FR-375's `history-row` with a new `history-literal` production, widens the
declared type from `type-name` to `type-ref`, states that a non-conforming
literal refuses `ill_typed`/`type-mismatch` at the literal, adds
FR-375-AC-9 and adds TC-310 mutation 10 (old step 10 becomes step 11).

Checked and clean:

- Every nonterminal the new EBNF uses is defined in the shared grammar:
  `type-ref`, `qualified-name` and `exact-number` in the complete-V1 facet
  grammar, `signed-int` in the bounded grammar, `string` lexically. The
  production parses; `'true'`/`'false'` are reserved, so they do not collide
  with `qualified-name`.
- The renumbering is consistent: TC-310 lists 11 items, Expected Results says
  "mutation 10" and "step 11", and no other file cites TC-310 by step number
  (TC-312 cites only the positive declaration).
- FR-375-AC-9 and the TC-310 change collide with no open PR (#274, #273,
  #267, #207, #200, #59 touch neither FR-375, TC-310 nor shared-grammar.md).
- The two changed files validate clean. Whole-scope `quire validate`
  (quire 0.36.2) fails the same documents on this branch and on origin/main
  fd7a16b: 206 with `**/*.md`, 134 with `spec/**/*.md`, byte-identical
  output after path normalisation. The author's 140 is a different glob; the
  failures are pre-existing and not caused by this PR.
- No hash, digest or pin is introduced.

## Verdict

Mergeable after the two medium findings are decided; no high finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `5` for `Int[0, 4]` refusing `ill_typed`/`type-mismatch` has no rule behind it. FR-151 defines conformance between member types, not literal-in-range checks, and no type-model FR says an integer literal outside an `Int[lo,hi]` range is a type mismatch. QSL FR-138 states that static conformance compares the native type `Integer` and not its range, and open PR #273 handles out-of-range integer values with `undefined_expression`/`unproved-range`. One implementer will admit `5` (native type conforms), another will refuse it as `unproved-range`, and TC-310 mutation 10's oracle fails both. State the rule in FR-375: a history literal conforms when its value lies in the declared type's value set, with the range included, and refuses `type-mismatch` otherwise. Then flag QSL FR-138 to match. | spec/functional/analysis/FR-375-declare-model-refinement.md:144-146; spec/functional/analysis/FR-375-declare-model-refinement.md:188; spec/test-cases/TC-310-refinement-declaration-admission.md:48-49 |
| FND-002 | medium | TC-310 mutation 10 does not test most of FR-375-AC-9. It admits only the integer `0`. Nothing checks admission of `true`/`false`, a string, an `exact-number`, an enum member or a negative `signed-int`, and nothing tests the clause "a literal outside that list ... is refused" (for example `set[]`, `none` or `x + 1`). That clause also names no refusal code. A parser that implements only `signed-int` passes TC-310 in full. Add one admitted vector per literal kind against a matching bounded type, plus one outside-the-list vector, and name its code (presumably `invalid_syntax`). | spec/test-cases/TC-310-refinement-declaration-admission.md:48-49; spec/test-cases/TC-310-refinement-declaration-admission.md:67-68; spec/functional/analysis/FR-375-declare-model-refinement.md:188 |
| FND-003 | low | Two new lines were not rewrapped and exceed the file's line width. | spec/functional/analysis/FR-375-declare-model-refinement.md:83; spec/test-cases/TC-310-refinement-declaration-admission.md:67 |
