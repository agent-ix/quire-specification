---
id: SR-606
title: "Base review of the FR-290 capability-vocabulary widening"
type: SpecReview
analysis: base
scope: "quire-specification #134 diff 818f555..5d3aa25; FR-290; FR-331; interface_013; FR-349; FR-355; FR-060; FR-061; FR-240; FR-291; FR-292; FR-270; FR-271; FR-272; native-diagnostics 1-draft.5; AD-016; TC-266; TC-270; TC-271; spec/protocol/tests.md; spec/complete-v1/tests.md; tests/checked_package_v2.rs"
review_set: all
evaluated_revision: "5d3aa25+wt"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

The diff widens FR-290 in place from six protocol kinds to ten capability
kinds. The four new kinds are `value-validity`, `operation-contract`,
`finite-replay` and `temporal-satisfaction`. It also adds a descriptive family
table, a claim-form assignment table, (kind, mode) advertisement with an
extent/mode disposition table, the registry candidate-set rule and a
tool-absence result. FR-331 now carries a per-item `candidates` field and an
`unsupported` result. Catalog revision `1-draft.5` adds the code
`invalid_capability` and two new `unsupported_projection` causes. TC-271 is
new. All six scope items of #134 are addressed, and the diff follows the
settled rulings: one vocabulary, warned `unsupported` on absence, one
negotiation point, no mapping of QSL's four old kinds, and extent
representation left to QSL #222.

Two defects block. First, FR-290 requires every carrier of capability kinds to
declare `quire.capability-kind/v1` or be refused, but no carrier in this repo
declares it. Second, the empty-candidate warning can name no kind at all when
an item requires several kinds. The other findings are coverage gaps in TC-271
and consistency points.

## Verdict

**REVISE**: FND-001 (high) and FND-002 (medium) must be fixed. FND-003 through
FND-006 (medium) should be fixed in this PR. The low findings can be fixed in
this PR or recorded.

## Review set

