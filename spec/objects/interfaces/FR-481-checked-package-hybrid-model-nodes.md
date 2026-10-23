---
id: FR-481
title: "Checked-package hybrid model nodes and hybrid claims"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-193
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-469
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-447
    type: references
---

# FR-481: Checked-package hybrid model nodes and hybrid claims

## Description

When a checked package carries a `hybrid-decl`
([FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)
"Source form and the clock line"), the producer SHALL emit its checked
hybrid model as `quire.checked-semantic-graph/v2` nodes: one
`hybrid`/`hybrid_model` node, one `hybrid`/`hybrid_mode` node per mode and
one `hybrid`/`hybrid_transition` node per transition, with each continuous
variable a `value`/`parameter` node and each invariant, rate, guard and reset
value its own term node. It SHALL emit each claim over the model as a
`temporal`/`temporal_clause` node
([FR-370](FR-370-checked-package-temporal-clause-body.md)) whose `over`
parameter is typed by the `hybrid_model` node. The reader SHALL refuse a node
whose rules below do not hold, with the first refusal in the reader order
below. These nodes are the only carrier of the model and its claims: a
process provider reads them from the package bytes
([FR-331](FR-331-backend-provider-envelope.md) "Package bytes") and from
nothing else, so a hybrid item is served by an in-process provider and by a
process provider alike.

## Properties

### Continuous variables

- Each continuous variable is a `value`/`parameter` node
  ([FR-322](FR-322-checked-package-artifact.md)): its `name` is the
  variable's name, its `level` is the variable's position among the model's
  variables, and its `semantic_type` is the `scalar_type`/`rational` node,
  since a continuous variable's values are exact rationals.
- The variables are ordered by first appearance as an equation's left side
  across the declaration's modes (QSL FR-447 `CheckedHybridModel::variables`).

### Hybrid terms

- An invariant, a rate, a guard and a reset value is a term node: an
  `expression` node whose body root is an application, a `value`/`literal`
  node, or, for a rate, a reset value or an operand, one of the model's own
  continuous variable `value`/`parameter` nodes, so `x' = v` and the reset
  `x = v` reference `v`'s node directly. A numeric constant is a
  `value`/`literal` node typed `scalar_type`/`rational`, `value_kind`
  `rational`, whose `value` is spelled as FR-322 "Scalar literal values"
  spells a `rational` (`0`, `-3`, `1/2`).
- A term applies only the catalogued operations of the forms QSL FR-447
  admits: `quire.op.rational.negate`, `add`, `sub`, `mul` and `div`;
  `quire.op.rational.lt`, `le`, `eq`, `ge` and `gt`; and
  `quire.op.boolean.not`, `and`, `or` and `implies`. Its operands are
  continuous variable nodes, constants and other term nodes, each reached by
  `reference`, so every body keeps FR-322's fixed body grammar whatever the
  term's depth.
- A variable read in a term is a `reference` to that variable's
  `value`/`parameter` node.

### `hybrid`/`hybrid_model`

- **body** — `{term: "aggregate", members: [binding("variables", …),
  binding("modes", …)]}`, in this order:
  - `variables` — an aggregate of one entry per continuous variable, in
    variable order, each the aggregate `[binding("parameter", reference),
    binding("class", literal)]`: `parameter` references the variable's
    `value`/`parameter` node and `class` is a text literal, `clock` or
    `hybrid` (FR-193 "Source form and the clock line"). The class is the
    producer's classification of the source rate spellings, carried because
    the nodes cannot re-derive it: `t' = 1` and `t' = rational(2, 2)` lower
    to the same literal node, a clock and hybrid dynamics respectively. It
    enters the node key like every other member;
  - `modes` — an aggregate of one `reference` per mode, in declared order,
    to its `hybrid`/`hybrid_mode` node. The first is the initial mode.
- **declaration** — `{qualified_name}`, the declaration's name.
- **owner** — the declaring source unit's `SourceOwner`.
- **occurrences** — exactly one, of role `declaration`, at the `hybrid-decl`.

### `hybrid`/`hybrid_mode`

