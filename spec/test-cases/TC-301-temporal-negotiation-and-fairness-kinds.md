---
id: TC-301
title: "Temporal negotiation, provider commitments and fairness kinds"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-368
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
---

# TC-301: Temporal negotiation, provider commitments and fairness kinds

## Description

Negotiation of `temporal-satisfaction` items: extent classification, the
fairness-kind check, the no-candidate warning and what an `unbounded`
advertisement commits a candidate to.

## Test Procedure

Register in turn:

- `W`: advertises (`temporal-satisfaction`, `unbounded`) and
  (`temporal-satisfaction`, `bounded`), `fairness_kinds` {`weak`};
- `WS`: the same with `fairness_kinds` {`weak`, `strong`};
- `B`: advertises (`temporal-satisfaction`, `bounded`) only;
- none.

Submit the TC-299 claim under `strong each`, the TC-293 step 2 claim under
`weak whole`, and the TC-290 bounded claim, naming the one registered backend.

1. With only `W` registered, submit the strong item and the weak item.
2. With only `WS` registered, submit the weak item and run it as a bounded
   search with horizon `max_depth` 2.
3. With only `B` registered, submit the weak item and the bounded item.
4. With none registered, submit the weak item.

## Expected Results

1. The strong item settles `unsupported`, `unsupported-requested-capability`,
   with a warning naming `temporal-satisfaction`, `unbounded`, `strong` and the
   constraint (`strong`, `acquire`, `each`); it is never run as weak. The weak
   item is supported (FR-368-AC-3).
2. The weak item is routed with its constraint unchanged in kind and
   granularity (FR-368-AC-4), and its depth-2 run returns `inconclusive`,
   `bound-reached`, accepted as a valid result of the candidate, alongside
   exactly one verdict per item (FR-368-AC-6).
3. The infinite-trace item classifies `unbounded` and settles `unsupported`,
   `unbounded-extent`, with a warning; the bounded item classifies `bounded`
   and is supported (FR-368-AC-2, FR-161-AC-7).
4. `unsupported`, `unsupported-requested-capability`, with a warning naming
   `temporal-satisfaction`; it never holds or blocks (FR-368-AC-5,
   FR-161-AC-7).
