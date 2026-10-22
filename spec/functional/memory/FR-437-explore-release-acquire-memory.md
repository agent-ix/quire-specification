---
id: FR-437
title: "Explore a parallel under the RC11 release-acquire memory model"
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

# FR-437: Explore a parallel under the RC11 release-acquire memory model

## Description

When a `parallel`'s resolved memory model is `ra`, the analyzer SHALL explore
it as RC11 without promises: per-location message sequences in modification
order, per-thread views, a global `seq_cst` fence view and the live part of
RC11's partial SC order, by the rules below, with visibility fairness on every
infinite-trace clause.

## Inputs

A protocol subject with an `ra` `parallel`, its access classes
([FR-435](FR-435-classify-memory-accesses-and-orderings.md)) and a protocol
state.

## Outputs

The `ra` memory component of each state, the message or slot choices of each
access step, and the visibility fairness constraint.

## Behavior

### Component

- For each shared location `ℓ`, a **message sequence** in modification order
  (mo). Each message holds a value, a **message view** and an **attached**
  mark when an RMW wrote it directly after its predecessor.
- A **view** maps each shared location to one of its messages. Each thread
  `b` holds a current view `cur_b`, an acquire view `acq_b` and a release view
  `rel_b(ℓ)` per location. One global view `S` serves `seq_cst` fences.
- The SC event graph below and the race summary
  ([FR-438](FR-438-report-data-races.md)) complete the component.
- Entering an outermost weak `parallel`, each location holds one message, its
  current value, with every view pointing at it.

A message is named by its **depth**: 0 for the mo-last message, counting back.
A depth is a decimal string.

### Typed form in the state key

The state key's `memory` member is an FR-181 map, empty outside a weak
`parallel`. Its entries are:

| Key | Value |
| --- | --- |
| `{"type":"messages","location":<location>}` | `{"type":"sequence","elements":[{"value":<encoded>,"view":<view>,"attached":true\|false}, ...]}`, the messages oldest first |
| `{"type":"views","thread":<thread path>}` | `{"cur":<view>,"acq":<view>,"rel":<map from location to view>}`, one entry per live thread |
| `{"type":"sc-view"}` | the view `S` |
| `{"type":"sc-graph"}` | `{"events":[<event>, ...],"edges":[[<event>,<event>], ...],"owed":<map from thread path to [<event>, ...]>,"summaries":<map from location to [<event>, ...]>}` |
| `{"type":"race-summary","location":<location>}` | FR-438's race summary |

A **view** is `{"type":"view","at":<map from location to message depth>,"frontier":<map from thread path to count>,"owed":[{"location":<location>,"event":<event>}, ...]}`.
`at` holds every shared location; a pointer at a removed message encodes as the
depth of its location's earliest live message. `frontier` maps a thread to the
number, as a decimal string, of that thread's live SC events the view covers,
which are the earliest of them in program order, so the count names the
frontier's latest covered event; a thread whose count is 0 has no entry.
`owed` lists the view's pairs.

An **event** is `{"thread":<thread path>,"kind":"read"|"write"|"rmw"|"fence","ordinal":"<decimal>","location":<location>,"read":"<depth>","write":"<depth>"}`.
`ordinal` is the number of live SC events of the thread before it in program
order. `location` is absent for a fence; `read` is present for a read or an
RMW and `write` for a write or an RMW, each the depth of that message in the
state. In `sc-graph`, `events` lists every live SC event, `edges` the pairs
`[e, x]` of `P`, `owed` each thread's `owed(b)` and `summaries` each `F(ℓ)`.
Each array of events, owed pairs or edges lists its elements ascending by
canonical bytes. Equal histories that admit the same futures encode to equal
bytes.

### Access rules

| Access by `b` | Choices | Effect |
| --- | --- | --- |
| load of `ℓ` | any message `m` of `ℓ` at or after `cur_b(ℓ)` in mo | `cur_b(ℓ)` moves to `m` and `acq_b` joins `m`'s view; an `acquire` or `seq_cst` load also joins `m`'s view into `cur_b`; a non-atomic load reads as `relaxed` |
| store to `ℓ` | any slot after `cur_b(ℓ)`, except directly after a message whose successor is attached | inserts the new message and moves `cur_b(ℓ)` to it; a `release` or `seq_cst` store's message view is `cur_b`, and `rel_b(ℓ)` becomes `cur_b`; a `relaxed` or non-atomic store's message view is `rel_b(ℓ)` joined with the new message |
| RMW of `ℓ` | a message `m` at or after `cur_b(ℓ)` with no attached successor | reads `m` as a load with the read part of its ordering and inserts its message directly after `m`, attached, as a store with the write part; its message view also joins `m`'s view. Two RMWs never read one message |
| `acquire` fence | — | `cur_b` := `cur_b` joined with `acq_b` |
| `release` fence | — | `rel_b(ℓ)` := `cur_b` for every `ℓ` |
| `acq_rel` fence | — | both |
| `seq_cst` fence | — | both, then `cur_b` and `S` := `cur_b` joined with `S`; an SC event, which enters the frontier of `cur_b`, of `S` and of every `rel_b(ℓ)` as it becomes one |

