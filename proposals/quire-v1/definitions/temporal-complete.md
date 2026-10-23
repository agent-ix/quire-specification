# Complete temporal definition

Definition identity: `quire.temporal.complete/v1`.
Language: `ix:native` / `1-draft`. This definition is the complete-V1 timed
model root: the time source, clocks, resets, clock constraints, time
invariants and urgency of a state model. Implementation and qualification
remain separately reported.

## Required definition

A package that selects this root also selects the complete-model root
[`quire.model.complete/v1`](model-complete.md), which this root requires (its
`header_selectable_layers` row in the
[complete-value lock](complete-value-lock.json)), and so the complete-value
root, the state-graph layer and every definition those roots select through
[`complete-model-lock.json`](complete-model-lock.json) and
[`complete-value-lock.json`](complete-value-lock.json).

## Admitted forms

The forms this root adds over the complete-model root are the timed forms of
the shared grammar's "Probabilistic, timed and real-time forms" section:

- the `time` member of a state model, `time M dense unit u;` and
  `time M tick O period p unit u;` (`time-decl`);
- the `Clock` type in a type position (`type-ref =/ 'Clock'`), including a
  `Clock` field of an object type, a `Clock` parameter or result, a record
  field, a tuple element, a union payload, a type alias target and the
  argument of `Option` and of a collection type;
- an atomic clock constraint, `x ~ c` over a `Clock` and a time
  `quantity-literal`, in an operation precondition, a postcondition reset
  `self.x = c`, and the `clock-bound` of a time invariant;
- clock places and clock collections as
  [FR-415](../../../spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md)
  "Clock places" and "Clock collections" define them: a `Clock` held by a
  record, tuple, union payload or object field; a collection (`Set`,
  `Sequence`, `Bag`, `OrderedSet`, `Map`) of `Clock` or of a clock-bearing
  type, including nested collections and a field with multiplicity above one;
  `Option<Clock>` and `Option` of a clock-bearing type, with `present` and
  `value` guards on the place; `Clock` parameters and results of operations,
  functions and predicates, bound by place, and clock-reading predicates and
  functions whose quantity parameter is a bound supplied by constant arguments;
- clock reads and resets over clock collections: element and slot selection
  (`self.ts[i].x`, `self.m[k].x`), `forall(e in p: B)` and `exists(e in p: B)`
  over atomic clock constraints, and reset by selection, `self.p[i].x = c` and
  `forall(e in p: P(e) implies e.x = c)`;
- `Reference<Clock>` and its place operations as FR-415 defines them: a
  `Reference<Clock>` or `Option<Reference<Clock>>` held by a field, parameter,
  result, record component or collection element, `ref(p)` naming the clock
  place `p`, and `deref(r)` as a place expression in a clock constraint or a
  reset (admitted where `present(r)` is established), with `=` and `!=` over
  two references as data comparisons;
- a package clock constraint, a `max c` constraint whose `appliesTo` is `Clock`
  or an alias of it, as the time invariant `x <= c` on every typed clock place
  (FR-415 "Clock places"; FR-154-AC-18 for its intake); a `min` constraint on
  `Clock` is admitted by intake and refused `invalid_timed_model`/
  `time-invariant-shape` by FR-415;
- `time invariant when P { ... }` (`time-invariant`);
- `urgent when P;` and `urgent O;` (`urgent-decl`).

[FR-415](../../../spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md)
gives their meaning and their refusals under `invalid_timed_model`.

A unit whose selected layers do not include this root and that writes one of
these forms receives `unsupported_construct`, cause `declaration-form` for
`time-decl`, `time-invariant` and `urgent-decl` and for a `Clock` in a type
position, naming the unit's selected layers and this root as the layer that
admits the form. The probabilistic, scheduling and temporal-clause forms of the
same shared-grammar section are admitted under the facets and profiles that
own them, and selecting this root neither admits nor refuses them.

## Qualification catalog and package selection

The canonical lock is [`complete-temporal-lock.json`](complete-temporal-lock.json).
Its `requires` entry names the complete-model lock. Its
`qualification_catalog` closes every artifact this root selects by role,
authority, identity and artifact path, and its `package_selection` places every
catalog role in `always`; this root has no conditional or alternative profile.
A package selection is checked with the same closed refusal codes and order as
the complete-value lock.

## Selected rules

Each rule below is a `rule_*` role of the lock's `qualification_catalog`.

| Rule |
| --- |
| [Clocks and timed models](../../../spec/functional/temporal/FR-415-declare-clocks-and-timed-models.md) |
| [Exact rational value](../../../spec/objects/foundation/FR-205-exact-rational-value.md) |

The exact-quantity rule of [FR-142](../../../spec/functional/type-model/FR-142-evaluate-quantities-and-units.md)
is selected by the complete-value root and applies unchanged to time
quantities.
