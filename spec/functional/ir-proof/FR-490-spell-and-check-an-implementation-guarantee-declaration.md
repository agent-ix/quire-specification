---
id: FR-490
title: "Spell and check an implementation guarantee declaration"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-488
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-450
    type: references
  - target: ix://agent-ix/quire-specification/FR-451
    type: references
  - target: ix://agent-ix/quire-specification/FR-353
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-368
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-492
    type: references
---

# FR-490: Spell and check an implementation guarantee declaration

## Description

When a source unit that selects `quire.model.complete/v1` declares a
`guarantee-decl`, the compiler SHALL check it into one FR-488
implementation-guarantee declaration, carry it in the checked package as one
`correspondence`/`implementation_guarantee` node, and derive from it the
guarantee item and each premise item below, each with its own obligation
identity and source correspondence, refusing each inadmissible member with
the code and cause below.

## Inputs

- The unit's `guarantee-decl` declarations, recognized by the
  [complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md)
  "Implementation guarantee declaration" production.
- The unit's temporal clauses, its `abstraction-decl` declarations
  ([FR-450](FR-450-spell-abstraction-relation-declaration.md)) and the
  admitted domain packages its `model` declarations select.

## Outputs

One checked implementation-guarantee declaration per `guarantee-decl`, its
checked-package node, and its derived items; or a located refusal per
inadmissible member.

## Behavior

### Declaration form

`guarantee G of P using A { initial to "f"; members }` declares guarantee `G`
of the unit's temporal clause `P` through the unit's abstraction relation
`A`:

| Member | Meaning |
| --- | --- |
| `of P` | `P` is the FR-488 clause; its resolved fairness set ([FR-362](../temporal/FR-362-apply-fairness-constraints.md)) is `F_P`, and its model subject is the FR-488 model subject |
| `using A` | `A` is the FR-353 relation; the checked package that carries it fixes the relation revision ([FR-451](../../objects/interfaces/FR-451-checked-package-abstraction-relation-body.md): a rebinding is a new package) |
| `initial to "f";` | `f`, a `rust-path`, is the Rust function whose result is an implementation initial state |
| `progress M::T::o decreases "m";` | the no-divergence premise of operation `o`: `m`, a `rust-path`, names the well-founded measure, a function of the bound function's state into the natural numbers that every stutter step of `o`'s bound function strictly decreases |
| `progress M::T::o bound N;` | the no-divergence premise of operation `o`: at most `N` (a positive `uint`) consecutive stutter steps of `o`'s bound function |
| `environment E by "s" fair K Q M::T::o;` | the scheduling premise `E` supplied by `s`: the implementation executes `o`'s bound function under fairness of kind `K` and granularity `Q` (FR-362's unwritten defaults apply) |
| `environment E by "s" assumes N;` | the assumption premise `E` supplied by `s`: the unit's `predicate` `N` holds of the environment |

A step of a bound function is one execution of its body between two
observable implementation states: one iteration of a loop, or the whole body
when it has no loop. A step is a **stutter** when the relation maps its pre
and post states to one abstract state.

### Static checks

The compiler SHALL refuse, in source order, naming the member:

1. A package that does not select `quire.model.complete/v1`:
   `unsupported_construct` at the declaration. A declaration with no
   `initial` member, a second `initial` member, or an unquoted Rust string:
   `invalid_syntax` at the offending token.
2. `P` naming no temporal clause of the unit, or `A` naming no
   `abstraction-decl` of the unit: `missing_declaration`/`missing-name`.
3. `P` not selecting `quire.temporal.infinite-trace/v1`, or `P`'s `using`
   model alias differing from `A`'s: `invalid_model_binding`/
   `malformed-declaration`, naming `P`.
4. A `progress` or scheduling `environment` operation name that resolves to
   no operation of the model subject: `missing_declaration`/`missing-name`;
   an `assumes` name that resolves to a declaration other than a `predicate`:
   `invalid_model_binding`/`malformed-declaration`.
