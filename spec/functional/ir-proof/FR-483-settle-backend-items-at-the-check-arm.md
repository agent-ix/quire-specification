---
id: FR-483
title: "Settle a value-validity or operation-contract item at the check arm"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-462
    type: references
  - target: ix://agent-ix/quire-specification/FR-484
    type: references
  - target: ix://agent-ix/quire-specification/FR-485
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
  - target: ix://agent-ix/quire-specification/FR-461
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
---

# FR-483: Settle a value-validity or operation-contract item at the check arm

## Description

When negotiation has settled a `value-validity` or `operation-contract` request item `supported` for one candidate backend, that backend's check arm SHALL decide the item over its checked package and settle it with exactly one of four arm outcomes, `supported`, `violation`, `unknown` or `unsupported`, each mapped to exactly one [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) result as the mapping table below states.

The check arm is the step that follows [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)'s single-candidate arm. Negotiation decides from kind, extent and the candidate's advertised modes and `domains` alone and reads no claim; the check arm reads the claim. [FR-484](FR-484-check-value-validity-items.md) states the input mapping of the `value-validity` kind and [FR-485](FR-485-check-operation-contract-items.md) that of the `operation-contract` kind. This requirement states what both share.

## Inputs

One request item that [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) settled `supported` for the candidate, with its `request_index`, `kind`, `claim` (`{node, occurrence}`), `extent`, `options` and the candidate's name; the request's `deadline_ms`; the checked package, as the typed value of an in-process provider or the decoded `quire.checked-package/v2` bytes of a process provider ([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) "In-process providers", "Package bytes"); and the candidate's admitted descriptor.

## Outputs

For the item, exactly one result entry, its `counterexamples` entry when the result is `refuted`, its accounting record, and no other member. An item refused before the arm runs has a disposition and no result entry ([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) "Artifacts, counterexamples and accounting").

## Behavior

### The check input

The arm SHALL derive from the item one *check input* before it checks anything, by the mapping that [FR-484](FR-484-check-value-validity-items.md) or [FR-485](FR-485-check-operation-contract-items.md) states for the item's kind. A check input has exactly these members:

| Member | Content |
| --- | --- |
| `form` | the closed claim form the item's claim node and occurrence select, from the kind's form list |
| `variables` | the ordered checked-package parameter nodes the claim quantifies over, each with its declared type and its domain under the item's extent |
| `premises` | the ordered Boolean nodes assumed, empty when none |
| `goal` | the obligation checked under the premises, a Boolean node or a named definedness obligation |
| `locus` | the claim's `{node, occurrence}`, which is also the locus of every `where` the result carries |

A *witness item*, which [FR-453](../tooling/FR-453-preserve-parent-profile-refusals-under-layering.md) requests with one layer clause, is an ordinary item of its kind. The arm reads no layer identity and gives a witness item no case of its own.

The domain of a variable is its declared type, restricted by every `ProofBound` of a `bounded` extent whose `DomainKey` names that variable's node or a path below it. A `bounded` item's `proved` result states the claim over that bounded domain only, and the bounds are part of the request identity. An `unbounded` item has the declared type as its domain, and only a candidate that advertises `unbounded` for the kind receives it as `supported` ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) "Single-candidate arm").

If the claim's node and occurrence select none of the kind's forms, or select a node the package does not hold, then the arm SHALL settle the item `declined` with cause `invalid-input` and a diagnostic that names the node, the occurrence and the kind, and SHALL run no check. If the package lacks a node the form requires, such as the operation anchor of a clause, then the arm SHALL settle it `declined` with cause `incomplete-input` and a diagnostic that names the missing node.

### Outcomes and their mapping

An arm outcome names what the check found. The outcome `supported` is distinct from the negotiation disposition `supported`, and the outcome `unsupported` is distinct from the disposition `unsupported`; an item keeps its `supported` disposition when its arm outcome is `unsupported`, as [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) "Tool absence" keeps it, and its accounting record is its one terminal record.

