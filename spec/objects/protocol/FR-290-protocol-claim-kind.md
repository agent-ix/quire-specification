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
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/FR-390
    type: references
  - target: ix://agent-ix/quire-specification/FR-395
    type: references
  - target: ix://agent-ix/quire-specification/FR-396
    type: references
  - target: ix://agent-ix/quire-specification/FR-397
    type: references
  - target: ix://agent-ix/quire-specification/FR-407
    type: references
  - target: ix://agent-ix/quire-specification/FR-419
    type: references
  - target: ix://agent-ix/quire-specification/FR-390
    type: references
  - target: ix://agent-ix/quire-specification/FR-483
    type: references
  - target: ix://agent-ix/quire-specification/FR-484
    type: references
  - target: ix://agent-ix/quire-specification/FR-485
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-507
    type: references
---
# FR-290: Capability kind vocabulary

## Description

Each requested item SHALL require exactly one member of this closed
vocabulary. The strict reader SHALL keep each reported claim's premises and
outcome independent from every other member.

## Values

| Value | Description |
| --- | --- |
| value_validity | Wire spelling `value-validity`. Whether a Boolean clause over admitted values, pure expressions and function applications holds for every assignment of its declared domain. |
| operation_contract | Wire spelling `operation-contract`. Whether a state operation's precondition, postcondition, invariant and frame clauses hold for every admitted pre-state and input. |
| finite_replay | Wire spelling `finite-replay`. Whether one finite execution, recorded or generated, conforms step by step to the selected protocol semantics. |
| temporal_satisfaction | Wire spelling `temporal-satisfaction`. Whether a temporal requirement holds over the traces of its selected temporal profile. |
| global_conformance | Wire spelling `global-conformance`. Finite global protocol conformance under the exact selected closure premises. |
| `monitorability` | Whether the selected admitted fragment can be monitored under the declared observations. |
| local_projection | Wire spelling `local-projection`. Whether one selected participant projection is defined under its exact premises. |
| `refinement` | Whether one exact protocol refines another under a selected relation; its two subjects are protocol models (FR-177). |
| `realizability` | Whether the selected protocol admits a causal implementation strategy. |
| `composition` | Whether selected components compose under independently discharged assumptions. |
| probabilistic_satisfaction | Wire spelling `probabilistic-satisfaction`. Whether a probabilistic claim's bound holds over a model subject under its workload or every scheduler ([FR-407](../../functional/analysis/FR-407-state-probabilistic-claims.md)), with `statistical` or `exact` evidence. |
| `schedulability` | Whether a task set is schedulable, or a task's response time is bounded, under a named scheduling policy ([FR-419](../../functional/analysis/FR-419-analyze-task-set-schedulability.md)). |

## Families

| Family | Members |
| --- | --- |
| value | `value-validity` |
| state-model | `operation-contract` |
| finite-replay | `finite-replay` |
| temporal-trace | `temporal-satisfaction` |
| protocol | `global-conformance`, `monitorability`, `local-projection`, `refinement`, `realizability`, `composition` |
| probabilistic | `probabilistic-satisfaction` |
| real-time | `schedulability` |

A family is a descriptive grouping of members. Requests, advertisements and
results name members only. A protocol claim is a claim whose kind is in the
`protocol` family. The protocol result, fragment and backend contracts
([FR-240](FR-240-canonical-protocol-result.md),
[FR-291](FR-291-admitted-analysis-fragment.md),
[FR-292](FR-292-analysis-backend.md)) carry protocol-family members only.

## Vocabulary authority

These twelve labels are the single capability-kind vocabulary of the Quire
ecosystem. `quire-spec-language`'s `Capability` type, the backend descriptors
registered in its registry and `quire-protocol`'s advertised claim kinds all
take their members from this table.

The vocabulary identity of this table is `quire.capability-kind/v1`. A kind
serializes as its exact label, a JSON string. Each carrier selects the
vocabulary as follows:

| Carrier | Selection |
| --- | --- |
| [FR-331](../interfaces/FR-331-backend-provider-envelope.md) manifest and request | its `capability_vocabulary` member, exactly `quire.capability-kind/v1` |
| FR-240 result, FR-291 fragment, FR-292 backend | their own contract identities, each of which selects `quire.capability-kind/v1` |
| Any other cross-repository carrier | the member its owner assigns, exactly `quire.capability-kind/v1` |

A reader SHALL refuse a carrier whose selection is any other identity or
version, or is absent, with `invalid_capability`/`unknown-vocabulary`, and
SHALL read none of its labels.

A reader SHALL refuse a kind that is absent or `null` with
`invalid_capability`/`absent-kind`, and any other label that is not byte-equal
to a member with `invalid_capability`/`unknown-kind`. A refused label is
reported with its exact received bytes and no replacement member.

## Claim-form assignment

Each requested item is one claim and requires exactly one kind. A clause's
kind comes from the clause's own claim form; an expression nested in it, such
as a function application or a `case` expression, adds no kind. A function
declaration carries no clause; each scalar operation application in its body
is a claim of its own, as the table below states. The third column names the
declaration family of the claim ("Declaration-family applicability").

