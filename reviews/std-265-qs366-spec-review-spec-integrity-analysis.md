---
id: SR-15501
title: "Integrity review of STD-265 invalid_result causes (QSpec PR #366)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@b34f4d4f25cfcaac5892d8b83f6e7f102232848d; spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, spec/test-cases/TC-431-backend-provider-wire-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# SR-15501: Integrity review of STD-265 invalid_result causes (QSpec PR #366)

## Summary

Ticket: STD-265. PR: quire-specification#366, head b34f4d4 (live head verified after fetch), scoped to `git diff origin/main...b34f4d4` (one commit, 6 files, +66/-4): FR-272 and native-diagnostics `invalid_result` rows (six causes, `document_pointer`/`expected`/`actual`), FR-331 'Result entries' table and validation redirect, FR-331-AC-34, FR-272-AC-23, TC-233 step, TC-431 BP-25, and the new test tc_233_invalid_result_cause_set_is_closed. Gates in a detached temp worktree: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines (grep -ciE 'failed|fail'), identical line-for-line to origin/main 429c59d, none new in a touched file; `cargo test --test checked_package_v2 --test backend_provider_v1` passes (119 + 23). quoin 0.28.3, quire 0.36.2 (engine 0.50.2). Cross-checks: the six causes against the vectors. BP-10's six certification adverse vectors (vectors.json adverse 90-95) patch temporal-outcome results/1 (partial-order `reduced`, `uncertified`) and certification-outcome results/4 (`bounded-solver`, `uncertified`) and results/6 (copy-swap+symmetry `reduced`, `uncertified`). Each FR-331-AC-34 assignment matches the patch and the Certification table: missing -> missing-member, refuted+certification -> unknown-member, `checked` -> invalid-value, and the three `certified` patches -> certification-mismatch expected `uncertified`. No other adverse vector patches `certification`. QSL FR-290 certification steps 1-4 match the QSpec causes, including `vouched` -> invalid-value and `certified`/`uncertified` -> certification-mismatch expected `trusted`. QSL's Unsupported mapping matches QSpec's new unknown_required_feature sentence. Claim 'no QSpec or QSL test parses the invalid_result row': confirmed on both mains. QSpec tests reference NATIVE_DIAGNOSTICS/FR-272 only through code-specific row parsers and never for `invalid_result`; FR-272-AC-15's tests (TC-350/358/367) do not read the row. QSL code reads no catalog text (qsl-semantics diagnostics_catalog holds the identity only). qsl-replay's PluginResultRefusal::diagnostic emits `unknown-cause` and `missing-member`, which stay in the set. IDs: FR-272-AC-23, FR-331-AC-34 and BP-25 are unused on QSpec main and on the four other open QSpec PRs (#365, #267, #207, #59). #1219 adds no ids. Merge order #366 -> #1219 is required: #1219 cites FR-331-AC-34, 'Result entries' and three causes that exist only after #366. Language: behaviour-only. Neither diff has status or changelog wording.

Units examined:

