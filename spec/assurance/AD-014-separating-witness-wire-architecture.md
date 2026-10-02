---
id: AD-014
title: "Separating witness record and native-run-result/2 wire architecture"
type: ArchitectureDescription
status: accepted
owner: quire-agent-a
system: separating witness record and native-run-result/2 wire
relationships:
  - target: ix://agent-ix/quire-specification/AD-003
    type: references
  - target: ix://agent-ix/quire-specification/FR-351
    type: references
  - target: ix://agent-ix/quire-specification/FR-352
    type: references
---

# Separating witness record and native-run-result/2 wire architecture

## System Boundary

FR-351 owns one family-neutral separating witness record shared by the
collection, temporal and protocol families' decisive settlements. FR-352 owns
`native-run-result/2`, the explicitly selected closed wire that carries that
record. Neither artifact amends quire-protocol's own canonical result wire
(`quire.protocol.result/v1-draft.1`, FR-240); a change to that wire is separate
and owned by quire-protocol.

## Views

```mermaid
flowchart TD
  D[Decisive settlement forall/exists, temporal, protocol] --> W[FR-351 separating witness record]
  W --> V2[native-run-result/2 explicit selection, FR-352]
  V2 --> S[QSL #186 forall witness channel and /2 serializer]
  V2 --> C[Codegen #50 canonical replay]
  V2 --> R[Verification #38 witness consumer]
  P[quire-protocol result wire, FR-240] -.-> B[Protocol #96 result wire change]
```

`native-run-result/2` is the wire identifier, fixed by the producer at
production time and never inferred from payload content; it carries the
FR-351 record exactly when the settlement basis is `decisive-witness` or
`decisive-counterexample`. A reader reads the bytes only under that
identifier and refuses any other identifier (FR-352-AC-2).

## Decisions

| Choice | Rejected alternative | Consequence |
| --- | --- | --- |
| `native-run-result/2` is an explicit, separately selected wire | Read an optional witness member inferred from payload shape | A reader never infers a wire from content. |
| One family-neutral separating witness record for collection, temporal and protocol | A per-family witness shape for `forall`/`exists`, temporal and protocol results | Every consumer that replays a counterexample (codegen, verification, protocol) replays one record shape. |
| Exhausted, cancelled and unsupported results stay witness-free | Emit a placeholder or best-effort witness for an incomplete or unsupported result | Matches AD-003's incomplete-execution and `unsupported_projection` dispositions; no substitute witness is invented. |

## Risks

| Risk | Control |
| --- | --- |
| A consumer infers `/2` from payload shape instead of an explicit selection | FR-352 requires an explicit production-time selection; an unselected or unrecognized version refuses under FR-014's unsupported-version rule. |
| A partial or foreign-shaped record is accepted as a witness | FR-351's one-axis-substitution refusal, extending FR-094-AC-7 to the witness axis. |
| quire-protocol's own result wire drifts out of step with this witness shape | A change to it is owned separately by [agent-ix/quire-protocol#96](https://github.com/agent-ix/quire-protocol/issues/96), referenced here and not amended by this architecture. |
