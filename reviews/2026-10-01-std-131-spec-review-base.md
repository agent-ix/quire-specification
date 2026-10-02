---
id: SR-800
title: "Base review of STD-131/STD-132: temporal properties over every behaviour and fairness"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ca8033614b06c21ea8362f2ea1b2fa38ad910739; git diff origin/main...HEAD (PR #170): spec/functional/temporal/FR-361..FR-369, spec/objects/interfaces/FR-370, amended FR-048, FR-090, FR-091, FR-092, FR-114, FR-161, FR-181, FR-250, FR-254, FR-255, FR-272, FR-290, FR-322, FR-331, FR-360, AD-012, shared-grammar.md, native-diagnostics.md, US-011, TC-200, TC-238, TC-239, TC-290..TC-305, spec/temporal/tests.md, spec/complete-v1/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-361
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-362
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-363
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-364
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-365
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-366
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-367
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-368
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-369
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-370
    type: reviews
---
# SR-800: Base review of STD-131/STD-132: temporal properties over every behaviour and fairness

## Summary

Ticket: STD-131 (with STD-132). PR agent-ix/quire-specification#170, draft.

Checked the PR diff against QSL ADR-018 (branch spec/366-temporal-properties,
head 404a5a88, matching origin) and ADR-019 (qsl-365-strong-fairness), the
wave B/C owner rulings, ID format, AC testability, AC-to-TC coverage and the
test matrices. Every new and amended AC carries a TC and a test-matrix row,
except FR-250-AC-6 (FND-011). The new files pass `quire validate`; the failures
on amended object files and tests.md column headers are the repo-wide failures
main already has. Ticket ids appear only in References. No new pin, ledger,
provenance record or compat path is added; the `[a,*]` spelling is removed
outright, and FR-161 drops its "exact tool pins" and "provenance" wording.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-365 ranks an undefined state against the stem of the first passing component by prefix length. ADR-018 UE-4 runs a TP-4 item's second phase only when the first phase found no undefined evaluation, so any undefined state up to depth `k` wins over a liveness lasso. FR-365-AC-6 makes the same comparison. A conforming QSL EN-1 returns a different canonical counterexample than FR-365 requires. | FR-365:104-111, FR-365:122 |
| FND-002 | medium | The counterexample `kind` is the bare value `undefined`. ADR-018 UE-2, DL-4 and QS-8 make the kind `UndefinedEvaluation{where, cause}`, carried in the counterexample. FR-364 keeps `where` and `cause` on the item's cause, and replay step 6 reads them from there. So the counterexample does not replay on its own, and the wire disagrees with the ADR. | FR-364:59, FR-364:108-113 |
| FND-003 | medium | A read through the `over` binding of an object that is absent at a reachable position settles the item `failed`, execution `failed`. Under the QSL-366 ruling a claim that is not defined on an admitted behaviour does not hold: refuted, cause UndefinedEvaluation. TC-305 reads the same absent object through `lookup ... absent undefined` and gets `refuted`. Two spellings of one absent read give `failed` and `refuted`. ADR-018 QS-5 left this reading to QSpec. Either map it to undefined (refuted) or state why a dangling read is a refusal and not undefined. | FR-361:119-127, FR-361:138, TC-290 step 6 |
| FND-004 | medium | The search depth `k` is called "a caller-configurable limit of the request" with a published default, and FR-368 lists `max_depth` among the request `limits`. The owner ruling says depth is never a limit kind, and ADR-018 §1 says depth is a method parameter (the search horizon), not a limit. State `k` as the search horizon the request supplies, separate from the resource limits. | FR-363:137-140, FR-368:58-61 |
| FND-005 | medium | TC-298 step 3 expects `max_automaton_states` 50 to stop the check of `R`. Under ADR-018 IV-3's counter construction the automaton for the negation of `R` has 12 states (IV-5), so a limit of 50 is never reached. FR-367 does not fix the construction, so the automaton-state count is engine-dependent, and the vector cannot fail or pass reliably. Pick a limit below the minimum automaton size for `R` (for example 5), or a formula whose size is fixed by the construction FR-367 states. | TC-298:40, TC-298:60-63 |
| FND-006 | medium | FR-370 requires `member` `null` and six arguments on a `quire.op.temporal.clause` body, and FR-370-AC-2 says both V2 schemas enumerate the `quire.op.temporal.*` identities and member kinds. The published `proposals/checked-package-v2/schema.json` lists only `quire.op.temporal.clause`. The published clause fixtures carry a `profile_operator` member `eventually` with one argument. TC-233, which tests amended FR-322-AC-22 against the published clause package, and TC-303 contradict the published artifacts. No AC, TC or reference says who updates them. | FR-370:33-38, FR-370:132, FR-322:684, proposals/checked-package-v2/fixtures/positive-clause-operations.json:18,54 |
| FND-007 | medium | `undecided-successor` is defined as "a contract conjunction evaluated undecided during expansion", but no QSpec requirement defines an undecided contract evaluation (it is QSL FR-120's `ContractUndetermined`). TC-294 step 3 produces it with an unguarded read of an optional field. That is an undefined value, so the vector blurs the line between this cause and the undefined-evaluation rule. Define when a contract conjunction is undecided, and build the vector from that definition. | FR-363:96, FR-363:156, TC-294:26-28 |
| FND-008 | low | The FR-360 links in FR-161 and FR-114 were rewritten from `../../objects/...` to `../objects/...`. That resolves to `spec/functional/objects/`, which does not exist, so the links are broken. | FR-161:82, FR-161:133, FR-114:29 |
| FND-009 | low | "Classify each claim into one form", but `always holds(I)` matches both reachable-state invariant and safety. No precedence is stated. TC-298 step 2 expects reachable-state invariant. | FR-363:66-74 |
| FND-010 | low | FR-361-AC-3 asserts the profile contrast with "the same `eventually` claim", but TC-290 step 2 uses `always[0,4]`. | FR-361:135, TC-290:34, TC-290:54-58 |
| FND-011 | low | FR-250-AC-6 changes meaning (interval operators are now admitted under infinite-trace), but its Verification is a bare `Test` with no TC and no matrix row. TC-238 and TC-298 already exercise the behaviour, so trace them. | FR-250:105 |
| FND-012 | low | The AD-012 Analysis row still says `unsupported` "until a liveness backend registers". FR-161-AC-7 replaces this with the FR-290/FR-368 candidate settlement. The row is stale, and it reads as a schedule. | AD-012:63 |
| FND-013 | low | The touched FR-331 manifest line keeps "pinned tool identity" and the tool lock, which is a pin and a version check under the rulings. The rewrite also dropped "and", so it now reads "pinned tool identity, where the backend declares one, its obligation-form set", which is ambiguous. | FR-331:29, FR-331:41 |

## Verdict

**Not mergeable as it stands.** FND-001 contradicts the ADR the PR implements,
and FND-002 to FND-007 are real gaps. The core of the PR follows ADR-018 and
ADR-019 closely and is well traced: the model subject, weak and strong fairness
and their lasso rules, the verdict table, the SCC refinement and greedy-walk
loop, deadlock freedom, interval operators under infinite-trace, the
strong-fairness hint, and undefined → refuted in FR-363. The rulings hold
throughout. The automaton-state limit has a published default and a named
raise path, and no fixed cap is added.

Merge note: the hand-resynced lock-file digests in
`proposals/quire-v1/definitions/complete-value-lock.json`,
`complete-model-lock.json` and `value-complete-rules.json` are slated for
deletion by #174. They are not reviewed here as a finding.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-014 | medium | The fix for FND-013 removes the tool lock and the pinned tool identity from FR-331 (manifest, request, identity, AC-7), but other files still define or test them. FR-290:263 says "The pinned tool identity is the FR-331 request's tool lock". interface_013:19 gives `execute` the input "exact tool lock". TC-271's procedure builds descriptors with "a pinned tool identity", and its vector TA-05 still expects a request with a mismatched tool lock to refuse before consumption, a refusal FR-331 no longer defines. FR-331's `results` still compares "expected and actual" tool identity with nothing left to supply the expected one. Finish the removal across these files, or the spec and the TC contradict FR-331. | FR-290:263, interface_013-backend-provider.md:19, TC-271:26, TC-271:65, FR-331:35 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-015 | medium | TC-302 expects a request that differs from `Q` only in its limits (`max_automaton_states` 50) to have a different request identity, citing FR-368-AC-1 and FR-331-AC-11. 30564d0 narrowed FR-331-AC-11 to `subject`, `item_kind` or `fairness`, FR-368-AC-1 never named limits, and FR-331's identity paragraph names "claim/domain/bounds/options" with neither `limits` nor `max_depth`. No AC states whether `limits` or `max_depth` enter the request identity, so the TC-302 expectation has no owning AC and two implementers can differ. State it in FR-368 and FR-331 and trace TC-302 to that AC. | TC-302:28-29, TC-302:37, FR-331:41, FR-331:59, FR-368:124 |
| FND-016 | low | FR-370-AC-2 says both V2 schemas enumerate "the `temporal` family". The schemas enumerate no operand families at all; families live only in the operation catalog, as FR-370's own "Published artifacts and the operation catalog" section says. Move the family to the catalog clause of the AC. | FR-370:136 |
| FND-017 | low | FR-370 names the catalog `quire.checked-operation-catalog-v1`; FR-322, interface_004, FR-149 and the V2 README name it `quire.checked-operation-catalog/v1`. The paragraph also ends "These catalog entries are an overlap item with quire-verification-contracts", a coordination note rather than a statement of what is. | FR-370:152, FR-370:157 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-018 | medium | 9b9114e makes FR-370 state that the catalog holds the `temporal` family "a member of the `any_term` group" (the qvc#11 catalog does). FR-322's group table, which defines each group's families, still lists `any_term` as "`any_value` plus `function`, `population`, `aggregate`, `binder`, `clause`", and FR-322's term-family rule names no `temporal` family. The two FRs now disagree on `any_term`'s membership. Add `temporal` to FR-322's `any_term` row and its term-family rule, citing FR-370. | FR-322:565, FR-322:557-560, FR-370:153-154 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 7405e1e |
| FND-002 | fixed | 7405e1e |
| FND-003 | fixed | 7405e1e |
| FND-004 | fixed | 7405e1e |
| FND-005 | fixed | 7405e1e |
| FND-006 | still-open | 7405e1e touches no FR-370, FR-322, schema.json or fixture file. FR-370-AC-2 and the amended FR-322-AC-22 still contradict the published V2 schema and clause fixtures, and no AC, TC or reference says who aligns them. |
| FND-007 | fixed | 7405e1e |
| FND-008 | fixed | 7405e1e |
| FND-009 | fixed | 7405e1e |
| FND-010 | fixed | 7405e1e |
| FND-011 | fixed | 7405e1e |
| FND-012 | fixed | 7405e1e |
| FND-013 | fixed | 7405e1e |
| FND-006 | fixed | 30564d0 |
| FND-013 | deferred | 30564d0 reverted 7405e1e's FR-331 change: FR-331's tool-lock and pinned-tool-identity text is now byte-identical to main, so this PR adds no pin. Removal is owned by quire-specification#175 (STD-137/139), which edits FR-331, FR-290, interface_013 and TC-271. The ambiguous "and" is gone with the revert. |
| FND-014 | fixed | 30564d0 |
| FND-015 | fixed | 9b9114e |
| FND-016 | fixed | 9b9114e |
| FND-017 | fixed | 9b9114e |