- FR-272-AC-23 (examined): The native-diagnostics row and this requirement's row of `invalid_result` each list exactly `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch` and `unknown-cause`, and each names `document_pointer`, `expected` and `actual`; each refused result entry 
- FR-331-AC-34 (examined): A reader refuses each of these before consumption with the `invalid_result` cause given, naming the pointer given: `certification-missing`, `missing-member` at `/certification`; `certification-on-refuted`, `unknown-member` at `/certification`; `certification-other-value`, `invalid-value` at `/certif
- FR-272 (examined): | `invalid_result` | `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch`, `unknown-cause` | The result item index and the offending member as `document_pointer`: its RFC 6901 pointer relative to the entry that carries it ... `certification-mismatch` n
- native-diagnostics/invalid_result (examined): | `invalid_result` | `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch` or `unknown-cause`; retain the result item index and the offending member as `document_pointer`: its RFC 6901 pointer relative to the result entry or `counterexamples` entry that
- FR-331 (examined): A reader SHALL refuse a result entry, or the `counterexamples` entry a `refuted` result names, before consumption with the FR-272 `invalid_result` cause that names its defect, retaining the result item index and the offending member's RFC 6901 pointer relative to that entry (the empty pointer names 
- FR-331 (examined): Unknown contract versions, missing required properties, duplicate keys, out-of-domain values and non-canonical encodings refuse before consumption; in a result entry or a `counterexamples` entry, an unknown member and an out-of-domain value refuse with the `invalid_result` causes of "Result entries"
- TC-431 (examined): | BP-25 | The reader, given BP-10's adverse vectors `certification-missing`, `certification-on-refuted`, `certification-other-value`, `certification-reduced-certified`, `certification-bounded-solver-certified` and `certification-symmetry-certified`, and a plugin's `proved` entry carrying `certified`
- TC-233 (examined): Parse the `invalid_result` rows of both and require FR-272-AC-23's causes and payload keys

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | major: FR-272-AC-23's second clause ('each refused result entry of FR-331-AC-34 carries the cause that criterion gives it') is verified as Test (TC-233), but TC-233's new step and tc_233_invalid_result_cause_set_is_closed only parse the two catalog rows; no test reads any result entry or its cause, and FR-331-AC-34 itself is Inspection. The matrix would show the clause covered when nothing can fail on it. Fix: drop the clause from AC-23 (FR-331-AC-34 already owns it), or split it into its own AC verified by Inspection. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:201 |
| FND-002 | medium | minor (author's default 2, judged): `invalid-value` here covers both a wrong JSON kind and an out-of-domain value, while the two closed member-shaped tables, `invalid_package` and `invalid_runtime_input`, both split `wrong-value-kind` from `invalid-value` under the same spellings, and their `missing-member`/`unknown-member` match this row's. The same cause name then means different things under different codes. Recommendation: split it and add `wrong-value-kind` for a member of the wrong JSON kind. QSL's readers already check kind and domain as separate steps (FR-290 'Basis and cause' step 1 and 8, certification steps 2 and 3), so `Malformed` maps to two causes without changing the reading order. Doing this before merge is cheaper than after both repos pin six causes. If the owner keeps one cause, the row should state the reason. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:105 |
| FND-003 | low | minor: the 'Identity and validation' redirect sends only 'an unknown member and an out-of-domain value' in a result or counterexamples entry to the invalid_result causes. A missing member and a member of the wrong JSON kind, which 'Result entries' also assigns to invalid_result, stay under the generic 'missing required properties ... refuse before consumption' sentence with no code. Fix: 'a missing or unknown member and a value of the wrong kind or outside its domain'. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:590-591 |
| FND-004 | low | minor: FR-272-AC-23 sits between AC-15 and AC-16. Every other row of the table is in numeric order (AC-19 to AC-22 follow AC-18), so readers and the matrix list it out of sequence. Fix: move the row after AC-22. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:201 |

## Verdict

Not mergeable: one major (FND-001, AC-23's untested second clause). The rest are minor. The six causes are consistent across native-diagnostics, FR-272 and FR-331, and every FR-331-AC-34 assignment matches its vector and the Certification table.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | major (new, disposition pass 1; missed in the review pass): FR-331's 'Result entries' table defines `unknown-cause` as 'a result cause or proof basis spelling outside the closed set', and the next paragraph says a value inside the closed sets that the reader's types cannot hold is `unknown_required_feature`/`unsupported-feature`, not an invalid result. Merged QSL FR-289-AC-14 and #1219's FR-290 refuse in-set spellings as `unknown-cause`: `replay-parity` (listed among FR-331's inconclusive causes, line 264) and every core-only basis (`closed-form`, `exact-value`, ... listed among FR-331's proof bases). The new reading-order example and AC-34 ('a plugin's `proved` entry carrying `certified` and a core-only basis') use 'core-only basis' too, which QSpec never defines. A reader that follows QSpec gives a plugin's `closed-form` basis `unsupported-feature` (or admits it), and QSL gives it `invalid_result`/`unknown-cause`. Two conforming readers disagree, and QSL cannot conform to both. #1219's new `invalid-value` at `/counterexample/type` for a plugin's TemporalCounterexample (an FR-331 counterexample) is the same question. Fix in FR-331: say which spellings a plugin's entry may carry (FR-305 / FR-462: a plugin's basis excludes the core-checked bases, it sends no QSL-only cause, and its counterexample is canonical assignments) and that any other in-set spelling from a plugin is `unknown-cause` (basis or cause) or `invalid-value` (counterexample `type`). Then extend 'outside the closed set' to 'outside the closed set, or outside the subset its backend may send'. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:510-516,537-538,669 |

## Dispositions

Round 1, reviewed QSpec #366 at ba8287988.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | Added in all three sources, AC-23, AC-34 (`certification` `7`), BP-25 and the test. |
| FND-003 | fixed | - |
| FND-004 | fixed | - |

Round 1 verdict: FND-001 to FND-004 fixed. FND-005 is a new major, so #366 is not mergeable until it is fixed.
