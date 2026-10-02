# Native simulation sampler definition

Definition identity: `quire.simulation.sampler/v1`; revision: `1-draft.1`.
This is a leaf definition selected by a simulation request
([FR-181](../../../spec/functional/runtime/FR-181-simulate-finite-executions.md));
it has no reverse dependency on the value or model definition roots.

## Selected interpretation

The generator is counter-mode. `seed` is the run's `u64` seed, written as a
decimal string. `trace` is the trace's 0-based index within the run, and
`step` is the transition's 0-based index within that trace, each a decimal
string of a `u64` value. At a given `trace`/`step`, `draw` is a decimal
string of a `u64` value that restarts at 0 and increments after every digest
computed at that step, accepted or rejected.

Draw `draw` at `trace`/`step` is `SHA-256(JCS{seed, trace, step, draw})`. Let
`v` be the digest bytes read as a big-endian `u256`, and `n` the exact size
of the domain being sampled — the canonically ordered enabled successor set
of the exploration contract at that step. `v` is accepted when
`v < n * floor(2^256 / n)`, selecting index `v mod n`; a rejected `v`
increments `draw` and recomputes the digest. `n = 1` selects index 0 with no
digest. `n = 0` ends the trace without a draw.

Sampling selects one index into the canonically ordered enabled-successor set
that FR-181's exploration contract already defines.

## Identity

The recorded generator is this definition's identity. Two runs recording
equal seeds and the same sampler identity draw the identical sequence and
therefore reproduce identical traces (FR-181-AC-2). Changing the seed changes
the run identity.