5. A Rust string whose content is not a `rust-path` (FR-450 "Rust
   spellings"), or `bound 0`: `invalid_model_binding`/
   `malformed-declaration`, naming the first malformed segment and its span.
6. Two `progress` members for one operation, or two `environment` members of
   one name: `invalid_model_binding`/`conflicting-binding`, naming both
   members.

A model element the guarantee reads that `A` does not bind (the object types
and populations `P` reads, and every operation of the model subject) is not a
declaration refusal: the guarantee item refuses `missing_declaration`/
`missing-name` scoped to that item (FR-353-AC-3, FR-488 "Settlement"), and
every other item of the package is unaffected. The declaration requests no
capability kind itself; [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)
classifies the items it derives.

### Derived items

A checked declaration `G` derives the planner-local FR-488
implementation-guarantee item (role `guarantee`; no FR-331 item, FR-368 "Item
kinds"), whose obligation identity binds `G`'s node id and the package
identity, and, in this order, its premise slots (FR-492), each with a slot
identity that binds `G`'s node id, the package identity, the role and, where
the role is per operation or per constraint, the operation key or the
constraint:

| Role | Slot | Kind | Settled by |
| --- | --- | --- | --- |
| `model-claim` | `P`'s `formula` item over the model subject under `F_P`, with `stutter_closure` when FR-363 classifies `P` as `liveness` (FR-368 "Request members") | `temporal-satisfaction` | the routed provider |
| `initialization` | one `operation-contract` item over `f`: the relation maps every result of `f` to a model initial state | `operation-contract`, declaration family `relation-claim` | the FR-350 backend |
| `transition-simulation` | one `operation-contract` item per operation of the model subject over its bound function: every step maps to a step of the operation with the pre and post abstract states the relation gives, or is a stutter | as above | the FR-350 backend |
| `frame` | one `operation-contract` item per operation of the model subject: no step changes abstract state outside the operation's frame (FR-013) | as above | the FR-350 backend |
| `environment` | one slot per `environment` member, in member order; then, for a `liveness` `P`, one **unfilled** slot per constraint of `F_P` whose operation no scheduling `environment` member names (a required premise the declaration does not list) | none: an FR-492 environment record fills a listed slot | the planner |
| `no-divergence` | for a `liveness` `P` only, one slot per operation of the model subject: an `operation-contract` item when the operation has a `progress` member (every stutter step decreases the measure, or no run of the bound function has more than `N` consecutive stutter steps), and an **unfilled** slot when it has none | as above | the FR-350 backend |
| `fairness-correspondence` | for a `liveness` `P` only, one planner-local slot per constraint of `F_P` | planner-local | the planner, by FR-488 "Fairness correspondence" |

An unfilled slot has no item; FR-492 settles it `absent`, and FR-488 settles
the guarantee item `premise-unproved` naming the first such slot. Each
derived `operation-contract` item carries the anchor `claim.node` of `G`'s
node and the member `correspondence` `{role, operation}` (`operation` absent
for `initialization`), so two roles over one function are distinct
obligations. A safety `P` (`reachable-state invariant` or `safety`, FR-363)
derives neither progress role and no unfilled `environment` slot.

### Checked-package node

`G` is emitted as one `quire.checked-semantic-graph/v2` node with
`node_tag: "correspondence"` and `semantic_form: "implementation_guarantee"`:

- **body** — `{term: "implementation_guarantee", clause, relation, initial,
  progress, environment}`, all required:
  - `clause` — the `NodeRef` of `P`'s claim node; `relation` — the `NodeRef`
    of `A`'s `correspondence`/`abstraction_relation` node;
  - `initial` — FR-451's `RustPath`;
  - `progress` — one entry per `progress` member: `{context: NodeRef,
    operation: Identifier, measure}`, `context` and `operation` the
    operation key as FR-451's frame entries spell it, and `measure`
    `{decreases: RustPath}` or `{bound: PositiveCount}`;
  - `environment` — one entry per `environment` member: `{name:
    Identifier, supplier: string, supplies}`, `supplies` `{fairness:
    {kind, granularity, context, operation}}` or `{assumption: NodeRef}`,
    the target a `predicate` node.
- **Canonical order** — `progress` by `context` node id then `operation`
  bytes; `environment` by `name` bytes. Out of order refuses
  `invalid_semantic_graph`.
- **semantic_type** — the node's own `node_id`; **dependencies** — exactly
  the unique, node-id-ascending set of `clause`, `relation` and every
  `context` and `assumption` target; **occurrences** — one, role
  `declaration`, at the `guarantee-decl`; each member keeps its own source
  region in the node's `source_map` entry, so every derived item's source
  correspondence resolves to the member that produced it; **node_id** —
  FR-322's structural id `{package, path, ordinal}`.

The reader runs this step after FR-451's abstraction step, node by node in
ascending node-id order, and reports the first defect: identity and
dependencies, else `invalid_semantic_graph`; canonical order, else
`invalid_semantic_graph`; a target that is not a declared dependency,
`missing_declaration`/`missing-name`; a `clause` target that is not an
infinite-trace temporal claim, a `relation` target that is not an
`abstraction_relation` node, or an `assumption` target that is not a
`predicate`, `invalid_model_binding`/`malformed-declaration`; then static
checks 4 to 6 above on the read values. Reading the emitted bytes back yields
the declaration that was checked, and the same derived items with the same
obligation identities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-490-AC-1 | A declaration over a liveness clause of a one-operation subject, with one `initial`, one `progress ... decreases` and one scheduling `environment` member covering `F_P`'s one constraint, checks and derives the guarantee item and, in order, the model-claim slot with `stutter_closure`, the initialization, transition-simulation and frame slots, one environment slot, one no-divergence slot and one fairness-correspondence slot, each with a distinct identity. | Test (TC-984) |
| FR-490-AC-2 | The same declaration over a safety clause derives no no-divergence, fairness-correspondence or unfilled environment slot, and its model-claim slot has no `stutter_closure`. | Test (TC-984) |
| FR-490-AC-9 | The liveness declaration of AC-1 without its `progress` member derives an unfilled no-divergence slot, and without its `environment` member derives an unfilled environment slot naming `F_P`'s constraint; neither is a refusal. | Test (TC-984) |
| FR-490-AC-3 | A package not selecting `quire.model.complete/v1` refuses `unsupported_construct`; a declaration with no `initial` member and one with an unquoted Rust path refuse `invalid_syntax` at the offending token. | Test (TC-984) |
| FR-490-AC-4 | `P` or `A` naming no declaration refuses `missing_declaration`/`missing-name`; a finite-trace `P`, and `P` and `A` over different model aliases, refuse `invalid_model_binding`/`malformed-declaration` naming `P`. | Test (TC-984) |
| FR-490-AC-5 | A `progress` operation that is not an operation of the subject refuses `missing_declaration`/`missing-name`; `bound 0` and the Rust path `"9lives"` refuse `invalid_model_binding`/`malformed-declaration`; two `progress` members for one operation refuse `invalid_model_binding`/`conflicting-binding` naming both. | Test (TC-984) |
| FR-490-AC-6 | An operation of the model subject with no frame binding in `A` refuses only the guarantee item `missing_declaration`/`missing-name`; the package checks and its other items are derived unchanged. | Test (TC-984) |
| FR-490-AC-7 | The emitted `correspondence`/`implementation_guarantee` node reads back to the same declaration, derived items and obligation identities; each derived item's source correspondence resolves to the member that produced it; a node whose `dependencies` omit `relation`, and one whose `progress` entries are out of order, refuse `invalid_semantic_graph`. | Test (TC-984) |
| FR-490-AC-8 | A unit holding the abstraction relation and no `guarantee-decl` derives exactly the items it derives without the relation: the relation alone requests nothing (FR-450 "What the declaration is not"). | Test (TC-984) |

## Dependencies

- [FR-488](FR-488-attach-a-temporal-guarantee-to-implementation-rust.md): the
  semantics and settlement of the items this declaration derives.
- [FR-450](FR-450-spell-abstraction-relation-declaration.md),
  [FR-451](../../objects/interfaces/FR-451-checked-package-abstraction-relation-body.md)
  and [FR-353](FR-353-declare-abstraction-relation.md): the relation, its
  Rust spellings and its node.
- [FR-350](FR-350-emit-verus-contracts.md): the backend of the derived
  `operation-contract` items.
- [FR-492](../../objects/interfaces/FR-492-planner-settlement-request.md):
  the settlement request of the planner-local items.
- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md):
  the graph, node ids and reader order.

## References

- Tickets: Linear QSL-1541.
