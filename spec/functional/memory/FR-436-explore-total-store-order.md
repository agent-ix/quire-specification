---
id: FR-436
title: "Explore a parallel under the x86-TSO memory model"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-434
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-435
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-439
    type: references
---

# FR-436: Explore a parallel under the x86-TSO memory model

## Description

When a `parallel`'s resolved memory model is `tso`, the analyzer SHALL give
each of its threads a FIFO store buffer and SHALL explore loads, stores,
locked accesses, fences, flushes, forks, joins and sends by the rules below,
with flush fairness on every infinite-trace clause.

## Inputs

A protocol subject with a `tso` `parallel`, its access classes
([FR-435](FR-435-classify-memory-accesses-and-orderings.md)) and a protocol
state.

## Outputs

The `tso` memory component of each state, its memory steps, gates and
fairness constraints.

## Behavior

### Component

The memory component is one store buffer per thread of the active weak
`parallel`: a sequence of (location, value) pairs in program order. Memory is
the model state's value of each shared location. The state key's `memory`
member maps `{"type":"store-buffer","thread":<thread path>}` to
`{"type":"sequence","elements":[{"location":<location>,"value":<encoded>}, ...]}`,
oldest first, with an entry for each live thread of the weak `parallel`.

### Rules

| Access | Enabled when | Effect |
| --- | --- | --- |
| store by `b` to `ℓ` of `v` (non-atomic, `relaxed` or `release`) | the attempt's conditions | appends `(ℓ, v)` to `b`'s buffer; memory is unchanged |
| load by `b` of `ℓ` | the attempt's conditions over the value read | reads the newest `(ℓ, v)` in `b`'s buffer, else memory's value of `ℓ` |
| RMW, or store with `seq_cst` (locked access) by `b` | `b`'s buffer is empty | reads and writes memory directly in one step |
| `seq_cst` fence by `b` | `b`'s buffer is empty | none |
| `acquire`, `release` or `acq_rel` fence | always | none |
| memory step `flush(b)` | `b`'s buffer is non-empty | removes the oldest `(ℓ, v)` and writes `v` to `ℓ` in memory |

`flush(b)` has the memory-step identity
`{"type":"memory-step","model":"tso","step":"flush","thread":<thread path>}`
and the anchor `{"type":"memory","step":"flush"}`.

**Gates, split and merge.** A `fork` of a weak `parallel` is enabled when the
entering thread's buffer is empty, and every branch starts with an empty
buffer. The `join` is enabled when every branch has completed and every
branch's buffer is empty; flushes stay enabled until then. A `send` is enabled
when the sender's buffer is empty. After `join all`, memory holds one value
per location and no buffer exists.

**Atom values.** A `holds` atom reads each shared location's memory value;
buffered stores are not visible to atoms. Branch-local locations hold their
thread's value.

**Terminal states.** `flush(b)` is enabled whenever a buffer is non-empty, so
every terminal state has every buffer empty, and a fence, locked access,
`send` or `join` waiting on a buffer is never a deadlock.

### Flush fairness

Every infinite-trace clause over a subject with a `tso` `parallel` carries,
for each thread `b` of it, the constraint weak `each` over `flush(b)`, origin
`memory`, target `{"type":"memory-step","model":"tso","step":"flush","thread":<thread path>}`.
On a lasso, every thread whose buffer is non-empty somewhere in the loop
flushes in the loop. The constraint is machine-closed, so it never makes a
proof vacuous and never changes a safety verdict.

### Instruction correspondence

An ordering corresponds to an x86 instruction by the standard C11-to-x86
mapping: every load is `MOV`; a non-atomic, `relaxed` or `release` store is
`MOV`; a `seq_cst` store is `XCHG`; every RMW is `LOCK`-prefixed; a `seq_cst`
fence is `MFENCE`; other fences emit nothing. The rules above are that
mapping's x86-TSO behaviour.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-436-AC-1 | The store-buffering protocol under `tso` reaches exactly 34 states inside the `parallel` and 4 joined states, and the claim that both loads never read 0 settles `refuted` with the canonical depth-4 counterexample `Sx`, `Sy`, `Lx`, `Ly` in which both stores are still buffered. | Test (TC-390) |
| FR-436-AC-2 | Each `tso` row of the litmus table (TC-391) gives its stated outcome: SB allowed; SB with `seq_cst` fences, SB with `seq_cst` accesses, MP, MP with relaxed flags, LB, IRIW, 2+2W and CoRR forbidden. | Test (TC-391) |
| FR-436-AC-3 | A locked access and a `seq_cst` fence are disabled while their thread's buffer is non-empty; `fork`, `join` and `send` are disabled until the relevant buffers are empty; no terminal state has a non-empty buffer. | Test (TC-390) |
| FR-436-AC-4 | A lasso whose loop keeps a thread's buffer non-empty without flushing it is rejected by flush fairness, and the fairness set lists one `memory` constraint per thread. | Test (TC-390) |
| FR-436-AC-5 | A `holds` atom over a location with a buffered store reads memory's value until the store is flushed. | Test (TC-390) |

## Dependencies

- [FR-434](FR-434-select-memory-models-for-parallel.md) owns the
  contributions these rules fill.
- [FR-435](FR-435-classify-memory-accesses-and-orderings.md) owns the access
  classes.
- [FR-439](FR-439-bound-replay-and-report-weak-memory-runs.md) owns the buffer
  bound, verdicts and replay.

## References

- Design: quire-spec-language ADR-025 TSO-1 to TSO-8, MS-2, MS-4, MF-1 and
  MF-3; Sewell et al., x86-TSO (CACM 2010); Batty et al., POPL 2011.
- Tickets: Linear STD-138; paired QSL-372.
