---
id: FR-290
title: "Capability kind vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-060
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-331
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-146
    type: references
---
# FR-290: Capability kind vocabulary

## Description

When a claim is requested of a backend, the dispatcher SHALL match exactly one
member of this closed vocabulary. The strict reader SHALL keep each reported
claim's premises and outcome independent from every other member.

## Values

| Value | Description |
| --- | --- |
| `value-validity` | Whether a Boolean clause over admitted values, pure expressions and function applications holds for every assignment of its declared domain. |
| `operation-contract` | Whether a state operation's precondition, postcondition, invariant and frame clauses hold for every admitted pre-state and input. |
| `finite-replay` | Whether one finite execution, recorded or generated, conforms step by step to the selected state and protocol semantics. |
| `temporal-satisfaction` | Whether a temporal requirement holds over the traces of its selected temporal profile. |
| `global-conformance` | Finite global protocol conformance under the exact selected closure premises. |
| `monitorability` | Whether the selected admitted fragment can be monitored under the declared observations. |
| `local-projection` | Whether one selected participant projection is defined under its exact premises. |
| `refinement` | Whether one exact protocol/contract refines another under a selected relation. |
| `realizability` | Whether the selected protocol admits a causal implementation strategy. |
| `composition` | Whether selected components compose under independently discharged assumptions. |

## Families

| Family | Members |
| --- | --- |
| value | `value-validity` |
| state-model | `operation-contract` |
| finite-replay | `finite-replay` |
| temporal-trace | `temporal-satisfaction` |
| protocol | `global-conformance`, `monitorability`, `local-projection`, `refinement`, `realizability`, `composition` |

A family groups members; it is descriptive and is not a second vocabulary, and
no request or advertisement names a family. A protocol claim is a claim whose
kind is in the `protocol` family.

## Vocabulary authority

These ten labels are the single capability-kind vocabulary of the Quire
ecosystem. Protocol claims draw their kind from the six `protocol`-family
members. `quire-spec-language`'s `Capability` type, the backend descriptors
registered in its registry and `quire-protocol`'s advertised claim kinds all
take their members from this table. A capability-kind label outside this table
is refused and is never mapped to a member.

The vocabulary identity of this ten-member table is `quire.capability-kind/v1`.
A kind serializes as its exact label, a JSON string. A carrier of capability
kinds declares this identity; a carrier that declares any other identity or
version, or none, is refused as an unsupported version and none of its labels
is read. The member that carries the identity on each cross-repository format
belongs to that format's owner.

## Claim-form assignment

| Claim form | Required kind |
| --- | --- |
| Boolean clause over pure expressions, including function application | `value-validity` |
| Clause containing a `case` expression over a sum type | `value-validity` |
| FR-146 `case` exhaustiveness obligation | none; it is discharged during language admission, and an unproved obligation refuses as `undefined_expression`/`unproved-exhaustiveness` |
| Operation precondition, postcondition or invariant ([FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md)) | `operation-contract` |
| Frame obligation over `modifies`, `creates` and `deletes` sets | `operation-contract` |
| Replay of one finite execution against state or protocol semantics | `finite-replay` |
| Temporal requirement under a finite-trace or infinite-trace profile | `temporal-satisfaction` |
| Protocol analysis claim ([FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)) | its `protocol`-family member |

A proof technique a backend applies is not a capability kind. Unbounded
quantification (FR-346), induction (FR-347), compositional discharge (FR-348)
and mathematical integers (FR-147) discharge a claim that already has one kind
from this table. The negotiation arm of the selected backend settles which
obligation forms it discharges.

## Advertised mode

A backend advertises a set of (kind, mode) pairs. A mode is exactly `bounded`
or `unbounded`. An advertised kind outside this table or a mode outside these
two refuses the backend's registration. A requested item records its claim's
extent as `bounded` or `unbounded`. Candidates match on kind alone. `negotiate_*` then compares the
extent with the one candidate's advertised modes for that kind:

| Extent | Advertised modes for the kind | Disposition |
| --- | --- | --- |
| `bounded` | includes `bounded` | the negotiation arm's own disposition for the IR form |
| `unbounded` | includes `unbounded` | the negotiation arm's own disposition for the IR form |
| `unbounded` | `bounded` only, and a finite bound is available | `requires-bound` |
| `unbounded` | `bounded` only, and no finite bound is available | `unsupported`, warned |
| `bounded` | `unbounded` only | the negotiation arm's own disposition for the IR form |

