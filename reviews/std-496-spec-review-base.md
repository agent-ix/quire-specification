---
id: SR-496001
title: 'Spec review of quire-specification PR #462 (STD-496, STD-497, STD-510): FR-466 over/finish precedence, dimension-equivalence wire precedence, owner_model phase'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@961ddf00 (PR #462, std-496-qspec) off main 02dfc7dd; FR-322 owner_model bullet and dimension-equivalence paragraph, FR-322-AC-118, FR-322-AC-139, FR-464 Reader order step 6 and Owner-model edge phase, FR-464-AC-16, FR-464-AC-20, FR-466 Decision facts and FR-466-AC-14, FR-466-AC-16, TC-233, TC-280 MU-23 and MU-24, TC-946 OM-03 to OM-11 and O10 to O12, TC-948 DF6, DF6b, DF7d to DF7f, protocol-node-vectors.json, node-identity-vectors.json'
review_set: subset
date: '2026-10-27'
---

## Summary

Ticket: STD-496 (with STD-497 and STD-510 batched in the same PR). PR: quire-specification#462, head 961ddf00, main 02dfc7dd (no movement during review). No ids collide with main or the four other open PRs (#432, #433, #440, #444); a grep over all 737 remote branches found no SR-496xxx, SR-497xxx or SR-510xxx.

Verdict: changes requested. The three rulings are right and mutually consistent with merged #456, #459, #460 and #461. One test-case row contradicts the STD-497 ruling (FND-001), and two smaller consistency gaps should be closed in this PR (FND-002, FND-003).

Checked and clean:
- STD-496 (prose wins). FR-466 prose, FR-466-AC-14, FR-466-AC-16, FR-464-AC-8 and AC-12 and FR-466-AC-3 agree: a direct read of `over` in a choice or repeat `visible` entry or guard is `ill_typed`/`operator-ineligible` before binder scope; the activation binder (in C0, readable in a run slot) stays `wrong_snapshot`/`ineligible-kind`; the finish binder and compensation-template binders are not in the `run` stage list of FR-464 "Readable binders", so `missing_declaration`/`missing-name` with `read` `stage` or `foreign` precedes the decision-fact table. DF6, DF6b, DF7, DF7b, DF7d to DF7f match AC-16, including the two-read precedence (over, then out-of-scope binder, then ineligible binder). No other spec or vector text still lists the finish or protocol-input binder as `wrong_snapshot`.
- STD-497 (schema wins). Verified in schema.json and node-identity-preimage.schema.json: DimensionEquivalencePreimage requires `root_scale`, `PositiveIntegerString` on numerator and denominator, `minItems` 2 on `dimension_node_ids`, and DeclarationOccurrenceRule forbids `declaration` on a node with only an `anchor` occurrence. Wire precedes graph (FR-322), so schema wins is the right call. It does not conflict with FR-322-AC-51 (owner presence; no dimension-equivalence rule) or IR FR-063-AC-5 (IR must align; see list). `-unreduced-scale`, `-descending-ids`, `-same-node-twice`, stale pair and stale scale remain schema-valid, so stay `invalid_semantic_graph`.
- node-identity-vectors.json: a structural JSON diff against main shows exactly four changes, `invalid_mutations[16,19,20,21].expected_code` `invalid_semantic_graph` to `malformed_wire`; no key, patch, retained digest or kind changed. An independent Python JCS SHA-256 recompute reproduces all 23 vector keys on main and on the branch, and every non-stale mutation's retained digest equals its base key while the stale mutations differ.
- STD-510 phase. One section, FR-464 "Owner-model edge phase", is consistent with FR-322 (owner join before any body step), FR-375 (step 7 reads only an edge that passed), FR-464 steps 6 and 7, FR-464-AC-16, and #456 (owner_model is the object-type node of a model a selected declaration owns), #459 (FR-375 catalog), #461 (owner join precedes completeness; model-root-vectors). FR-322 line 280 confirms a dangling local target with a matching dependency passes the graph-shape stage, so OM-09's `missing-name` case is reachable. O10 to O12 have real V7, B6 and M2 rows.
- Ids FR-322-AC-139, FR-464-AC-20, FR-466-AC-16, MU-24, OM-05 to OM-11, O10 to O12, DF6b, DF7d to DF7f are unused on main and on #432, #433, #440, #444. `quire matrix` lists the three new ACs as tagged real table rows next to their neighbours. `quire validate` on spec/**: 115 failing documents on main and on the branch; the diagnostics are identical once line numbers are normalised.
- Tests. `cargo test --test checked_package_v2`: 156 passed, 2 failed on both main and the branch (K15 in protocol_node_case_vectors, and the execution-point-initialization failure in the tc_233 operation vectors). K15 aborts the case-vector test early; an independent script over TC-946, TC-947 and TC-948 (343 id rows, both text columns) finds only K15 without a vector and no text mismatch, so nothing else fails after K15. Case counts [123, 85, 166] match the file. IR `conformance_qspec` at bcbb4af9: 66 passed, 5 failed, 1 ignored against both this branch and main, same five failures (dimension_equivalence time graph, extension TC-459, three timed_model). QSL: see the SR comment for the qsl-semantics vector checks.

IR work implied (IR-1053 and siblings; IR origin/main bcbb4af9, read from source, not changed):
- FR-064-AC-4 and TC-048: split per FR-464-AC-20. Today `protocol_body.rs` (clause loop after membership) emits `ill_typed`/`operator-ineligible` at `/owner_model` for a non-model target; spec says `invalid_model_binding`/`malformed-declaration`. `is_model_declaration_node` tests only tag `model`/`relation`, so a selected `state_machine`, `process`, `record_value_type`, relation node or `model_root` target is not refused by form (the model_root target fails later in `recover` as unselected); spec requires `malformed-declaration` for each. `recover` failure is reported at `/owner_model` with the recover cause; an unselected target must be refused earlier at the target's own `node_id` as `missing_declaration`/`missing-selection` by the owner join (the join in `owner.rs` already runs before body steps; confirm the locus). The dependency comparison (clause `dependencies` against rebuilt list plus `owner_model`) lives in `check_clause_slots`, after the edge check and after `check_clause`/`check_clause_causality`; spec puts `invalid_semantic_graph` at `/dependencies` in the graph stage before any body step. A valid selected second object type must stay admitted. `missing-name` for a local target naming no node already matches.
- Wire: omitted edge is `malformed_wire` at the node (matches `validate_owner_schema`); an extra `owner_model` on another node is reported at `/owner_model` while the spec says "at the node"; an external key or non-NodeRef is checked by `validate_node_id`, whose code and locus need confirming.
- FR-063-AC-5: absent, zero or negative `root_scale`, a one-entry pair and a present `declaration` become `malformed_wire` at the node (IR `validate_dimension_equivalence` currently requires them at graph stage, `positive_integer` on root_scale and `declaration.is_none()`); unreduced scale, one missing dependency, non-self `semantic_type`, owner outside the lock and stale keys stay `invalid_semantic_graph`. Add FR-322-AC-139 to the claimed ACs.
- FR-038-AC-292: drop "finish or protocol-input binder" from the `wrong_snapshot` list, add the protocol activation binder; state that finish and compensation-template binders are `missing-name` first. FR-038-AC-291 already agrees with FR-466-AC-16 for `over`.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-466-AC-14 | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:330 | examined |
| FR-466-AC-16 | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:332 | examined |
| FR-466 Decision facts prose and table | spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:164 | examined |
| FR-464 Owner-model edge phase | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:514 | examined |
| FR-464 Reader order step 6 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:483 | examined |
| FR-464 Readable binders | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:347 | context_only |
| FR-464-AC-16 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:584 | examined |
| FR-464-AC-20 | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:588 | examined |
| FR-322 owner_model bullet | spec/objects/interfaces/FR-322-checked-package-artifact.md:506 | examined |
| FR-322 dimension equivalence paragraph | spec/objects/interfaces/FR-322-checked-package-artifact.md:1386 | examined |
| FR-322-AC-96 | spec/objects/interfaces/FR-322-checked-package-artifact.md:2459 | context_only |
| FR-322-AC-118 | spec/objects/interfaces/FR-322-checked-package-artifact.md:2480 | examined |
| FR-322-AC-139 | spec/objects/interfaces/FR-322-checked-package-artifact.md:2501 | examined |
| TC-280 MU-23 | spec/test-cases/TC-280-model-owned-member-types.md:220 | examined |
| TC-280 MU-24 | spec/test-cases/TC-280-model-owned-member-types.md:221 | examined |
| TC-233 mutation paragraph | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:126 | examined |
| TC-946 OM-03 to OM-11, O10 to O12 | spec/test-cases/TC-946-checked-package-protocol-node-family.md:146 | examined |
| TC-946 A8, A9, C13 | spec/test-cases/TC-946-checked-package-protocol-node-family.md:138 | context_only |
| TC-948 DF6, DF6b, DF7d to DF7f | spec/test-cases/TC-948-checked-package-protocol-control-bodies.md:172 | examined |
| node-identity-vectors.json invalid_mutations | proposals/checked-package-v2/node-identity-vectors.json:139 | examined |
| protocol-node-vectors.json | proposals/checked-package-v2/fixtures/protocol-node-vectors.json:935 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TC-280 MU-23 still lists "with a `declaration`" among the equivalence-node mutations that refuse `invalid_semantic_graph`, while MU-24, FR-322-AC-118, FR-322-AC-139 and the FR-322 paragraph now say a `declaration` member is `malformed_wire`. The two rows contradict each other on the same input. | spec/test-cases/TC-280-model-owned-member-types.md:220 |
| FND-002 | medium | The other schema-refused node-identity mutations keep `expected_code` `invalid_semantic_graph`: `dimension-negative-zero-exponent`, `unit-negative-zero-scale`, `unit-negative-zero-offset` and `model-owner-carries-version` (invalid_mutations 9 to 12). Their `-0` and extra-`version` spellings fail the same closed wire schema (IntegerString, closed ModelOwner), and FR-322-AC-51 already says a `ModelOwner` with `version` is `malformed_wire`. The PR leaves two codes for one rule and special-cases only the `dimension-equivalence-` name prefix in the Rust assertion. | proposals/checked-package-v2/node-identity-vectors.json:149 |
| FND-003 | medium | The same clause now has two codes for one defect shape: a target missing from `dependencies` is `invalid_semantic_graph` at `/dependencies` for `owner_model` (FR-464-AC-20, FR-322-AC-96), but `missing_declaration`/`missing-name` for the body references of TC-946 A8, A9, C13 and FR-466-AC-2 (ten rows), and FR-464 still says "a reference defect (absent from `dependencies`, then wrong form)" without scoping it. The conflict predates the PR (FR-322-AC-96 versus A8, A9, C13) and the PR is right to rule only owner_model, but the unscoped sentence and the PR body's "unresolved observation" leave implementers no rule for the other references. | spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:509 |
| FND-004 | low | TC-946 OM-07 and FR-464-AC-20 name "a source-local declaration (a node with a source `owner`)" as a model-target case, but a `model`-tagged node carrying a `SourceOwner` fails the V2 owner-presence rule (FR-322-AC-51, `malformed_wire`). The case is reachable only as a non-model node with a declaration (a declared record or function); the wording makes the row look unreachable. AC-20 also calls the package `model_root` "a selected declaration's node". | spec/test-cases/TC-946-checked-package-protocol-node-family.md:150 |

## Verdict

Changes requested. FND-001 is a plain contradiction in a test-case row and must be fixed before merge (delete "with a `declaration`" from MU-23; MU-24 already carries it). FND-002: fix here, it is four string values in node-identity-vectors.json plus replacing the name-prefix condition in the Rust assertion with `refused_by == "schema"`; neither IR nor QSL reads `invalid_mutations[].expected_code`, so the change is local; if the author prefers to defer, AC-139 must state that the `-0` and `ModelOwner` `version` schema refusals are also `malformed_wire`. FND-003: not a blocker to ruling, but add the scoping sentence to FR-464 ("Within one node") and file a follow-up for A8, A9, C13, FR-466-AC-2 and the other eight rows. FND-004 is a wording fix.

Judgement on the three asked questions: (b) schema wins is correct and does not conflict with FR-063-AC-5 once IR aligns, nor with FR-322-AC-51; (c) the A8/A9/C13 versus FR-322-AC-96 contradiction is pre-existing and need not be settled in this PR beyond FND-003; (d) the author's claim that nothing else fails after K15 is confirmed.
