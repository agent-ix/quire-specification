---
id: TC-003
title: "Preserve selected JCS bytes and domain separation"
type: TC
relationships:
  - target: "ix://agent-ix/quire-specification/FR-018"
    type: verifies
---
# TC-003: Preserve selected JCS bytes and domain separation

## Description

Accepted portable golden and contract controls for FR-018-AC-1–4 and
FR-018-AC-6 with independently authored preimage/output expectations. This artifact does not claim executed portfolio coverage.

## Test Procedure

Validate the new semantic reference and call B's Rust jcs_canonicalize
and jcs_sha256 functions on the exact vector value. Compare canonical bytes and
prefixed digest to the recorded expected values. Use reordered object members,
a supplementary-plane key beside a high BMP key, numeric -0/1e-7 inputs and
ordered arrays. Mutate canonical domain, version, algorithm and digest one at
a time, including sha256 and bare-hex substitutions. Exercise requiredFeatures
as empty, singleton, two reordered unique features, duplicate decoded features
and an unknown feature. Validate both unchanged IR domain forms and reject JCS
prefix substitutions in them. Resource controls use IT-003's byte and value
limits, including exact limit and one-over-limit; missing dependencies fail setup.

## Expected Results

Canonical bytes and digest match B's functions. Object member reordering does not
change them; changing array order does. Incorrect selections/syntax/digests refuse
at their declared layer. Feature-set equality is independent of canonical array
ordering. Prior IR forms retain bare hexadecimal digests and their original
preimage definitions; shape checking is not a requalification of those algorithms.
Resource exhaustion yields incomplete without a digest result.
