---
id: SR-511
title: "Integrity review of the scope progress and closure vocabulary"
type: SpecReview
analysis: integrity
scope: "Branch objects/closure-axes-vocabulary at 7a050c9: whether every normative clause added to FR-061 is grounded, and whether FR-284 asserts anything FR-061 does not declare"
review_set: subset
review_date: "2026-09-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-284, type: reviews }
---
## Summary

Each clause added to FR-061 is traced to the accepted text it rests on, and each
clause FR-284 states is traced to FR-061. The split FR-200 requires — an object
artifact cites, a behavioral requirement declares — holds: FR-284 asserts no rule
that FR-061 does not now carry.

Three clauses are **declarations**, not extractions, and are marked as such
below. They are the reason the change touches a behavioral requirement at all.

## Grounding table

| FR-061 clause | Status | Basis |
| --- | --- | --- |
| The four facts exist with exact scope, authority and boundary identities | extracted | FR-061 Behavior, verbatim, unchanged. |
| Each carries exactly one state from `open` or `closed` | **declared** | Both labels are used for both scopes by FR-059, FR-061, FR-091, FR-094-AC-3, FR-115, and `open` is used on a progress axis by FR-061's own pending row. That the set is *closed* at two members is new. |
| A progress fact is `closed` when its point covers the required boundary | **declared** | FR-094 fixes the condition — settlement is permitted "only when `W >= D` and completeness covers every required source, subject and valuation through `D`". Attaching the label to that condition is new. |
| A closure fact is `closed` when the exact closure authority has closed it | extracted | FR-091 "Closure of that decision scope"; FR-061 "complete under an exact closure authority". |
| The four are independent; no axis populates another | extracted | FR-061 "SHALL NOT use ... any other progress/closure fact as a substitute for any closure axis"; FR-094-AC-3 "closing one axis cannot close or complete another". |
| A `closed` progress fact never closes its own scope's closure fact | extracted | FR-094 "It shall not restamp the surrounding execution as closed"; FR-091 "it does not imply closure of the surrounding execution". |
| `incomplete`, `contradicted`, `not-required` are fifth-axis only | extracted | FR-061 "another closure axis relabelled `not-required` is refused"; FR-247 holds the five-member set; FR-061-AC-9 routes contradiction through supersession. |
| `not-started`, `advanced`, `stalled`, `unobserved` are members of no axis | **declared** | Named by no requirement. Stated explicitly so the labels agent B deleted cannot return by silence. |
| Absence is not a member and is never defaulted | extracted | FR-061 "A strict reader SHALL refuse an omitted required axis"; FR-258 already declared the same absence rule. |

## FR-284 asserts nothing of its own

| FR-284 section | Cited to |
| --- | --- |
| Values table | FR-061 (the declaration), with FR-094's coverage condition and FR-091's closure-authority sentence quoted. |
| The four axes this vocabulary governs | FR-061's decomposition sentence, verbatim. |
| Independence paragraph | FR-061, FR-091, FR-094 — three quotations, no new rule. |
| Not the fifth axis | FR-247, FR-061, FR-059 — quoted. |
| Non-members | FR-247, FR-094-AC-3, FR-094, FR-241, FR-061. The fourth bullet records that four labels are named by no requirement, which is an absence, not an assertion. |
| Identity and comparison | FR-061 for the no-display-name rule, FR-258 for the fact's own key, FR-094 and FR-252 for monotone advance, FR-201 for the digest-domain line. |
| Key order | Declared as lexicographic and explicitly stripped of meaning, per FR-200's two-order discipline. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Three clauses are declarations rather than extractions and are so marked; a reader who assumes the whole vocabulary was already in the prose would be wrong. | FR-061, FR-284 |
| FND-002 | low | `closed` is one label string in two vocabularies (the four axes and FR-247's five), which a careless implementation could treat as one enum. | FR-284, FR-247 |
| FND-003 | low | FR-284's key order puts `closed` before `open` lexicographically, the reverse of the intuitive progression. | FR-284 |

### FND-001

Recorded rather than removed. The alternative was to leave four axes
unenumerated, which is the state that blocks agent B. The three declared clauses
are confined to FR-061; FR-284 cites them and adds nothing.

### FND-002

FR-284 states this directly: "A label equal to `closed` or `open` on one of the
four axes and on the fifth is one label string in two distinct vocabularies.
Equality of the label does not make the axes equal or substitutable."
FR-284-AC-3 tests it from the other side — the three extra members are accepted
on the fifth axis and refused on the four. A shared Rust enum across five axes
would pass AC-2 and fail AC-3.

### FND-003

Intentional and stated. FR-284's key-order bullet says the ordering "carries no
progression from `open` towards `closed`, no precedence and no causal meaning",
and FR-284-AC-7 checks that nothing cites it as one. This follows FR-247, whose
key order carries the same disclaimer for the same reason.