- **body** — `{term: "aggregate", members: [binding("invariant", …),
  binding("rates", …), binding("transitions", …)]}`, in this order:
  - `invariant` — a `reference` to the Boolean term node of the mode's
    invariant;
  - `rates` — an aggregate of one `reference` per continuous variable, in
    variable order: entry `i` is the term node of the right side of variable
    `i`'s equation in this mode;
  - `transitions` — an aggregate of one `reference` per transition, in
    declared order, to its `hybrid`/`hybrid_transition` node; empty for a
    mode with none.
- **declaration** — `{qualified_name}`, the model's `qualified_name` with
  the mode's name appended, so `Moving` of `Braking` is `[Braking, Moving]`.
- **owner** — the same `SourceOwner` as its model.
- **occurrences** — exactly one, of role `declaration`, at the `mode`
  clause.

### `hybrid`/`hybrid_transition`

- **body** — `{term: "aggregate", members: [binding("target", …),
  binding("guard", …), binding("resets", …)]}`, in this order:
  - `target` — an `integer` literal, the zero-based position of the target
    mode in its model's `modes`;
  - `guard` — a `reference` to the Boolean term node of the guard;
  - `resets` — an aggregate of one entry per reset conjunct, in declared
    order, each the aggregate `[binding("variable", literal),
    binding("value", reference)]`: `variable` is the zero-based position of
    the reset variable in its model's `variables` and `value` references the
    term node of the value it takes, read over the values before the jump.
    A reset `true` is the empty aggregate, and a variable no entry names
    keeps its value.
- **declaration** and **owner** — none. A transition is content-addressed:
  two transitions with equal content share one node.
- **occurrences** — one or more, of role `expression`, one per source
  `transition` clause that checks to this node.

Indices, not references, name a transition's target mode and a reset's
variable, so a transition into an earlier mode adds no graph cycle.

### Every `hybrid` node

- **semantic_type** — the node's own `node_id`.
- **dependencies** — exactly the unique, digest-ascending set of the body's
  `reference` targets.
- **node key** — a `hybrid` node's body holds no application, so it is a
  structural node keyed at FR-322's derived-shape key stage by the
  `quire.structural-node/v1` preimage `{version, node_tag: "hybrid",
  semantic_form, semantic_type: null, declaration, recursion: null, owner,
  body}`, `declaration` `null` and `owner` absent for a transition, as
  [FR-469](FR-469-checked-package-extension-nodes.md) keys an extension
  node. The node carries no `recursion_group` and no
  `nominal_identity_preimage`.

### Hybrid claims

- A **hybrid claim** is a `temporal`/`temporal_clause` node whose `over`
  argument references a `value`/`parameter` node whose `semantic_type` is a
  `hybrid`/`hybrid_model` node: the claim's **model**.
- Its `temporal_profile` law is `quire.temporal.infinite-trace/v1`, its
  clock argument is the text `model-time`, its activation is `origin`, and
  its captures and fairness arguments are empty. Its formula is the FR-370
  formula nodes of the claim's body; `true` and `false` are
  `quire.op.temporal.true` and `quire.op.temporal.false`, and every other
  `holds(e)` is `quire.op.temporal.holds` over `e`'s Boolean term node. An
  interval bound counts seconds and takes FR-370's integer form, `null`,
  `[a,*]` or `[a,b]`, the forms that profile admits.
- Each `holds` operand is a Boolean term node, and a claim's read `h.x` of
  the continuous variable `x` is a `reference` to `x`'s `value`/`parameter`
  node, the node the model's `variables` names.
- The claim's **horizon** is the reach of its formula, the latest time in
  seconds whose state the formula reads, as QSL FR-447 defines it over the
  formula's operators and intervals; it is unbounded when an operator on the
  way has no interval or the upper bound `*`. The package carries no separate
  horizon member: every reader derives it from the formula nodes.
