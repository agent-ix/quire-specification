---
id: FR-352
title: "Mint the native-run-result/2 witness wire"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-014
    type: references
  - target: ix://agent-ix/quire-specification/FR-351
    type: references
---
# FR-352: Mint the native-run-result/2 witness wire

## Description

When a native run result is produced or read, the strict reader SHALL treat `native-run-result/1` as the closed wire FR-014 already governs, unchanged. `native-run-result/2` is accepted only as an explicitly selected, distinct closed wire that adds the FR-351 separating-witness record.

## Inputs

An explicit wire identifier selection; a native run result's existing FR-014 members; the FR-351 separating-witness record where the settlement basis is `decisive-witness` or `decisive-counterexample`.

## Outputs

A `native-run-result/1` record under FR-014's unchanged contract, a `native-run-result/2` record carrying every FR-014 member plus the FR-351 witness slot, or a located wire diagnostic.

## Behavior

`native-run-result/1` and `native-run-result/2` are two distinct registered wire identifiers in FR-014's registry, each an opaque string compared exactly. `native-run-result/1` remains byte-for-byte the wire FR-014 defines; this requirement changes none of its members or refusal rules. `native-run-result/2` is a new, explicit wire identifier that a producer selects at production time and fixes in the produced artifact; a reader never infers the identifier from message content, and a document whose identifier is not the selected one refuses before consumption (FR-014 `unsupported-version`). `/2` adds exactly one closed member to the `/1` grammar — the FR-351 separating-witness record, present exactly when the settlement basis is `decisive-witness` or `decisive-counterexample` and absent otherwise (FR-351) — and changes no existing `/1` member's meaning. Two otherwise-identical results, one produced on `/1` and one on `/2`, carry distinct digests; each wire's bytes are read only under its own identifier, and a read under any other identifier refuses before consumption (FR-014 `unsupported-version`). A consumer that requires the witness record selects and regenerates against `/2`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-352-AC-1 | `native-run-result/1` decodes under FR-014's unchanged member set and refusal rules; this requirement adds no member and removes none. | Test (TC-255) |
| FR-352-AC-2 | `native-run-result/2` is accepted only from an explicit producer selection fixed at production, never inferred from payload content; a document whose identifier is not the selected one refuses before consumption (FR-014 `unsupported-version`). | Test (TC-255) |
| FR-352-AC-3 | `native-run-result/2` carries every `/1` member unchanged plus the FR-351 separating-witness record, present exactly with a `decisive-witness` or `decisive-counterexample` settlement basis and absent otherwise. | Test (TC-255) |
| FR-352-AC-4 | Two otherwise-identical results, one on `/1` and one on `/2`, produce distinct digests. | Test (TC-255) |
| FR-352-AC-5 | A `/1` record reads only under the `/1` identifier and a `/2` record only under the `/2` identifier; a read under the other identifier refuses before consumption (FR-014 `unsupported-version`), and a consumer requiring the witness record regenerates against `/2`. | Test (TC-255) |

## Dependencies

- [FR-014](FR-014-read-exact-wire.md) supplies the closed-wire contract `/1`
  continues to satisfy unchanged, and the identifier refusal `/2` reuses.
- [FR-351](../../objects/protocol/FR-351-separating-witness-record.md) supplies
  the record `/2` carries.
- [US-011](../../usecase/US-011-analyze-complete-models.md) supplies the
  replay/comparability need this wire serves.

## Cross-repo note

quire-protocol's own canonical result wire (`quire.protocol.result/v1-draft.1`,
FR-240) is a distinct contract and is not amended by this requirement. Its
corresponding draft-revision bump for carrying the FR-351 witness shape is
owned by [agent-ix/quire-protocol#96](https://github.com/agent-ix/quire-protocol/issues/96).

## Status

Draft. Mints `native-run-result/2` normatively over the FR-014 closed-wire
contract. quire-spec-language's wire-catalog implementation and replay
evidence remain planned under
[agent-ix/quire-spec-language#186](https://github.com/agent-ix/quire-spec-language/issues/186),
[agent-ix/quire-contract-codegen#50](https://github.com/agent-ix/quire-contract-codegen/issues/50)
and [agent-ix/quire-verification#38](https://github.com/agent-ix/quire-verification/issues/38).
