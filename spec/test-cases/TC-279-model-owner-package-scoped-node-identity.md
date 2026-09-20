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
package's `identity` and `version` beside `node`, that the published
`ModelOwner` vectors validate against the closed preimage schema and
recompute their recorded JCS SHA-256, and that a `ModelOwner` node's id is
scoped to its declaring domain package's `identity@version`, per QSL
ADR-013 O-04, QC-3 and QC-18.

## Preconditions

- FR-321 admits at most one version of a domain-package `identity`.
- The `proposals/checked-package-v2/node-identity-preimage.schema.json`
  `ModelOwner` definition requires `kind`, `identity`, `version` and `node`.

## Test Procedure

Validate the published `model-enum-order-status`,
`model-enum-order-status-version-2` and `model-dimension-order-mass` vectors
in `node-identity-vectors.json` against the closed preimage schema, recompute
each vector's JCS SHA-256 and compare digests across the version-only
mutation.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| MO-01 | `model-enum-order-status`: a `ModelOwner` enum declaration with `identity: acme/orders`, `version: 1.0.0` | Validates against the closed schema; recomputed SHA-256 equals the published digest | FR-322-AC-28 |
| MO-02 | `model-dimension-order-mass`: a `ModelOwner` dimension declaration with `identity: acme/orders`, `version: 1.0.0` | Validates against the closed schema; recomputed SHA-256 equals the published digest | FR-322-AC-28 |
| MO-03 | `model-enum-order-status-version-2`: the same owner `identity` and `node` as MO-01 with `version: 2.0.0` | Validates against the closed schema; computes a node id distinct from MO-01's | FR-322-AC-28 |
| MO-04 | A `ModelOwner` preimage with `identity` and `node` present and `version` omitted | Refused by the closed preimage schema before digest computation | FR-322-AC-28 |

## Expected Results

MO-01 and MO-02 find general `ModelOwner`-owned declarations admitted and
correctly digested. MO-03 finds that a version-only change to the declaring
domain package yields a different node id, so a `NodeKey` stays unique
across domain-package versions. MO-04 finds a `ModelOwner` preimage missing
`version` refused by the schema.
