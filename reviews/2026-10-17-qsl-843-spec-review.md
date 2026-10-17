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
