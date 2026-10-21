---
id: SR-13700
title: 'Spec integrity review of QSpec PR #323: FR-322 integer literal operands and AC-92 (QSL-940)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@de9af077e; git diff origin/main...de9af077e (PR #323): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md; cross-checked against agent-ix/quire-spec-language@ca37b7138 (PR #1161) FR-093, FR-357, FR-358; ticket QSL-940'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: QSL-940. PR: quire-specification#323, head de9af077e. Method: spec-review with the spec-integrity sub-analysis. The PR adds an "Integer literal operands" paragraph to FR-322's body grammar, FR-322-AC-92 and a TC-233 procedure paragraph. The paragraph is a lead default, flagged as such in the PR body.

Cross-check against QSL at ca37b7138:

- QSL FR-093's Literal row makes an integer literal operand of `quire.op.integer.add`, `sub`, `mul` or `negate`, and of an integer `eq` or `ne`, an inline `literal{type, value_kind, value}` typed at the builtin `Integer` node, with no node. Every other literal operand is a reference to its node: an ordering's literal, a literal narrowed to a bounded integer peer of an equality, and composite or non-integer equality operands. FR-322's paragraph names the same six operators, the same type and the same exceptions. The narrow case is the literal under `quire.op.numeric.narrow`, and "any other position" covers the ordering case.
- QSL FR-358 step 6 makes an integer literal operand `inline_literal` with the singleton range and a composite literal operand `graph_child`, and FR-357 requires an `InlineLiteral` operand to be an inline integer literal at its position. Both agree with the paragraph.
- No other QSpec text says an integer operator's literal operand is a reference. The searches covered the old keys, the old package ids, "literal operand", "literal node" and "graph child". The body-grammar Member and Leaf rows admit both forms, so nothing contradicts the new paragraph.
- The new text contains no changelog wording apart from FND-003.
- `quire validate --scope . 'spec/**/*.md'`: 134 failed documents on the merge base and at the head. The output is identical apart from one warning's line number in FR-322 moving from 1757 to 1767.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 "Integer literal operands" paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md:771-779 | examined |
| FR-322-AC-92 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1840 | examined |
| TC-233 AC-92 procedure paragraph | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:382 | examined |
| FR-322 body grammar table and composite-subterm rule | spec/objects/interfaces/FR-322-checked-package-artifact.md:757-800 | context_only |
| FR-322 `application_node_preimage` | spec/objects/interfaces/FR-322-checked-package-artifact.md:365-380 | context_only |
| QSL FR-093 Literal row, FR-357 operand check, FR-358 step 6 | quire-spec-language@ca37b7138 spec/functional/FR-093-lower-checked-value-expressions-to-fr-322-terms.md:139, FR-357-replay-scalar-parity-claims.md:96-115, FR-358-settle-a-composite-bounded-shadow-item.md:256-273 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The paragraph is a producer obligation only and says nothing about a reader. It sits in the body grammar section, whose rule is that a reader refuses a body outside the grammar as `malformed_wire`. One reader could refuse an `integer.add` that references an integer literal node, and another could admit it, as the suite's reader does (restoring the reference form fails only the fixture-shape tests). The FR should say which. | spec/objects/interfaces/FR-322-checked-package-artifact.md:771-779,787-790 |
| FND-002 | low | FR-322-AC-92 checks only literals "typed at the builtin Integer node", but the statement requires every integer literal operand of the six operators to be inline and typed at Integer. An `integer.add` operand that references an integer literal node of an `integer_range` type passes the AC's scan. Only the fixed inline count in the bound test catches it, and only for the named applications. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1840; tests/checked_package_v2.rs:14654-14658 |
| FND-003 | low | AC-92's "while the literal nodes for `7` and `-2` remain for the operands that reference them" describes a change from an earlier fixture, not a present property. State it directly, for example that the `integer.div` and `integer.rem` applications reference the literal nodes `7` and `-2`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1840 |

## Verdict

Not clean, but consistent with the language spec. FR-322's paragraph matches QSL FR-093's Literal row, FR-357's operand check and FR-358 step 6, including the narrowed-literal and non-integer-equality exceptions. No stale QSpec statement says these operands are references. The touched files add no validation failure. FND-001 is the gap that matters: the paragraph does not say whether a reader refuses the reference form. FND-002 and FND-003 are wording fixes to AC-92.

## New findings (disposition pass 1)

Head 406c078b5. `quire validate --scope . 'spec/**/*.md'` still fails 134 documents, the same set as the merge base, with only one FR-322 warning line moving from 1757 to 1769.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | The new sentence "a reader SHALL admit the reference form of such an operand as well ... and SHALL NOT refuse it for that form" has no acceptance criterion and no test. AC-92 covers only the producer form of the published packages. No published package or vector now carries the reference form under one of the six operators, so a reader that refused it would fail nothing. | spec/objects/interfaces/FR-322-checked-package-artifact.md:779-782,1842 |

## New findings (disposition pass 2)

Head 1eba298ac. The FND-004 fix adds FR-322-AC-93, `positive-integer-literal-references.json` and `tc_233_a_reader_admits_the_reference_form_of_an_integer_literal_operand`. Replacing the fixture with the inline form, or giving it a stale key, fails the test. `quire validate --scope . 'spec/**/*.md'` still fails the same 134 documents as the merge base.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | FR-322-AC-92 now contradicts FR-322-AC-93. AC-92 says "In every published package" no operand of the six operators is a `reference` to an integer `value`/`literal` node, and TC-233 says "Scan every published package". AC-93 publishes `fixtures/positive-integer-literal-references.json`, whose `three` `integer.add` references two such nodes. The AC-92 test passes only because `positive_fixtures()` leaves the new fixture out, a scope the AC text does not state. AC-92 and the TC-233 sentence need to scope out the reference-form fixture by name, or state that they cover producer-output packages. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1842-1843; spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:382; tests/checked_package_v2.rs:12342 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
