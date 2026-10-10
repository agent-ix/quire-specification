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
| BP-08 | Each of the seven domain kinds paired with each of the three bound types in a `ProofBound`; then a `ProofBound` without `kind` and one with an unknown kind | Exactly four of the 21 pairs are admitted: `collection` and `population` with `cardinality`, `integer` with `integer-range`, `recursive` with `depth`; every other pair, the missing kind and the unknown kind are refused | FR-331-AC-21 |
| BP-09 | Every positive manifest descriptor; the `kani-hello` descriptor without `domains`, with `domains` [], [`integer`, `quantity`] and [`integer`, `integer`]; a descriptor advertising only `unbounded` modes with no `domains`; and that unbounded-only descriptor with `domains` [], [`quantity`], [`size`] and [`integer`, `integer`] | Each positive descriptor that advertises a `bounded` mode carries a nonempty `domains`; each defective `domains`, on the bounded and the unbounded-only descriptor alike, is refused at its descriptor, the FR-290 `invalid_capability`/`invalid-domains` registration refusal; the unbounded-only descriptor with no `domains` is admitted | FR-331-AC-22 |
| BP-10 | The `proved` entries of the positive outcome vectors, `certification-outcome` among them; then `certification-missing`, `certification-other-value`, `certification-on-refuted`, `certification-reduced-certified`, `certification-bounded-solver-certified`, `certification-symmetry-certified`, `certificate-rejected-without-rule` and `cancelled-source-other-value` | The Kani `bounded-proof`, the `witnessed`, the `value-bounds`, the copy-swap `reduced` and the verified SMT `bounded-complete` proofs carry `certified`, the `reduced`, the `bounded-solver` and the unverified SMT `inductive` proofs `uncertified` and the plugin proof `trusted`, and the `certificate-rejected{rule, at}` cause, each admitted; a `proved` entry without `certification`, one with a value outside the three, a `refuted` entry carrying `certification`, a symmetry- or partial-order-`reduced` or `bounded-solver` proof carrying `certified`, a `certificate-rejected` cause without `rule`, and a `cancelled` source other than `requested` or `deadline` are each refused at the patched member | FR-331-AC-23; FR-331-AC-19 for `cancelled-source-other-value` |
| BP-11 | `certificate-rejected-loci`; then its adverse vectors `certificate-locus-unknown-query-part`, `certificate-locus-empty-string`, `certificate-locus-empty-object`, `proof-step-unknown-part`, `proof-step-without-part`, `proof-step-without-index`, `proof-step-extra-member`, `proof-step-index-number`, `proof-step-index-negative`, `proof-step-index-overflow`, `proof-step-index-leading-zero`, `product-state-automaton-state-padded`, `hyper-product-state-without-states`, `state-locus-not-a-state-key`, `simulation-step-verdict-other-cause`, `zone-node-negative` and `certificate-member-unknown`; then the schema's `U64String` over `""`, `00`, `01`, `-0`, `-1`, `+1`, ` 1`, `1 `, `1.0`, `1e3`, `1` followed by zeros and all nines at each length from 1 to 21 digits, and each 20-digit string that keeps a prefix of `18446744073709551615`, changes the next digit to each of `0` to `9`, and fills the rest with `0` or with `9` | Every locus form of the vector is admitted: the query loci `unrolling`, `base` and `step`, the proof steps with index `0`, `41` and `18446744073709551615`, two product states, three hyper product states (one an HP-1 member with no automaton key, one with a witness set), four states, the three simulation-step forms, two zone nodes, a zone edge and a zone component, and a certificate member; each adverse vector is refused at the patched member; `U64String` admits a string exactly when Rust's `u64` parse accepts it and prints it back unchanged | FR-331-AC-24 |
| BP-12 | `certificate-rejected-loci`, whose rules include `not-refutation` and `malformed` at a query part; then its adverse vectors `certificate-rule-unknown`, `certificate-rule-variant-name`, `query-rule-at-proof-step`, `proof-step-rule-at-query`, `product-state-rule-at-hyper-product-state`, `product-state-without-automaton-state`, `state-rule-at-zone-node`, `zone-component-other-rule`, `certificate-member-other-rule`, `simulation-step-initial-with-edge`, `simulation-step-post-differs-without-edge` and `simulation-step-fails-without-verdict` | Each rule at a locus listed with it is admitted; each adverse vector, an unknown or misspelled rule or a rule at a locus not listed with it, is refused at the patched member | FR-331-AC-25 |

## Expected Results

After wire bindings are settled, BP-13 exercises FR-331-AC-26 through the
actual reader with originating request and checked claim context: an
`inequality-fails` cause bound to an EN-1 temporal claim, a model-only
`successor-missing` bound to FR-338, and absent/conflicting/ambiguous
context all refuse before consumption. Each valid owning context preserves
the rejection. A global shape-schema run alone cannot discharge BP-13.
BP-14 exercises FR-331-AC-27 through serialization and the actual reader,
including HP-3 `n = 0`, missing required tuples/witness sets, labelled HP-1
edge tuples with shared pre-states and distinct post-states, and repeated
model states with distinct probabilistic product components.

BP-12 also includes `certificate-rejected-rule-completion`: positives for
`bad-state`, `undefined-member`, `predicate-undefined`, `rank-missing`,
`order-broken`, `count-broken` and `successors-differ`. The independent
finite oracle `tc_431_certificate_rules_have_positive_coverage` enumerates
all 24 owned rule spellings in Rust, then compares both the schema rule
set and the positive-vector rule set against it. Neither expected set is
derived from the schema or vectors. A deletion from both inputs must fail.
The claim is each rule at a listed locus, not every rule/locus pair or
semantic checker acceptance. Synthetic state/automaton bytes qualify
transport shape only.

Every published vector is admitted or refused as stated, and the package
member carries exactly the checked package its `package_id` names.