| Claim form | Required kind | Declaration family |
| --- | --- | --- |
| Boolean clause over pure expressions, including function application | `value-validity` | `value` |
| Clause containing a `case` expression over a sum type | `value-validity` | `value` |
| Scalar operation application in a function body ([FR-146](../../functional/expressions/FR-146-check-total-pure-functions.md)): the application is defined and its result lies in its result bound, for every argument assignment under which its path condition holds (the guards it is nested under: the condition of an enclosing `if`, required true in the `then` branch and false in the `else` branch, and, for the right operand of an enclosing `and`, `or` or `implies`, its left operand, as FR-146's propagation rule carries guard facts) | `value-validity`, one claim per occurrence | `value` |
| Range narrowing (`quire.op.numeric.narrow`) in a function body | none. When its operand is a scalar operation application, its target range is that application's result bound; otherwise its FR-146 range obligation discharges it | none |
| FR-146 `case` exhaustiveness obligation | none; it is discharged during language admission, and an unproved obligation refuses as `undefined_expression`/`unproved-exhaustiveness` | none |
| Operation precondition, postcondition or invariant ([FR-196](../../functional/ir-proof/FR-196-generate-kani-contracts.md)) | `operation-contract` | `state-model` |
| Frame obligation over `modifies`, `creates` and `deletes` sets | `operation-contract` | `state-model` |
| Replay of one finite execution against protocol semantics | `finite-replay` | `protocol` |
| Temporal requirement under a finite-trace or infinite-trace profile | `temporal-satisfaction` | `temporal-trace` |
| State-graph claim (`possible`, `always possible`, `unique path`) over a model subject ([FR-390](../../functional/analysis/FR-390-analyze-state-graph-claims.md)) | `temporal-satisfaction`; its form is an obligation form | `temporal-trace` |
| `hyper` clause over `behaviours` of model subjects ([FR-395](../../functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md)) | `temporal-satisfaction`; its form is an obligation form | `temporal-trace` |
| Model claim of a `relation` clause over model operations ([FR-397](../../functional/analysis/FR-397-check-step-relations-over-model-operations.md)) | `temporal-satisfaction`; its form is an obligation form | `relation-claim` |
| Code claim of a `relation` clause over model operations whose every operation has an implementation binding | `operation-contract`; one claim per relation, with every declared invariant of each execution's model as its pre-state premise | `relation-claim` |
| Refinement between two protocols ([FR-177](../../functional/protocol/FR-177-check-protocol-refinement.md)) | `refinement` | `protocol` |
| Whole refinement between two model subjects ([FR-377](../../functional/analysis/FR-377-decide-model-refinement.md)) | `temporal-satisfaction` | `relation-claim` |
| Per-step simulation obligation of a model refinement, or a relation-family refinement gate between two operation contracts | `operation-contract`; each implication between the two contracts' clauses is one `operation-contract` claim, and a model refinement's step-row and initial-state obligations are such implications | `relation-claim` |
| Claim transferred through a model refinement ([FR-380](../../functional/analysis/FR-380-transfer-claims-through-refinement.md)) | `temporal-satisfaction` | none; a transfer item is planner-local and is not a request item (FR-380) |
| Implementation guarantee of a temporal claim ([FR-488](../../functional/ir-proof/FR-488-attach-a-temporal-guarantee-to-implementation-rust.md), [FR-490](../../functional/ir-proof/FR-490-spell-and-check-an-implementation-guarantee-declaration.md)) and its fairness-correspondence items | `temporal-satisfaction` | none; planner-local, not a request item |
| Initialization, transition-simulation, frame or no-divergence obligation of an implementation guarantee (FR-490) | `operation-contract`, carrying `correspondence` | `relation-claim` |
| `assume-guarantee` composition of component guarantees ([FR-489](../../functional/analysis/FR-489-carry-liveness-and-fairness-premises-through-composition.md), [FR-491](../../functional/analysis/FR-491-spell-and-check-a-component-composition-declaration.md)) | `composition` | none; planner-local, not a request item |
| Composition of component guarantees under `protocol-refinement`, `refinement-transfer` or `interface` (FR-491), and FR-176's fixed point over an accepted cycle | `composition` | `protocol` |
| FR-353 abstraction relation | none; it is a premise of the `operation-contract` claim it binds | none |
| Other protocol analysis claim ([FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)) | its `protocol`-family member | `protocol` |
| Probabilistic claim of any FR-407 form | `probabilistic-satisfaction` | `probabilistic` |
| Temporal claim over a timed subject under any profile, and the derived time-lock-freedom item ([FR-417](../../functional/temporal/FR-417-settle-time-lock-freedom.md)) | `temporal-satisfaction` | `temporal-trace` |
| `schedulable` or `response` claim over a task set | `schedulability` | `real-time` |
| Hybrid-model claim ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)) | `temporal-satisfaction` | `temporal-trace` |
| Derived deadlock-freedom item over a model subject ([FR-366](../../functional/temporal/FR-366-check-deadlock-freedom.md)) or a protocol subject (FR-429), `item_kind` `deadlock-freedom` ([FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)) | `temporal-satisfaction` | `temporal-trace` |
| Derived race-freedom item over a protocol subject (FR-438), `item_kind` `race-freedom` (FR-368) | `temporal-satisfaction` | `temporal-trace` |