- A provider request names a hybrid claim's item as `claim {node,
  occurrence}` with `node` the `temporal_clause` node
  ([FR-331](FR-331-backend-provider-envelope.md) "Items").

## Identity

Every member of every `hybrid` node enters its node key and, through the
`identity_projection`, the `package_id`: the variables and their classes,
the order of the modes, each mode's name, invariant, rates and transitions,
and each transition's target, guard and resets. A change in a term node
changes the key of every node that references it, up to the model node, the
`over` parameter typed by it and each claim over it. Two packages whose
hybrid models differ in a mode, a rate, a guard, a reset or a classification
therefore have different `package_id`s, and every obligation identity bound
to the package differs with them. Source whitespace and comments enter no
key.

## Reader order

Strict wire validation precedes every body step. The published schema fixes
each `hybrid` body's layout: the binding names and their order, the members
of each `variables` and `resets` entry, the literal kinds of `target`,
`variable` and `class`, the class values `clock` and `hybrid`, at least one
mode and one rate, and each form's occurrence roles. A node outside it
refuses `malformed_wire` at the member's pointer, so no layout defect
reaches this step.

The hybrid step is a body step of
[FR-322](FR-322-checked-package-artifact.md) "Body steps": it runs after the
step-relation step and before the protocol step. Mode names are declared
names, so FR-322's name rules have already refused a second mode of one name
in one model (`ambiguous_declaration`/`ambiguous-name`). The step visits
every `hybrid`/`hybrid_model` node in ascending node-id digest order. For one
model, the checks run in this order and the first failing check is its
defect; the locus is the binding or target named:

1. **Modes listed once.** A `modes` reference to a node an earlier entry
   references refuses `invalid_semantic_graph` at the later entry.
2. **Variables**, in variable order: a `parameter` target that is not a
   `value`/`parameter` node typed `scalar_type`/`rational` whose level is the
   entry's position, or whose name an earlier entry's target carries, refuses
   `invalid_model_binding`/`malformed-declaration` at the target.
3. **Modes**, in body order: a target that is not a `hybrid`/`hybrid_mode`
   node whose `qualified_name` is the model's with one segment appended
   refuses `invalid_model_binding`/`malformed-declaration` at the target.
   Then, for each mode in body order:
   1. **Rate count.** `rates` holding a number of entries other than the
      model's number of variables refuses
      `invalid_model_binding`/`malformed-declaration` at `rates`.
   2. **Terms.** An `invariant` target that is not an `expression` or
      `value`/`literal` node typed `scalar_type`/`boolean` refuses
      `ill_typed`/`non-boolean-root`; a rate target that is neither such a
      node typed `scalar_type`/`rational` nor one of the model's variable
      nodes refuses `ill_typed`/`type-mismatch`; a `transitions` target that
      is not a `hybrid`/`hybrid_transition` node refuses
      `invalid_model_binding`/`malformed-declaration`.
   3. **Transitions**, in body order: a `target` that is not less than the
      model's number of modes, or a reset `variable` that is not less than
      its number of variables, refuses `missing_declaration`/`missing-name`;
      a `guard` target that is not a term node typed `scalar_type`/`boolean`
      refuses `ill_typed`/`non-boolean-root`; a second reset entry for one
      variable refuses `invalid_package`/`conflicting-definition`, naming
      both entries; a reset `value` target that is neither a term node typed
      `scalar_type`/`rational` nor one of the model's variable nodes refuses
      `ill_typed`/`type-mismatch`.
4. **Reads.** Every `value`/`parameter` node that an invariant, rate, guard
   or reset value is, or holds in its dependency closure, is one of the
   model's variable nodes, else `missing_declaration`/`missing-name` at the
   binding that references that term.
5. **Classes.** A `clock` variable whose rate in some mode is not a
   `value`/`literal` node of value `1` refuses
   `invalid_package`/`invalid-value` at its class literal, the first such
   variable in variable order; then a model with no `hybrid` variable
   refuses `invalid_package`/`invalid-value` at `variables`. The reader
   checks no converse: a `hybrid` variable whose every rate is the literal
   `1` is admitted, since its source rate may be `rational(2, 2)`. The class
   is producer-attested, and the reader does not re-derive the clock line
   of FR-193.

Once every model's own checks hold, a `hybrid`/`hybrid_mode` node no model
references, and a `hybrid`/`hybrid_transition` node no mode references,
refuses `invalid_semantic_graph` at that node, in ascending node-id digest
order. Then the step visits every hybrid claim in ascending node-id digest
order:

6. **Claim form.** A `temporal_profile` law other than
   `quire.temporal.infinite-trace/v1` refuses
   `invalid_package`/`invalid-value` at `/body/operation/laws/0`; a clock
   other than `model-time` refuses `invalid_package`/`invalid-value` at
   `/body/arguments/1`; an activation other than `origin` refuses
   `invalid_package`/`invalid-value` at `/body/arguments/2`; a non-empty
   captures or fairness argument refuses `invalid_semantic_graph` at that
   argument. Under `quire.temporal.timed/v1` an integer-form interval has
   already refused `invalid_package`/`operation-member-mismatch` at
   FR-370's temporal step, which runs first.
7. **Operators.** A formula node the claim's formula reaches that applies
   an operator other than `holds`, `true`, `false`, `not`, `and`, `or`,
   `implies`, `always`, `eventually`, `until` and `release` refuses
   `ill_typed`/`operator-ineligible` at that formula node.
8. **Claim reads.** Every `value`/`parameter` node that a `holds` operand
   is, or holds in its dependency closure, is a variable node of the claim's
   model, else `missing_declaration`/`missing-name` at that `holds` formula
   node.

A `dependencies` list that differs from the body's targets is FR-322's
graph-shape refusal, `invalid_semantic_graph` at the node's `/dependencies`,
and a retained key that is not the digest of its preimage is FR-322's
`invalid_package`/`stale-node-key`; both precede this step.

## Hybrid trace correspondence

The reader of a hybrid item's result entry, the host that requested the
item and holds its checked model and these nodes, checks every `HybridTrace`
(FR-193 "Hybrid trace") against the claim's model, whichever provider
returned it, in process or in a process of its own: a trace `mode` names the
last `qualified_name` segment of one of the model's `hybrid_mode` nodes,
`start` is in the first mode, a jump's `transition` is a position in the
previous state's mode's `transitions`, the mode a jump reaches is that
transition's `target`, and the `enclosure` keys are the names of the model's
variable nodes. A process provider reads these nodes to serve the item; its
own reading does not stand for the host's check.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-481-AC-1 | QSL FR-447's `Braking` produces one `hybrid`/`hybrid_model` node with `declaration` `[Braking]`, its `SourceOwner` and one `declaration` occurrence, whose `variables` reference the `value`/`parameter` nodes of `v`, `a` and `t` at levels 0, 1 and 2, typed `scalar_type`/`rational`, classed `hybrid`, `hybrid` and `clock`, and whose `modes` reference `[Braking, Moving]` and `[Braking, Stopped]` in that order; `Moving`'s `rates` reference the nodes of `-a`, `0` and `1`, and its one transition node has `target` 1, a guard referencing the node of `v = 0` and one reset `(0, 0)`, no `declaration` and an `expression` occurrence; every `hybrid` node is its own `semantic_type` and its `dependencies` are exactly its body's targets. | Test (TC-966) |
| FR-481-AC-2 | Each `hybrid` node's key is the digest of its `quire.structural-node/v1` preimage, which meets the published `HybridNode` preimage schema, and every term node's key is the digest of its own FR-322 preimage. | Test (TC-966) |
| FR-481-AC-3 | Replacing `Moving`'s rate of `v` by `-(2 * a)` and rekeying changes the keys of `Moving`, `Braking`, the `over` parameter typed by `Braking` and `SlowEnough`'s clause node; rekeying unchanged nodes keeps every key. | Test (TC-966) |
| FR-481-AC-4 | With `dependencies`, keys and `package_id` rebuilt after each edit, the reader refuses: a variable typed `scalar_type`/`integer` or at the wrong level, `invalid_model_binding`/`malformed-declaration`; `Moving` with a fourth rate, `invalid_model_binding`/`malformed-declaration` at `rates`; an integer-typed invariant, `ill_typed`/`non-boolean-root`; a Boolean rate, `ill_typed`/`type-mismatch`; `target` 2, `missing_declaration`/`missing-name`; two resets of `v`, `invalid_package`/`conflicting-definition` naming both; a guard reading a parameter `w` that is no variable, `missing_declaration`/`missing-name`; `t` classed `clock` with rate `2 - 1` in `Stopped`, `invalid_package`/`invalid-value` at `t`'s class literal; every variable classed `clock`, the same at `v`'s class literal, since `v`'s rate `-a` is not the literal `1`; a one-mode model `Timer` whose only variable `t` is classed `clock` with rate `1` and invariant `t <= 5`, the same at `variables`; `modes` listing `Moving` twice, `invalid_semantic_graph` at the second entry; a second mode node `[Braking, Moving]`, `ambiguous_declaration`/`ambiguous-name`; an unreferenced transition node, `invalid_semantic_graph`. | Test (TC-966) |
| FR-481-AC-5 | A transition node with a `declaration`, a mode node whose occurrence has role `claim`, a model body with `modes` before `variables`, a `class` `continuous`, a transition whose `target` is a reference, a mode with no rates, a `hybrid` node of form `hybrid_flow`, and a model node without its `owner` each fail the published `SemanticNode` schema, and a reader refuses each `malformed_wire` at strict wire validation, before the hybrid step. | Test (TC-966) |
| FR-481-AC-6 | `SlowEnough` is one `temporal`/`temporal_clause` node whose `temporal_profile` law is `quire.temporal.infinite-trace/v1`, whose `over` references the `h` parameter node typed by `Braking`'s model node, with clock `model-time`, activation `origin`, empty captures and fairness, and a formula whose root is `quire.op.temporal.always` with interval `[0, 10]` over a `quire.op.temporal.holds` node whose operand is the node of `v < 30`, which reads `v`'s variable node. | Test (TC-966) |
| FR-481-AC-7 | A process provider handed the v2 bytes of a package holding `Braking` and `SlowEnough` reads the model and the claim from those nodes alone and derives horizon 10. The host that reads its result entry checks the returned `HybridTrace` against `Braking`: a trace naming `Moving` then `Stopped` by `transition` 0 is kept on the `inconclusive` record, and one naming mode `Halted` is refused by the host `invalid_result`/`invalid-value` at its `mode`. | Test (TC-966) |
| FR-481-AC-8 | The reader refuses `SlowEnough` with clock `model-steps` at `/body/arguments/1`, with an `each` activation at `/body/arguments/2`, and with the `temporal_profile` law `quire.temporal.event-position.false-extension/v1` at `/body/operation/laws/0`, each `invalid_package`/`invalid-value`; under `quire.temporal.timed/v1` its `[0, 10]` interval refuses `invalid_package`/`operation-member-mismatch` at FR-370's temporal step; a `once` formula node refuses `ill_typed`/`operator-ineligible`; and a `holds` operand reading `h` itself refuses `missing_declaration`/`missing-name`. | Test (TC-966) |
| FR-481-AC-9 | `positive-all-families.json` carries the `hybrid` tag as the model `Tank` with variables `x` and `y`, both classed `hybrid`, and one mode `Filling` whose rate of `x` is a reference to `y`'s variable node, whose rate of `y` is the rational literal `1`, and whose one transition resets `x` to a reference to `y`'s variable node; the full reader admits the package, so a bare-variable rate and reset value and a `hybrid` variable whose every rate is the literal `1` are each admitted. | Test (TC-966) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits the `hybrid` node tag with
the forms `hybrid_model`, `hybrid_mode` and `hybrid_transition` and their
closed body shapes and occurrence roles; `node-identity-preimage.schema.json`
defines the closed `HybridNode` preimage; `hybrid-node-vectors.json` holds
the `Braking` and `SlowEnough` nodes and the schema refusals; and
`fixtures/positive-all-families.json` carries the `Tank` model.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the node form, keys,
  `declaration`, `owner`, `dependencies` and the reader's refusal order.
- [FR-193](../../functional/analysis/FR-193-analyze-continuous-hybrid-models.md)
  owns the hybrid model's meaning, the clock line, the solver contract and
  the trace.
- [FR-370](FR-370-checked-package-temporal-clause-body.md) owns the
  temporal clause and formula nodes a hybrid claim is.
- [FR-331](FR-331-backend-provider-envelope.md) owns the request item and
  the package bytes a process provider reads.
- QSL FR-447 owns the source checks, the classification and the horizon.

## References

- STD-385 (this requirement); quire-spec-language FR-447 and ADR-026 HY-1
  to HY-3.
