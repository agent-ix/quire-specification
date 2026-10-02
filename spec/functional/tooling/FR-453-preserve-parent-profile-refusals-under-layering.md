---
id: FR-453
title: "Preserve a parent profile's refusals under its child profile"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-003
    type: references
  - target: ix://agent-ix/quire-specification/FR-001
    type: references
  - target: ix://agent-ix/quire-specification/FR-301
    type: references
  - target: ix://agent-ix/quire-specification/FR-452
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-345
    type: references
---

# FR-453: Preserve a parent profile's refusals under its child profile

## Description

When a refinement test gate compares the parent and child side of an
AD-003 profile-layering case whose parent side refuses the unit with a typed
refusal and whose child side admits it or prohibits its form, the gate SHALL
report a regression naming the case and its edge.

When a parent layer admits its edge's distinguishing unit, the gate SHALL
report a regression naming the edge.

When a layer's witness unit is refused by the layer, or its witness item,
requested once from each candidate backend, settles `supported` under none
of them, the gate SHALL report a regression naming the layer.

## Inputs

A corpus of layering cases. A case names one layering edge and two source
units, the parent unit and the child unit, that are byte-equal except for
the identity string of one `profile` declaration. Each edge also has one
distinguishing unit and each layer one witness unit.

## Outputs

One case result per case, one result per edge for its distinguishing unit,
one result per layer for its witness unit, one verdict for the run and its
FR-301 exit code, as
[FR-452](FR-452-preserve-admission-across-specification-revisions.md)
reports them.

## Behavior

### Layering edges

Each `header_selectable_layers` row of the
[complete-value lock](../../../proposals/quire-v1/definitions/complete-value-lock.json)
is a layer a `profile` declaration selects by naming its identity (FR-001).
The row carries the layer's identity, the layers it `requires`, its
`admitted_forms` definition, what it `admits` and its `witness_request`
(an FR-290 kind and an FR-331 extent classification). A layering edge is one AD-003 `requires` edge between two of those
layers. The gate covers exactly these five edges:

| Edge | Parent | Child |
| --- | --- | --- |
| E1 | `quire.state.core/v1` | `quire.state.queries/v1` |
| E2 | `quire.state.queries/v1` | `quire.state.graph/v1` |
| E3 | `quire.state.core/v1` | `quire.value.complete/v1` |
| E4 | `quire.state.graph/v1` | `quire.model.complete/v1` |
| E5 | `quire.value.complete/v1` | `quire.model.complete/v1` |

AD-003 forbids a child from weakening an inherited rule, so a unit the parent
refuses with a typed refusal stays refused under the child.

### One unit under both sides

- The parent unit has one `profile` declaration naming the parent layer, and
  the child unit names the child layer in the same declaration under the same
  alias, so every declaration's `using` names the same alias on both sides.
- A case whose two units differ in any byte outside that identity string is
  a tool failure naming the case.
- Each side's selected definitions follow FR-001: its layer and the layer's
  `requires` closure, plus the qualification rows the complete-value lock's
  `package_selection` gives for those layers.
- A case naming a pair of layers that is not one of E1 to E5 is a tool
  failure naming the case.
- Each side compiles in the running build. No outcome is recorded or
  written by hand; both sides are compiled.

### Comparison

For each case, with parent result `R` and child result `C`, the first
matching rule gives the case result:

1. either side is a tool failure: tool failure;
2. `R` is a typed refusal and `C` is a typed refusal: holds, reporting both
   sides' codes;
3. `R` is a typed refusal and `C` is admitted or prohibited: regression,
   naming the case, the edge and `R`'s codes;
4. `R` is a typed refusal and `C` is unsupported or incomplete: unresolved;
5. `R` is incomplete: unresolved;
6. otherwise: not applicable.

A parent that prohibits a form (`unsupported_construct` with a catalog
cause) gives not applicable: a child may add forms its parent prohibits
(AD-003), so a prohibition is not a refusal the child preserves. Verdict,
exit code and evidence status are FR-452's.

### Each layer is a subset users pick