| Arm outcome | Meaning | FR-331 result | Carries |
| --- | --- | --- | --- |
| `supported` | the goal holds for every assignment of the variables in their domains that satisfies the premises, and at least one such assignment exists | `proved` | `basis` and `certification` ("Proof evidence") |
| `violation` | some assignment in the domains satisfies the premises and makes the goal false or undefined | `refuted` | `counterexample`; `cause` `undefined-evaluation{where, cause}` when the goal is undefined |
| `unknown` | the engine neither established the goal nor found a violation, or no assignment satisfies the premises, or a candidate violation did not replay | `inconclusive`, or `incomplete` when a limit or the deadline ended the run | the cause in "Unknown and stopped runs" |
| `unsupported` | the item's form or an element it reads lies outside what this arm decides, with the reason | `unsupported` | `diagnostic` `unsupported_projection`/`unsupported-requested-capability` whose `detail` names the kind, the form and the reason |

The arm SHALL NOT map any other result onto an arm outcome: `tested`, `measured` and `failed` are not arm outcomes. A run that breaks around the check is a provider failure and settles `failed` with the [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) causes of its provider form (FR-462 for a process provider).

If the premises are satisfied by no assignment in the domains, then the arm SHALL settle the item `inconclusive` with the vacuity cause `kani_vacuous_proof`, the one vacuity cause [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) closes (FR-331-AC-8), and SHALL NOT settle it `proved`. The cause keeps that spelling for every provider; it is not specific to Kani.

If the arm cannot decide whether the goal holds for a form it has accepted, then it SHALL settle the item `unknown`. It SHALL NOT turn an `unknown` into `supported` by dropping a premise, widening a bound or assuming an undetermined subterm, and it SHALL NOT turn an unsupported element into `violation` or `supported` by approximating it.

### Proof evidence

An arm outcome `supported` settles `proved` with exactly one proof basis from the [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) closed list ("Wire spellings of result causes and proof bases"), fixed by the method that discharged the item. [FR-354](FR-354-record-proof-trust-basis.md) fixes the certification of an SMT proof, not its basis; this table fixes both for every method:

| Method | Basis | Certification |
| --- | --- | --- |
| evaluation of every assignment of a finite domain | `exhaustive` | `uncertified`, because the method has no core certificate checker (FR-331 "Certification") |
| Kani harness whose SUCCESS check count is positive | `bounded-proof{checks}`, `checks` the SUCCESS count | `certified` |
| refutation of the negated goal by the SMT backend, `bounded` extent | `bounded-complete{depth}` | `certified` when the core checker verifies the certificate, `uncertified` when no certificate arrives or a step uses a rule outside the checked set (FR-354); a rejected certificate settles `inconclusive` `certificate-rejected{rule, at}` instead |
| the same, `unbounded` extent | `inductive{depth}` | as the row above |
| any method, in a backend loaded as a plugin ([FR-305](../tooling/FR-305-load-explicit-rust-plugins.md)) | a basis that is not core-only ([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) "Result entries"), such as `exhaustive` | `trusted` |

For an SMT basis, `depth` is the number of transition steps in the encoded query: 0 for a `value-validity` item and 1 for an `operation-contract` item, whose query reads one step `(s, s')` or one independent execution per relation variable. No other basis is a `value-validity` or `operation-contract` basis.

### Counterexample and evidence kind

An arm outcome `violation` settles `refuted` with one `counterexamples` entry whose canonical counterexample has the kind that the item's kind fixes:

| Item kind | Counterexample `type` | Content |
| --- | --- | --- |
| `value-validity` | `assignments` | one [FR-181](../runtime/FR-181-simulate-finite-executions.md) typed value for each `variables` node, in `variables` order ([FR-197](FR-197-replay-backend-counterexamples.md) `Input` arm) |
| `operation-contract` | `contract-counterexample` | per execution, its pre-state, arguments, result and post-state, as [FR-485](FR-485-check-operation-contract-items.md) states |

The arm SHALL replay the candidate counterexample through the native evaluator of the same checked package before it reports `refuted`, and SHALL report `refuted` only when the replay reproduces the violation: the goal false, or undefined with the cause the result names, under the premises. If the replay does not reproduce it, then the arm SHALL settle the item `inconclusive` with cause `replay-parity` and SHALL record no counterexample. A reader compares no counterexample with another, because several assignments may violate one goal; the verdict and cause are compared exactly and the counterexample is checked by replay.

A plugin's counterexample entry carries `assignments` for a `value-validity` item and `contract-counterexample` for an `operation-contract` item, and no `transcript`.

