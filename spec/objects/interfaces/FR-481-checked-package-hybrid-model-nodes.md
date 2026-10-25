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
  - target: ix://agent-ix/quire-specification/FR-362
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
  the exact type of every constant the model writes; the variable's value
  ranges over the reals its flows and jumps reach (FR-193 "Continuous values
  and expressions").
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
- A term takes only the forms FR-193 "Continuous values and expressions"
  admits: an application of `quire.op.rational.negate`, `add`, `sub`, `mul`
  or `div`, of `quire.op.rational.eq`, `ne`, `lt`, `le`, `ge` or `gt`, or of
  `quire.op.boolean.not`, `and`, `or` or `implies`; an
  `expression`/`conditional` node (`if c then a else b`); and an
  `expression`/`call` node applying a `function` node whose parameters and
  result are `scalar_type`/`boolean` or numeric (FR-322). A `let` is
  substituted before emission and has no node. Its operands are
  continuous variable nodes, constants and other term nodes, each reached by
  `reference`, so every body keeps FR-322's fixed body grammar whatever the
  term's depth.
- A variable read in a term is a `reference` to that variable's
  `value`/`parameter` node.

### `hybrid`/`hybrid_model`

- **body** — `{term: "aggregate", members: [binding("variables", …),
  binding("modes", …), binding("initial", …)]}`, in this order, the
  `initial` binding present only as stated below:
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
    to its `hybrid`/`hybrid_mode` node;
  - `initial` — an aggregate of one entry per initial entry (FR-193 "Initial
    states"), in source order, each the aggregate `[binding("mode", literal),
    binding("condition", reference)]`: `mode` an `integer` literal, the
    zero-based position of the entry's mode in `modes`, and `condition` a
    `reference` to the Boolean term node of its condition. The producer SHALL
    omit the binding when the checked initial entries are exactly the one
    default entry, the first mode with the condition `true`, and SHALL write
    it otherwise; a model with no `initial` binding has that default entry,
    so its first mode is its one initial mode.
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
  - `resets` — an aggregate of one entry per reset conjunct `x ~ e`
    (FR-193 "Resets"), in declared order, each the aggregate
    `[binding("variable", literal), binding("value", reference),
    binding("relation", literal)]`: `variable` is the zero-based position of
    `x` in its model's `variables`, `value` references the term node of `e`,
    read over the values before the jump, and `relation` is a `text` literal,
    `ne`, `lt`, `le`, `ge` or `gt`, present only when `~` is not `=`. An
    entry with no `relation` sets `x` to the value; the other entries bound
    the value `x` takes. A reset `true` is the empty aggregate, and a variable
    no entry names keeps its value.
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
  `hybrid`/`hybrid_model` node: the claim's **model**. Its arguments are
  FR-370's, with the meaning FR-193 "Hybrid claims" gives them.
- Its `temporal_profile` law is the profile the claim's `using` alias
  selects, any profile FR-370 admits. Its clock argument is the text
  `model-time` or `model-steps`.
- Its activation is `origin`, or `each` with a `trigger` that references the
  `on each` parameter's `value`/`parameter` node, named by the parameter at
  level 1 (the `over` parameter's node being at level 0) and typed by the
  claim's model node, and a `guard` that is a Boolean term node when the clause writes
  `when`.
- Its captures argument holds one binding per capture, named by the capture
  and holding its initializer term node, in source order.
- Its fairness argument references one `temporal`/`fairness` node per
  resolved constraint (FR-362, FR-193 "Fairness"), each a
  `quire.op.temporal.fair` application whose `fairness` member names hybrid
  transitions: `declaration` the claim's `hybrid_model` node and `name` a
  mode's name for `fair S`, every transition of `S`; or `declaration` the
  `hybrid_mode` node of `S` and `name` the target mode's name for
  `fair S::T`.
- Its formula is the FR-370 formula nodes of the claim's body, future and
  past operators alike; `true` and `false` are `quire.op.temporal.true` and
  `quire.op.temporal.false`, and every other `holds(e)` is
  `quire.op.temporal.holds` over `e`'s Boolean term node. Each interval takes
  the one form FR-370 "Placement and profile fit" admits under the claim's
  profile: the integer form under the integer profiles, counting seconds
  under `model-time` and positions under `model-steps`; and the timed form in
  exact rational seconds under `quire.temporal.timed/v1`, where a hybrid
  claim also admits the timed `[q, *]`, `upper` `null` with `upper_end`
  `open`. So `[2, *]` has the one encoding `{lower: "2", upper: null}` under
  `quire.temporal.infinite-trace/v1`, and FR-370's profile fit refuses the
  other form.
- Each `holds` operand, guard and capture initializer is a term node of
  "Hybrid terms". A read `h.x` of the `over` parameter is a `reference` to
  `x`'s variable node. With `n` continuous variables, a read `q.x` of the
  `on each` parameter is a `reference` to a `value`/`parameter` node named
  `x` at level `n + i`, `i` the position of `x`, typed
  `scalar_type`/`rational`, the value `x` had at the activation position;
  and a read of the `j`-th capture is a `reference` to a `value`/`parameter`
  node named by the capture at level `2n + j`, typed by the capture's type.
- The claim's **horizon** is the reach of its formula, as QSL FR-447 defines
  it over the formula's future and past operators and intervals: unbounded
  for an `each` activation and when a future operator on the way has no
  interval or the upper bound `*`, and in seconds under `model-time` and in
  positions under `model-steps`. The package carries no separate horizon
  member: every reader derives it from the formula nodes.
- A provider request names a hybrid claim's item as `claim {node,
  occurrence}` with `node` the `temporal_clause` node
  ([FR-331](FR-331-backend-provider-envelope.md) "Items").

## Identity

Every member of every `hybrid` node enters its node key and, through the
`identity_projection`, the `package_id`: the variables and their classes,
the order of the modes, the initial entries, each mode's name, invariant,
rates and transitions, and each transition's target, guard and resets with
their relations. A change in a term node
changes the key of every node that references it, up to the model node, the
`over` parameter typed by it and each claim over it. Two packages whose
hybrid models differ in a mode, an initial entry, a rate, a guard, a reset or
a classification
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
step-relation step and before the timed-model step; the timed-model step runs
before the protocol step. Mode names are declared names, so FR-322's name
rules have already refused a second mode of one name
in one model (`ambiguous_declaration`/`ambiguous-name`). The step visits
every `hybrid`/`hybrid_model` node in ascending node-id digest order. For one
model, the checks run in this order and the first failing check is its
defect; the locus is the binding or target named:

1. **Modes listed once.** A `modes` reference to a node an earlier entry
   references refuses `invalid_semantic_graph` at the later entry.
   A present `initial` that is exactly the default entry, mode 0 with the
   condition `true`, refuses `invalid_semantic_graph` at `initial`, the
   default having one spelling; the `initial` layout is the schema's.
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
      refuses `ill_typed`/`non-boolean-root`; a variable with an entry that
      has no `relation` and any second entry refuses
      `invalid_package`/`conflicting-definition`, naming both entries; a
      reset `value` target that is neither a term node typed
      `scalar_type`/`rational` nor one of the model's variable nodes refuses
      `ill_typed`/`type-mismatch`. The reset entry layout and its `relation`
      spelling are the schema's.
4. **Initial entries**, in body order: a `mode` that is not less than the
   model's number of modes refuses `missing_declaration`/`missing-name`, and
   a `condition` target that is not a term node typed `scalar_type`/`boolean`
   refuses `ill_typed`/`non-boolean-root`, each at the binding.
5. **Reads.** Every `value`/`parameter` node that an invariant, rate, guard,
   reset value or initial condition is, or holds in its dependency closure
   outside the bodies of the `function` nodes its calls apply, is one of the
   model's variable nodes, else `missing_declaration`/`missing-name` at the
   binding that references that term. A called `function` node's parameters
   and result are `scalar_type`/`boolean` or numeric, else
   `ill_typed`/`type-mismatch` at the call.
6. **Classes.** A `clock` variable whose rate in some mode is not a
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

7. **Claim form.** A clock other than `model-time` and `model-steps`
   refuses `invalid_package`/`invalid-value` at `/body/arguments/1`; an
   `each` activation whose `trigger` is not a `value`/`parameter` node typed
   by the claim's model node refuses `ill_typed`/`type-mismatch` at the
   trigger; a `guard` or capture initializer that is not a term node of
   "Hybrid terms" typed `scalar_type`/`boolean` (a guard) or of its
   capture's type refuses `ill_typed`/`type-mismatch` at it. The claim's
   profile and the fit of each interval to it are FR-370's temporal step's
   checks, which run first, with the hybrid `[q, *]` of "Hybrid claims".
8. **Fairness.** A hybrid fairness member whose
   `declaration` is neither the claim's model node nor one of its
   `hybrid_mode` nodes, or whose `name` is no mode of the model, or that
   names no transition, refuses `missing_declaration`/`missing-name` at the
   member; FR-370 "Fairness resolution" does not apply to it. An interval in
   a form the claim's profile does not admit is refused before this step by
   FR-370's profile fit, `invalid_package`/`operation-member-mismatch`.
9. **Claim reads.** Every `value`/`parameter` node that a `holds` operand,
   guard or capture initializer is, or holds in its dependency closure
   outside the bodies of called `function` nodes, is a variable node of the
   claim's model, an activation read node of an `each` claim or a capture
   node of the claim, each with the name and level "Hybrid claims" gives it,
   else `missing_declaration`/`missing-name` at that formula node or
   argument.

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
`start` is in an initial mode, a jump's `transition` is a position in the
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
| FR-481-AC-8 | QSL FR-447's `Braking2` produces a `hybrid_model` node whose body is `variables`, `modes` and an `initial` of two entries, `(mode 0, condition v = 20 and a >= 1 and a <= 2 and t = 0)` and `(mode 1, condition v = 0 and t = 0)`, each condition one Boolean term node, and whose `Moving` transition carries the resets `(variable 0, value 0)` with no `relation` and `(variable 1, value 1, relation ge)`; `Braking`, whose initial entries are the default, carries no `initial` binding; every node of `restored_nodes` meets `SemanticNode` and carries its own key. | Test (TC-966) |
| FR-481-AC-9 | `Slows` is a `temporal_clause` node whose `temporal_profile` law is `quire.temporal.timed/v1`, with clock `model-time`, an `each` activation whose `trigger` is the `q` parameter node at level 1 typed by `Braking2`'s model node and whose `guard` reads the activation read node of `v` at level 3, one capture `v0` holding that read node, a fairness node whose member names the `[Braking2, Moving]` mode node and `Stopped`, and an `eventually` formula node carrying the timed interval `[0, 21/2]` whose `holds` operand reads `v`'s variable node and the capture node `v0` at level 6, typed by the `bounded_domain`/`rational_range` node of `Rational[0, 20; 1, 1]`; `Recent`, under `quire.temporal.infinite-trace/v1`, has clock `model-steps` and a `once` formula node with the integer interval `[0, 2]`. | Test (TC-966) |
| FR-481-AC-10 | With `dependencies`, keys and `package_id` rebuilt after each edit, the reader refuses: `Braking`'s model with the written default `initial` `(mode 0, condition true)`, `invalid_semantic_graph` at `initial`; an initial `mode` 2, `missing_declaration`/`missing-name`; an initial condition referencing the rational literal `20`, `ill_typed`/`non-boolean-root`; a reset `(variable 0, value 0)` beside `(variable 0, value 1, relation ge)`, `invalid_package`/`conflicting-definition` naming both; a fairness member naming `Halted`, `missing_declaration`/`missing-name`; and a `holds` operand reading a parameter `v` at level 4, `missing_declaration`/`missing-name`. | Test (TC-966) |
| FR-481-AC-11 | A model body with an empty `initial`, an initial entry without `condition`, and a reset entry with the `relation` `approx` or `eq` each fail the published `SemanticNode` schema, so a reader refuses each `malformed_wire` at strict wire validation; a timed interval with `upper` `null` meets the schema, and the reader admits it only in a hybrid claim under `quire.temporal.timed/v1`, refusing it in any other clause as FR-370 refuses an interval its profile does not admit; under `quire.temporal.infinite-trace/v1` `[2, *]` is the integer form `{lower: "2", upper: null}`, and a hybrid claim carrying it in the timed form there is refused `invalid_package`/`operation-member-mismatch` at the application. | Test (TC-966) |
| FR-481-AC-12 | `SlowEnough` written under a profile alias selecting `quire.temporal.timed/v1`, its interval in FR-370's timed form `[0, 10]` with both ends closed in exact rational seconds, is admitted with the `timed/v1` law, as it is under `quire.temporal.infinite-trace/v1` in the integer form; and a `holds` operand reading `h` itself refuses `missing_declaration`/`missing-name` at the `holds` node. | Test (TC-966) |
| FR-481-AC-13 | `positive-all-families.json` carries the `hybrid` tag as the model `Tank` with variables `x` and `y`, both classed `hybrid`, and one mode `Filling` whose invariant and transition guard are the Boolean literal `true`, whose rate of `x` is a reference to `y`'s variable node, whose rate of `y` is the rational literal `1`, and whose one transition resets `x` to a reference to `y`'s variable node; the full reader admits the package, so a bare-variable rate and reset value and a `hybrid` variable whose every rate is the literal `1` are each admitted. | Test (TC-966) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits the `hybrid` node tag with
the forms `hybrid_model`, `hybrid_mode` and `hybrid_transition` and their
closed body shapes and occurrence roles; `node-identity-preimage.schema.json`
defines the closed `HybridNode` preimage; `hybrid-node-vectors.json` holds
the `Braking` and `SlowEnough` nodes, the `restored_nodes` of `Braking2`,
`Slows` and `Recent`, and the schema refusals; and
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
