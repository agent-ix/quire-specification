---
id: FR-289
title: "Observation-to-temporal boundary mapping"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-113
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-266
    type: references
---
# FR-289: Observation-to-temporal boundary mapping

## Description

When observations from a half-open window are used to assess a native inclusive
temporal interval, the adapter SHALL select and retain one explicit boundary
mapping that proves coverage of every admitted instant through the inclusive
upper bound without treating the half-open end as an included instant.

## Properties

- **clock binding** — exact clock identity shared by both domains.
- **native interval** — inclusive lower and upper bounds `[a,b]`, with `a <= b`.
- **observation carrier** — selected half-open bounds `[a,e)` in the same clock
  domain, where `e > b`.
- **membership mapping** — an observation participates in the native interval
  exactly when it is admitted by the selected carrier and `a <= instant <= b`;
  carrier members with `b < instant < e` remain observations but do not
  participate in this assessment.
- **progress mapping** — half-open progress settles the inclusive upper bound
  only when it proves complete coverage for every required source/subject at
  all admitted instants `<= b`; a watermark equal to `b` is insufficient.
- **discrete successor** — for an integer position or fixed-sample index domain,
  the canonical minimal carrier end is checked `b + 1`. Overflow refuses.
- **non-discrete end** — for event-time domains without a canonical successor,
  `e` is supplied explicitly and must compare greater than `b`; the adapter
  never invents an epsilon.

## Identity and comparison

- **Identity kind** — *value*.
- **Key** — (clock binding, native interval, observation carrier, progress
  authority selection). Equality is componentwise.
- **Key order** — lexicographic for deterministic collections only and carries
  no clock progress or causal meaning.
- **Causal order** — supplied by the selected clock/progress authority only.
- **Absence** — a missing carrier end, clock binding or progress authority is
  incomplete/refused and never defaulted.
- **Digest domain** — not applicable; the mapping carries no digest.

An empty or reversed native interval is not representable under the selected
bounded-temporal-operator contract and is refused before mapping. A point
interval `[a,a]` is admitted only with a carrier end strictly greater than `a`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-289-AC-1 | Events exactly at `a` and `b` participate, while events below `a` or above `b` do not, even when an above-`b` event belongs to the carrier window. | Test (TC-115, TC-143) |
| FR-289-AC-2 | A half-open end or watermark equal to `b` does not establish inclusive coverage through `b`. | Test (TC-115, TC-143) |
| FR-289-AC-3 | Discrete domains use checked successor and refuse overflow; non-discrete domains require an explicit `e > b` and never invent epsilon. | Test (TC-115, TC-143) |
| FR-289-AC-4 | `[a,a]` is mapped as one inclusive point, while a reversed interval, foreign clock or absent progress authority cannot settle. | Test (TC-115, TC-143) |
| FR-289-AC-5 | Reusing the carrier for an adjacent assessment neither double-counts an endpoint nor changes the immutable observation identities. | Test (TC-143) |

## Dependencies

- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) and
  [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establish inclusive temporal membership and progress.
- [FR-113](../../functional/observation/FR-113-preserve-time-progress-and-late-data.md)
  establishes half-open observation membership and authority.
- [FR-255](FR-255-bounded-temporal-interval.md) and
  [FR-266](../observation/FR-266-event-time-window.md) type the two retained
  boundary conventions.

## Status

Proposed shared ruling for `quire-specification#40`.
