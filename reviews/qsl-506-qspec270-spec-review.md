---
id: SR-12099
title: "QSL-506 QSpec PR #270 spec review: unbounded Population<T> node (FR-322-AC-74)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@9a7d251; spec/objects/interfaces/FR-322-checked-package-artifact.md (Derived type forms and forwarding, Population link of a reaches, FR-322-AC-74); spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md; proposals/checked-package-v2/fixtures/adverse.json (unbounded_population_cases); tests/checked_package_v2.rs (derived_shape_semantic_refusal, append_population_chain, tc_233_a_model_population_with_no_max_is_the_unbounded_population, tc_233_unbounded_population_preimage_vectors)"
review_set: subset
---

## Summary

Ticket: QSL-506. PR quire-specification#270 (branch unbounded-population-node), one commit 9a7d251 on origin/main a08dc63, reviewed by reading. The builds were `quire validate` and the scoped `checked_package_v2` tests (filters `population`, `derived_form`). Method: spec-review with the integrity and failure-domain lenses applied to the diff. Context read: QSL FR-333-AC-2 and FR-334-AC-1..AC-4 on quire-spec-language origin/main, QSpec FR-153-AC-9 and FR-144-AC-9/AC-13, the FR-322 reaches population paragraph merged from #268, and open QSpec PRs #267 and #271.

Independent checks:

- PO1 preimage (from the test) is already RFC 8785 canonical; sha256 = 859d6b01a132a14711a8a4568e24a7ab54e3255b78bbcc62b14e4293dbf71f9a, the published key.
- PO1 with `body.members` set to `[]`, re-canonicalized: sha256 = f65a8239f4fbc1b6e75bfeed4a578e1c219a27f155a82ddfb7aeeb22657c3b8b, equal to the published PO0 and to the test's PO0 const. Same S1 `semantic_type`.
- FR-322-AC-74 is absent from origin/main and from every open QSpec PR (#271 adds FR-464..466 ACs and touches FR-322 elsewhere; #267 touches FR-154). #268 is merged into the base.
- The schema `literal` term allows a string `value` for `value_kind: integer`, so the `"three"` and `"-1"` cases pass the V2 schema and reach the reader; the empty aggregate is schema-valid (`DomainNode` constrains only tag and form).
- quire validate on FR-322 and TC-233: same 6 FR-322 diagnostics as a08dc63 (value_object `id`/`fields`, legacy properties form, EARS line 64, ac shape line 1469 moved to 1487 by the +18 lines). TC-233 output identical.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-74's refusal claim (`invalid_package`/`invalid-derived-shape` at the population node) and the fixture's `expected_code`/`expected_cause` are not asserted. The test matches `Err(ReadRefusal::GraphShape(_))`, a variant the reader also returns from the application-shape step, discards the node id, and never reads the fixture fields; the sibling RM-POP-MAX test asserts `GraphShape(population_id)`. | tests/checked_package_v2.rs:12915-12922; tests/checked_package_v2.rs:2883; proposals/checked-package-v2/fixtures/adverse.json:73-75 |
| FND-002 | medium | "the reader does not treat the absent max as 0 or as any bound" cannot fail: the reader only admits or refuses, and `max` 0 is also admitted, so a reader reading absent as 0 passes the test. The testable form is a key claim (PO0 differs from the `max` 0 node key), which the test does not check (it compares PO0 with PO1 and with S1 only). | spec/objects/interfaces/FR-322-checked-package-artifact.md:1542; tests/checked_package_v2.rs:12965-12970 |

## Verdict

PASS. No major defect. Both findings are mediums, recorded, not blocking.

Clean, examined:

- PO0 and PO1 recompute independently to the published keys.
- The unbounded node cannot share a key with another meaning: the preimage keeps `node_tag`/`semantic_form`, so the empty-aggregate `model_population` differs from the dimensionless `compound_unit` `aggregate{[]}`, from every bounded `Population<O>[N]` and from `Set<Reference<O>>` S1. One meaning has one key: there is a single unbounded body form and `max` is only valid as the sole member.
- The reader rule (empty, or exactly one canonical nonnegative integer `max`) is stated once in the matrix row, the required-`max` paragraph and the shape-check sentence, and the code implements exactly that (`match members.as_slice()`).
- The reaches population paragraph is consistent with #268: role-end type rule, `dependencies` rule and occurrence rule unchanged; the absent `max` is admitted and states no bound.
- AC-74 says "the V2 schema" (singular) and the test checks `schema.json` (whole package, and its `SemanticNode` def for the preimage wire); claim and test agree.
- TC-233 rows name the five cases and both preimage vectors; both new tests carry `#[trace("TC-233", "FR-322-AC-74")]`.
- Agrees with QSpec FR-153-AC-9 (a `Population<T>` declares no maximum) and gives QSL FR-334-AC-2 the distinct node it needs.
- No QSL internal, IR name or ticket id appears in the QSpec text.

## Provenance

Files read at 9a7d251; comparison base origin/main a08dc63. Tools: quire 0.36.2 (engine 0.50.2), quoin 0.28.3, python3 hashlib/json for the key recomputation. Scoped tests at 9a7d251: tc_233_unbounded_population_preimage_vectors, tc_233_a_model_population_with_no_max_is_the_unbounded_population, tc_233_a_reaches_population_link_names_a_population_over_its_role_end_type, tc_322_derived_form_key_mutations and tc_322_recomputed_derived_forms_refuse_semantic_mutations, 5 passed, exit 0 (the old absent-`max` refusal is no longer asserted anywhere). Read-only: nothing was edited; the temporary target dir was deleted.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The refusal cases now assert `GraphShape` at the population node's id and the fixture's `expected_code` and `expected_cause`. |
| FND-002 | fixed | AC-74 states the key claim: PO0 differs from the key of the same node with `max` `0`. `tc_233_unbounded_population_preimage_vectors` recomputes that key and asserts it. |
