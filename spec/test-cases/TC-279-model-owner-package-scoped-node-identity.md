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

Show that a `ModelOwner` node-identity preimage carries the declaring domain
package's `identity` beside `node` and no version label, that the published
`ModelOwner` vectors validate against the closed preimage schema and
recompute their recorded JCS SHA-256, and that a `ModelOwner` node's id
depends only on content.

## Preconditions

- FR-321 admits at most one domain package per `identity`.
- The `proposals/checked-package-v2/node-identity-preimage.schema.json`
  `ModelOwner` definition requires exactly `kind`, `identity` and `node`.

## Test Procedure

Validate the published `model-enum-order-status` and
`model-dimension-order-mass` vectors in `node-identity-vectors.json` against
the closed preimage schema and recompute each vector's JCS SHA-256. Apply the
`model-owner-carries-version` mutation and validate the result.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| MO-01 | `model-enum-order-status`: a `ModelOwner` enum declaration with `identity: acme/orders` | Validates against the closed schema; recomputed SHA-256 equals the published digest | FR-322-AC-28 |
| MO-02 | `model-dimension-order-mass`: a `ModelOwner` dimension declaration with `identity: acme/orders` | Validates against the closed schema; recomputed SHA-256 equals the published digest | FR-322-AC-28 |
| MO-03 | `model-owner-carries-version`: MO-01 with a `version` member added to its owner | Refused by the closed preimage schema before digest computation | FR-322-AC-28 |

## Expected Results

MO-01 and MO-02 find `ModelOwner`-owned declarations admitted and correctly
digested from content alone. MO-03 finds a `ModelOwner` preimage that carries
a version label refused by the schema.
