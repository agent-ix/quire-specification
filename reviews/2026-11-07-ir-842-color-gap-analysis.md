---
id: SR-20261106202641
title: "IR-842 Color fixture scoped gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR635 at c5ad4c8c; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-234-domain-package-model-intake.md, proposals/checked-package-v2/domain-package-acme-shapes.json, proposals/checked-package-v2/domain-package-acme-colors.json, proposals/checked-package-v2/model-enum-comparison.native, src/lib.rs, tests/model_variant_fixture.rs, proposals/checked-package-v2/README.md; FR-154-AC-11, TC-234 I34"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-234
    type: references
---

## Summary

Ticket: IR-842. This change-scoped review checks the fixture identity regression and its new producer-data guard. No whole-repository assurance, successful compiler admission or runtime enum-agreement claim is made.

## Verdict

**PASS for the scoped fixture correction** — no new in-scope implementation or trace defect found. The broad native intake criterion is not fully discharged by this producer fixture test.

## Coverage

Plan completion: not assessed

Reconciliation: quire matrix --scope . --format json, exit 0; static axis only, no run-evidence store read. The repository-wide output reports 447 tagged, 2447 untagged and 312 method-without-symbol criteria. Those existing repository-wide gaps are outside the explicitly assigned fixture change and are not being treated as a repository PASS.

FR-154-AC-11 is tagged by shapes_variant_case_identities_preserve_declared_case in tests/model_variant_fixture.rs:14. This is a real artifact-value regression for seven minted identities, original payloads and origins. A tag establishes a binding; it does not make the test evidence for every behavior in this broad consumer criterion. Malformed-case refusal attribution and successful runtime enum comparison remain consumer obligations. No FR-014-AC-2 tag or claim was added.

## In-scope Inspection

The shared owner schema requires the canonicalization descriptor with algorithm rfc8785-v1. This descriptor names a serialization algorithm; it is not a content hash, digest, pin or tracking manifest. All seven Shape/Color/List case identities now preserve their existing declared case. The JSON diff preserves case names, payload references, origins, type identities and declaration order. No QSL rule was relaxed and no fixture data was pruned.

The new dependency-free Rust target parses the actual published JSON, checks the descriptor, requires exactly seven variant entries and finds each expected owner by identity rather than displayName. Its independent golden table verifies every case identity and payload presence/absence; original origins are checked against their owning declarations. Missing or incorrect cases fail unconditional assertions. The expected identity strings are the authored-data contract, not a replacement minting or intake implementation. The trace has exactly TC-234 and FR-154-AC-11, in that order. No ignored branch, mock, unsafe code, local schema copy, dependency change or public API was introduced.

Reviewed focused evidence: the new target passes 1/1, strict Clippy for that target passes, and the changed Rust target formatting/diff checks pass (/tmp/ir842-color-isolated-focused.log). Four existing TC-940 fixture/projection controls pass (/tmp/ir842-color-existing-vector-checks.log). Seven authored documents validate structurally. The actual QSL CLI comparison retains the original malformed-case refusal and shows the corrected exact fixture reaching a separate assembly ill_typed/type-mismatch refusal; it does not prove successful complete compilation, enum evaluation or IR-842/FR-014-AC-2 agreement.

Gate limitation: repository-wide formatting reports drift in 14 untouched files, and all-target Clippy reports 18 existing lints in tests/checked_package_v2.rs. That file is byte-unchanged by this PR. These are not new in-scope source defects, and this review neither suppresses them nor claims the required aggregate gate passed. The lead must run and report the unchanged repository gate honestly before any merge decision. No Cargo or aggregate gate was run by this reviewer.


## Assurance Context

No AssuranceProfile is declared in this repository's spec tree. Direct change-scoped review is authorized by the lead; the repository-wide matrix is disclosed without a whole-repository acceptance claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Published standalone colors input review

Candidate: 67847241f5b067a6845142dcf8c84b36e8d9a848.

**PASS for the in-scope change** — Two producer guards have truthful FR-154-AC-11 bindings. Private QSL compile/run receipts strengthen current consumer evidence without inflating the producer test into FR-014-AC-2 generated agreement coverage.

The new acme/colors document is a complete independently identified Semantic IR model containing only the two nullary Color cases. It does not modify or prune the complex acme/shapes declaration set. The owner-required canonicalization descriptor, construct meaning, source identity, type identity, case identities and source origins agree. The four no-argument Boolean functions encode the two truth arms of equality and inequality: redEqualsRed=true, redEqualsGreen=false, redDiffersFromRed=false, redDiffersFromGreen=true.

The dependency-free library exposes exact authored input bytes with two documented include_str constants. They are justified consumer-facing definition accessors, not placeholder compiler APIs, copied validators or duplicate minting algorithms. No Cargo dependency, lockfile, schema, executable helper or QSL source changed. The second Rust guard checks the published model through its public accessor, requires one construct and one type, checks both exact nullary cases and their original source origins, and has an exact two-id TC-234/FR-154-AC-11 trace. The golden table reads the fixture under test and cannot pass missing or incorrectly cased entries.

