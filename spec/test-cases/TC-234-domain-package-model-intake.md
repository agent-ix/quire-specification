---
id: TC-234
title: "Domain package model intake"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
---

# TC-234: Domain package model intake

## Description

Admit a domain package lifted from an artifact bundle, and mutate its digest, identity, nodes and references.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-154. Execute its positive case, each declared boundary, and each explicit refusal mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select `quire.model.complete/v1` at revision `1-draft.1` by its exact
DefinitionRefs in
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json).
Node names and declaration keys follow TC-195.

Fixtures:

- **B1** (artifact bundle): `entity-001` titled `A` with a Properties row for
  field `x` typed `A`, presence `optional`, multiplicity `0..1`; and
  `entity-002` titled `B` with `specializes: entity-001`.
- **K1**: the domain package that quire-rs extraction and the FCD
  extraction-frontend lift produce from B1, with identity `test/orders` and
  version `1`: TC-195 F1's three IR nodes, each with its artifact id and span.
- **S1**: `model M = "test/orders" version "1" digest "<SHA-256 over K1's JCS bytes>";`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| I01 | Extract and lift B1, then admit K1 under S1 | K1's digest equals S1's digest; three original declarations with keys `(test/orders, ix://test/orders/A)`, `(test/orders, ix://test/orders/A/x)` and `(test/orders, ix://test/orders/B)`, each with `digest_domain: sha256-jcs` and its artifact and span; three `normalize.record` charges in that order |
| I02 | S1 with digest domain `quire-native-bytes-1`; S1 with an empty package input, and S1 with its digest's last hex digit changed while K1 is supplied under its own digest; K1 with one byte changed, supplied under S1's digest; K1 relabelled version `2`, supplied under the SHA-256 of the relabelled bytes, with S1 naming that digest and version `1` | `refused { code: stale_dependency, cause: digest-domain-mismatch }`; `refused { code: missing_import, cause: missing-selection }` for both lookups; `refused { code: stale_dependency, cause: byte-digest-mismatch }` with the expected and actual digests; `refused { code: invalid_model_binding, cause: wrong-model-selection }` with both versions; no run charges `normalize.record` or builds a declaration |
| I03 | K1 with `A/x.multiplicity.lower` set to `"zero"` and `B.supertypes` set to `[ix://test/orders/Z]` | after three `normalize.record` charges, two refusals in node order: `refused { code: invalid_model_binding, cause: malformed-declaration }` naming `ix://test/orders/A/x`, member path `multiplicity.lower`, artifact `entity-001` and the row's span; then `refused { code: missing_declaration, cause: missing-name }` naming `ix://test/orders/B`, absent node `ix://test/orders/Z`, artifact `entity-002` and the frontmatter span; no effective view |
| I04 | B1 with `entity-002` retitled `Bee`, lifted and admitted under a re-digested S1; then the same package with a clause naming `M::B` | keys `(test/orders, ix://test/orders/A)` and `(test/orders, ix://test/orders/A/x)` equal I01's; the type from `entity-002` has the new key `(test/orders, ix://test/orders/Bee)`, keeps artifact `entity-002` as its origin, and still has supertype `A`; no declaration has key `(test/orders, ix://test/orders/B)`; the clause is `refused { code: missing_declaration, cause: missing-name }` naming absent node `ix://test/orders/B` |
| I05 | I01 under `ModelNormalizationLimitsV1` with `declaration_records: 3`, then `2`, every other counter unlimited | admitted with three declarations; `incomplete { limit_kind: declaration_records, limit: 2, consumed: 2, next_charge: 3, charge_point: normalize.record }` with no declaration exposed |
| I06 | Validate every preimage in the published effective-declaration vectors against the published schema, and compare each declaration key's `package` with the `model_selection` identity of the view that contains it | every preimage is schema-valid; every `original` and every fact input names package `test/orders`, equal to the view's `model_selection.identity`, and carries no digest |
| I07 | K1 published as version `2` with equal nodes and admitted under its own selection | every declaration key equals I01's and the ModelSelection digest differs, so the effective declarations are byte-equal and the effective view and object universe differ, as the vectors `n01-view`/`n01v2-view` and `n01-universe`/`n01v2-universe` show |

## Expected Results

Every positive and boundary result matches FR-154; each refusal names its artifact and span where FR-154 requires, emits no declaration, and preserves independent sibling results.
