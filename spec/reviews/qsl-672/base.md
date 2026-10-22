---
id: SR-5662
title: "QSL-672 QSpec grammar admission review"
type: SpecReview
analysis: base
scope: "quire-specification@732fce3f405a1ec877aa21acd424a73482250719; proposals/quire-v1/shared-grammar.md; FR-147; FR-039; TC-192; FR-044; FR-149; companion QSL FR-091"
review_set: subset
---

## Summary

Reviewed QSpec PR 220 against QSL-672 and companion QSL FR-091 at 3d25cb835acae883973d85f5c9a0b15fd6a925c2. The integer operators now have consistent grammar, selected laws and refusal staging, but the rational-slash clarification leaves contradictory admission requirements.

## Verdict

**FAIL** — one semantic consistency finding; one low severity verification-cell finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new expected-rational promotion rule does not restrict itself to complete value, while FR-044 still requires implicit conversion refusal under every profile. The accepted DIV-18 rational slash therefore conflicts with its named definedness requirement. | proposals/quire-v1/shared-grammar.md:1070; spec/functional/foundation/FR-039-normalize-exact-rational-literals.md:25; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:29; FR-044-AC-3; FR-044-AC-9; TC-192 DIV-18 |
| FND-002 | low | The new FR-147-AC-11 Verification cell is `Test (TC-192)`; the installed spec-review checklist requires a method only. Keep the TC association in the test-case relationship or other trace content, and use `Test` in the cell. | spec/functional/expressions/FR-147-evaluate-integer-division-domains.md:116 |

## Review evidence

- PR head equals the requested frozen head; PR body is exactly `Refs QSL-672`.
- Shared grammar product productions at lines 132 and 381 already recognize infix `div`, `rem`, and `mod`.
- FR-147 retains exactly one checked division law for `div`/`rem`, rejects missing/multiple laws, and gives `mod` independent Euclidean meaning. The selected-law table and signed vector oracle agree.
- FR-147-AC-11 maps the requested acceptance/refusal distinction to TC-192 DIV-17 and DIV-18. Those vectors cover product precedence, operator locations, state-only refusal and dead branches; existing DIV-09 covers missing/conflicting division laws.
- Companion FR-091-AC-39 and the resolved integer-operator OQ5 row agree with the integer admission rules. The other-construct OQ5 row remains separate and does not leave integer operators unresolved.
- FR-147 implements US-010, which traces to StR-002; TC-192 verifies FR-147. No applicable AssuranceProfile was found in this specification tree.
- Read-only `quire validate --scope . <three changed spec files> --summary` exited 0: 3/3 grammar-clean, no grammar findings. Author-provided validation/parse/lint evidence was inspected too.
- `quire matrix --scope . --format json --strict` exited 1: 1717 untagged, 108 tagged, and 304 method-without-symbol criteria. All 11 FR-147 criteria, including AC-11, are untagged. Authored TC coverage is present; runnable implementation evidence is not established. This ticket is the pre-implementation specification gate for QSL-633, so the review does not require that blocked implementation to be written here.

## Required reconciliation

Give the new promotion wording an explicit profile boundary. Reconcile FR-044 Behavior and AC-3/AC-9 with FR-149's exact rational-slash promotion exception, retaining state-only refusal. Add or clarify paired expected-rational slash vectors under state-only and complete-value selections. A concrete witness is integer operands in `7 / 3` with an expected rational domain containing 7/1, 3/1 and 7/3: the new rule accepts promotion, while FR-044's blanket implicit-conversion rule refuses it.

The blanket FR-044/FR-149 inconsistency predates this PR. It remains directly relevant because the changed FR-039, shared grammar, FR-147-AC-11 and DIV-18 now explicitly depend on both conflicting rules; it was not treated as an unrelated repository-wide defect.

## Limitations and tools

Review performed by the assigned GPT-6/Codex agent; this is not an Opus-model certification. No source edits, builds, cargo runs, tracker posts or merge occurred. Artifacts are under `/tmp` as dispatched.

Machine: terra. Executables: `/home/peter/.local/bin/quire` (0.36.2, engine 0.50.2) and `/home/peter/.local/bin/quoin` (0.28.3). Validation/matrix emitted duplicate ADR/Plan/Review/SpecReview/Standard archetype warnings, duplicate inverse `part_of` warnings, and an inline-data-schema advisory. These diagnostics match the supplied author logs and do not make the semantic pass clean. Full stderr is preserved in `/tmp/qsl672-qspec-review-validate.log` and `/tmp/qsl672-qspec-review-matrix.log`.


## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 84c229da6a31fc31d9ddf9249847b3bc98316a5b: Shared grammar, FR-039, FR-044 Behavior and AC-3/AC-9, FR-147-AC-11 and state-contract explicitly restrict integer promotion for expected-rational slash to complete value; TC-044 and TC-192 now pair state-only refusal with complete-value acceptance. |
| FND-002 | fixed | 84c229da6a31fc31d9ddf9249847b3bc98316a5b: FR-147-AC-11 Verification is exactly `Test`. |
| FND-003 | fixed | 9b709948c39ecc40210589ba5ec418ef5a81c295: TC-044 and TC-192 planned trace annotations now contain only FR-044-AC-3, FR-044-AC-9 and FR-147-AC-11 as applicable; TC IDs removed. |
| FND-004 | fixed | 9b709948c39ecc40210589ba5ec418ef5a81c295: FR-044-AC-3 and FR-044-AC-9 Verification cells are both exactly `Test`. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Newly added planned trace annotations include TC IDs alongside AC IDs, contrary to the installed review checklist requiring test trace tags to name AC IDs only. Remove the TC-044 and TC-192 arguments; preserve test-case associations in the existing verifies relationships. | spec/test-cases/TC-044-check-composed-numeric-definedness.md:39; spec/test-cases/TC-192-integer-division-profiles.md:60; spec/test-cases/TC-192-integer-division-profiles.md:61 |
| FND-004 | low | The revised FR-044-AC-3 and AC-9 Verification cells add test-case IDs although the installed review checklist requires a method only, the same rule used to correct FR-147-AC-11. Use `Test` and retain TC associations in their existing verifies relationships. | spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:65; spec/functional/foundation/FR-044-check-composed-numeric-definedness.md:71 |

## Delta verdict

**FAIL** at 84c229da6a31fc31d9ddf9249847b3bc98316a5b for two low checklist findings. Both original findings are fixed and no new arithmetic/profile contradiction was found. New findings remain open.

The new trace finding's primary unit is TC-044 Test Procedure, excerpt: `#[trace("FR-044-AC-3", "FR-044-AC-9", "TC-044")]`. Related units are TC-192 DIV-17 and DIV-18. The new Verification finding's primary unit is FR-044-AC-3, whose Verification excerpt is `Test (TC-044, TC-192)`; related FR-044-AC-9 reads `Test (TC-193, TC-044)`.

Examined units include shared-grammar rational slash admission; state-contract retained-rule scope; FR-039 Behavior; FR-044 Behavior and AC-3/AC-6/AC-9; FR-147-AC-11; TC-044 profile controls; and TC-192 DIV-17/DIV-18. Context-only units include FR-149 rational division promotion and state-core admitted forms. FR-149 explicitly makes integer operand promotion exact and obligation-free, so TC-044's promoted 2/1 outside Q is not a new operand-domain contradiction; its quotient 1/2 is within Q.

PR 220 body is exactly `Refs QSL-672`; GitHub and the clean local worktree both name this exact head. Author logs `/tmp/qsl672-qspec-delta-{validate,parse,lint}.log` were inspected: 5/5 changed spec documents grammar-clean, 7/7 changed documents parsed, and lint has only duplicate inverse-edge warnings. Independent changed-spec validation exited 0, 5/5 grammar-clean, zero grammar findings (`/tmp/qsl672-qspec-delta-review-changed-validate.log`). Independent spec/plan validation exited 3 with 141 structurally failing documents (`/tmp/qsl672-qspec-delta-review-full-validate.log`). The original full log and independent log both have exactly 141 unique failing paths and zero intersection with the seven changed files. Full validation is therefore not a pass; its failures are outside this delta.

No build, repository edit, tracker post or merge occurred. The dispatch restricts output to /tmp; the lead retains tracker/artifact custody. Installed tool: /home/peter/.local/bin/quire, version 0.36.2 (engine 0.50.2). Duplicate module/inverse warnings and inline-data-schema advisory remain visible in the logs, matching the supplied author logs.


## Delta verdict (disposition pass 2)

**PASS** at 9b709948c39ecc40210589ba5ec418ef5a81c295. All four findings have fixed dispositions; no new findings. The diff from 84c229da changes only the three offending trace annotations and the two Verification cells, preserving the reviewed semantic reconciliation and FR-147-AC-11 method-only cell.

Fixed excerpts: TC-044 now specifies `#[trace("FR-044-AC-3", "FR-044-AC-9")]`; TC-192 specifies `#[trace("FR-147-AC-11")]` and `#[trace("FR-044-AC-3", "FR-044-AC-9")]`. Both FR-044 Verification cells read `Test`.

Local clean worktree HEAD and GitHub PR 220 head both equal the reviewed head; PR body is exactly `Refs QSL-672`. Author logs `/tmp/qsl672-qspec-trace-delta-{validate,parse,lint}.log` were inspected: validation reports 3/3 grammar-clean with zero grammar findings; parse and lint report 3/3 completed. Only the previously observed module/inverse and inline-schema advisories occur. No builds, repository edits, posts or merge performed. This limited final cleanup creates no reason to repeat the full-repository validation already reproduced in disposition pass 1; that full run remains exit 3 for 141 unrelated documents.
