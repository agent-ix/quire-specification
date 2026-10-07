---
id: SR-1310
title: "Code review of PR #196 (QSL-654): tests/backend_provider_v1.rs ProofBound kind test"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@de376ea8da8678238b60b23decdf471ec61aca5c; PR #196 diff against origin/main a73731e (commits 3e350df and de376ea); tests/backend_provider_v1.rs (proof_bounds, tc_431_proof_bounds_carry_their_domain_kind, tc_431_bounded_descriptors_carry_domains) with proposals/backend-provider-v1/{schema.json,vectors.json} as data"
review_set: subset
---
# Code review of PR #196 (QSL-654)

## Summary

Ticket: QSL-654. This reviews the new tests
`tc_431_proof_bounds_carry_their_domain_kind`, with its helper `proof_bounds`,
and `tc_431_bounded_descriptors_carry_domains`.
The Rust-review lane is folded in. The five new adverse vectors run through
the existing `tc_431_adverse_vectors_are_refused`, which checks that each
refusal is located at its patch. I also re-ran them with Python jsonschema:
each is refused at the patched `ProofBound`.

Examined:
- `proof_bounds` helper (examined)
- tc_431_proof_bounds_carry_their_domain_kind (examined)
- tc_431_bounded_descriptors_carry_domains (examined)
- the six new adverse vectors through tc_431_adverse_vectors_are_refused (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new test checks the schema's layout and a list of vector names, not the rule. It asserts that the `$ref` strings at two fixed JSON paths equal `#/$defs/DomainKind`, and that five adverse names exist in vectors.json. Its one behavioural check, that each positive `kind` is in the enum, already follows from positive admission. The pairing rule at the core of AC-21 has two of 21 cells exercised: integer to collection, and quantity. Replace the layout and name assertions with a behavioural matrix through `definition("ProofBound")`. Assert that each of the 7 kinds times 3 bound types is admitted exactly when it is collection or population with cardinality, integer with integer-range, or recursive with depth. All 21 cells match `finite_kind` today, as measured with Python jsonschema. | tests/backend_provider_v1.rs:526-574 |
| FND-002 | low | `tc_431_bounded_descriptors_carry_domains` asserts that the adverse vector named `bounded-descriptor-without-domains` exists. That is a name-inventory check. `tc_431_adverse_vectors_are_refused` already proves the vector is refused at its patch. The rest of the test is behavioural and good: it builds an unbounded-only descriptor, a bounded one without `domains`, and a bounded one with an empty `domains`, and checks each through `definition("BackendDescriptor")`. Drop the name assertion. | tests/backend_provider_v1.rs:601-607 |

## Verdict

The helper is correct and idiomatic, and the trace tags match BP-08 and BP-09.
`tc_431_bounded_descriptors_carry_domains` is a good behavioural test, apart
from its name check. The findings are about what the tests assert: FND-001
pins file layout instead of exercising the pairing rule AC-21 states, and
FND-002 is a redundant name-inventory check. Fix both in this PR. Once they
are fixed it is mergeable. The suite was not rerun at de376ea, per the
coordinator, while the build lock is busy. The author reports
backend_provider_v1 at 9 passed.

## Dispositions

Round 1, reviewed at 3d98a2e (fix range b8a2c42..3d98a2e).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 3d98a2e |
| FND-002 | fixed | 3d98a2e |
