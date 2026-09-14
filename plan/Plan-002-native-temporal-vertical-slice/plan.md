---
id: Plan-002
title: "Native temporal vertical slice"
type: Plan
status: active
relationships:
  - { target: ix://agent-ix/quire-specification/FR-090, type: references }
  - { target: ix://agent-ix/quire-specification/FR-091, type: references }
  - { target: ix://agent-ix/quire-specification/FR-092, type: references }
  - { target: ix://agent-ix/quire-specification/FR-093, type: references }
  - { target: ix://agent-ix/quire-specification/FR-094, type: references }
  - { target: ix://agent-ix/quire-specification/FR-095, type: references }
  - { target: ix://agent-ix/quire-specification/NFR-040, type: references }
---
# Implementation Plan: Native temporal vertical slice

## Requirements Summary

### Functional Requirements

- [ ] **FR-090**: select exact temporal profile and clock.
- [ ] **FR-091**: evaluate bounded future operators pointwise and fail closed.
- [ ] **FR-092**: evaluate bounded past only with authoritative history.
- [ ] **FR-093**: preserve trigger activation and immutable captures.
- [ ] **FR-094**: bind progress, closure and result preservation.
- [ ] **FR-095**: lower only exact native-to-TL correspondences.

### Non-Functional Requirements

- [ ] **NFR-040**: bound arithmetic, work, instances and retained state without Boolean fallback.

## Dependency Graph

- `FR-090 -> FR-091, FR-092, FR-093, FR-094, FR-095`: the selected
  digest-addressed profile and tagged clock establish the meaning of every
  later temporal input and result.
- `FR-093 + FR-094 -> FR-091, FR-092`: a monitor needs authenticated
  activation/captures and history/progress before it can settle a truth result.
- `FR-091 + FR-092 + FR-025 -> FR-095`: lowering requires native operator
  semantics and only total Boolean predicate propositions.
- `FR-095 + FR-094 + NFR-040 -> cross-repository integration`: an evaluator
  result is admissible only with its correspondence, progress and resource facts.

### Shared dependencies

- **`quire.compiled-protocol/2` selection record** is the single compiler-to-
  runtime admission seam. It must authenticate the temporal definition digest
  and tagged clock configuration while keeping `/1` readable.
- **Observation/progress assertion** is the single runtime seam: it supplies
  event/correlation identity, causal order, completeness and monotonic progress.
- **Closed native result envelope** is shared by activation, truth, completeness,
  resource disposition and correspondence. These axes must not become a nullable
  Boolean or a message-parsed error.

### Cross-cutting constraints

- `NFR-040` applies to compiler bounds, observer retention, Contract-IR
  construction and evaluator continuation. Use checked arithmetic, explicit
  ceilings and deterministic reservation before expansion.
- Rust boundaries use versioned wire structs, closed enums, identity newtypes,
  checked integer/unit conversions and no ambient callback/network/evaluator
  action during construction.

## Test Plan

### Admission and compiler tests

- [ ] **TC-110**: exact profile, digest and clock selection; `/2` missing or
  mismatched temporal fields refuse while `/1` remains strictly readable.
- [ ] **TC-113**: event-position, fixed-sample and timestamped-event clock
  identities remain distinct; no implicit conversion or ingestion-order tie break.

### Evaluator and monitor tests

- [ ] **TC-111**: bounded-future truth tables, including pointwise connectors.
- [ ] **TC-112**: bounded-past origin versus cutoff and closure discriminators.
- [ ] **TC-114**: repeated triggers, captures, false/unestablished guards and
  conflicting duplicate receipt behavior.
- [ ] **TC-115**: inclusive progress and silent-deadline settlement.
- [ ] **TC-116**: open, closed-complete and closed-incomplete behavior plus
  independently settled sibling preservation.
- [ ] **TC-118**: overflow, exhaustion, retention loss and Boolean-fallback mutation.

### Bridge and integration tests

- [ ] **TC-117**: independent native/TL differential corpus, correspondence
  identity mutation and unsupported finite-window/timestamped/past mappings.
- [ ] **TC-038 / TC-039**: Contract-IR total-Boolean projection and exact
  native-temporal correspondence with real strict TL readers.

## Remaining Work

### Delivered foundations

- **Task-001 is done.** The cross-repository architecture and correspondence
  ownership were accepted by specification PR #15 (`4d6230e`) and its composed
  base-plus-seven review. No replacement architecture review is required.
- **Task-002 is done for its compiler-owned boundary.** `quire-spec-language`
  PRs #75, #77 and #82 deliver strict compiled-protocol `/2` temporal selection,
  authenticated evaluation/mapping inputs and the corrected closure selector.
  The independent Contract-IR/TL bridge remains Task-004/Task-006 work.
- **Task-003 is done for its declared authority seam.** `quire-observation`
  Plan-001 and PR #19 (`9ac80e9`) deliver typed observation, completeness,
  progress, replay and resource contracts. Producer-qualified subject/topology
  adoption remains separately tracked by observation #18/#13.
- **Task-004 is done.** `quire-contract-ir` PRs #67, #68 and #77 (current
  accepted implementation `202210c`) deliver strict total-Boolean projection
  and the selected temporal correspondence contract. Runtime composition is
  Task-006 / qcir #71.

### Track A: Critical Path (serial)

- **A3 = Task-005** Implement native temporal runtime result model and evaluator — Hard;
  exit: bounded future/past and activation semantics emit closed, non-Boolean
  failure states as specified.
- **Gate = Task-007** End-to-end conformance gate — measures semantic equivalence
  and preservation; pass: every declared differential vector agrees or is
  explicitly unsupported, with zero Boolean fallbacks under injected exhaustion.

### Track C: Post-foundation bridge

- **C1 = Task-006** Compose exact native-to-TL correspondence — Hard; exit:
  supported mappings preserve every identity axis; unmatched profiles produce no
  substitute formula.

## Parallel Execution Summary

```text
delivered architecture + compiler /2 ──────────────────┐
delivered observation/progress + predicate projection ─┼─> A3 runtime ─> C1 bridge ─> Gate
delivered TL owner APIs ────────────────────────────────┘
```

## Task File Mapping

| Task | Track | Owns | Verified by | Status |
| --- | --- | --- | --- | --- |
| Task-001 | A | FR-090, FR-095 | TC-110, TC-113, TC-117 | done — PR #15 / `4d6230e` |
| Task-002 | A | FR-090 | TC-110, TC-113 | done — SL PRs #75/#77/#82 |
| Task-003 | B | FR-093, FR-094 | TC-114, TC-115, TC-116 | done — observation Plan-001 / PR #19 / `9ac80e9` |
| Task-004 | B | FR-095 | TC-038, TC-117 | done — qcir PRs #67/#68/#77 / `202210c` |
| Task-005 | A | FR-091, FR-092, FR-093, FR-094, NFR-040 | TC-111, TC-112, TC-114, TC-115, TC-116, TC-118 | in progress — TL owner APIs merged; native composition remains qcir #71 |
| Task-006 | C | FR-095 | TC-039, TC-113, TC-117 | in progress — qcir #71; required owner APIs are merged |
| Task-007 | A | FR-090 through FR-095, NFR-040 | TC-110 through TC-118 | not_started |

## Coordination Rules

- Agent A is the sole writer for the native compiler AST and `/2` wire; do not
  repurpose `/1` or add an implicit clock default.
- Agent F is the sole writer for observation authority and replay/retention
  transport. The temporal runtime consumes its identified assertions.
- Contract IR owns FR-025 and FR-026; no compiler/runtime branch duplicates its
  formula, proposition-map or correspondence reader.
- The remaining merge order is Task-005/Task-006 under qcir #71, then Task-007.
  Any unavailable predecessor leaves its downstream request
  typed unsupported/incomplete rather than partially implemented.