Reviewed evidence at the exact committed pair: cmp succeeds for both private probe copies against the tracked model/source files. The real public QSL compile produced one model root, one variant-type declaration, two enum values, four pure functions, four binary applications and four checked equality decisions with enum member metadata. Four real native-run results are completed Boolean true/false/false/true for the functions named above. This is actual QSL compiler/value evidence, not generated-code or CG/QSL agreement evidence; the Rust producer guards do not themselves execute the compiler. The complex shapes fixture still reaches its separate downstream assembly refusal; nothing hides that result.

The two fixture guards and strict library/new-target Clippy pass in /tmp/ir842-colors-published-focused.log; changed source/test formatting and diff checks pass. Eight typed documents validate with zero grammar findings. The existing proposal README is plain untyped Markdown and is not claimed to pass typed artifact validation. The static matrix exit 0 now binds both guard functions to FR-154-AC-11; this remains partial producer evidence for that broad intake criterion.

The previously disclosed global formatting drift and 18 untouched legacy Clippy errors remain final-gate constraints. This reviewer ran no Cargo or aggregate gate and grants no waiver or unconditional merge-ready claim. No new in-scope finding was identified.

## New findings (disposition pass 3)

Candidate: b766811b1dafef40e6de7d1f9003db4e7c62878c. The aggregate gate exposed an in-scope regression that the earlier scoped review missed. Earlier PASS records are historical and are not rewritten.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The new source fixture breaks an existing whole-fixture consumer; isolated model guards and a direct source probe did not cover this collection boundary. | proposals/checked-package-v2/fixtures/model-enum-comparison.native; tests/checked_package_v2.rs:11064–11065 |

## Aggregate follow-up verdict

**FAIL** — cargo test --locked at the reviewed aggregate head fails tc_233_float_bits_literals_spell_fixed_width_lowercase_hex. Its existing read_dir loop feeds every entry in proposals/checked-package-v2/fixtures to serde_json::from_str, without an extension filter. The newly introduced .native file starts with an SPDX comment rather than JSON and causes expected value at line 1, column 1. The log records 166 passed and one failed in that target. This failure is introduced by this PR, not a baseline float defect.

Required fix: move the Native input out of the JSON-only fixture directory, remove the old path, and update the public include_str accessor and documentation references while preserving all source bytes and four operations. Keep the existing consumer test and its assertions unchanged; do not skip it, weaken its parser or keep a duplicate asset in the old directory. Recheck the actual fixed head and its unchanged public bytes before disposition.

Evidence: /tmp/IR842-color-combined-final-b766811b.log lines 257–270, command cargo test --locked exit 101. No Cargo command was run by this reviewer. Existing unrelated formatting, legacy Clippy and Quire structural failures remain separately disclosed; they do not excuse this new regression.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-002 | still-open | The failing aggregate head contains the Native input under the JSON-only directory; the forthcoming relocation fix has not yet been independently reviewed. |
| FND-002 | fixed | At c5ad4c8c the Native asset is outside the JSON-only fixture directory, its old path is absent, accessor/documentation references point to the relocated file, and the existing failing test passes unchanged. |

## Placement fix verification

Candidate: c5ad4c8c8f3d744b40f83b04062868cf41c4db72.

**PASS for the in-scope change** — FND-002 is fixed; no new in-scope finding was identified. The earlier aggregate FAIL is preserved above as the actual result on b766811b, not relabelled baseline or retroactively green.

The git diff reports a 100% rename from fixtures/model-enum-comparison.native to the proposal root. Direct byte comparison of the old committed asset with the new file succeeds, as does comparison with the exact source that previously produced the four QSL truth results. The old path no longer exists and no non-JSON files remain in the immediate fixtures directory. Public include_str and README/TC-234 references use the new path. tests/checked_package_v2.rs has no diff: its unfiltered JSON consumer and every assertion remain unchanged.

Reviewed /tmp/ir842-enum-source-placement-focused.log: the previously failing tc_233_float_bits_literals_spell_fixed_width_lowercase_hex passes 1/1, both model fixture guards pass 2/2, and targeted strict library/test Clippy exits 0. Changed source formatting and git diff checks pass; the typed document validation reports 8/8 grammar-clean. No Cargo or aggregate gate was run by this reviewer.

The public source accessor still exposes identical bytes and the four equality/inequality source functions are unchanged. No skip, parser relaxation, duplicate old asset, quota change or fixture pruning was introduced. The corrected final aggregate still belongs to the lead. Previously disclosed global formatting, legacy Clippy and structural documentation failures remain separate constraints; this finding disposition does not waive them or claim full gate success.
