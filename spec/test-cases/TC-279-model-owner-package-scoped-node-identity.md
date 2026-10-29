---
id: TC-279
title: "Model-owner package-scoped node identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-279: Model-owner package-scoped node identity

## Description

Show that a `ModelOwner` carries the declaring domain
package's `identity` beside `node` and no version label, and that the
published `ModelOwner` vectors validate against the closed schema.

## Preconditions

- FR-321 admits at most one domain package per `identity`.
- The `proposals/checked-package-v2/schema.json`
  `ModelOwner` definition requires exactly `kind`, `identity` and `node`.

## Test Procedure

Validate the `model-enum-order-status` and
`model-dimension-order-mass` declarations of the vector table below against
the closed schema. Apply the
`model-owner-carries-version` mutation and validate the result.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| MO-01 | `model-enum-order-status`: a `ModelOwner` enum declaration with `identity: acme/orders` | Validates against the closed schema | FR-322-AC-51 |
| MO-02 | `model-dimension-order-mass`: a `ModelOwner` dimension declaration with `identity: acme/orders` | Validates against the closed schema | FR-322-AC-51 |
| MO-03 | `model-owner-carries-version`: MO-01 with a `version` member added to its owner | Refused by the closed schema | FR-322-AC-51 |

## Expected Results

MO-01 and MO-02 find `ModelOwner`-owned declarations admitted. MO-03 finds a `ModelOwner` that carries
a version label refused by the schema.