A scalar operation application is an application in a function body whose
operation identity has one of the prefixes `quire.op.integer` (FR-044,
FR-147), `quire.op.rational` (FR-149, including `quire.op.rational.narrow`),
`quire.op.decimal` (FR-140), `quire.op.ieee` (FR-148), `quire.op.quantity`
(FR-142, including `quire.op.quantity.convert`), `quire.op.numeric`,
`quire.op.text` and `quire.op.enum` (FR-141), other than the range narrowing
(`quire.op.numeric.narrow`). Every other application in a body (`let`, `if`,
a Boolean connective, Boolean, structural or reference equality, a projection, an option read, a call, a
dispatch, a collection or model operation, and an operation whose prefix is not
listed, such as `quire.op.string`, `quire.op.uuid` and `quire.op.timestamp`)
is not a claim of its own. The
application's result bound is the target range of the range narrowing whose
operand it is, when there is one, and otherwise its own result type. Each
occurrence of a scalar operation application in the body is one claim, so an
application written twice is two claims; an occurrence in the function's
termination measure is not in the body. The claim is requested beside the
function's FR-146 admission: FR-146's definedness and range obligations still
refuse an unproved occurrence when the body is linked, and the claim does not
discharge or replace them.

Unbounded quantification (FR-346), induction (FR-347), compositional discharge
(FR-348), mathematical integers (FR-147), the three state-graph forms (FR-390)
and the hyperproperty and step-relation forms (FR-396) are obligation forms
inside a claim that has one kind from this table. The selected backend's negotiation arm
settles the obligation forms it discharges.

## Declaration-family applicability

A *declaration family* is the kind of declaration that holds a request item's
claim. The declaration families form their own closed vocabulary, separate
from the Families table above, which groups kinds. Six labels are spelled as
kind groups are. `relation-claim` has no kind group; it is named apart from
the checked-package node tag `relation`, which also covers relationships and
populations. `finite-replay` is a kind group but not a declaration family.

| Declaration family | Claims it holds | Kinds that apply |
| --- | --- | --- |
| `value` | Boolean value clauses, including those with a `case` expression, and the scalar operation applications of function bodies | `value-validity` |
| `state-model` | State operation preconditions, postconditions and invariants, and frame obligations | `operation-contract` |
| `temporal-trace` | Temporal clauses under any profile, including over a timed or hybrid subject; state-graph claims; `hyper` clauses; the derived deadlock-freedom, time-lock-freedom and race-freedom items | `temporal-satisfaction` |
| `relation-claim` | `relation` clauses over model operations (the model claim and the code claim); model refinements (the whole refinement and each per-step or initial-state obligation); relation-family refinement gates | `temporal-satisfaction`, `operation-contract` |
| `protocol` | Protocol analysis claims, refinement between two protocols and a replay against protocol semantics | `finite-replay`, `global-conformance`, `monitorability`, `local-projection`, `refinement`, `realizability`, `composition` |
| `probabilistic` | Probabilistic claims of every FR-407 form | `probabilistic-satisfaction` |
| `real-time` | `schedulable` and `response` claims over a task set | `schedulability` |

For each family, the kinds that apply are exactly the kinds the claim-form
assignment table assigns to the rows whose "Declaration family" cell names
that family. One declaration may therefore hold claims of two kinds; a
`relation` clause with an implementation binding is an example. A row whose
cell is "none" requests no item and takes no label. That covers a union's
`case` exhaustiveness obligation, an FR-353 abstraction relation and a
planner-local FR-380 transfer item.

### Reading the family from a request item

`negotiate_*` reads an item's declaration family from the request, never from
the item's kind. When the item carries an `item_kind` other than `formula`
(FR-331, FR-368), the item kind decides:

| `item_kind` | Declaration family |
| --- | --- |
| `deadlock-freedom` | `temporal-trace` |
| `time-lock-freedom` | `temporal-trace` |
| `race-freedom` | `temporal-trace` |
| `refinement` | `relation-claim` |

This holds whatever the derived item's anchor node is: a deadlock-freedom or
race-freedom item anchored on a `protocol`/`protocol_clause` node is
`temporal-trace`.

Every other item takes the family of the checked-package node its `claim`
names, by the node's tag and `semantic_form` (FR-322) and its occurrence role:

| Claim node (`tag`/`semantic_form`) and occurrence | Declaration family |
| --- | --- |
| a Boolean `expression` node, occurrence role `claim` or `declaration` (FR-484 forms `clause` and `case-clause`) | `value` |
| an `expression` node of a scalar operation application, occurrence role `expression`, in the body of a `function` node (FR-484 form `scalar-operation`) | `value`, the family of the enclosing function declaration |
| `state`/`state_clause`, occurrence role `claim` | `state-model` |
| `state`/`operation_anchor`, occurrence role `anchor` (frame obligation) | `state-model` |
| `temporal`/`temporal_clause` | `temporal-trace` |
| `claim`/`state_graph` | `temporal-trace` |
| `claim`/`hyperproperty` | `temporal-trace` |
| `claim`/`step_relation` | `relation-claim` |
| `relation`/`refinement` | `relation-claim` |
| `claim`/`probabilistic_claim` | `probabilistic` |
| `claim`/`schedulability` | `real-time` |
| any `protocol` node | `protocol` |

A claim that names a node of no row has no declaration family. Examples are
any other `expression` occurrence, a `claim`/`verification_claim`,
`analysis_claim` or `synthesis_request` node, and a node absent from the
package. Every kind is inapplicable to such a claim (see below).

### The refusal

If a request item's admitted kind does not apply to its claim's declaration
family, then `negotiate_*` SHALL settle the item `invalid-request` with
`invalid_capability`/`inapplicable-kind`. The cause's payload is:

- `request_index`, the item's request index;
- `capability`, the item's kind label;
- `family`, the claim's declaration family label, absent when the claim has no
  family (an absent `family` compares only to absence, FR-272).

No negotiation arm and no engine runs the item, and no other kind is
substituted for its kind.

`negotiate_*` SHALL NOT refuse `inapplicable-kind` an item whose kind is the
kind the claim-form assignment table assigns to its claim's form.

- **Order.** This is the first item rule after kind admission. An item with an
  absent or unknown kind settles `absent-kind` or `unknown-kind` and is not
  checked for applicability. An inapplicable item settles `inapplicable-kind`
  whatever its extent, its evidence kind, its statistical method, its
  candidates or the backend it names. It never settles `absent-evidence-kind`,
  `unknown-evidence-kind`, `not-statistical`, `absent-extent` or a
  candidate-table cause. FR-331's refusals before consumption are not item
  dispositions: an item's `check` or `requirements` member on a kind that does
  not carry it, a malformed `extent` and a missing `candidates` each refuse the
  whole request before any item rule runs. This check does not change them.
- **Record.** An inapplicable item stays in the FR-331 request at its
  position. Its `request_index`, the other items' indices and the request
  identity are those of the same request without this check. Its terminal
  record is its FR-331 disposition `invalid-request` carrying the cause, with
  an accounting record `settled_by` `disposition`. It has no result entry, no
  artifact and no counterexample.
- **Scope.** The refusal settles only its own item. The declaration and the
  request's other items are unaffected.
- **Producers.** A producer's own declaration catalogue maps each declaration
  that holds a request item's claim to exactly one label of this vocabulary,
  the label of its claims' rows. For `quire-spec-language` that mapping is QSL
  FR-507.

## Check arm

After negotiation settles an item `supported`, the candidate's check arm
decides the claim. [FR-483](../../functional/ir-proof/FR-483-settle-backend-items-at-the-check-arm.md)
states the outcomes, evidence and refusals of the arm for `value-validity` and
`operation-contract` items, [FR-484](../../functional/ir-proof/FR-484-check-value-validity-items.md)
the input mapping of the first kind and [FR-485](../../functional/ir-proof/FR-485-check-operation-contract-items.md)
that of the second.

## Advertised mode

A backend advertises a set of (kind, mode) pairs. A mode is exactly `bounded`
or `unbounded`.

If a backend's registration advertises an absent kind, a label outside this
table or a mode outside these two, then the registry SHALL refuse that
registration with `invalid_capability` (`absent-kind`, `unknown-kind` or
`unknown-mode`), keyed by backend identity.

A registration's FR-331 `domains` is a nonempty set of distinct ADR-014
boundable domain kinds (`collection`, `population`, `integer`, `recursive`)
for which it accepts a finite bound. A registration that advertises any
(kind, `bounded`) pair carries `domains`; a registration that advertises only
`unbounded` modes may omit it. The registry SHALL refuse a registration with
`invalid_capability`/`invalid-domains`, keyed by backend identity, in exactly
two cases:

1. its `domains` is present and malformed: empty, holding a non-boundable
   kind (`quantity`, `loop`, `infinite-trace`) or an unknown kind, or
   repeating a kind. This applies to every registration, including one that
   advertises only `unbounded` modes.
2. its `domains` is absent and it advertises a (kind, `bounded`) pair. A
   registration that advertises only `unbounded` modes and omits `domains` is
   admitted.

The domain-kind check below reads `domains`.

