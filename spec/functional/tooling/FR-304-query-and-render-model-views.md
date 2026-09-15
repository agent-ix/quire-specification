---
id: FR-304
title: "Query and render deterministic model views"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
---

# FR-304: Query and render deterministic model views

## Description

When a user requests model metadata or a rendered view, the tool SHALL evaluate
the typed query against the exact checked package and report every omitted or
unrepresentable semantic fact.

## Inputs

Checked package, typed view subject/query, renderer profile and limits.

## Outputs

Canonical metadata or deterministic rendered artifact plus source/provenance and
loss records.

## Behavior

View subjects use qualified declaration/runtime identities. Canonical ordering
is defined for set-valued metadata. Rendering cannot become a clause authority,
infer missing model relationships or claim equivalence from visual inclusion.

## View contract

A query declares its source artifact, identity root, traversal relation, filter,
projection and finite result bound. Results preserve qualified identities and
canonical semantic ordering. A renderer consumes only that typed result and a
versioned style profile, producing an artifact plus an identity-to-region map.
Truncation is marked incomplete with the omitted frontier; visual adjacency,
layout or labels never create a semantic relationship.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-304-AC-1 | Equal package/query/renderer inputs produce byte-identical output and loss records. | Test (TC-224) |
| FR-304-AC-2 | Every displayed/omitted fact maps to its exact semantic/source identity. | Test (TC-224) |
| FR-304-AC-3 | Unknown subject, unsupported view meaning or exhausted rendering refuses/incompletes without a misleading partial-equivalence claim. | Test (TC-224) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
