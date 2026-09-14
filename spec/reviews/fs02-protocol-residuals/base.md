---
id: SR-531
title: "Base review of FS02 protocol residual rulings"
type: SpecReview
analysis: base
scope: "quire-specification #31; FR-050, FR-052, FR-054, FR-056, FR-057, FR-058, FR-111 and FR-210, FR-216, FR-217, FR-220, FR-233, FR-237, FR-239, FR-262"
review_set: subset
evaluated_revision: "task/31 working tree based on dcc22bd"
review_date: "2026-09-13"
---
# Base review of FS02 protocol residual rulings

## Summary

PASS. The packet closes all seven ruling groups owned by issue #31 with exact
value domains, key/equality boundaries, absence behavior and explicit
refusal/incomplete outcomes. It uses the compiled Spec Language contract and
current Protocol/Observation consumers as reality evidence without treating
their partial representations as qualification. FR-281 and its overlapping
FR-061/TC-061/TestMatrix edits are explicitly assigned to the already-active
#40 branch rather than duplicated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: FR-216 originally declared six independent premises although the implemented native contract carries five and derives duplicate permission from the delivery interval. FR-054/216 now define the five-member key, closed ordering variants, inclusive cardinality interval and exact FIFO expression/value scope. | FR-054; FR-216; TC-054 |
| FND-002 | high | Resolved: FR-220's partial branch/join/maximum key would allow omitted semantic graph content or a position-only fingerprint to stand for exact identity. The graph now uses exactly FR-288, with the full typed graph as consistency content and same-key/different-content as refusal. | FR-052; FR-220; FR-288; TC-052 |
| FND-003 | high | Resolved: FR-210 called `participant` a collective synonym despite separate producer/consumer binding kinds. FR-050/217 now distinguish static role, runtime role instance, concrete participant and component; participant uses FR-287 and its binding remains a relationship. | FR-050; FR-210; FR-217; TC-050 |
| FND-004 | medium | Resolved: registration replay and one-effect/multiple-definition behavior were not distinguishable. FR-056/233 now use the exact static compensation-definition plus concrete effect pair and keep policy/relationship facts outside that key. | FR-056; FR-233; TC-056 |
| FND-005 | medium | Resolved: retry counting and commit disposition allowed incompatible interpretations. Distinct admitted attempt keys count once, over-bound candidates never mutate state, and only the exact `commit Node` selection forbids; `never` is not unbounded closure. | FR-057; FR-058; FR-237; FR-239; TC-057; TC-058 |
| FND-006 | high | Resolved: FR-111/262 mixed `unknown` with `missing`, conflated communication subjects and omitted endpoint/domain qualification. The four outcomes, ordered FR-287 endpoints, exact relationship selection and producer digest domain are now explicit. | FR-111; FR-262; TC-141 |
| FND-007 | medium | Resolved during self-review: FR-216 initially retained the old claim that no key component carried a digest even though static role and qualified FIFO-expression keys include FR-288. It now preserves every referenced FR-201 domain and forbids cross-domain equality. | FR-216; FR-288 |
| FND-008 | medium | Resolved during self-review: internality was said not to participate in the role key without reconciling FR-288's source digest. It is now semantic consistency content, not an appended component; changing it requires another FR-288 and same-key/different-content refuses. | FR-217; FR-288 |
| FND-009 | high | Resolved during self-review: four objects retained legacy “no key order” or “no digest component” language after their complete keys were established, violating FR-200's mechanical-order/domain discipline. FR-217/220/233/239 now state exact canonical comparison orders, and FR-233 preserves nested FR-287/288 digest domains. | FR-200; FR-217; FR-220; FR-233; FR-239 |

## Ruling disposition

| Issue #31 group | Authoritative disposition |
| --- | --- |
| Channel premises | Five independent components. Ordering is `unordered` or typed FIFO; delivery is a nonnegative inclusive per-send interval; duplicate permission derives from its maximum. |
| Causal graph | Identity is exactly FR-288. Complete typed graph content is consistency state; an old identity on changed content refuses. |
| Relationship | One exact producer-qualified value with relationship identity and ordered FR-287 endpoints. Outcomes are `resolved`, `missing`, `conflicting`, `ambiguous`. |
| Role internality and participant | `internal`/`external` are exact closed static-role labels. Participant is a separate concrete FR-287 binding object, never a role or role-instance alias. |
| Registration | Key is the qualified declaration-local compensation definition paired with the concrete business-effect key. |
| Retry | Count distinct admitted attempt keys including the initial attempt; replay counts once and refused excess is never retained. |
| Commit | `commit Node` selects the only forbidding boundary for that compensation; `commit never` selects no earlier cutoff and no closure. |

## Coverage and scope

TC-050, TC-052, TC-054, TC-056, TC-057, TC-058 and TC-141 exercise every
changed rule. TM-004 and TM-007 map the new and changed criteria as planned
evidence only. No Rust, executable coverage, release work, native-v1 resource or
TL-* file is changed. Claim-strength closure was removed from #31 when the
active #40 worktree was found to own FR-281 and the shared result files.