Each requested item carries its claim's extent in the representation that the
boundedness architecture
([quire-spec-language#222](https://github.com/agent-ix/quire-spec-language/issues/222))
defines, together with that architecture's classification of the extent as
`bounded` or `unbounded`. A finite domain classifies as `bounded`. #222 owns
the representation, the classification and the "finite bound is available"
predicate.

If an item carries no extent classification, then `negotiate_*` SHALL settle it
`invalid-request` with `invalid_capability`/`absent-extent`.

### Single-candidate arm

At the single candidate's arm, `negotiate_*` SHALL run these steps in order:

1. The candidate table below has given the item exactly one candidate that
   advertises the item's kind.
2. **Domain-kind check.** The item's domain kinds are compared with the
   candidate's `domains`. For a `bounded` item they are the `kind` of each
   FR-331 `extent.bounds` entry; an item with empty `bounds` has none and
   passes. For an `unbounded` item they are the boundable kinds
   (`collection`, `population`, `integer`, `recursive`) among its FR-331
   `extent.domains` entries, compared only when the candidate does not
   advertise `unbounded` for the item's kind; a candidate that advertises
   `unbounded` decides the item without bounds, so it is not compared. The
   non-boundable kinds `quantity`, `loop` and `infinite-trace` are never
   compared: no finite bound stands for them, so they go on to the
   advertised-mode table, where they make no finite bound available. An item
   with a compared domain kind the candidate's `domains` does not hold
   settles `unsupported`, warned,
   `unsupported_projection`/`unsupported-requested-capability`, naming the
   domain kind and the candidate, whatever its `finite_bound_available` says.
3. The advertised-mode table below settles every other item.

`negotiate_*` SHALL settle an item with exactly one candidate that passes the
domain-kind check from the item's extent classification and that candidate's
advertised modes for the item's kind:

| Extent | Advertised modes for the kind | Disposition |
| --- | --- | --- |
| `bounded` | includes `bounded` | the negotiation arm's own disposition for the IR form |
| `bounded` | `unbounded` only | the negotiation arm's own disposition for the IR form |
| `unbounded` | includes `unbounded` | the negotiation arm's own disposition for the IR form |
| `unbounded` | `bounded` only, and a finite bound is available | `requires-bound` |
| `unbounded` | `bounded` only, and no finite bound is available | `unsupported`, warned, `unsupported_projection`/`unbounded-extent` |

An unbounded extent settles `supported` only on a candidate that advertises
`unbounded` for the kind. A model refinement's per-step simulation
obligation is an `operation-contract` item whose extent is classified over
its roots as [FR-468](../../functional/analysis/FR-468-certify-per-step-refinement-obligations.md)
states (Claim extent); it takes this table like every other
`operation-contract` item, so an `unbounded` per-step obligation settles
`requires-bound` under a candidate that advertises `operation-contract` only
in `bounded` mode when a finite bound is available.

A backend that advertises `temporal-satisfaction` also advertises the fairness
kinds it decides, a subset of {`weak`, `strong`}. At the single candidate's
arm, an item whose fairness set holds a kind the candidate does not advertise
settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability`,
with a warning naming the missing fairness kind; negotiation never drops or
weakens a fairness constraint
([FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)).

## Evidence kinds

A backend that advertises `probabilistic-satisfaction` also advertises a
nonempty set of evidence kinds, each exactly `statistical` or `exact`. A
request item of kind `probabilistic-satisfaction` names exactly one evidence
kind; an item naming none, or another value, settles `invalid-request` with
`invalid_capability`/`absent-evidence-kind` or
`invalid_capability`/`unknown-evidence-kind`. A registration that advertises
`probabilistic-satisfaction` with no evidence kind or another value is refused
with `invalid_capability`/`unknown-evidence-kind`, keyed by backend identity.
For such an item a backend is a candidate only when it advertises both the
kind and the item's evidence kind. Negotiation never substitutes one evidence
kind for the other: an item naming `exact` with no candidate advertising
`exact` settles `unsupported` with
`unsupported_projection`/`unsupported-requested-capability`, even when a
`statistical` candidate exists, and the converse holds as well. A
`statistical` candidate's negotiation arm settles `unsupported` an item whose
claim states no confidence parameters (`missing-confidence`), whose scheduler
clause is `under every scheduler` (`every-scheduler`), whose form is an
exact-only form (`exact-only-form`), or whose regenerative method has no quantile bound for its confidence (`confidence-beyond-precision`) (FR-408). One claim may be
requested twice, once per evidence kind; the two items have separate terminal
records.

## Candidate set and negotiation

A backend identity is unique within a registry. A candidate is the backend
identity of a registered backend. Candidates are ordered by backend identity,
compared bytewise as UTF-8.

A registry snapshot is a function of the set of admitted registrations it has
received. A registration refused under "Advertised mode" is not admitted and
takes no part in the identity rules below. For each backend identity:

- When exactly one admitted registration carries the identity, the registry
  SHALL hold it.
- When two or more admitted registrations carry the identity, whatever their
  manifests, the registry SHALL refuse the identity with
  `invalid_capability`/`duplicate-backend`, once for the identity, and SHALL
  hold no registration of that identity. A backend identity is registered once.

A registration of an identity that conflicts is refused whenever it arrives:
one admitted before the conflicting registration is withdrawn from the
snapshot, and one arriving after is refused on arrival. The snapshot reports
its registration refusals, so a withdrawn registration's refusal appears in
every snapshot taken after the conflict. An identity whose
registrations were refused is unregistered, so a request that names it
receives the unknown-backend mark and it is a candidate for no item.
`duplicate-backend` never refuses a registration of another identity.

An item's `candidates` value in the [FR-331](../interfaces/FR-331-backend-provider-envelope.md)
request is valid when it equals the registry snapshot's candidate set for the
item. A backend *matches* an item when it advertises the item's kind, its
evidence kind for a `probabilistic-satisfaction` item ("Evidence kinds"),
and, for a `temporal-satisfaction` item, the item's dynamics: `hybrid` in
its `dynamics` for an item carrying the hybrid-dynamics requirement and
`discrete` for one carrying none, a descriptor with no `dynamics` deciding
`discrete` only ([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)
"Routing a hybrid item").

- When the request names a registered backend, the set is that backend if it
  matches the item, and is empty otherwise.
- When the request names an unregistered backend, the value is the
  unknown-backend mark carrying that identity.
- When the request names none, the set is every registered backend that
  matches the item.

The `quire-spec-language` registry computes `candidates`
([quire-spec-language#185](https://github.com/agent-ix/quire-spec-language/issues/185)).
Capability negotiation happens at one point, `quire-contract-codegen`'s
`negotiate_*` ([AD-016](../../assurance/AD-016-semantic-family-extension-path.md)).
`negotiate_*` SHALL evaluate each item's rules in this order, and the first
that matches settles the item: an absent kind or an unknown kind; a kind that
does not apply to the claim's declaration family ("Declaration-family
applicability"); an absent extent classification; then the candidate table
below, top row first. The declaration-family check is part of `negotiate_*`
and not a separate point: it reads the request's checked package, which the
request already carries.

A state-graph item is a `temporal-satisfaction` item with one FR-390 form.
A registered backend matches that item only when its FR-331
`state_graph_forms` includes that exact form. An absent member advertises no
state-graph forms. This filter applies only to state-graph items; it neither
advertises nor removes support for temporal, hyperproperty or step-relation
items. When a named backend does not advertise the item's state-graph form,
the candidate set is empty and the empty-candidate row below settles it.

| Candidates | Disposition |
| --- | --- |
| unknown-backend mark, or a registered backend with no negotiation arm | `invalid-request`, `invalid_capability`/`unknown-backend`, naming the backend |
| a candidate absent from the request's manifest, a candidate that does not match the item, or a set that disagrees with the named backend | `invalid-request`, `invalid_capability`/`inconsistent-candidates`, naming each offending candidate |
| empty | `unsupported`, with a warning naming the item's kind, its `requirements` when it carries them (FR-193), and, when the request names one, the named backend; `unsupported_projection`/`unsupported-requested-capability` |
| more than one, and the request names none | `invalid-request`, `invalid_capability`/`ambiguous-backend`, naming every candidate in candidate order |
| exactly one | that candidate's negotiation arm settles `supported`, `requires-bound`, `unsupported` (warned) or `invalid-request` under the domain-kind check and then the advertised-mode rules; for a state-graph item, its form must be in the candidate's FR-331 `state_graph_forms` |

`negotiate_*` SHALL NOT select among several candidates. Registration order,
display names and ambient state never change a candidate set, its order or a
disposition. The caller resolves a multi-candidate item by
naming one backend. Candidates match on kind, evidence kind and dynamics,
and state-graph items additionally match on exact `state_graph_forms`
membership. A bounded-only and an unbounded-only registrant of one kind are
both candidates, and the mode is compared only at the single candidate's arm.

The item-level causes (`absent-kind`, `unknown-kind`, `inapplicable-kind`, `absent-extent`,
`unknown-backend`, `inconsistent-candidates`, `ambiguous-backend`) settle only
their own item. `unknown-vocabulary` refuses its whole carrier.
Registration causes `absent-kind`, `unknown-kind`, `unknown-mode`,
`unknown-evidence-kind`, `invalid-domains`, `invalid-fairness-kinds` (FR-368),
`invalid-dynamics` (FR-193) and `invalid-state-graph-forms` refuse only that
registration. `duplicate-backend` refuses every registration of its
identity and no other. Registration refusals are reported ordered bytewise by
backend identity.

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
- **Tool absence** — the routed backend's adapter probes its tool after
  routing and before the run; the probe's placement and fault-injection
  evidence belong to `quire-contract-codegen`. The probe also reads the
  version of the executable it is about to run, and a version other than
  the descriptor's `tool.version` is a failed probe whose `detail` names
  both versions ([FR-331](../interfaces/FR-331-backend-provider-envelope.md)
  "Backend tool"). If the probe finds the tool
  missing, finds another version, errors, or exceeds its limit, then the adapter SHALL record the
  FR-331 result `unsupported` for that item with a warning naming the
  capability kind and the backend, cause
  `unsupported_projection`/`tool-unavailable`. The item keeps its `supported`
  disposition, and its accounting record is its one terminal record. Artifacts
  the item emitted before the probe are retained in the envelope and carry no
  verdict. If the tool becomes unavailable after a passing probe and before the
  run completes, then the adapter SHALL record the FR-331 result `failed` with
  cause `unsupported_projection`/`tool-unavailable`, naming the capability
  kind and the backend; the result `failed`, not the cause, distinguishes it
  from absence found at probe. No other candidate runs, no other mode runs,
  and nothing waits for the tool.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-290-AC-1 | The twelve exact labels are the complete accepted set; absent, unknown and display-derived labels refuse with their named `invalid_capability` cause and are never mapped to a member; a carrier selecting a vocabulary other than `quire.capability-kind/v1`, or none, is refused with none of its labels read. | Test (TC-060, TC-061, TC-271) |
| FR-290-AC-2 | Every pair of claim kinds may retain independent execution and outcome states without promotion or premise transfer. | Test (TC-060, TC-271) |
| FR-290-AC-3 | A non-global claim cannot carry the complete-global-conformance premise set as evidence for its own success. | Test (TC-060, TC-061, TC-271) |
| FR-290-AC-4 | An item whose candidate set is empty settles `unsupported` with a warning naming its capability kind, and the named backend when there is one. | Test (TC-060, TC-271) |
| FR-290-AC-5 | Each claim form in the claim-form assignment table requires exactly its listed kind; an expression nested in a clause adds no kind; a scalar operation application in a function body requires `value-validity` once per occurrence and a range narrowing requires none; a `case` exhaustiveness obligation and an FR-353 abstraction relation request no backend. | Test (TC-271, TC-330, TC-342, TC-343) |
| FR-290-AC-6 | For each row of the advertised-mode table, negotiation settles the listed disposition; an unbounded extent on a bounded-only advertisement never settles `supported`; an item with no extent classification settles `invalid-request`. | Test (TC-271) |
| FR-290-AC-7 | Each row of the candidate table settles its listed disposition and cause; an item with two or more candidates and no named backend settles `invalid-request` naming every candidate in candidate order, identically under every registration order; a malformed registration is refused and keyed by backend identity. | Test (TC-271) |
| FR-290-AC-8 | A routed item whose tool is missing, or whose probe finds a version other than the descriptor's `tool.version`, errors or exceeds its limit, records the FR-331 result `unsupported` with cause `unsupported_projection`/`tool-unavailable` naming the kind and backend; its disposition stays `supported`, and no other registered candidate or mode runs; a tool that becomes unavailable after a passing probe records the FR-331 result `failed` with the same cause. | Test (TC-271) |
| FR-290-AC-9 | Withdrawn: equal manifests of one backend identity are no longer merged; a backend identity registered more than once is refused (FR-290-AC-10). | Withdrawn |
| FR-290-AC-10 | Two or more admitted registrations of one backend identity, whatever their manifests, are refused with `invalid_capability`/`duplicate-backend`, one refusal per identity, ordered by identity; the snapshot holds no registration of that identity, a request naming it receives the unknown-backend mark, and registrations of other identities stand; the snapshot, its refusals and every candidate set are identical under every registration order. | Test (TC-282) |
| FR-290-AC-11 | A probabilistic claim requires `probabilistic-satisfaction`, a task-set claim `schedulability`, and a timed temporal claim and the time-lock-freedom item `temporal-satisfaction`. | Test (TC-353, TC-370, TC-372) |
| FR-290-AC-12 | A `probabilistic-satisfaction` item is a candidate only for backends advertising its evidence kind; an `exact` item with only a `statistical` candidate, and the converse, settle `unsupported`, `unsupported-requested-capability`; an item with an absent or unknown evidence kind settles `invalid-request` with its named cause. | Test (TC-353) |
| FR-290-AC-13 | At the single candidate's arm, the domain-kind check runs after the candidate table and before the advertised-mode table: a bounded item whose `extent.bounds` hold a kind outside the candidate's `domains` settles `unsupported`, `unsupported-requested-capability`, naming the kind, and one with empty `bounds` passes. An unbounded item on a candidate that advertises only `bounded` for its kind has only its boundable `extent.domains` kinds compared: when one is outside `domains` it settles `unsupported`, `unsupported-requested-capability`, naming the kind, whatever `finite_bound_available` says; when every boundable kind is in `domains`, the advertised-mode table settles it, `requires-bound` with a finite bound available and `unsupported`, `unbounded-extent` without one, so a `quantity`, `loop` or `infinite-trace` domain never settles `unsupported-requested-capability`. An unbounded item is not compared on a candidate that advertises `unbounded`. Any registration, including one that advertises only `unbounded` modes, whose `domains` is present and empty, holds a non-boundable or unknown kind or repeats a kind is refused `invalid_capability`/`invalid-domains`, keyed by backend identity; a registration with no `domains` is refused the same way only when it advertises a `bounded` pair, and one that advertises only `unbounded` modes is admitted without `domains`. | Test (TC-271) |
| FR-290-AC-14 | A per-step simulation obligation whose extent is `unbounded` with one boundable `population` domain, on a single candidate that advertises `operation-contract` only in `bounded` mode with `domains` holding `population`, settles `requires-bound` when `finite_bound_available` is true and `unsupported`, `unsupported_projection`/`unbounded-extent` when it is false; on a bounded-only candidate whose `domains` lack `population` it settles `unsupported`, `unsupported_projection`/`unsupported-requested-capability` naming `population`, whatever `finite_bound_available` says; on a candidate that advertises `unbounded` it takes the negotiation arm's own disposition; the same obligation with that population bounded is `bounded` and takes the arm's own disposition on either mode. | Test (TC-950) |
| FR-290-AC-15 | With `qsl-model-check` advertising `temporal-satisfaction` with no `dynamics` and `dreal` advertising it with `dynamics` `["hybrid"]`, a `temporal-satisfaction` item carrying `"requirements": ["hybrid-dynamics"]` and naming no backend has the candidate set `[dreal]`, and one carrying no `requirements` `[qsl-model-check]`; naming `qsl-model-check`, the hybrid item's set is empty and it settles `unsupported`, `unsupported-requested-capability`; a hybrid item whose `candidates` is `[dreal, qsl-model-check]` settles `invalid-request`, `inconsistent-candidates`, naming `qsl-model-check`; and with `dreal`'s `dynamics` `["discrete", "hybrid"]` the item carrying no `requirements` has the set `[dreal, qsl-model-check]` and settles `ambiguous-backend`. | Test (TC-373) |
| FR-290-AC-16 | For each FR-390 form, an EN-1 registration whose FR-331 `state_graph_forms` contains that form is a candidate and a temporal backend whose member omits it is not; an omitted member matches no state-graph form. Naming the backend that omits the form gives an empty candidate set and settles `unsupported`, `unsupported-requested-capability`, naming the state-graph form. An empty, duplicate or unknown form list, or a form list on a registration that does not advertise `temporal-satisfaction`, is refused `invalid_capability`/`invalid-state-graph-forms`, keyed by backend identity. Membership filtering is exact and does not change candidate sets for temporal formulas, hyperproperty items or step-relation items. | Test (TC-968) |
| FR-290-AC-17 | An item whose admitted kind is not listed for its claim's declaration family settles `invalid-request` with `invalid_capability`/`inapplicable-kind` and payload `request_index`, `capability` (its kind label) and `family` (the declaration family label). The family is read from the item's `item_kind` when it is not `formula`, and otherwise from its claim node's tag, `semantic_form` and occurrence role by the tables of "Reading the family from a request item". An item whose claim node is in no row has no `family` member and is refused for every kind. No negotiation arm or engine runs the item, whatever its extent, evidence kind, statistical method, candidates or named backend, so it never settles `absent-evidence-kind`, `unknown-evidence-kind`, `not-statistical`, `absent-extent` or a candidate-table cause. An item with an absent or unknown kind settles `absent-kind` or `unknown-kind` instead. The item keeps its request position, its disposition is its terminal record with no result entry, and the request's other items settle independently. For every claim-form row whose family is not "none", the request item of that row's kind on such a claim is never refused this way. This includes both claims of one `relation` clause with an implementation binding, and the derived deadlock-freedom and race-freedom items anchored on a protocol clause node. | Test (TC-271) |
| FR-290-AC-18 | The declaration-family vocabulary has exactly seven labels: `value`, `state-model`, `temporal-trace`, `relation-claim`, `protocol`, `probabilistic` and `real-time`. For each label, the kinds the family table lists are exactly the kinds the claim-form assignment table assigns to the rows whose "Declaration family" cell names it; `state-model` lists `operation-contract` only, and `finite-replay` is listed for `protocol` only. Every node row and `item_kind` row of "Reading the family from a request item" names a label of that vocabulary, and the rows whose family is "none" request no item. | Inspection |

## Dependencies

- [FR-060](../../functional/protocol/FR-060-separate-protocol-claims.md)
  declares independent dispatch for the six protocol claim kinds.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  declares their canonical result retention and non-substitution rules.
- [FR-331](../interfaces/FR-331-backend-provider-envelope.md) carries the
  vocabulary selection, advertised (kind, mode) pairs, the candidate set and
  the per-item result.
- [FR-272](../foundation/FR-272-typed-diagnostic-cause.md) types the
  `invalid_capability` and `unsupported_projection` causes.
- [AD-016](../../assurance/AD-016-semantic-family-extension-path.md) places the
  one negotiation point.

## Status

Proposed shared ruling for `quire-specification#40`, widened to the value,
state-model, finite-replay and temporal-trace families by
`quire-specification#134`.