Each choice of message or slot is a separate transition, and the transition
identity's `memory_choice` member names it as `{"read_depth":"<decimal>"}`,
`{"slot_depth":"<decimal>"}` or both.

**Split, merge and channels.** Each branch starts with its parent's views. The
`join` sets the parent's views to the join of its branches' views. A `send`
carries the sender's `cur_b`, and its `receive` joins that view into the
receiver's `cur` and `acq`. `ra` has no gate and no memory step.

**Atom values.** A `holds` atom reads each shared location's mo-last message
value. Branch-local locations hold their thread's value.

**Garbage collection.** After every step, every message of `ℓ` earlier in mo
than the earliest message any view points to at `ℓ` (every thread's three
views, every message view, and `S`) is removed. No step can read a removed
message. After an outermost `join all` only the parent's views remain and each
location keeps one message.

### RC11 `seq_cst` accesses: the SC event graph

- **State.** The live SC events, each a `seq_cst` access or fence with its
  thread, kind, location and message; a directed acyclic graph `P` over them,
  `e → x` when `e` precedes `x` in RC11's `psc`, closed under transitivity;
  for each location `ℓ` the set `F(ℓ)` of live SC events that precede in
  `psc` every SC write to `ℓ` still to come; and the `owed` sets of the
  collection rule below, one per thread and the pairs of every view.
- **Frontier.** Every view also records, for each thread, the latest live SC
  event of that thread that happens before the view or that the join of `S`
  reaches; every live SC event of that thread no later in program order is
  covered by the view too. Frontiers are carried and joined wherever views
  are, a join taking the later event for each thread. A `seq_cst` fence is
  recorded in the frontier of its thread's views as it becomes an SC event.
  "Happens before the view" below means covered by the frontier.
