---
id: SR-16253
title: "spec-review of the restored hybrid features in quire-specification PR 415 (STD-409 to STD-415)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@7bbc566d; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md; spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/test-cases/TC-373-hybrid-verdict-map.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md; proposals/quire-v1/shared-grammar.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/node-identity-preimage.schema.json; proposals/checked-package-v2/hybrid-node-vectors.json; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-193
    type: references
  - target: ix://agent-ix/quire-specification/FR-481
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# Spec review of the restored hybrid features in quire-specification PR 415

## Summary

Tickets: STD-409 to STD-415. This is one spec-review pass over
`git diff origin/main...7bbc566d`, read together with the QSL counterpart
agent-ix/quire-spec-language#1365 (head caf3aab8) and the open fix-forward
PR #414 (head 59cad4f4).

Gates at 7bbc566d: `quire validate --scope . 'spec/**/*.md'` (quire 0.36.2,
engine 0.50.2) gives 86 `fail` lines, as on main, but the structural count
rises from 116 to 117 documents (FND-001). `cargo test --test
checked_package_v2 --test backend_provider_v1` passes, 134 and 23 tests. The
QSL conformance run `cargo test -p qsl-package --lib checked_v2` with
QSPEC_DIR at this head is recorded in the Linear comment. origin/main is an
ancestor of the head.

Hand derivations, all over QSL FR-447's `Braking` (`Moving`: invariant
`v >= 0 and v <= 20 and a >= 1 and a <= 2`, flow `v' = -a; a' = 0; t' = 1`,
transition to `Stopped` when `v = 0` with reset `v = 0`; `Stopped`:
invariant `v = 0`):

- FR-193-AC-17 and HY-16: from `(v, a, t) = (20, 2, 0)` the solution is
  `v(s) = 20 - 2s`, so `v(10) = 0`, `a = 2` and `t = 10` equal `before`.
  The invariant holds on `[0, 10]`, the guard `v = 0` holds at `before`, the
  reset gives `v = 0` with `a` and `t` kept, and `Stopped`'s invariant holds.
  `always [0, 12] holds(h.v > 0)` is false at `t = 10` inside the prefix, so
  the prefix is bad and the vector settles `refuted`. With `before` `v = 1`
  check 2 fails first, as the AC says. A flow `v' = v` has a cycle, so the
  trace is static-only. All of this is correct.
- FR-193-AC-14 interval conversions: `1500 ms` is `3/2` s, `0.5 s` is `1/2`
  s, and FR-481-AC-9's `10500 ms` is `21/2` s, which the vector carries as
  the timed form `[0, 21/2]`, both ends closed. Correct.
- FR-481-AC-9 levels: `Braking2` has `n = 3` variables, so `q.v` is at
  level `n + 0 = 3` and capture `v0` at `2n + 0 = 6`. The vectors agree, and
  AC-10's parameter `v` at level 4 is the `q.a` slot with the wrong name.
  Correct.
- FR-193-AC-13 resets: `pre(v) / 2` reads the pre-jump `v`, and `a >= 1 and
  a <= 2` bounds the post value of `a`. Correct.

