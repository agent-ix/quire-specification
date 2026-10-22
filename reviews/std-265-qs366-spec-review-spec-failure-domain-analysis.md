---
id: SR-15503
title: "Failure-domain review of STD-265 invalid_result causes (QSpec PR #366)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@b34f4d4f25cfcaac5892d8b83f6e7f102232848d; spec/objects/interfaces/FR-331-backend-provider-envelope.md, proposals/quire-v1/definitions/native-diagnostics.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
---
# SR-15503: Failure-domain review of STD-265 invalid_result causes (QSpec PR #366)

## Summary

Ticket: STD-265. PR: quire-specification#366, head b34f4d4 (live head verified after fetch), scoped to `git diff origin/main...b34f4d4` (one commit, 6 files, +66/-4): FR-272 and native-diagnostics `invalid_result` rows (six causes, `document_pointer`/`expected`/`actual`), FR-331 'Result entries' table and validation redirect, FR-331-AC-34, FR-272-AC-23, TC-233 step, TC-431 BP-25, and the new test tc_233_invalid_result_cause_set_is_closed. Gates in a detached temp worktree: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines (grep -ciE 'failed|fail'), identical line-for-line to origin/main 429c59d, none new in a touched file; `cargo test --test checked_package_v2 --test backend_provider_v1` passes (119 + 23). quoin 0.28.3, quire 0.36.2 (engine 0.50.2). Failure modes examined: an entry with several defects, the computability of `expected`, the empty pointer for a non-object entry, pointers relative to a counterexamples entry, a closed-set value outside the reader's types (unknown_required_feature, consistent with QSL FR-290 steps 5 and 8), a `trusted` value from a first-party engine, and `unknown-cause` on a basis vs a cause.

Units examined:

- FR-331 (examined): A reader SHALL refuse a result entry, or the `counterexamples` entry a `refuted` result names, before consumption with the FR-272 `invalid_result` cause that names its defect, retaining the result item index and the offending member's RFC 6901 pointer relative to that entry (the empty pointer names 
- native-diagnostics/invalid_result (examined): | `invalid_result` | `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch` or `unknown-cause`; retain the result item index and the offending member as `document_pointer`: its RFC 6901 pointer relative to the result entry or `counterexamples` entry that
- FR-272 (examined): | `invalid_result` | `measured-with-truth`, `missing-member`, `unknown-member`, `invalid-value`, `certification-mismatch`, `unknown-cause` | The result item index and the offending member as `document_pointer`: its RFC 6901 pointer relative to the entry that carries it ... `certification-mismatch` n
- FR-331-AC-34 (examined): A reader refuses each of these before consumption with the `invalid_result` cause given, naming the pointer given: `certification-missing`, `missing-member` at `/certification`; `certification-on-refuted`, `unknown-member` at `/certification`; `certification-other-value`, `invalid-value` at `/certif

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | minor: 'the invalid_result cause that names its defect' assumes one defect per entry. No order is given across causes, so an entry with two defects has two conforming diagnostics: a `refuted` entry with `certification: 7` (unknown-member or invalid-value), a `proved` entry missing `basis` and carrying an extra member (missing-member or unknown-member), a `proved` entry with a core-only basis and `certification: certified` from a plugin. Member-name order is stated only within one cause. QSL FR-290 fixes its own order, but QSpec does not. Fix: state the order across causes (for example missing-member, unknown-member, invalid-value, unknown-cause, certification-mismatch, then measured-with-truth), or state that any one applicable cause conforms. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:498-516 |
| FND-002 | medium | minor: `certification-mismatch` retains `expected`: 'the certification FR-331's Certification table gives the proof', but the table does not give one value from the entry alone for every proof. `exhaustive`, `bounded-complete` and `inductive` are `certified` with a verified certificate and `uncertified` without one ('any proof whose method has no core certificate checker'), and `trusted` on a first-party proof has two candidate expected values. A reader cannot always compute `expected`, and two readers may disagree on whether `uncertified` on an `exhaustive` entry is a mismatch at all. Fix: restrict the cause to proofs whose table row fixes one value from the entry and its backend (plugin: `trusted`; symmetry- or partial-order-`reduced` and `bounded-solver`: `uncertified`; Kani `bounded-proof`: `certified`), and say what `trusted` on a first-party entry is. | proposals/quire-v1/definitions/native-diagnostics.md:100 |

## Verdict

Mergeable for failure domains: two minor ambiguities that make two conforming readers emit different diagnostics for the same entry. Neither is a contradiction.

## Dispositions

Round 1, reviewed QSpec #366 at ba8287988.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Checked against the vectors: certification-missing gives step 4, certification-on-refuted step 5, other-value and the three `certified` patches step 6. Both worked examples follow from the order (`refuted` + `certification` `7` gives step 5 before 6; plugin `certified` + core-only basis gives step 6 before 7). The order matches QSL FR-290's driver (certification before basis) and its basis, cause and assignments orders. |
| FND-002 | fixed | - |