Base checklist over every artifact the diff touches: ID formats and uniqueness,
link integrity, FR and TC quality, the six coverage rules, and consistency
across FR-290, FR-331, interface_013, the catalog chain (native-diagnostics,
FR-270, FR-271, FR-272), the protocol-family narrowing (FR-060, FR-061, FR-240,
FR-291, FR-292), the SMT backend chain (FR-349, FR-355, TC-266, TC-270), AD-016
and both test matrices. Settled rulings named in the task brief are not
reopened.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-290 "Vocabulary authority" refuses any carrier of capability kinds that does not declare `quire.capability-kind/v1`, and FR-290-AC-1 tests this. No carrier in the repo declares that identity: not FR-331's `manifest` or `request` (owned here), not FR-240/FR-061 results, not FR-291 `claim kinds`, not FR-292 `claim capabilities`. As written, every existing carrier is refused. Fix: add a capability-kind vocabulary member, fixed to `quire.capability-kind/v1`, to FR-331's `manifest` and `request`, and add it to request identity. Then either add it to FR-240, FR-291 and FR-292, or state in FR-290 which of these formats pins it through its own contract version. | FR-290; FR-331; FR-240; FR-291; FR-292 |
| FND-002 | medium | FR-290 candidate-set table, `empty` row: the warning names "every required kind that ... every registrant leaves unadvertised". An item can require several kinds (FR-331 `request`: "its required FR-290 kinds"). If backend A advertises only `value-validity` and backend B only `operation-contract`, and the item requires both, the candidate set is empty but no kind is unadvertised by every registrant, so the warning names nothing. The `unsupported-requested-capability` payload in native-diagnostics and FR-272 has the same gap. Fix: the warning and cause retain every required kind together with, for each registrant (or the named backend), the required kinds that registrant leaves unadvertised. Or state that an item requires exactly one kind and make FR-331 singular. | FR-290; FR-331; FR-272; native-diagnostics.md |
| FND-003 | medium | `invalid_capability` is a new code. Revision `1-draft.5` does not say that its spelling is catalog-owned. The earlier precedent for `ambiguous_dispatch` and `foreign_reference` states this so that a code missing from the pinned compiler inventory (QSL `f444d03`, `Code::as_str()`) is still selected. Without that sentence, the catalog's own rule ("A current compiler enum ... cannot silently extend this immutable selection") leaves `invalid_capability` unselected. Fix: add `invalid_capability` to the catalog-owned-spellings paragraph, or add a 1-draft.5 sentence that says the same. | native-diagnostics.md; FR-270; FR-271 |
| FND-004 | medium | FR-290-AC-6 requires that "each row of the advertised-mode table" settles its listed disposition. TC-271 has vectors for rows 2, 3 and 4 only (NG-07, NG-05, NG-06). Row 1 (bounded extent, bounded advertised) and row 5 (bounded extent, unbounded-only advertisement) have no vector. Fix: add NG-10 for bounded/bounded and NG-11 for bounded extent on an unbounded-only advertisement, each expecting the negotiation arm's own disposition. | FR-290-AC-6; TC-271 |
| FND-005 | medium | FR-290-AC-1 (ten labels accepted) still traces to TC-060 and TC-061. Those TCs exercise the protocol dispatcher and protocol result, which this diff narrows to protocol-family kinds (FR-060, FR-061-AC-17, FR-240). There, a non-protocol label such as `value-validity` must refuse, so TC-060/TC-061 cannot back "the ten exact labels are the complete accepted set". FR-060, FR-061 and FR-240 also give no refusal cause for a well-formed non-protocol kind on a protocol result. Fix: trace FR-290-AC-1 to TC-271 only, or keep TC-060/TC-061 only for the refusal half. In FR-061-AC-17, name the refusal and its cause for a known FR-290 member outside the protocol family (`unknown-kind` does not fit, because the member is known), and add a TC-061 vector for it. | FR-290-AC-1; FR-060; FR-061-AC-17; FR-240; TC-060; TC-061 |
| FND-006 | medium | TC-271 is registered with priority P0 in spec/complete-v1/tests.md and P1 in spec/protocol/tests.md. The complete-v1 row lists whole FRs (`FR-290, FR-331, FR-272`), while the protocol row lists ACs, including FR-331 and FR-272 ACs that the protocol matrix does not otherwise track. The complete-v1 V1-BACK-017 row (FR-290 → TC-060, TC-231) was not updated to add TC-271. Fix: choose one priority (P0 matches FR-331/FR-272 in complete-v1), use it in both rows, and add TC-271 to V1-BACK-017. | spec/complete-v1/tests.md; spec/protocol/tests.md; TC-271 |
| FND-007 | low | TC-271 has error-path gaps. It has no vector for a registration advertising an unknown kind or with no or a wrong vocabulary version (FR-290 says both refuse registration; CK-04 covers only an unknown mode). It has no vector for the `unsupported-requested-capability` obligation-form variant that FR-272-AC-9 requires (TC-266 SMT-01b checks the warning, not the catalog cause). CK-05 omits operation precondition and invariant, and the infinite-trace temporal profile, from the claim-form table that FR-290-AC-5 says is checked row by row. Fix: add CK-07 (registration with unknown kind, then with a bad version), NG-12 (obligation form not discharged, asserting `unsupported_projection`/`unsupported-requested-capability` naming the form), and extend CK-05 to every claim-form row. | TC-271; FR-290-AC-5; FR-272-AC-9 |
| FND-008 | low | FR-290-AC-4 still reads "A claim kind whose required capability has no registered backend ... naming that capability", while the body, FR-349-AC-5 and TC-266 now say "required capability kind". Fix: reword to "An item whose required capability kind no registered backend advertises settles `unsupported` with a warning naming that kind." | FR-290-AC-4 |
| FND-009 | low | The FR-271 meaning for `unsupported_projection` lists three cases but drops the obligation form that the one candidate's negotiation arm does not discharge, which native-diagnostics and FR-272 include under `unsupported-requested-capability`. Fix: add "or an obligation form the one candidate's negotiation arm does not discharge". | FR-271; FR-272; native-diagnostics.md |
| FND-010 | low | The FR-290 file slug and directory (`objects/protocol/FR-290-protocol-claim-kind.md`) still say protocol claim kind, while the title is now "Capability kind vocabulary" and covers five families. This is a naming mismatch only, and links resolve. Fix: owner decision. Rename only if the owner asks. Otherwise leave it and accept the mismatch. | FR-290; spec/spec.md |
| FND-011 | low | Some new text states what things are not: FR-290 Families ("is not a second vocabulary"), Kani ("it is not a `quire-spec-language` backend"), FR-349 ("are proof techniques, not capability kinds") and Tool absence ("never a hold, never a refusal ..., never a fallback ..., never a downgrade"). The Kani sentence is required word for word by #134 item 5, and the "never a hold" wording follows the settled solver-absence ruling. Fix: optional. Where it is not required by #134 or a ruling, restate positively (for example "Families are descriptive groupings of FR-290 members; requests and advertisements name members"). | FR-290; FR-349 |
| FND-012 | low | TC-266 adds vector `SMT-01b`, which breaks the `SMT-NN` sequence. Fix: renumber it `SMT-04`. | TC-266 |