Examined, clean: the `initial` grammar and its default, the union and
invariant intersection of initial entries, the expression-language rules
and the undefined-evaluation rule, the reset rules and refusals, positions
under `model-time` and `model-steps`, past operators, activations and
captures, hybrid fairness (`whole`/`each`, `S` and `S::T`), the static
replay widening by `delta`, the `HybridTrace` members `before`, `delta` and
`loop`, the FR-370 changes for `upper: null`, and the FR-481 reader checks 4,
7, 8 and 10. The diff has no deferral or narrowing language. Every
restriction it states (point traces only for exact replay, per-variable
reset conjuncts, rational constants only) is the scope the tickets
themselves give.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-481's frontmatter is broken YAML. The `FR-469` relationship is indented one level too deep (`    - target: ix://agent-ix/quire-specification/FR-469`), so the block mapping fails to parse. quire now reports `required 'type' is missing from frontmatter` for FR-481, and the structural count rises from 116 to 117 documents. The 86-line gate still passes only because that message has no word `fail` in it. In the body, the `modes` and `resets` bullets also gained an extra indent and now nest under the previous bullet. Fix: restore two-space indentation for the FR-469 entry, and for the `modes` and `resets` bullets. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:14-17,101,132 | wrong-requirement |
| FND-002 | high | FR-331 still refuses every plugin counterexample other than `assignments`, so the hybrid refutation path FR-193 adds can never be read. FR-331 "Result entries" says "A plugin's `counterexamples` entry carries a `counterexample` whose `type` is `assignments`", and its `invalid-value` row refuses any other type. FR-193 now lets "a provider" report `refuted` with a `HybridCounterexample`, and FR-193-AC-17 and HY-16 settle a provider's `refuted` entry by replay. Hybrid solvers are plugin providers (dReal and others; BP-36's vector uses backend `dreal`). The FR-331 reader therefore refuses `invalid-value` at `/counterexample/type` before any replay runs. Fix: amend FR-331's plugin-counterexample rule, its reading order and AC-34/BP-25 to admit `hybrid-counterexample` on a hybrid item's entry, subject to FR-193 replay, and add the plugin vector. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:518-519,544; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:336-341 | missing-requirement |
| FND-003 | high | FR-481-AC-8 and `restored_nodes` say they are "QSL FR-447's `Braking2`", but they do not match the QSL source. The initial conditions in the vector are `v = 20` and `v = 0`, the eq nodes `b559…` and Stopped's invariant `bbe5…`. QSL FR-447 (#1365) writes `v = 20 and a >= 1 and a <= 2 and t = 0` and `v = 0 and t = 0`, and FR-193 "Initial states" shows the same entries. The capture `v0` is typed by the generic `scalar_type`/`rational` node (`9587…`), while QSL declares `capture v0: Rational[0, 20; 1, 1]`, and FR-481 and FR-447 say the capture node is "typed by the capture's type". The vector drops the range that FR-193 "Activations and captures" makes semantic (a value outside `T`'s range has no value). QSL's S4 output cannot equal these nodes. Fix: regenerate `restored_nodes` from QSL's `Braking2`/`Slows` source, with the full conditions and `v0` typed by the `Rational[0, 20; 1, 1]` type node, and restate AC-8 and AC-9. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:352-353; proposals/checked-package-v2/hybrid-node-vectors.json (restored_nodes 1, 5, 9) | wrong-requirement |
| FND-004 | medium | A hybrid `[a, *]` interval has two possible encodings. FR-481 says an interval "whose bounds are whole seconds with both ends closed takes the integer form, and every other interval the timed form", and the timed form with `upper` `null` and `upper_end` `open` is `[q, *]`. For `[2, *]` or `[2 s, *]` the `*` end is neither open nor closed as written, so the rule picks neither the integer `{lower: "2", upper: null}` nor the timed `{lower: 2, upper: null, lower_end: closed, upper_end: open}`. The two spellings key differently, and reader check 9 cannot refuse the wrong one. Fix: state which form a whole-bounded `[a, *]` takes (for example the integer form, as before this PR) and that the reader refuses the other, with a vector. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:185-190; spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md:115-121 | wrong-requirement |
| FND-005 | medium | Exact replay is decidable as stated only for rates; every other expression it evaluates can still call a recursive function. FR-193 limits a "polynomial flow" to rates after "expansion of non-recursive calls", then says every atom along a dwell is "a univariate rational function of time whose sign on an interval is decidable". An invariant, guard, reset, initial condition or claim atom may call a recursive pure function (FR-193 admits any pure function whose body takes these forms). Along a dwell such an atom is not a rational function, so exact check 2 or 5 has no decision procedure. Fix: include in the point-trace condition that every call that replay evaluates expands (no recursive call), else the trace is static-only, and add a vector. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:444-458 | missing-requirement |
| FND-006 | medium | The precedence between a formula violation and an undefined evaluation in one replayed prefix is unspecified. FR-193 says "When every check holds" the item settles `refuted` of kind `formula`, and "When an expression has no value at an instant of the replayed prefix" it settles kind `undefined-evaluation`. Both can hold at once. QSL FR-450-AC-5 is that case: `1 / (h.v - 5) < 100` is already false on `[1499/200, 15/2)`, before the undefined instant `15/2`. Neither the kind nor which instant `where` names is determined. Fix: state the precedence (for example the earliest violating instant decides, an undefined value at it giving `undefined-evaluation`), and say which instant `where` names. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:473-481 | wrong-requirement |
| FND-007 | medium | The static replay outcome has nowhere to be recorded. FR-193 says "the reader keeps the outcome with the trace" (`consistent` or `inconsistent{at, check}`, and `not-reproduced{at, check}` for exact replay), and the SolverInconclusive schema description says the trace is "kept with its replay outcome". No FR-331 record member, schema member or spelling carries that outcome. An implementer cannot write it to the terminal record, and two readers would differ. Fix: define the outcome's record member and spelling in FR-331, for example an optional `replay` on the `solver-inconclusive` cause, with schema and vectors. Otherwise state that the outcome stays internal to the reader. | spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:440-442; proposals/backend-provider-v1/schema.json:SolverInconclusive | missing-requirement |
| FND-008 | high | Conflicts with the open fix-forward #414. Textual conflicts are in FR-481, TC-966 and `tests/checked_package_v2.rs` (`git merge-tree`). Ids collide: both PRs add FR-481-AC-8, FR-481-AC-9 and TC-966 HN-11 with different content. The semantics conflict too. #414's FR-481 pins a hybrid claim's `temporal_profile` law to `quire.temporal.infinite-trace/v1`, and its new FR-481-AC-8 refuses `model-steps` and an `each` activation, which reverses STD-410. #414 moves layout defects to strict wire validation (`malformed_wire`), while this PR adds layout refusals as `invalid_semantic_graph` (initial shape, written default `initial`, reset entry members). #414 admits a variable node directly as a rate or reset term. Its FR-193 trace-reader wording ("the host that requested the item") replaces the paragraph that this PR's "Trace replay" builds on. Fix: rebase onto whichever lands first, renumber the colliding ids, drop #414's profile pin and `model-steps`/`each` refusals in favour of the restored forms, and recast the new layout checks under #414's strict-wire step. | spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md:223-330,352-355; spec/test-cases/TC-966-checked-package-hybrid-model-nodes.md:41; tests/checked_package_v2.rs | wrong-requirement |
| FND-009 | low | BP-36's positive `loop` vector is a degenerate lasso. Result 1's trace has one jump and `loop` 1, so it names its own last state: the loop has zero dwell and denotes no time-divergent behaviour (FR-193 exact check 4 rejects it). Static check 5 is trivially true for it. FR-193 neither refuses nor defines a `loop` equal to the last state's index. Fix: use `loop` 0 with a returning trace in the positive vector, and state whether `loop` naming the last state is admitted. | proposals/backend-provider-v1/vectors.json:3635; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:381-384 | wrong-requirement |

## Verdict

Not mergeable. FND-001 breaks FR-481's frontmatter and is invisible to the
86-line gate. FND-002 means the new hybrid refutation path is refused by
FR-331's own reader. FND-003 means the restored vectors are not what QSL's
`Braking2` and `Slows` emit. FND-008 is the conflict with #414, including id
collisions and a direct semantic contradiction. The restored semantics
themselves are complete for the tickets' scope, and the hand-derived replay,
reset, level and unit-conversion vectors are correct. No scope deferral was
found.
