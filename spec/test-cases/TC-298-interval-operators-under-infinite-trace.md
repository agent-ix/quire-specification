---
id: TC-298
title: "Interval operators under infinite-trace"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-367
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-363
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-091
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-092
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-255
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-250
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-368
    type: verifies
---

# TC-298: Interval operators under infinite-trace

## Description

The interval vectors: the recovery-stability formula over lassos that satisfy
and violate it, the safety classification of a bounded response, a run that
reaches `max_automaton_states`, previous, stutter invariance and the
past-origin rule.

## Test Procedure

Let `R` be `always (holds(fail) implies eventually always[0,10] holds(healthy))`
under `quire.temporal.infinite-trace/v1`.

1. Evaluate `R` on lasso A: a loop of 12 positions, `fail` at the first and
   `healthy` at the other 11; and on lasso B: a loop of 6 positions, `fail` at
   the first and `healthy` at the other 5.
2. Classify `R`, `always (holds(req) implies eventually[0,5] holds(ack))`,
   `always holds(x)` and `always eventually holds(p)`.
3. The **Restless** subject: a model in which `fail` can recur at any step.
   Check `R` over it with `max_automaton_states` 5, then with the request's
   limit omitted.
4. Evaluate `once[1,1] holds(p)` at position 0 and at position 3 of a lasso
   with `p` true at position 2 only.
5. Evaluate `always (holds(p) implies once[1,1] holds(q))`,
   `eventually[0,1] holds(r)` and `always eventually holds(r)` on a trace and
   on the same trace with one state repeated.
6. Evaluate `historically[0,3] holds(p)` at position 1 of a trace whose `p` is
   true at positions 0 and 1, and compare the key of `[0,3]` under the
   infinite-trace profile with `[0,3]` under the event-position profile.
7. Evaluate `eventually[2,4] holds(p)` nested under unbounded `always` and
   `p since[1,2] q` on lassos covering every endpoint.
8. Evaluate `eventually[2,*] holds(p)` at position 0 of a lasso whose `p`
   holds only at position 3 and of a lasso whose `p` holds only at position 1;
   `once[2,*] holds(p)` at position 1; `eventually[0,*] holds(p)` against
   `eventually holds(p)`; evaluate `holds(p) until[2,*] holds(q)` at position 0
   and `holds(p) since[2,*] holds(q)` at position 4 on traces where the operand
   holds only at the witness and at the single offset position `a`, and again
   with that operand false at `a`; classify `always eventually[2,*] holds(p)`; author
   `eventually[2,*] holds(p)` under the event-position profile.

## Expected Results

1. `R` is true on lasso A and false on lasso B, `trace_position` at a `fail`
   position (FR-367-AC-2).
2. `R` is liveness; the bounded response is safety; `always holds(x)` is a
   reachable-state invariant; `always eventually holds(p)` is liveness
   (FR-363-AC-2).
3. Every automaton for `R` or its negation must tell apart the 0 to 10
   healthy positions seen since a failure, so it has more than 5 states under
   any construction. With limit 5: FR-331 `incomplete`, FR-360 `failed`, execution
   `resource-incomplete`, naming `max_automaton_states`, value 5 and the
   request member that raises it, with no counterexample (FR-367-AC-5,
   FR-363-AC-4). With the limit omitted, the run uses the published default
   1,048,576, completes and settles `refuted` with a reproducing counterexample
   (FR-368-AC-7).
4. False at position 0; true at position 3 (FR-367-AC-3, FR-092-AC-7).
5. Repeating a state changes the truth of the first two formulas and leaves the
   third unchanged (FR-367-AC-4, FR-092-AC-8).
6. False at position 1, since positions before 0 read `p` as false; the two
   intervals compare unequal (FR-367-AC-6, FR-255-AC-6).
7. Each evaluates with its FR-091 or FR-092 offset meaning over the infinite
   trace, with no closed-boundary value at any position, the profile having no
   closed-boundary rule (FR-250-AC-6, FR-091-AC-9,
   FR-092-AC-8, FR-367-AC-1).
8. True on the first lasso and false on the second; `once[2,*]` is false at
   position 1; `until[2,*]` and `since[2,*]` are true when `p` holds at every
   position from `i + a` (or down to `i - a`) up to the witness and false when
   `p` fails at `i + a` (or `i - a`), whatever `p` is before it
   (FR-367-AC-7); `[0,*]` agrees with the unbounded operator at every position;
   the formula is liveness; under the event-position profile the spelling
   receives `unsupported_construct` (FR-367-AC-7).
