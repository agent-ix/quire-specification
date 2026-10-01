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
  - target: ix://agent-ix/quire-spec-language/FR-345
    type: references
---

# FR-453: Preserve a parent profile's refusals under its child profile

## Description

When a refinement test gate compiles one source unit under the parent and
the child side of a V1 profile-layering edge, the gate SHALL report a
regression for every case whose parent side refuses the unit with a typed
refusal and whose child side admits it or prohibits its form.

## Inputs

A corpus of layering cases. A case names one layering edge and two source
units, the parent unit and the child unit, that are byte-equal except for
their `profile` declarations.

## Outputs

One case result per case, one verdict for the run and its FR-301 exit code,
as [FR-452](FR-452-preserve-admission-across-specification-revisions.md)
reports them.

## Behavior

### V1 layering edges

AD-003 gives each child profile explicit dependency edges to its parents and
forbids a child from weakening an inherited rule. A V1 layering edge is an
AD-003 dependency edge whose parent and child are each the root of a V1
definition lock, so that a package selects either side through its
`profile` declarations (FR-001). V1 has exactly one such edge:

| Parent | Child | Why |
| --- | --- | --- |
| `quire.value.complete/v1`, root of `complete-value-lock.json` | `quire.model.complete/v1`, root of `complete-model-lock.json` | The model lock requires the value lock, and the model root selects the value root's closure unchanged. |

The state core, state queries and state graph layers are selected inside a
lock's package selection, never by a `profile` declaration, so no V1 package
compiles under one of them alone and none is a side of a V1 layering edge.
Their inherited rules are verified by the conformance vectors of the root
that selects them.

### One unit under both sides

- The parent unit's `profile` declarations select the parent root, and the
  child unit's select the child root, under the same alias, so every
  declaration's `using` names the same alias on both sides.
- A case whose two units differ in any byte outside their `profile`
  declarations is a tool failure naming the case.
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

quire-spec-language FR-345 implements this gate.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-453-AC-1 | A case on the value-to-model edge whose unit the value root refuses with a typed refusal and the model root admits is a regression naming the case, the edge and the parent's codes; verdict violation, exit 10. | Test (TC-414) |
| FR-453-AC-2 | A case both roots refuse holds and reports both codes; a case whose unit uses a model form, which the value root prohibits and the model root admits, is not applicable. | Test (TC-414) |
| FR-453-AC-3 | A case whose two units differ outside their `profile` declarations is a tool failure, verdict tool failure, exit 30. | Test (TC-414) |
| FR-453-AC-4 | A case naming a pair of profiles that is not a V1 layering edge, such as state core and state queries, is a tool failure naming the case. | Test (TC-414) |

## Dependencies

- [AD-003](../../assurance/AD-003-native-quire-language-charter.md) owns the
  profile hierarchy, its dependency edges and the non-weakening rule.
- [FR-001](../foundation/FR-001-select-native-profile.md) owns profile
  selection by `profile` declaration.
- [FR-452](FR-452-preserve-admission-across-specification-revisions.md)
  owns the case results, verdict and exit code.
- [FR-301](FR-301-expose-complete-cli.md) owns the severity order and exit
  codes.

## References

- Capability row V1-TOOL-012.
- quire-spec-language ADR-017 §2 RF-3, §6 Q-5; FR-345.
- Specification tickets STD-119, STD-117.

## Status

Proposed. The V1 edge set and the two-unit case selection are the
recommended answer to ADR-017 Q-5 and await the owner's ruling (see
References). The comparison is ADR-017 RF-3's, already decided.