The replay of `assignments` is [FR-197](FR-197-replay-backend-counterexamples.md)'s `Input`-arm replay. The replay of a `contract-counterexample` is owned by [FR-485](FR-485-check-operation-contract-items.md) "Replay"; FR-197 does not own it.

### Unknown and stopped runs

| Stop | Result | Cause |
| --- | --- | --- |
| the engine returns `unknown` or its decision procedure does not cover a subterm | `inconclusive` | `solver-inconclusive` |
| a certificate the core checks is rejected | `inconclusive` | `certificate-rejected{rule, at}` |
| the examined case count reaches `max_cases` | `incomplete` | `limit-reached{limit, value, setting}` with `limit` `max_cases` and `setting` `value_check.max_cases` for `value-validity` and `contract_check.max_cases` for `operation-contract` |
| the provider's own wall-clock budget ends | `incomplete` | `timed-out` |
| the caller cancels or the caller's `deadline_ms` ends the run | `incomplete` | `cancelled{source}`, `source` `requested` or `deadline` |

An item's `max_cases` is its `check.limits.max_cases`, a dedicated limits member of the request item beside `hybrid.limits` and `temporal.limits`, a `PositiveCount` that only a `value-validity` or `operation-contract` item may carry; an absent member takes the descriptor's published default under the key `max_cases` in `bounds` ([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) "Items"). The setting names are `value_check.max_cases` and `contract_check.max_cases`, each the engine name, a dot and the member as [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) fixes, so the CLI raises one with `--limit value_check.max_cases=<n>`. `check.limits` is part of request identity, because a different case budget can produce a different result. A `max_cases` that is zero or not a count is refused before consumption by the wire schema. The count is the number of cases the engine examined, where one case is one assignment or pre-state tuple evaluated, one solver query, or one Kani harness run.

### Refusal dispositions

Every way an item stops short of a check has one settlement, and none is reported as another:

| Situation | Settled by | Settlement |
| --- | --- | --- |
| absent or unknown kind, absent extent, unknown backend, inconsistent or ambiguous candidates | negotiation | `invalid-request`, [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)'s causes |
| no registered backend advertises the kind, or the named backend does not | negotiation | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| a compared domain kind the candidate's `domains` lacks | negotiation | `unsupported`, `unsupported_projection`/`unsupported-requested-capability` |
| `unbounded` extent and a `bounded`-only candidate | negotiation | `requires-bound` when a finite bound is available, otherwise `unsupported`, `unsupported_projection`/`unbounded-extent` |
| claim selects no form (`invalid-input`) or the package lacks a node the form requires (`incomplete-input`) | the arm | `declined` |
| `check.limits.max_cases` zero or not a count, or `check` on another kind | the wire reader | refused before consumption |
| form or element the arm does not decide | the arm | `unsupported` result with the diagnostic above |
| premises satisfied by no assignment | the arm | `inconclusive`, `kani_vacuous_proof` |
| tool absent after routing | the adapter | `unsupported`, `unsupported_projection`/`tool-unavailable` ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) "Tool absence") |

### Descriptor modes, domains and registration

A backend that advertises `value-validity` or `operation-contract` SHALL have a check arm that implements this requirement and the kind's section ([FR-484](FR-484-check-value-validity-items.md), [FR-485](FR-485-check-operation-contract-items.md)). The registration is the [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) descriptor under the [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) rules, and no other registration rule applies to these kinds. A provider SHALL NOT register a descriptor that advertises a kind for which it has no arm, and no descriptor stands in for a provider that does not exist: a registry snapshot composed from linked providers has no candidate for the kind until such a provider is linked.

A descriptor advertises the mode its arm decides:

- `bounded` when the arm decides items whose every domain is finite under the item's bounds, with `domains` the boundable kinds the arm can substitute, a nonempty subset of {`collection`, `population`, `integer`, `recursive`} as [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) "Advertised mode" requires. A descriptor lists `population` for either kind only when its arm binds a population by cardinality; an item whose extent holds a `population` domain that the single candidate's `domains` lacks settles `unsupported` at the domain-kind check ([FR-290-AC-13](../../objects/protocol/FR-290-protocol-claim-kind.md), TC-271 NG-18), so the arm never receives it.
- `unbounded` when the arm decides the claim over the declared type without bounds, as an inductive or symbolic engine does.
- Both pairs when the arm decides in both ways.

An arm SHALL NOT report `proved` for an `unbounded` item unless its descriptor advertises `unbounded` for the kind.

