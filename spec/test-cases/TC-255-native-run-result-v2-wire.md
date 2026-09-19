---
id: TC-255
title: "Mint native-run-result/2 without disturbing /1 or admitting silent substitution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-014
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-032
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-352
    type: verifies
---
## Description

Exercises the `native-run-result/2` wire minted by FR-352 against the FR-014
closed-wire contract `/1` continues to satisfy. Planned.

## Test Procedure

Decode an existing `native-run-result/1` record and confirm every FR-014
refusal (duplicate member, omitted or duplicate requiredFeatures, unknown
version) is unchanged. Produce one result on `/2` explicitly, and attempt to
decode it without an explicit version selection and under the `/1` version
identifier. Produce two otherwise-identical results, one on `/1` and one on
`/2` carrying the added FR-351 witness slot, and compare their digests.
Attempt to read `/1` bytes as `/2` and `/2` bytes as `/1`.

## Expected Results

`/1` decodes and refuses exactly as FR-014 states, with no added or removed
member. `/2` is accepted only from an explicit selection; an unselected or
foreign-version read refuses as unsupported-version. `/2` carries every `/1`
member plus the witness slot, present only with a `decisive-witness` or
`decisive-counterexample` basis. The `/1` and `/2` digests differ. Every
cross-version read attempt refuses rather than substituting one version's
bytes for the other.