A layer is a real user subset when a user who selects it gets a smaller
language that registered backends serve. The gate checks
this against the running build in two parts:

- **Witness.** Each layer has a witness unit that uses the forms its
  `admits` statement names, for example a bounded-scalar invariant for
  state core. The witness selects only that layer and holds one clause, the
  witness item. Its required FR-290 kind and extent classification are the
  layer's `witness_request`: `operation-contract`, `bounded` for state
  core, state queries, state graph and complete model, whose witnesses are
  operation and invariant clauses; `value-validity`, `bounded` for complete
  value, whose witness is a Boolean clause over values. The gate compiles
  the witness, takes the registry's FR-290 candidate set for that kind,
  and submits the item in one FR-331 request per candidate, each naming
  that candidate's backend, so `negotiate_*` settles each through that
  backend's negotiation arm (FR-331-AC-4). The layer result is holds when
  the layer admits the witness and at least one candidate settles the item
  `supported`: some registered backend accepts the subset. Otherwise it is
  regression naming the layer and either the refusing stage or every
  candidate's backend identity and disposition; an empty candidate set is
  named as such.
- **Proper subset.** Each edge has a distinguishing unit that uses one form
  its child adds over its parent. The edge result is holds when the parent
  prohibits that form with `unsupported_construct` naming the parent layer
  and the child admits the unit, and regression naming the edge otherwise. A
  parent that admits its child's distinguishing unit adds nothing a user
  would select it for.

The distinguishing form of each edge is the one the child's definition file
adds: a named predicate or query for E1; a reference or `reaches` for E2;
whole-record equality or a `record` declaration for E3; conformance or
dispatch over a supertype for E4; object navigation for E5.

quire-spec-language FR-345 implements this gate.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-453-AC-1 | On each of E1 to E5, a case whose unit the parent layer refuses with a typed refusal and the child layer admits is a regression naming the case, the edge and the parent's codes; verdict violation, exit 10. | Test (TC-414) |
| FR-453-AC-2 | On each of E1 to E5, a case both layers refuse holds and reports both codes, and a case whose unit uses a form the parent prohibits and the child admits is not applicable. | Test (TC-414) |
| FR-453-AC-3 | A case whose two units differ outside the identity string of their `profile` declaration is a tool failure, verdict tool failure, exit 30. | Test (TC-414) |
| FR-453-AC-4 | A case naming a pair of layers that is not one of E1 to E5, such as state core and complete model, is a tool failure naming the case. | Test (TC-414) |
| FR-453-AC-5 | Each of the five layers, selected alone by a `profile` declaration, admits its witness unit, and its witness item, requested with the layer's `witness_request` kind and extent once from each candidate backend by name, settles `supported` under at least one candidate; with two candidates of which one supports and one does not, the layer holds. A layer that refuses its witness, or whose item settles `supported` under no candidate, gives a regression naming the layer and the stage, or each candidate's backend identity and disposition. | Test (TC-414) |
| FR-453-AC-6 | On each of E1 to E5, the parent prohibits the edge's distinguishing unit with `unsupported_construct` naming the parent layer and the child admits it; a parent that admits it gives a regression naming the edge, verdict violation, exit 10. | Test (TC-414) |

## Dependencies

- [AD-003](../../assurance/AD-003-native-quire-language-charter.md) owns the
  profile hierarchy, its `requires` edges and the non-weakening rule.
- [FR-001](../foundation/FR-001-select-native-profile.md) owns profile
  selection by `profile` declaration.
- The `header_selectable_layers` rows of the
  [complete-value lock](../../../proposals/quire-v1/definitions/complete-value-lock.json)
  own each layer's `requires` edges, admitted-form set and accepting
  targets.
- [FR-452](FR-452-preserve-admission-across-specification-revisions.md)
  owns the case results, verdict and exit code.
- [FR-301](FR-301-expose-complete-cli.md) owns the severity order and exit
  codes.

## References

- Capability row V1-TOOL-012.
- quire-spec-language ADR-017 §2 RF-3, §6 Q-5; FR-345.
- Specification tickets STD-119 (owner ruling, option A), STD-117.

## Status

Proposed.
