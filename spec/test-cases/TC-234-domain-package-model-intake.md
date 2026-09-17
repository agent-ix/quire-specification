---
id: TC-234
title: "Domain package model intake"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-208
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

- **B1** (artifact bundle): the `entity` artifact with id `A` and title
  `Alpha`, with a Properties row for field `x` typed `A`, presence `optional`,
  multiplicity `0..1`; and the `entity` artifact with id `B` and title `Beta`,
  with `specializes: A`.
- **K1**: the domain package that quire-rs extraction and the FCD
  extraction-frontend lift produce from B1, with identity `test/orders` and
  version `1`: TC-195 F1's three IR nodes, each with its artifact id and span.
  Its `constructs` table has one entry, for kind
  `{module: test/objects, name: entity}`, whose `meaning` is
  `quire.meaning.model.object-type/v1`; types `A` and `B` name that kind and
  carry `displayName` `Alpha` and `Beta`.
- **S1**: `model M = "test/orders" version "1" digest "<SHA-256 over K1's JCS bytes>";`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| I01 | Extract and lift B1, then admit K1 under S1 | K1's digest equals S1's digest; three original declarations with keys `(test/orders, ix://test/orders/A)`, `(test/orders, ix://test/orders/A/x)` and `(test/orders, ix://test/orders/B)`, each with `digest_domain: sha256-jcs` and its artifact and span; three `normalize.record` charges in that order |
| I02 | S1 with digest domain `quire-native-bytes-1`; S1 with an empty package input, and S1 with its digest's last hex digit changed while K1 is supplied under its own digest; K1 with one byte changed, supplied under S1's digest; K1 relabelled version `2`, supplied under the SHA-256 of the relabelled bytes, with S1 naming that digest and version `1` | `refused { code: stale_dependency, cause: digest-domain-mismatch }`; `refused { code: missing_import, cause: missing-selection }` for both lookups; `refused { code: stale_dependency, cause: byte-digest-mismatch }` with the expected and actual digests; `refused { code: invalid_model_binding, cause: wrong-model-selection }` with both versions; no run charges `normalize.record` or builds a declaration |
| I03 | K1 with `A/x.multiplicity.lower` set to `"zero"` and `B.supertypes` set to `[ix://test/orders/Z]` | after three `normalize.record` charges, two refusals in node order: `refused { code: invalid_model_binding, cause: malformed-declaration }` naming `ix://test/orders/A/x`, member path `multiplicity.lower`, artifact `A` and the row's span; then `refused { code: missing_declaration, cause: missing-name }` naming `ix://test/orders/B`, absent node `ix://test/orders/Z`, artifact `B` and the frontmatter span; no effective view |
| I04 | B1 with artifact `A` retitled `Alpha Prime`, lifted and admitted under a re-digested S1, with one clause naming `M::A::x`; then B1 with artifact `A`'s id changed to `Ay`, its `x` row typed `Ay` and `B` specializing `Ay`, lifted and admitted under a re-digested S1, with one clause naming `M::A` and another naming `M::A::x` | the retitled run yields exactly I01's three declaration keys, type `A` has `displayName` `Alpha Prime`, and the clause resolves to `(test/orders, ix://test/orders/A/x)`; in the id-changed run, key `(test/orders, ix://test/orders/B)` equals I01's, the type from artifact `Ay` has the new key `(test/orders, ix://test/orders/Ay)` and its field the new key `(test/orders, ix://test/orders/Ay/x)`, `B` has supertype `Ay`, no declaration has key `(test/orders, ix://test/orders/A)` or `(test/orders, ix://test/orders/A/x)`, and each clause is `refused { code: missing_declaration, cause: missing-name }`, naming absent node `ix://test/orders/A` and `ix://test/orders/A/x` respectively |
| I05 | I01 under `ModelNormalizationLimitsV1` with `declaration_records: 3`, then `2`, every other counter unlimited | admitted with three declarations; `incomplete { limit_kind: declaration_records, limit: 2, consumed: 2, next_charge: 3, charge_point: normalize.record }` with no declaration exposed |
| I06 | Validate every preimage in the published effective-declaration vectors against the published schema, and compare each declaration key's `package` with the `model_selection` identity of the view that contains it | every preimage is schema-valid; every `original` and every fact input names package `test/orders`, equal to the view's `model_selection.identity`, and carries no digest |
| I07 | K1 published as version `2` with equal nodes and admitted under its own selection | every declaration key equals I01's and the ModelSelection digest differs, so the effective declarations are byte-equal and the effective view and object universe differ, as the vectors `n01-view`/`n01v2-view` and `n01-universe`/`n01v2-universe` show |
| I08 | K1 with type `B`'s kind changed to `{module: test/objects, name: account}`, which names no `constructs` entry; then K1 with the `entity` construct's `meaning` removed; then K1 with that `meaning` set to `quire.meaning.model.object-type/v2`; then K1 plus population `PA` from artifact `PA`, with member types `[A]`, extent `closed` and kind `{module: test/objects, name: population}`, whose `constructs` entry has no `meaning` | the first run is `refused { code: invalid_model_binding, cause: malformed-declaration }` naming `ix://test/orders/B`, member path `kind`, artifact `B` and its span; the second and third runs each refuse `invalid_model_binding`/`malformed-declaration` twice, in node order, naming `ix://test/orders/A` then `ix://test/orders/B`, member path `constructs.meaning`, and each artifact and span; the fourth run refuses `invalid_model_binding`/`malformed-declaration` naming `ix://test/orders/PA`, member path `constructs.meaning`, artifact `PA` and its span; no run binds a meaning from the kind name `entity`, and no run exposes an effective view |

## Expected Results

Every positive and boundary result matches FR-154; each refusal names its artifact and span where FR-154 requires, emits no declaration, and preserves independent sibling results.