## Checklist results

| Check | Result |
| --- | --- |
| ID formats | FR-290-AC-5..AC-8, FR-331-AC-4..AC-6, FR-272-AC-9 and TC-271 are well formed, sequential and unduplicated. TC-271 is the next free TC id. SR-606 is the next free SR id. The vector id `SMT-01b` breaks the sequence (FND-012). |
| Links | FR-290 → FR-331, AD-016, FR-196 and FR-060 relative links resolve. FR-331 → FR-290 resolves. The frontmatter edges FR-290 → FR-331 (`specifies`), FR-196 and FR-146 (`references`), and FR-331 → FR-290 (`references`) resolve. The TC-271 `verifies` edges to FR-290, FR-331 and FR-272 resolve. |
| FR quality | FR-290 defines its inputs (label, carrier identity, (kind, mode), extent, named backend) and its outputs (disposition, result, catalog cause), and its behavior tables are complete over their stated axes. FR-331 defines the `candidates` shape, its ordering and its identity participation. The error conditions have catalog codes. Two gaps: the carrier identity member is missing from every carrier (FND-001), and the multi-kind empty-set warning is underdetermined (FND-002). |
| Terminology | "capability kind", "protocol-family claim kind", "obligation form" and "negotiation arm" are used consistently, except FR-290-AC-4 (FND-008) and the FR-271 meaning (FND-009). |
| Coverage rule | Every new AC has at least one TC in a matrix: FR-290-AC-5..8, FR-331-AC-4..6 and FR-272-AC-9 map to TC-271. |
| Option permutation | The advertised-mode table is partly permuted (FND-004). Candidate cardinalities (unknown, empty, one, several named, several unnamed) are all vectored. |
| Constraint boundary | Candidate-set sizes 0, 1 and 2 are vectored, and registration order is reversed. Extent × mode rows 1 and 5 are missing (FND-004). |
| Error path | Every `invalid_capability` cause has a request-side vector. The registration-side unknown kind or version and the obligation-form cause are missing (FND-007). |
| State transition | The tool-absence order (routed, then probed, then run) is vectored by TA-03. The disposition stays `supported` while the result is `unsupported` (TA-01). |
| Edge case | Casing and whitespace variants of labels, `null`, empty string, and extra labels (CK-02) are covered, as are identity divergence on `candidates` (NG-09) and a mismatched tool identity (TA-02). |
| TC fields | TC-271 carries Description, Procedure, Vectors and Expected Results, like its siblings. Type and priority are in the matrices, but the priorities conflict (FND-006). |
| Settled rulings | FR-290 is the single authority, widened in place. Absence is `unsupported` with a warning. CG `negotiate_*` is the only negotiation point. Unknown labels are refused and never mapped. Extent representation is deferred to QSL #222. All are honored. |
| Catalog chain | README, native-diagnostics, FR-270, FR-271 and FR-272 agree on revision `1-draft.5`. The TC-233 header pin moves to `1-draft.5` under the same trace. The catalog-owned-spelling statement for the new code is missing (FND-003). |
| #134 scope | Items 1 through 6 are each addressed (kinds, claim-form assignment for frame and sum/case, modes, FR-331 candidates, Kani wording, no preference order). |

## Evidence

