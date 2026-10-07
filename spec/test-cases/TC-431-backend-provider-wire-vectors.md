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
| BP-01 | Every positive vector, a request-stage vector receiving `positive-all-families.json` as `package` and `package_id` | Each is admitted; each member its `owners` map lists exists and names an FR that has an artifact under `spec/` | FR-331-AC-19 |
| BP-02 | Every adverse vector, one single-operation patch of a positive vector | Each is refused, and its stage schema reports an error at the patched member, under it, or at a non-root ancestor of it | FR-331-AC-19 |
| BP-03 | The `kani-request` vector's `package` | It has no padding, decodes to exactly the fixture's RFC 8785 bytes, re-encodes to the same spelling, and parses to a `quire.checked-package/v2` document; the SHA-256 of the RFC 8785 bytes of its `identity_preimage` is the request's `package_id` digest | FR-331-AC-18 |
| BP-04 | The spellings `eyJhIjoxfQ==`, `eyJhIjoxfQ=`, `+/+/`, `eyJhIjoxfR` and `eyJhI`, and the encodings of every one-byte string, of `ff` followed by every byte, and of every byte followed by `00 ff` | The strict decoder and the schema's `Base64Url` both refuse each spelling and both admit each encoding, which round-trips | FR-331-AC-18 |
| BP-05 | Every artifact and provision entry of the positive vectors | Each decodes; each provision digest is the SHA-256 of its bytes | FR-331-AC-18 |
| BP-06 | The Kani outcome vectors | Their `kani` results stand for exactly the Kani results table's thirteen rows, each `declined` cause separately: `bounded-proof`, `kani_vacuous_proof`, a transcript-backed `refuted`, each `declined` cause, `unsupported` with `tool-unavailable` naming the absent solver, `timed-out`, `limit-reached` with a `kani.` setting, `cancelled`, `replay-parity`, `tested`, `tool-failure`, `failed` with `tool-unavailable`, and `failed` with `runtime_invariant`; the adverse vectors that move a Kani cause outside its result's closed set are refused | FR-331-AC-20 |
| BP-07 | `request-item-level-defects`, then the same document with item 0's `candidates` removed | The first is admitted, since FR-290 settles an absent kind, an unknown backend and an unknown evidence kind per item; the second is refused | FR-331-AC-4 |
| BP-08 | The `kani-request` vector's bound and the `temporal-request` subject's proof bound, then the adverse vectors `proof-bound-without-kind`, `proof-bound-unknown-kind`, `proof-bound-kind-bound-mismatch`, `proof-bound-unboundable-kind` and `subject-proof-bound-without-kind` | Each positive `ProofBound` carries a `kind` that the unbounded `domains[].kind` vocabulary admits, and each `extent.bounds` empty list has no kind; each adverse vector is refused at the patched `ProofBound` | FR-331-AC-21 |
| BP-09 | Every positive manifest descriptor; the `kani-hello` vector with `domains` removed (`bounded-descriptor-without-domains`); and a descriptor advertising only `unbounded` modes with no `domains` | Each positive descriptor that advertises a `bounded` mode carries a nonempty `domains`; the adverse vector is refused at its descriptor, the FR-290 `invalid_capability`/`absent-domains` registration refusal; the unbounded-only descriptor is admitted | FR-331-AC-22 |

## Expected Results

Every published vector is admitted or refused as stated, and the package
member carries exactly the checked package its `package_id` names.
