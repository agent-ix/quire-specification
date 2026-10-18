---
id: SR-12101
title: "QSL-843 QSpec PR #271 spec review: checked-package body contract for the protocol node forms"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@4f3e2b8c0cfdb034f9063692c757e2bbac3c9ab2; spec/objects/interfaces/FR-464, FR-465, FR-466, FR-322, FR-326; spec/test-cases/TC-946, TC-947, TC-948; spec/spec.md"
review_set: subset
---

## Summary

Ticket: QSL-843. PR quire-specification#271 (branch protocol-node-body-contract), reviewed at 4f3e2b8 against origin/main a08dc63, by reading only. The only build was `quire validate`. Method: spec-review, with the integrity, object-review and failure-domain lenses applied to the diff. Read for consistency: FR-052, FR-053, FR-054, FR-170, FR-216, FR-340, FR-341, FR-370, FR-428, FR-429, FR-451, FR-272 (the cause catalog), shared-grammar.md and the checked-package-v2 schema and fixtures. Also read: QSL FR-218, FR-308 and FR-381 at the qsl-protocol-spa head c5d76970d, together with the SP-A final cause pairs the planner accepted, as relayed by the coordinator.

Re-derived by hand: the binder levels of B1 (view 0, opened 1, order 2, sent 3, got 4, charge 5, paid 6), the causal cycles of R2, R4, R5/AW8, S1 and CP9, and about 70 vector rows across TC-946/947/948. No new file publishes a node-key digest. The key claims (K1 to K4, CH13, CH17, QU2, L4 to L7, CP2) were checked as preimage statements against the FR-322 application-node preimage.

Every cause pair used exists in FR-272: `ill_typed`/`operator-ineligible`, `invalid_model_binding`/`malformed-declaration` and `wrong-export`, `missing_declaration`/`missing-name` and `missing-selection`, `ambiguous_declaration`/`ambiguous-name`, `invalid_package`/`definition-cycle`, `invalid-value`, the `operation-*` causes and `stale-node-key`, `unsupported_construct`/`declaration-form`, and `wrong_snapshot`/`wrong-observation`. The package codes `invalid_semantic_graph` and `malformed_wire` come from FR-322.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-464 says the published V2 schemas fix each operator's layout and that the published packages encode the clause body. Neither is true at this head: the published `protocol_clause` node applies `sequence` with no arguments, and FR-322-AC-66's admitted worked example (P, 12432a66…) is refused by the new rules. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:395-401; proposals/checked-package-v2/fixtures/positive-all-families.json; positive-clause-operations.json; FR-322:179-200 |
| FND-002 | high | FR-464's list-shape rules are wrong for lists whose meaning is a set. (a) "A node listed twice ... in two lists of one protocol refuses invalid_semantic_graph" covers `visible` and `contracts`, so TC-946's own base package refuses: `Outcome` and `Poll` both list the one `eastRes.ok` node. (b) Authored order is part of the key for roles, channels, queues, relationships, requirements, compensations, contracts and visible, so one protocol has many keys (K3). The generated `queues` list has no source order at all. FR-340 and FR-451 sort the same kind of list canonically (FR-322-AC-10). | FR-464 "Term shapes", "Clause body"; TC-946 base, K3; FR-340 "Canonical ordering"; FR-451 "Canonical order" |
| FND-003 | high | The handling of `over` and of captures contradicts SP-A. SP-A's final rule treats `over` as model state everywhere: a read of it in a slot that bars model state is `ill_typed`/`operator-ineligible`, which wins over the stage table, and captures are readable by guards without listing. FR-464, FR-465 and FR-466 instead refuse `over` with `missing_declaration`/`missing-name` (stage) or `wrong_snapshot`/ineligible-kind, and refuse captures as ineligible binders. TC-948 BS10 admits guards reading `order` and `view`, and CE6 admits an empty `visible`, which contradicts DF7 and DF8. | FR-464 "Readable binders", FR-464-AC-8; FR-465-AC-3; FR-466 "Decision facts", "Compensation scope", FR-466-AC-12; TC-946 B20; TC-948 BS10, CE6, DF7, CP29, CP31-CP33 |
| FND-004 | high | The reader order is defined twice and the two orders conflict. The member-node sections (FR-465 role check 4, channel check 3, finish and terminal, FR-466 per-operator checks 7 and 8, compensation check 5) run scope and decision-fact checks in FR-464 step 4. FR-464 step 6 runs binder scope after the cycle and progress checks. Expected vector O5 (cycle before B9) holds only under step 6. | FR-464 "Reader order" 4 and 6; FR-466 "Per-operator checks" 7-8; TC-946 O5 |
| FND-005 | high | The channel and queue contract contradicts the FRs it names as owners. FR-054 and FR-216 require an inclusive `[min,max]` delivery interval with duplication derived from the maximum, while FR-465 carries four policy texts. FR-170 makes `loss` apply a declared head/tail/new loss selector, and FR-465 carries none. The contradiction is left standing. | FR-465 "Channel", "Queue"; FR-216 Properties, AC-6; FR-054-AC-1; FR-170 "Queue transition system"; shared-grammar.md:183-185 vs 494-502 |
| FND-006 | high | The compensation bounds and commit rule contradict SP-A's final pairs. (a) `attempts.maximum: 0` and an inverted `within` (compensation and `await`) refuse `unsupported_construct`/`declaration-form`, where SP-A uses `ill_typed`/`type-mismatch` (member/expected/actual). (b) The commit check refuses any commit with a causal path to the forward effect. SP-A refuses only a commit that dominates the effect, that is, one on every path to it, and admits a commit in one case of a choice. (c) `commit-precedes-effect` does not retain the selected protocol profile, which SP-A's refusal does (the FR-052 precedent). | FR-466 "Per-operator checks" 6, "Compensation" checks 3-4, FR-466-AC-11; TC-948 CP9, CP11, CP13, AW9 |