An unbounded extent never settles `supported` on a bounded-only
advertisement. The representation of an extent and the "finite bound is
available" predicate belong to the boundedness architecture
([quire-spec-language#222](https://github.com/agent-ix/quire-spec-language/issues/222)).

## Candidate set and negotiation

The registry computes, for each requested item, the candidate set: the
backends that advertise every kind the item requires. When the request names a
registered backend, the set is that backend if it advertises every required
kind and is empty otherwise. When the request names an unregistered backend,
the set is marked unknown-backend. The candidate set travels in the
[FR-331](../interfaces/FR-331-backend-provider-envelope.md) request, and
capability negotiation happens at one point, `quire-contract-codegen`'s
`negotiate_*` ([AD-016](../../assurance/AD-016-semantic-family-extension-path.md)):

| Candidate set | Disposition |
| --- | --- |
| unknown-backend | `invalid-request`, naming the unregistered backend |
| empty | `unsupported`, with a warning naming every required kind that the named backend, or when none is named every registrant, leaves unadvertised |
| more than one, and the request names none | `invalid-request`, naming every candidate |
| exactly one | that candidate's negotiation arm settles `supported`, `requires-bound`, `unsupported` (warned) or `invalid-request` under the advertised-mode rules |

Candidates carry no preference order. Registration order, display names and
ambient state never select among them; the caller resolves a multi-candidate
item by naming one backend.

## Identity and comparison

- **Identity kind** — *value*.
- **Key** — the exact label.
- **Key order** — none; declaration order carries no strength or dispatch
  precedence.
- **Causal order** — none.
- **Absence** — a requested/result claim with no kind is refused and never
  defaulted to any member.
- **Backend absence** — a claim no registered backend can discharge settles
  `unsupported` with a warning naming the required capability kind. Backends
  register under one contract. Kani is one registered backend descriptor,
  implemented by `quire-contract-codegen` and `quire-contract-ir` and entered
  in the `quire-spec-language` registry; it is not a `quire-spec-language`
  backend. Further backends register under the same contract.
- **Tool absence** — an item settled `supported` and routed whose pinned tool is
  missing, or does not match the descriptor's pinned tool identity when the
  adapter probes it before the run, records the FR-331 result `unsupported`
  with a warning naming the capability kind, the backend and the expected tool
  identity. It is never a hold, never a refusal of the language form, never a
  fallback to another candidate and never a downgrade to another mode.
- **Digest domain** — not applicable.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-290-AC-1 | The ten exact labels are the complete accepted set; unknown, absent and display-derived labels are refused and never mapped to a member, and a carrier declaring a vocabulary identity other than `quire.capability-kind/v1`, or none, is refused with none of its labels read. | Test (TC-060, TC-061, TC-271) |
| FR-290-AC-2 | Every pair of claim kinds may retain independent execution and outcome states without promotion or premise transfer. | Test (TC-060) |
| FR-290-AC-3 | A non-global claim cannot carry the complete-global-conformance premise set as evidence for its own success. | Test (TC-060, TC-061) |
| FR-290-AC-4 | A claim kind whose required capability has no registered backend settles `unsupported` with a warning naming that capability. | Test (TC-060, TC-271) |
| FR-290-AC-5 | Each claim form in the claim-form assignment table requires exactly its listed kind; a `case` exhaustiveness obligation requires none and requests no backend. | Test (TC-271) |
| FR-290-AC-6 | For each row of the advertised-mode table, negotiation settles the listed disposition; an unbounded extent on a bounded-only advertisement never settles `supported`. | Test (TC-271) |
| FR-290-AC-7 | An item with two or more candidates and no named backend settles `invalid-request` naming every candidate, identically under every registration order; an unregistered named backend settles `invalid-request` naming it. | Test (TC-271) |
| FR-290-AC-8 | A routed item whose pinned tool is missing or mismatched records the FR-331 result `unsupported` with a warning naming the kind, backend and expected tool identity, and no other candidate runs. | Test (TC-271) |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  declares independent dispatch for the six protocol claim kinds.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  declares their canonical result retention and non-substitution rules.
- [FR-331](../interfaces/FR-331-backend-provider-envelope.md) carries the
  advertised (kind, mode) pairs, the candidate set and the per-item result.
- [AD-016](../../assurance/AD-016-semantic-family-extension-path.md) places the
  one negotiation point.

## Status

Proposed shared ruling for `quire-specification#40`, widened to the value,
state-model, finite-replay and temporal-trace families by
`quire-specification#134`.
