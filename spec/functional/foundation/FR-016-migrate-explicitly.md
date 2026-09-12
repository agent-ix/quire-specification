---
id: FR-016
title: "Require explicit immutable reference migration"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-004"
    type: traces_to
---
# FR-016: Require explicit immutable reference migration

## Description

When references from different versions are joined, the consumer shall require an explicitly accepted mapping for that join.

## Inputs

Exact old/new references, selected mapping artifact and claimed preserved aspects.

## Outputs

A justified equivalence or unsupported/ambiguous migration result.

## Behavior

Mappings retain historical bytes and identify their own version/digest. Common-kind ArtifactRef conversion may preserve byte-artifact correspondence only. Semantic obligations, reverse mappings, transitive composition and authored identity adoption are not inferred.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-016-AC-1 | An unselected mapping leaves the version join unsupported. | Test |
| FR-016-AC-2 | A common-kind byte mapping preserves the underlying artifact bytes. | Test |
| FR-016-AC-3 | Conflicting applicable mappings receive ambiguous-equivalence. | Test |
| FR-016-AC-4 | A byte-reference mapping does not grant semantic evidence reuse. | Test |

## Dependencies

- [US-004](../../usecase/US-004-retain-evidence-meaning.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/interchange.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
