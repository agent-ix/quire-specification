# Native simulation sampler definition

Definition identity: `quire.simulation.sampler/v1`; revision: `1-draft.1`.
This is a leaf definition selected by a simulation request
([FR-181](../../../spec/functional/runtime/FR-181-simulate-finite-executions.md));
it has no reverse dependency on the value or model definition roots.

## Selected interpretation

The generator is counter-mode. Draw `k` at simulation step `step` is
`SHA-256(JCS{seed, step, draw: k})`, where `seed` is the run's exact seed
value and `step` and `draw` are nonnegative integers, each starting at zero.
The digest bytes are read as a big-endian unsigned integer and reduced onto
the exact size of the domain being sampled — the canonically ordered enabled
successor set of the exploration contract at that step — by rejection
sampling: a digest whose reduction would introduce bias, because it falls
outside the largest exact multiple of the domain size, is rejected, `draw` is
incremented and the digest is recomputed. There is no modulo bias and no
floating-point intermediate.

Sampling selects one index into the canonically ordered enabled-successor set
that FR-181's exploration contract already defines; it never constructs a
successor the successor relation does not already admit.

## Identity

The recorded generator version is this definition's `DefinitionRef`: its
identity, revision and raw-byte digest, an exact member of the ecosystem
lock's `definitions`. Two runs recording equal seeds and equal
`DefinitionRef`s draw the identical sequence and therefore reproduce
identical traces (FR-181-AC-2). Changing the seed, or this definition's
revision or digest, changes the recorded generator version and the run
identity.