- Diff read in full: `git -C /home/peter/dev/worktrees/qspec-134 diff 818f555..5d3aa25`.
- Issue read: `gh issue view 134 --repo agent-ix/quire-specification`.
- Cross-checked the unchanged context: the catalog-owned-spellings paragraph in
  native-diagnostics.md, the V1-BACK-017 row in spec/complete-v1/tests.md, and
  the TC-060/TC-061 procedures that enumerate every FR-290 member.

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | FR-290 carrier-selection table and refusal, `spec/objects/protocol/FR-290-protocol-claim-kind.md:65-77`; FR-331 `capability_vocabulary`, identity and AC-7, `spec/objects/interfaces/FR-331-backend-provider-envelope.md:26,38-40,52`; FR-240:142-143; FR-291:27; FR-292:30 |
| FND-002 | resolved | One kind per item, FR-290:86-88; empty row names the item's kind and named backend, FR-290:176; FR-331:28; FR-272 and native-diagnostics:96 match |
| FND-003 | resolved | `invalid_capability` added to catalog-owned spellings, `proposals/quire-v1/definitions/native-diagnostics.md:102-103` |
| FND-004 | resolved | TC-271 NG-10 and NG-11, `spec/test-cases/TC-271-capability-kind-candidate-negotiation.md:54-55` |
| FND-005 | resolved | FR-061-AC-17 refuses a non-protocol member as mismatched, `spec/functional/protocol/FR-061-report-orthogonal-results.md:237`; TC-060:21 and TC-061:67-70 narrowed, with a non-protocol refusal vector; AC-1 traces to TC-271 |
| FND-006 | resolved | TC-271 is P0 in `spec/protocol/tests.md:259` and `spec/complete-v1/tests.md:1154`; V1-BACK-017 lists TC-271, `spec/complete-v1/tests.md:217` |
| FND-007 | resolved | CK-04 covers unknown kind, absent kind and duplicate registrations (TC-271:41); CK-03 covers the envelope version (TC-271:40); NG-15 covers the obligation-form cause (TC-271:59); CK-05 covers every claim-form row (TC-271:42) |
| FND-008 | resolved | FR-290-AC-4, FR-290:232 |
| FND-009 | resolved | FR-271 `unsupported_projection` meaning, `spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:62` |
| FND-010 | accepted-as-ruled | Relocating the file is an owner question. spec.md notes the cross-family authority, `spec/spec.md:1076` |
| FND-011 | resolved | "not a second vocabulary" restated positively, FR-290:51-52. The Kani sentence (FR-290:208-209) is required by #134 item 5. The FR-349 "not capability kinds" wording (FR-349:56-57) is optional. |
| FND-012 | resolved | SMT-01b is renumbered SMT-04, `spec/test-cases/TC-266-smt-backend-registration.md:40`. The fix adds SMT-03b; see FND-013. |
| FND-013 (new) | open | low. TC-266:39 adds vector `SMT-03b`, which breaks the `SMT-NN` sequence again. Fix: renumber it `SMT-05`. |
| FND-014 (new) | open | low. TC-060:21 still says "a backend supporting only finite replay/conformance". `finite-replay` is now a non-protocol member, and FR-292:30 limits a protocol backend to protocol-family members. Fix: reword to "a backend supporting only `global-conformance`". |

Round 2 verdict: ACCEPT WITH FINDINGS

## Post-round-2 author fixes

Every finding left open or raised in round 2 across SR-606..SR-613 is fixed in the committed revision:

| Finding | Fix |
| --- | --- |
| integrity FND-011 | FR-290 no longer lists advertised modes among the inputs that never change a disposition. |
| failure-domain FND-011 | FR-290 states one evaluation order: kind, extent classification, then the candidate table top row first. |
| risk FND-009 | FR-349: Z3 is the cross-check inside the one SMT descriptor, advertises no FR-290 pairs and is never a candidate. |
| evidence FND-004, dependency FND-007 | TC-271 CK-08 exercises FR-290-AC-2 and AC-3. |
| evidence FND-009 | CK-05 derives all six protocol members. |
| evidence FND-012 | CK-02 asserts the retained item index and the `1-draft.4` negative; CK-04 asserts the retained backend identity. |
| failure-domain FND-007 | FR-290: a tool change after a passing probe records FR-331 `failed`. |
| EARS FND-007, EARS FND-010 | FR-349 Description split into a registration SHALL and an If/then; FR-290 Description subject is the requested item. |
| base FND-013 | SMT-03b renumbered SMT-05. |
| base FND-014 | TC-060 uses a `global-conformance`-only backend. |
| failure-domain FND-012 | Registration refusals are ordered bytewise by identity, then digest. |
| integrity FND-012 | FR-331 manifest descriptor carries the declared obligation-form set. |
| integrity FND-013 | `unsupported-version` retains the refused carrier. |