- **A new SC event `x` by `b`.** `P` gains these edges, each from a live SC
  event: `e → x` for each `e` of `b`; `e → x` for each `e` on `x`'s location
  that happens before `b`'s current view (RC11's `hb|loc`); `f → x` for each
  `seq_cst` fence `f` of another thread that happens before `b`'s current view
  (`[Fsc] ; hb ; po ; [Esc]` for an access `x`, `[Fsc] ; hb ; [Fsc]` for a
  fence `x`); `e → x` for each live SC access `e` of another thread that
  happens before `b`'s current view, when `x` is a fence (`[Esc] ; scb ; hb? ;
  [Fsc]`, whose `scb` step is `po`, or `hb|loc` when `e` itself releases);
  `e → x` for each live SC write `e` mo-before the message `x` writes and,
  when `x` writes, each live SC read `e` of a message mo-before it; `x → e`
  for each live SC write `e` that `x` precedes in mo or fr; and, for a write
  to `ℓ`, an edge from each member of `F(ℓ)`. The happens-before edges are
  decided from the frontier of `b`'s current view at `x`, taken after `x`'s own
  joins: a `seq_cst` load's view includes the message it reads, and a
  `seq_cst` fence's view includes the joins of its acquire part and of `S`.
  RC11's fence clause `[Fsc] ; (hb ∪ hb ; eco ; hb) ; [Fsc]` builds its `hb`
  part as the fence-to-fence edge above and no edge for its `eco` part. `x` is
  enabled with a given message or slot choice only when the resulting `P` is
  acyclic, so the choices `P` forbids are pruned.
- **Omitted clauses.** `P` does not build three kinds of RC11 `psc` edge. (a)
  The `po|≠loc ; hb ; po|≠loc` edge from an SC access to an SC access, whose
  test names locations a view does not record. (b) The edges of `psc_base`
  with an SC fence at one end whose `scb` step is an `mo` or `rb`
  step with a non-SC access at an end on a fence's side, such as the `rb` step
  from a `relaxed` load in the fence's thread to an SC write; `P` holds SC
  events only. The `scb` steps `po` and `hb|loc` are built, so the
  fence-to-access edge of LT-14 is built. The fence-to-fence form is covered by
  `S`: every `seq_cst` fence joins `S` and enters its frontier, so every two
  live SC fences are ordered. (c) The `eco` part of the fence
  clause, `[Fsc] ; hb ; eco ; hb ; [Fsc]`, which the join of `S` at a
  `seq_cst` fence is meant to carry by not offering the message choices that
  would complete it. That it does is not proved; it was checked by exhaustive
  enumeration of two bounded families of programs with fences, with no RMW and
  no non-atomic access. Every RC11 execution is an `ra` execution, and `ra`
  also admits an execution only when every RC11 `psc` cycle through it uses
  an omitted edge; both claims were checked on bounded random programs without
  RMW or non-atomic accesses and are not proved. A `proved` verdict therefore
  holds under RC11, and a counterexample is an `ra` execution that replay
  checks under `ra`.
- **Collection.** When garbage collection removes a message, each live SC
  event on it leaves `P`, which is restricted to the remaining events with its
  transitive closure kept, and its live predecessors are kept for every later
  event they precede:
  - a removed write's predecessors, and a removed SC read's, join `F(ℓ)`; the
    read's own edges to later writes need nothing more, since a later event
    takes no edge to the read;
  - each thread `b` holds `owed(b)`, the live SC events that precede every
    later SC event of `b`; a removed event of `b` adds its live predecessors
    to it, and a new SC event of `b` takes it as predecessors;
  - every view, including a view that a `send` carries and no `receive` has
    yet joined, also holds pairs `(ℓ, p)`, `p` a live SC event that precedes
    every later SC event on `ℓ` of a thread whose current view includes the
    view; a removed event on `ℓ` adds `(ℓ, p)` for each live predecessor `p`
    to every view whose frontier covers it, wherever the view is held (by a
    thread, in a message, as `S`, or in flight on a channel); joining views
    unions the pairs;
  - a member of `F(ℓ)` or of `owed(b)`, or the `p` of a pair, that is itself
    removed is replaced by its live predecessors.
  An SC access is live only while its message is.
- **Fence lifetime.** A fence is on no message, so no message collection
  removes it; fence removal runs with garbage collection, after every step. A
  `seq_cst` fence `f` of thread `t` is live while some view has
  `f` as the latest live `seq_cst` fence of `t` that its frontier covers: a
  thread's three views, every message view, `S`, and every view a `send`
  carries that a `receive` has not yet joined. A view covers `f` when its
  frontier entry for `t` is `f` or a later event. When no view does, every view
  that covered `f` covers a later live fence of `t`, which takes each edge `f`
  would take, and `f` leaves `P`. Because a `seq_cst` fence enters the
  frontier of its thread's views as it becomes an SC event, a thread that
  fences twice with no store between holds the second fence, not the first, in
  every view after the second fence.
- **Key.** `P`, `F`, the frontiers and the `owed` sets are part of the state
  key, with events named by thread, kind, ordinal and message depth (the typed
  form above).

### Visibility fairness

Every infinite-trace clause over a subject with an `ra` `parallel` carries
one visibility fairness member, `{"kind":"strong","target":{"type":"visibility","model":"ra"},"granularity":"whole","origin":"memory"}`.
Its kind is `strong`: for each pair, "infinitely often an edge of `R` implies
infinitely often an edge of `G`" is the strong-fairness form, so FR-368 routes
an `ra` item only to a candidate whose `fairness_kinds` holds `strong`. The
member's (thread, location) pairs are derived from the resolved program, one
for each location a thread loads. The goal of a pair (`b`, `ℓ`) is that thread
`b` does not keep reading a stale message of `ℓ` forever. On a lasso, the loop
contains a load of `ℓ` by `b` from `ℓ`'s mo-last message for every pair
(`b`, `ℓ`) the loop loads. On an SCC it is an edge Streett condition. For each
pair, `R` is the set of edges on which `b` loads `ℓ` and `G ⊆ R` the set of
edges on which `b` loads `ℓ` from its mo-last message; a **loop** (a closed
walk, which may repeat states) is visibility-fair when, for every pair, it
contains no edge of `R` or an edge of `G`. The edges of an SCC `C` are the
edges of the current subgraph with both ends in `C`. `C` holds a
visibility-fair loop when every pair has no edge of `R` among the edges of `C`
or an edge of `G` among them; otherwise, for a pair with an edge of `R` in `C`
and none of `G`, every loop of `C` that loads `ℓ` by `b` is unfair, so the
edges of `R` in `C` are removed and the rest decomposed into SCCs, each tested
the same way. An SCC is therefore never rejected for a stale load alone: a
load-free sub-loop of an SCC that holds a stale load edge is fair. It is
machine-closed.

### What it admits

Coherence holds per location. Program order together with reads-from is
acyclic, so load buffering is excluded. IRIW and 2+2W are admitted with
release and acquire accesses and excluded with `seq_cst` accesses, as in
RC11. The omitted clauses admit the two shapes of TC-391 LT-12 and LT-13 that
RC11 excludes.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-437-AC-1 | The store-buffering protocol under `ra` reaches exactly 14 states inside the `parallel` and 4 joined states, and the claim settles `refuted` with the counterexample whose last load reads `x` at depth 1. | Test (TC-390) |
| FR-437-AC-2 | Each `ra` row of the litmus table (TC-391) gives its stated outcome: SB, MP with relaxed flags, IRIW with release/acquire and 2+2W with release stores allowed; SB with `seq_cst` fences, SB with `seq_cst` accesses, MP with release/acquire, LB, IRIW all `seq_cst` and CoRR forbidden. | Test (TC-391) |
| FR-437-AC-3 | With every store and load `seq_cst`, the store-buffering outcome is excluded because the second load's old-message choice would close the cycle `Sx → Ly → Sy → Lx → Sx` in `P`, and the claim settles `proved`. | Test (TC-390) |
| FR-437-AC-4 | Once every view points at or after a location's mo-last message, every earlier message of that location is removed in the same step; after an outermost `join all` each location keeps one message; and two histories that admit the same futures share one state key. | Test (TC-393) |
| FR-437-AC-5 | A lasso on which a thread keeps loading a stale message of a location without ever loading its mo-last message is rejected by visibility fairness. | Test (TC-391) |
| FR-437-AC-6 | The litmus vectors' outcomes under `ra` include every outcome that an independent RC11 execution-graph checker allows on the same tests, and differ from the checker's only on LT-12 and LT-13, which the checker forbids and `ra` allows. | Test (TC-391) |
| FR-437-AC-7 | On an accepting SCC with states `p`, `q`, `r` and the edges `p → q`, `q → p` and `r → q` (thread `a`, which loads nothing), and `q → r` (thread `b` loads `ℓ` from a message that is not mo-last), with `p` the only accepting state, visibility fairness removes the edge `q → r`, decomposes the SCC into `{r}` and `{p, q}`, and passes `{p, q}`, whose loop `p q p q …` is visibility-fair. | Test (TC-391) |
| FR-437-AC-8 | TC-391 LT-12 and LT-13 each reach their weak outcome under `ra`, the two shapes of the omitted clauses, and neither under `sc` or `tso`. | Test (TC-391) |
| FR-437-AC-9 | TC-391 LT-14 and LT-18 do not reach their weak outcomes under `ra`. | Test (TC-391) |
| FR-437-AC-10 | TC-391 LT-15 does not reach its weak outcome under `ra`, whether or not garbage collection has removed the SC event on `L := 2`, the earlier message the storing thread wrote: when it has, its live predecessors, among them `X := 1`, are kept for that thread, so `X := 2` is not offered the slot before the first writer's `X := 1` in modification order. | Test (TC-391) |
| FR-437-AC-11 | TC-391 LT-16 does not reach its weak outcome under `ra`: a thread that read the last message of `L` with a `relaxed` load and executed an acquire fence has a view that covers the collected SC event it synchronised with, though the message its next `seq_cst` load of `L` reads does not; the load takes the predecessors held in the view, so its thread's `X := 2` is not offered the slot before `X := 1` in modification order. | Test (TC-391) |
| FR-437-AC-12 | TC-391 LT-17 does not reach its weak outcome under `ra` in any interleaving, including those in which the fence's thread fences a second time before the other thread loads the flag: the first fence stays in `P` while the flag's message view covers it and not the second. | Test (TC-391) |
| FR-437-AC-13 | A thread that fences twice with `seq_cst` and stores no flag between has, after its second fence, the first fence absent from the SC event graph; with a `release` flag stored between the fences, the first fence remains while the flag's message is live and its view covers the first fence and not the second. | Test (TC-393) |
| FR-437-AC-14 | With every store and load `seq_cst` (TC-390 SB-05), after `Sx` under `ra`, the `sc-graph` entry holds one event `{"thread":<left>,"kind":"write","ordinal":"0","location":<x>,"write":"0"}`, no edge, no `owed` entry and no summary; `left`'s `cur` view has `at` mapping `x` to depth `"0"` and `y` to depth `"0"` and `frontier` mapping `left` to `"1"`, and `right`'s has an empty `frontier`. | Test (TC-394) |

## Dependencies

- [FR-434](FR-434-select-memory-models-for-parallel.md) owns the
  contributions these rules fill.
- [FR-435](FR-435-classify-memory-accesses-and-orderings.md) owns the access
  classes and orderings.
- [FR-438](FR-438-report-data-races.md) owns the race summary.
- [FR-439](FR-439-bound-replay-and-report-weak-memory-runs.md) owns the message
  bound, verdicts and replay.

## References

- Design: quire-spec-language ADR-025 RA-1 to RA-8, PSC-1 to PSC-7, MS-2,
  MF-2, RU-3 and RU-6; Lahav et al., PLDI 2017; Kang et al., POPL 2017;
  Lahav et al., OOPSLA 2021.
- Tickets: Linear STD-138; paired QSL-372.
