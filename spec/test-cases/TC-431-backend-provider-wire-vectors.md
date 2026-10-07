---
id: TC-431
title: "The backend provider wire schema admits its positive vectors and refuses its adverse ones"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
---

# TC-431: The backend provider wire schema admits its positive vectors and refuses its adverse ones

## Description

Validates `proposals/backend-provider-v1/vectors.json` against
`proposals/backend-provider-v1/schema.json`, with the checked-package schema
registered for the members it references, and checks the request's package
bytes against the checked package they carry. Runs as
`tests/backend_provider_v1.rs`.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| BP-01 | Every positive vector, a request-stage vector receiving `positive-all-families.json` as `package` and `package_id` | Each is admitted; each member its `owners` map lists exists and names an FR | FR-331-AC-19 |
| BP-02 | Every adverse vector applied to its base | Each is refused | FR-331-AC-19 |
| BP-03 | The `kani-request` vector's `package` | It has no padding, decodes to exactly the fixture's RFC 8785 bytes, re-encodes to the same spelling, and parses to a `quire.checked-package/v2` document whose `package_id` equals the request's | FR-331-AC-18 |
| BP-04 | The spellings `eyJhIjoxfQ==`, `+/+/`, `eyJhIjoxfR` and `eyJhI`, and the adverse vectors carrying them | The strict decoder and the schema both refuse each; other byte strings round-trip | FR-331-AC-18 |
| BP-05 | Every artifact and provision entry of the positive vectors | Each decodes; each provision digest is the SHA-256 of its bytes | FR-331-AC-18 |
| BP-06 | The Kani outcome vectors | Their results are exactly the eight Kani results, with `bounded-proof`, `kani_vacuous_proof`, `limit-reached`, `timed-out`, `cancelled`, `tool-failure` and each `declined` cause; the adverse vectors that move a Kani cause outside its result's closed set are refused | FR-331-AC-20 |
| BP-07 | `request-item-level-defects`, then the same document with item 0's `candidates` removed | The first is admitted, since FR-290 settles an absent kind, an unknown backend and an unknown evidence kind per item; the second is refused | FR-331-AC-4 |

## Expected Results

Every published vector is admitted or refused as stated, and the package
member carries exactly the checked package its `package_id` names.
