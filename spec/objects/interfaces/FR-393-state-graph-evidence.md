---
id: FR-393
title: "State-graph evidence wire"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-391
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-394
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
---

# FR-393: State-graph evidence wire

## Description

When a state-graph item settles from evidence, the producer SHALL encode that
evidence as one `GraphEvidence` value below, serialized with RFC 8785 JCS, and
the reader SHALL refuse a value that breaks a rule below before any replay
runs.

## Properties

- **ModelPath** — `{"type":"model-path","initial":"<decimal>","steps":[<step>, ...]}`:
  - `initial` — the index of the path's first state in the subject's initial
    states, in their canonical order, as a decimal string;
  - `steps` — the path's steps in order, possibly none. A step is the model
    trace step of a model-subject counterexample:
    `{"type":"step","transition":<transition>,"post_state":"<64 lowercase hex>"}`,
    with `transition` the FR-181 typed canonical form of the step's
    transition identity and `post_state` the post-state's
    `quire.simulation.state-key/v1` digest (FR-181).
- **Instance** — the `over` binding of the item's instance:
  `{"type":"instance","parameter":"<ident>","key":<encoded>}`, with `key`
  the bound object's universe key in FR-181 typed canonical form, or `null`
  for a clause with no `over` parameter.
- **WitnessSource** — exactly one of
  `{"type":"sampled","seed":"<decimal>","trace_index":"<decimal>"}`,
  `{"type":"explored"}` or `{"type":"unrolled","depth":"<decimal>"}`.
- **GraphEvidence** — exactly one of four arms, each carrying `instance`:
  - `{"type":"graph-evidence","arm":"witness","instance":<Instance>,"paths":[{"path":<ModelPath>,"source":<WitnessSource>}, ...]}`
    — one entry per initial state of the subject, ordered by `initial`, each
    path ending at a node where `P` holds. For a single-existential claim
    with a temporal body ([FR-396](../../functional/analysis/FR-396-classify-hyperproperty-forms.md))
    each entry also carries `"loop_entry":"<decimal>"`, the position at which
    the path's last step closes its loop, so the path is a lasso; no other
    form carries `loop_entry`;
  - `{"type":"graph-evidence","arm":"trap","instance":<Instance>,"stem":<ModelPath>}`
    — `stem` ends at the trap node;
  - `{"type":"graph-evidence","arm":"path-pair","instance":<Instance>,"stem":<ModelPath>,"first":[<step>, ...],"second":[<step>, ...]}`
    — `stem` ends at a node where `X` holds, and `first` and `second` are two
    step sequences from that node, each ending at its first `Y` node;
  - `{"type":"graph-evidence","arm":"undefined","instance":<Instance>,"stem":<ModelPath>,"undefined":<UndefinedEvaluation>}`
    — `stem` ends at the node where a predicate of the claim, or a guard,
    the `terminal when` predicate or a state constraint of the subject,
    evaluates undefined ([FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md));
    for a guard that node is the transition's pre-state. `undefined` carries
    that evaluation.
- **UndefinedEvaluation** — the
  [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  undefined-evaluation envelope,
  `{"type":"undefined-evaluation","where":{...},"cause":<reason>}`, with the
  model-arm keys of FR-364: `position`, the stem's number of steps;
  `state_key`, the `quire.simulation.state-key/v1` digest of the stem's last
  state; and `locus`, the source locus of the expression that has no value.
  For a predicate of the claim, a state-graph `where` also carries
  `predicate`, one of `"P"`, `"Q"`, `"X"` or `"Y"`, naming the predicate that
  has no value. For a guard it carries FR-364's `transition`, the FR-181
  transition identity whose guard it is. For the `terminal when` predicate or
  a state constraint it carries neither, and `locus` names the expression. It
  is the `where` and
  `cause` of the item's
  [FR-363](../../functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md)
  `undefined-evaluation`, so the evidence replays on its own.

Every decimal is `0` or `[1-9][0-9]*`.

## Envelope and identity

`GraphEvidence` travels in the witness envelope of the replay boundary with
its own model-graph replay-source arm, distinct from the backend-transcript
and input arms of [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md).
The envelope's obligation identity binds the model subject (initial states,
universes and proof bounds) and the checked claim, including its form and
instance. The evidence carries no canonical state, permutation or reduction:
evidence found by a reduced search is concretised before it is encoded.

## Validation

The reader SHALL refuse each of the following as
`invalid_runtime_input`/`invalid-value`, before replay, naming the offending
member:

- an `arm` outside the four above, a missing member, a duplicate key or a
  non-canonical encoding;
- a `witness` arm whose entries do not number exactly the subject's initial
  states, or whose `initial` values are not `0` to `n - 1` in order;
- an `initial` index outside the subject's initial states;
- a `loop_entry` on an entry of a form that carries none, an absent
  `loop_entry` on an entry of a form that requires one, or a `loop_entry`
  greater than the path's number of steps;
- an `instance` whose parameter or key does not match the item's instance;
- an `undefined` arm with no `undefined` member, or whose `cause` is not a
  catalogued undefined reason, or an `undefined` member on any other arm;
- an `undefined` arm whose `where` carries both `predicate` and `transition`,
  or a `transition` that is not a transition of the subject enabled-tested at
  the stem's last state.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-393-AC-1 | Each of the four arms serializes and reads back byte-identically under RFC 8785 JCS with exactly the members above. | Test (TC-335, TC-349) |
| FR-393-AC-2 | A `witness` arm carries one entry per initial state in index order, each with its source; a `sampled` source names its seed and trace index. | Test (TC-331, TC-335) |
| FR-393-AC-3 | An unknown arm, a missing or duplicate member, a witness arm with the wrong number or order of entries, an out-of-range initial index, a misplaced or out-of-range `loop_entry` and a mismatched instance each refuse as `invalid_runtime_input`/`invalid-value` before replay. | Test (TC-335) |
| FR-393-AC-5 | An `undefined` arm carries its `undefined-evaluation` value (state-key digest, locus and catalogued reason, plus the predicate for a claim predicate or the FR-181 transition identity for a guard) and reads back with it; an `undefined` arm without it, with an uncatalogued reason, with both `predicate` and `transition`, with a `transition` not enabled-tested at the stem's last state, or an `undefined` member on another arm refuses before replay. | Test (TC-335, TC-349) |
| FR-393-AC-4 | Two envelopes whose evidence is equal but whose subject or claim differs have different obligation identities. | Test (TC-335) |

## Dependencies

- [FR-181](../../functional/runtime/FR-181-simulate-finite-executions.md) owns
  transition identities, state keys and their typed canonical form.
- [FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md)
  decides which arm settles an item.
- [FR-394](../../functional/analysis/FR-394-replay-state-graph-evidence.md)
  replays each arm.
- [FR-197](../../functional/ir-proof/FR-197-replay-backend-counterexamples.md)
  owns the replay boundary and its other source arms.
- [FR-364](../../functional/temporal/FR-364-replay-temporal-counterexamples.md)
  owns the model-trace step that a `ModelPath` step reuses.

## References

- STD-135 (this requirement); QSL-369 and its ADR-022 (design).