The descriptor's `bounds` holds the default `max_cases` count under the key `max_cases`.

### Independence

Each item has its own check input, its own result and its own terminal record. An arm SHALL NOT use the result of one item as a premise of another, and a `proved` item discharges no other item, including the item with the same claim node and another kind ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)). A request that repeats a claim has two items with two results.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-483-AC-1 | The four arm outcomes map to exactly `proved`, `refuted`, `inconclusive` or `incomplete`, and `unsupported`; an arm outcome `unsupported` leaves the item's disposition `supported`, and no outcome yields `tested`, `measured` or `failed`. | Test (TC-972) |
| FR-483-AC-2 | A `supported` outcome settles `proved` with the basis and certification of the proof-evidence table: `exhaustive` and `uncertified` for finite-domain evaluation, `bounded-proof{checks}` and `certified` for a Kani harness, `bounded-complete{depth}` for an SMT proof of a `bounded` item and `inductive{depth}` for an `unbounded` one (`certified` only when the certificate verifies, otherwise `uncertified`), and `trusted` for a plugin; an outcome whose premises no assignment satisfies settles `inconclusive` with `kani_vacuous_proof`, never `proved`. | Test (TC-972) |
| FR-483-AC-3 | A `violation` settles `refuted` with one counterexample of the item kind's type that the native replay reproduces; a candidate the replay does not reproduce settles `inconclusive` with `replay-parity` and records no counterexample; an undefined goal also carries `undefined-evaluation{where, cause}`. | Test (TC-972) |
| FR-483-AC-4 | An engine `unknown` settles `inconclusive` `solver-inconclusive`; a rejected certificate settles `inconclusive` `certificate-rejected{rule, at}`; reaching `max_cases` settles `incomplete` `limit-reached` naming `max_cases`, its value and the setting `value_check.max_cases` or `contract_check.max_cases`; the provider's own time limit settles `incomplete` `timed-out` and the caller's deadline `incomplete` `cancelled{source: deadline}`; the item's `check.limits.max_cases` replaces the descriptor default, and a zero or non-count value, or `check` on another kind, is refused by the wire schema. | Test (TC-972) |
| FR-483-AC-5 | A claim that selects no form or no package node settles `declined` `invalid-input`; a missing required node settles `declined` `incomplete-input`; neither runs a check. | Test (TC-972) |
| FR-483-AC-6 | A bounded item's `proved` states the claim over the item's bounded domain only; an `unbounded` item reaches the arm only on a candidate that advertises `unbounded`, and a `bounded`-only candidate gets `requires-bound` or `unsupported`/`unbounded-extent` from negotiation, never a result. | Test (TC-972) |
| FR-483-AC-7 | A registry with no descriptor that advertises `value-validity` has an empty candidate set for a `value-validity` item and settles it `unsupported`, `unsupported_projection`/`unsupported-requested-capability`; the registry snapshot of a production source holds a descriptor for a kind only when an arm for it is linked, so a kind with no arm has no descriptor and no candidate. | Test (TC-972) |
| FR-483-AC-8 | An item whose extent holds a `population` domain settles `unsupported` at the domain-kind check under a candidate whose `domains` lacks `population`, and reaches the arm under a candidate that lists it, for both kinds. | Test (TC-972) |
| FR-483-AC-9 | Two items with one claim node and different kinds, and two items with the same claim and kind, have separate terminal records, and the `proved` of one is no premise of the other. | Test (TC-972) |

## Dependencies

- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the kinds, the candidate set and the negotiation dispositions this arm follows.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns the descriptor, the item, the result entries and the closed causes.
- [FR-462](../tooling/FR-462-speak-the-plugin-wire.md) owns the process-provider frames that carry the request and the outcome.
- [FR-197](FR-197-replay-backend-counterexamples.md) owns the replay of a canonical assignment.
- [FR-354](FR-354-record-proof-trust-basis.md) owns the SMT proof bases and their certification.
- [FR-485](FR-485-check-operation-contract-items.md) "Replay" owns the replay of a `contract-counterexample`.
- [FR-461](../foundation/FR-461-name-the-limit-value-and-setting.md) owns the limit and setting naming.
- [FR-484](FR-484-check-value-validity-items.md) and [FR-485](FR-485-check-operation-contract-items.md) own the two kinds' input mappings.