## Verdict

FIX. The six findings above are major. The rest of the contract is sound:

- The 22-operator catalog and each layout are exact, and so are the fixed declaration segments (K6).
- The containment tree rules are exact, and so are the derived edge kinds, the acyclicity rule with its back-edge exemption, and progress. Progress matches FR-052 and SP-A, including the nested-repeat case.
- The seven compensation stages match SP-A's FR-381 table row by row for binders and captures, including the `forward`, `foreign` and `stage` reasons.
- The FIFO key is binders-only, as is the activation guard.
- An unordered commit is admitted (CP10).
- The unobservable-fact pair `wrong_snapshot`/`wrong-observation` and the non-relationship target pair `invalid_model_binding`/`wrong-export` (C11, FR-466 check 5) match SP-A.
- The 'Equality-stable types' paragraph agrees with FR-322's structural-eq, event-equality and IEEE rules.

The six new docs validate with exit 0, as does spec.md. FR-322 and FR-326 fail only the existing `value_object` frontmatter check (`id`/`fields`), which this diff does not touch.

The ids FR-464 to FR-466 and TC-946 to TC-948 are free on main and in every open QSpec PR (#270, #267, #207, #200, #59).

The FR-322 hunks of #271 (frontmatter, lines 138-144, 244-251, the 1338 paragraph) do not overlap the hunks of #270 (845, 874-889, 1108, and the AC-74 row after 1521). Expect a clean textual rebase, with no semantic conflict.

The QSpec text is in the present tense and names no QSL internal and no ticket id.

On the author's open questions:

- FR-216 interval and loss selector: FR-465 follows the current grammar production, which is right, but it leaves FR-054, FR-216 and FR-170 contradicting it (FND-005).
- `scheduling adversarial` as a single-valued optional text: acceptable.
- A terminal condition that reads C0: acceptable against FR-429 (the model observation and captures), with `over` read as model state. It is medium-open for the activation binder under `on each` (M6). TC-947 TM2 reads `k`, which is not in C0.
- SP-A's repeat example reading `view`: SP-A now settles it by classing `over` as model state, and this PR is not aligned with that (FND-003).

Mediums and lows are recorded, not blocking:

- M1: FR-326 Properties still say `control_graph` is "authored acyclic causal edges", while FR-464 derives the edges. `bounds` is not mapped in "Carriage".
- M2: A binder node is shared by every place with equal name, level and type, but the text says "introduced at one place". Eligibility, `owner`, `foreign` and level checks need the introducing construct, which is ambiguous when a node is shared (for example, a receive and a send binder of one name and type in two choice cases).
- M3: Binder-type defects use `ill_typed`/`operator-ineligible`. SP-A (FR-218-AC-12, FR-381-AC-5, AC-9) and the FR-322 reader precedent use `ill_typed`/`type-mismatch` for the same cases, and SP-A uses `ill_typed`/`non-boolean-root` for a non-Boolean guard, relation or predicate. The places: FR-465 role check 3 and channel check 3, FR-466 per-operator check 5 and compensation check 2, and vectors RL12, CH5, E2, CP4, CP14, CP15, CE4, RP6 and FN2.
- M4: TC-946 V16 cites a sibling `await` that the base package does not have. TC-948 CP10 needs a restructured base that the vector does not state.
- M5: The FR-464 placement text says a `protocol_reference` carrier holding a protocol application refuses `operator-ineligible`. The schema refuses it first (V6, `malformed_wire`), so that text is unreachable.
- M6: The terminal condition reads the activation binder and the captures, but under `on each` a terminal state can have no live instance. Say what they denote there.
- M7: A required temporal clause that holds a `protocol_reference` to its own protocol closes a dependency cycle (clause, then temporal clause, then carrier, then clause), which FR-322 refuses. The PR does not state this.
- M8: FR-464 does not place the binders-only refusal relative to the scope and decision-fact checks (B14).
- L1: FR-464 cites "FR-322's phrase 'the FR-052 bounded control constructs'", but this PR removes that phrase.
- L2: TC-948's Description scope stops at FR-466-AC-13. It should include AC-14.
- L3: In the FR-466 Compensation section, `P` means both the protocol name and the protocol captures.
- L4: The finish binder's type is not stated or checked.
- L5: "No edge is derived from ... array order" sits next to sequence edges that come from the order of `children`.
- L6: Roles have `max: 0` → `invalid-value`.

## Provenance

The scoped files were read in full at 4f3e2b8. The comparison reads used origin/main a08dc63 and the qsl-protocol-spa head c5d76970d. SP-A's final pairs were taken as relayed by the coordinator. Tools: quire 0.36.2 (engine 0.50.2), quoin 0.28.3. Read-only: nothing was edited, and nothing was built except `quire validate`.

## Dispositions

| ID | Disposition |
| --- | --- |
| FND-001 | Fixed. FR-464 "Published artifacts" states what is true: no schema or fixture changed, the published `protocol_clause` stub predates the contract, FR-322-AC-66's example uses it only as a carrier target (note added to the example), regeneration is IR follow-up. Layout violations are `malformed_wire` reader refusals throughout. |
| FND-002 | Fixed. Duplicate rule scoped to declaration lists; sets (roles, relationships, channels, queues, requirements, compensations, visible, contracts, related, cases) are canonically sorted, `invalid_semantic_graph` when out of order; queues sort by channel digest. |
| FND-003 | Fixed. `over` is model state (`ill_typed`/`operator-ineligible`, precedes the stage table); captures readable without listing; compensation stages follow SP-A; BS10, CE6, DF7 rewritten. |
| FND-004 | Fixed. One reader order in FR-464; member sections defer slot checks to step 6. |
| FND-005 | Fixed. FR-054, FR-216, FR-170, FR-426, TC-054, TC-378, FR-222, FR-224, the shared-grammar channel production and the choreography surface now state the delivery policy, capacity and overflow; the loss selector is replaced by "the new message is lost". |
| FND-006 | Fixed. `attempts 0` and an inverted `within` are `ill_typed`/`type-mismatch`; the commit check is dominance and retains the profile. |
| M1-M5, M7, M8, L1-L6 | Fixed. |
| M6 | Fixed: the terminal condition reads no protocol binder or capture. |

## New findings (disposition pass 1)

Reviewed at 9dcf158068dc91c441598d6eb06b8ee74e050f26 (delta `git diff 4f3e2b8..9dcf158`), read-only. The only build was `quire validate`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | high | The `queues` argument has two different canonical orders. The "Term shapes" set shape requires its members to be in strictly ascending node-id digest order. The clause-body row says the order is "the ascending digest order of the channel nodes they name". A queue's digest is not its channel's digest, so a queues list in one order is refused as `invalid_semantic_graph` under the other order. As a result, two conforming readers disagree on admission and on the key. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md "Term shapes" (set), "Clause body" row 7 |
| FND-008 | high | The `terminal` condition now contradicts FR-429 and contradicts itself. (a) FR-429 states that `terminal when { P }` reads "the model observation and the protocol's captures", but FR-465 refuses a capture read (TM10, `missing-name`). (b) FR-464's "Slots that bar model state" list includes "the terminal condition's binders" and refuses `over` and model-state operations there, while the stage table, FR-465 and TM11 admit the model observation and `view`. | FR-465 "Terminal"; FR-464 "Readable binders" table row `terminal` and "Slots that bar model state"; TC-947 TM10, TM11; FR-429 "The `terminal` member" |

## Reviewer dispositions

Round 1, at 9dcf158. The author's table above records the author's claims. The rows below are the reviewer's outcomes, checked against the delta.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | "Published artifacts" now describes the head truthfully. But it adds a transitional case, which the planner's ruling rules out: "until then the example and the published packages are exempt from the clause body rules", together with "until the schemas encode them". That exemption contradicts TC-233, which says "the full reader admits every published positive fixture", and FR-322-AC-66, which says the worked package is admitted, because a conforming reader refuses the stub P. FR-322 also gains history text ("published before FR-464 fixed the clause body"), a plan ("follow-up work for the IR owner"), and keeps the stub digests 12432a66… and e761cf…. To fix: state only the target, drop the exemption, history, plan and stale digests, and file the fixture regeneration as a ticket. |
| FND-002 | fixed 9dcf158 | Sets are strictly ascending by digest, cases are sorted by label, `related` uses sorted records, and a misordered or repeated set is `invalid_semantic_graph`, per FR-322-AC-10, FR-340 and FR-451. Duplicate listing is scoped to declarations, and CE11 admits the shared `visible` node. K1, K3 and K13 are consistent. The `queues` order defect is FND-007. Medium M9: `branches` stays an ordered list although FR-052 gives branch order no meaning. |
| FND-003 | fixed 9dcf158 | `over` is model state: `ill_typed`/`operator-ineligible` before the stage table. Captures can be read without being listed, and the stage table now uses `trigger-guard` with `over` readable in protocol, registration and activation captures. This matches SP-A FR-381 and FR-218 on quire-spec-language main a612f3fc4 row for row. The B20–B26, BS10, BS11, CE6, CE6b, DF7, DF7b, DF7c, CP29 and CP31–CP33 vectors agree. |
| FND-004 | fixed 9dcf158 | FR-464 now has one reader order. The member sections defer binder scope, model-state and decision-fact checks to step 6, which orders them cycle, then progress, then the slots, and within a slot `over`/model state, then scope, then decision facts. O5 and O7–O9 agree. |
| FND-005 | fixed 9dcf158 | The interval and loss selector are replaced consistently in FR-054, FR-216, FR-170, FR-426, TC-054, TC-378, the shared-grammar production and the choreography surface, so no contradiction remains on the premise. The loss definition the author chose (the new message is lost, the queue is unchanged, and the binder records the loss) is acceptable as the single consistent rule. It differs from `reject` only in what the sender observes, and it narrows the head/tail/new choice that FR-170 had, so the planner should confirm it. Residual mediums: M10 and M12. |
| FND-006 | fixed 9dcf158 | `attempts 0` and an inverted `within` (compensation and `await`) are `ill_typed`/`type-mismatch` with member, expected and actual. The commit check is now dominance and retains the profile. CP9, CP10b, CP11 and CP13 agree, and so does SP-A FR-381 on main. Medium M13 records a dominance-under-`parallel` concern that applies to both specs. |
| FND-007 | still-open | New in this round, introduced by the FND-002 fix. |
| FND-008 | still-open | New in this round, introduced by the M6 fix. |

Mediums and lows from the first pass, at 9dcf158:

- M1 fixed (FR-326 now says the edges are derived and maps `bounds`).
- M2 fixed (Sharing paragraph).
- M3 still-open, residual: FR-466 check 5 makes a `related` end of the wrong type `ill_typed`/`type-mismatch` at the end, but FR-466-AC-9 and TC-948 E16 still say `ill_typed`/`operator-ineligible` at the control. The same situation has two different pairs.
- M4 fixed (V16 and CP10 were restated). Low: CP10's commit, "on a new branch of `Shipments` with no edge to `Applied`", follows `Applied` in `Main`, so the vector tests the "follows" case and not the "unordered" one.
- M5 fixed.
- M6 led to FND-008.
- M7 and M8 fixed.
- L1–L6 fixed.

New mediums:

- M9: `branches` is an ordered list with no meaning to its order (FR-052), so reordering two branches re-keys the same protocol. Sort the branches by label, as the cases are.
- M10: FR-214, FR-215, FR-222 and FR-224 still say "delivery cardinality and duplication premises". FR-222 and FR-224 quote FR-054 as binding "delivery cardinality", a phrase FR-054 no longer contains.
- M11: Negative bounds and capacities refuse `malformed_wire` here (QU7, FR-466 check 6), whereas FR-370 refuses a negative bound as `invalid_package`/`invalid-value` at strict wire validation.
- M12: `at-least-once` prohibits loss (FR-054), but an `at-least-once` channel with overflow `loss` loses the new message at capacity. Nothing states whether that combination is refused.
- M13: Path dominance treats a `parallel` as alternative paths. A commit in branch A of a `join all` parallel ahead of the effect is taken on every execution, yet it is admitted. This is the same in SP-A FR-381, so it is the planner's call.
- Low: the step 4 local-check classes (references, shapes, types, bounds) do not name FR-466's awaited-event, label and join checks.

Validation at 9dcf158:

- `quire validate` exits 0 on FR-464/465/466, TC-946/947/948, spec.md, FR-054, FR-170, FR-426, TC-054, TC-378 and this review file.
- FR-216, FR-222 and FR-224 exit 3 with the existing `value_object`/`entity` frontmatter (`id`/`fields`) failure, which is the same baseline class as FR-322 and FR-326 and not introduced by the delta.
- Every layout violation as `malformed_wire` is consistent with FR-322's reader vocabulary (FR-322: "refuses a body outside the grammar as `malformed_wire`"), apart from M11.

Verdict for the disposition pass: FIX. FND-001 is still open, and two new high findings (FND-007, FND-008) remain. Each is a small text fix.

## Author dispositions, round 2

| ID | Disposition |
| --- | --- |
| FND-001 | Fixed. FR-464 "Published artifacts" states only the target. FR-322 drops the stub digests, the history and the plan, and the worked example and FR-322-AC-66 are stated over a conforming clause node; TC-931 setup uses an FR-464 clause body. |
| FND-007 | Fixed. `queues` is ascending by each queue's own node-id digest like every other set. |
| FND-008 | Fixed. The terminal condition reads `over` (the model observation) and the protocol captures and no protocol binder; the bar list no longer names it; TM10 admits a capture read, TM12 refuses the activation binder. |
| M3 | Fixed. A `related` end of the wrong type is `ill_typed`/`type-mismatch` (FR-466-AC-9, E16). |
| M4 | CP10 restated: `Applied` and the commit sit in different branches of a new `parallel`. |
| M10 | FR-214, FR-215, FR-222 and FR-224 name the delivery policy. |
| M11 | Aligned with FR-370: a negative bound or capacity is `invalid_package`/`invalid-value` at strict wire validation (QU7, FR-466 check 6). |
| M12 | Stated: an `at-least-once` channel with overflow `loss` is admitted; overflow governs a full queue, delivery policy governs transport loss (QU11). |
| M9, M13 | Left for the planner. |

## New findings (disposition pass 2)

Reviewed at 3db1cb010f2e2d0a40c9e3bf5fbeb7d11ca3f1d1. The author's delta is 18e07bb and 502b6fa over 9dcf158, plus the merge of main with #270. Read-only; the only build was `quire validate`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | high | Vector CP9d cannot hold under FR-466's own rules. It says "`commit` names a `commit` control in both cases" of a `choice`, and expects `commit-precedes-effect`. The containment rule holds a control in exactly one position, so one commit node cannot sit in both cases; a node held twice is refused as `invalid_semantic_graph`. With two distinct commit nodes C1 and C2, `taken(choice)` is {C1} ∩ {C2} = ∅, so the template's `commit` is admitted. Either way the expected refusal never occurs. Restate CP9d with a result the rule yields: the held-twice refusal, or admitted for two distinct commits. If the intent is "every case commits", that needs a different rule. | spec/test-cases/TC-948-checked-package-protocol-control-bodies.md CP9d; FR-466 "Compensation" check 4 *Dominance*; FR-464 "Membership and containment" |

## Reviewer dispositions, round 2

Round 2, at 3db1cb0. The latest outcome is stated for every finding whose last reviewer row was still-open.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 18e07bb | FR-322 and FR-464 now state only the target. The FR-322 worked example and AC-66 are written over a conforming clause node, and TC-931 sets up an FR-464 clause body. The terms exempt, "until the schemas", "published before FR-464", 12432a66 and e761cf no longer appear in FR-322, FR-464–466, TC-233, TC-931 or TC-946–948. The one remaining "published before" at FR-322:1490 is the older key-counter sentence and is unrelated. Under the planner's target-state ruling, the published fixtures and schema that do not yet carry the layouts are an implementation gap. That gap needs a tracking ticket (fixtures plus schema), because TC-233 ("the full reader admits every published positive fixture") fails against the current fixtures until they are regenerated. |
| FND-007 | fixed 18e07bb | Clause-body row 7 now says the queues are ascending by each queue's own node-id digest, like every other set. Nothing else orders them. |
| FND-008 | fixed 18e07bb | Terminal reads `over`, the protocol captures and constants, and no protocol binder. This is consistent across the FR-464 stage table, FR-465 "Terminal", TM10 (capture, admitted), TM11 (`view`, admitted), TM12 (activation binder, `missing-name`) and FR-429. The FR-464 list of slots that bar model state no longer names the terminal. |
| FND-009 | still-open | New in this round, introduced by 502b6fa. |

Mediums at 3db1cb0:

- M3 fixed, 18e07bb (FR-466-AC-9 and E16 are now `type-mismatch`).
- M4 fixed (CP10 places the effect and the commit in sibling branches).
- M10 fixed (FR-214, FR-215, FR-222 and FR-224 now name the delivery policy). Low: the FR-215 table keeps a separate "Duplication premise" row, which FR-216-AC-6 ("no separate duplication premise") does not allow.
- M11 fixed: a negative bound is `invalid_package`/`invalid-value` (QU7, FR-466 check 6), as in FR-370.
- M12 fixed: QU11 admits it, and FR-465 states why.
- M13 fixed, 502b6fa: `taken(n)` makes a `join all` the union over its branches. I recomputed CP9 (refuse), CP9b (refuse: the union holds C), CP9c (`join any`, admitted: the intersection is ∅), CP9e and CP10 (siblings, admitted: no ancestor sequence holds C before E's child) and CP10b (one case, admitted: ∅). All are correct; CP9d is FND-009.
- M9 still-open: the rationale given is "the `joined` list maps to branches by position". But FR-466 check 4 matches `joined` texts to branch labels, and position only fixes the order of `joined`. Reordering two branches still re-keys a protocol with the same meaning. Branches could be sorted by label, as cases are. Medium, left with the planner.

Loss selector, 502b6fa: the production `overflow-policy = 'reject' | 'block' | 'loss', loss-selector` with `loss-selector = 'head' | 'tail' | 'new'` agrees with FR-170, FR-426, FR-054, the choreography surface and FR-465. FR-465 adds an optional fourth argument `loss`, present exactly when `overflow` is `loss`, and that argument enters the node key. I recomputed QU2: 2 capacities × (reject, block, loss head, loss tail, loss new) = 10 keys, which is right. QU6a and QU6b refuse `malformed_wire`. CH-08, CH-08h and CH-08t are consistent.

Two notes on the loss selector:

- Low: FR-170 "loses one message ... The queue is otherwise unchanged" does not say whether the arriving message is appended under `head` and `tail`. CH-08h and CH-08t say it is. State it in FR-170 and FR-465.
- For the planner: QSL main FR-218-AC-13 still writes `overflow loss` with no selector, so the QSL side must adopt the selector.

New medium: under `activation on each`, a terminal state can have no live instance. The text gives that as the reason no instance binder is readable, but the protocol captures read there are per-instance values too. FR-429 states it this way, so it is FR-429's call.

Validation at 3db1cb0: `quire validate` exits 0 on FR-464, FR-465, FR-466, TC-946, TC-947, TC-948, spec.md, TC-233, TC-378, TC-931, FR-054, FR-170 and FR-426. FR-322, FR-214 and FR-215 exit 3 with the existing `value_object`/`entity` frontmatter (`id`/`fields`) failure; that baseline is unchanged.

Verdict for disposition pass 2: FIX on one high finding, FND-009, which is a single vector row in TC-948. Every original finding FND-001 to FND-008 is fixed. Once CP9d is restated the PR passes. M9 and the QSL selector adoption go to the planner.
