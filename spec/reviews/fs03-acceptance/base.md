---
id: SR-540
title: "Base review of FS03 state semantic acceptance"
type: SpecReview
analysis: base
scope: "FS03 #4; state core/query/graph definitions, FR-033/034/038-046, NFR-010 and portable vectors"
review_set: subset
evaluated_revision: "task/4 working tree based on 08bc99d; QSL 0c83dcc; qcir 202210c"
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-033, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-034, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-040, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-041, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-043, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-044, type: reviews }
  - { target: ix://agent-ix/quire-specification/NFR-010, type: reviews }
---
# Base review of FS03 state semantic acceptance

## Summary

PASS. FS03 accepts the already-authored and composed-reviewed state semantics as
one ordered `state-core → state-queries → state-graph` closure. The change
does not redesign the architecture, reinterpret historical profiles or claim
that planned standard vectors have executed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: the state definitions, owning requirements and master index still called the reviewed state closure proposed. They now record FS03 acceptance and keep implementation/qualification independent. | state-contract; state-core; state-queries; state-graph; FR-033/034/038-046; NFR-010 |
| FND-002 | high | Resolved: accepting definitions without fixing their dependency order could permit a flat union of backend-supported constructs. The acceptance envelope now makes core, queries and graph a strict ordered closure and forbids permission transfer from an installed backend or wider caller. | FR-040; TC-040 |
| FND-003 | medium | Resolved: “accepted vectors” could be mistaken for executed conformance evidence. TC-033/034/038-046/048 now identify accepted portable expectations while TM-003 retains every affected row as Planned. | TM-003; TC-033/034/038-046/048 |
| FND-004 | medium | Resolved: the state definitions depend on the draft edition. FS03 now selects only the exact state-reachable grammar forms from edition `1-draft.2`; temporal, protocol and package meanings retain their own acceptance tickets. | state-contract; state-core |

## Acceptance check

- Exact rational normalization/division, checked integer/rational definedness,
  source control facts, aliases, anchors and captures have one selected meaning.
- Sequences remain ordered and duplicate-bearing; empty identities, total domains,
  every-prefix checks and authored maxima are explicit.
- Reference identity contains universe, declared type and object identity;
  traversal is positive-length, finite and identity-aware without a false cutoff.
- Optional absence, dangling references, unavailable observations, malformed
  inputs and resource exhaustion remain different non-Boolean outcomes.
- Unsupported kinds, casts, recursion and unproved definedness refuse without a
  partial backend artifact.

## Evidence boundary

Closed QSL #36/#37 establishes native predicate/query/graph implementation and
closed QSL #83/#84 establishes bounded-scalar native/oracle/proptest/Kani parity.
Neither closes the wider standard matrix. Contract-IR #69 remains the sole wider
definedness/object/graph/collection Kani campaign and must consume these semantics.
