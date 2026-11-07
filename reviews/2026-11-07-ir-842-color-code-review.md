---
id: SR-20261106202636
title: "IR-842 Color case identity code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; PR635 at c5ad4c8c; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-234-domain-package-model-intake.md, proposals/checked-package-v2/domain-package-acme-shapes.json, proposals/checked-package-v2/domain-package-acme-colors.json, proposals/checked-package-v2/model-enum-comparison.native, src/lib.rs, tests/model_variant_fixture.rs, proposals/checked-package-v2/README.md; FR-154-AC-11, TC-234 I34"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-234
    type: references
---

## Summary

Ticket: IR-842. Generic code review of the two authored Markdown changes; no Rust lane applies before implementation. This is the pre-implementation review at the frozen specification-only candidate.

## Verdict

**PASS** — The specification-only diff keeps identity ownership with the existing producer and consumer contract. It introduces no runtime implementation, duplicate algorithm, source pin, tracking machinery or fixture-admission claim.

## Scope Examined

- FR-154 variant identity bullet at lines 186–194, including the Color/Red/Green example.
- FR-154-AC-11 identity-refusal obligation, with existing origin and duplicate-identity precedence as context.
- TC-234 I34 positive nullary cases and isolated lowercase identity mutations.
- Producer context: the public extraction frontend slug contract preserves ASCII letter case; existing QSL intake uses that owner function.

## Assurance Context

No AssuranceProfile is declared in this repository's spec tree. Direct subset review applies. No enforced review run was started, and no human acceptance or final gate is claimed.

## Evidence and Limits

The scoped Quire check passed for both changed artifacts: 2/2 grammar-clean, zero grammar findings. Git diff checking passed. No fixture or Rust implementation has changed yet, and no Cargo command ran. The current published fixture is intentionally awaiting this same PR's implementation stage. This review does not claim native model admission or FR-014-AC-2 runtime enum-comparison evidence. Implementation and focused test evidence will be reviewed separately in the same PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Implementation-stage review

Candidate: 5039e654289dbf377b483e8be4519b31e49df3e0.

**PASS for the in-scope change** — Rust, artifact and test review: no new in-scope findings. Ownership, fixed expected values, complete case cardinality, source preservation, safe indexing in test data and real fixture assertions are appropriate for this data-only correction.

The shared owner schema requires the canonicalization descriptor with algorithm rfc8785-v1. This descriptor names a serialization algorithm; it is not a content hash, digest, pin or tracking manifest. All seven Shape/Color/List case identities now preserve their existing declared case. The JSON diff preserves case names, payload references, origins, type identities and declaration order. No QSL rule was relaxed and no fixture data was pruned.

The new dependency-free Rust target parses the actual published JSON, checks the descriptor, requires exactly seven variant entries and finds each expected owner by identity rather than displayName. Its independent golden table verifies every case identity and payload presence/absence; original origins are checked against their owning declarations. Missing or incorrect cases fail unconditional assertions. The expected identity strings are the authored-data contract, not a replacement minting or intake implementation. The trace has exactly TC-234 and FR-154-AC-11, in that order. No ignored branch, mock, unsafe code, local schema copy, dependency change or public API was introduced.

Reviewed focused evidence: the new target passes 1/1, strict Clippy for that target passes, and the changed Rust target formatting/diff checks pass (/tmp/ir842-color-isolated-focused.log). Four existing TC-940 fixture/projection controls pass (/tmp/ir842-color-existing-vector-checks.log). Seven authored documents validate structurally. The actual QSL CLI comparison retains the original malformed-case refusal and shows the corrected exact fixture reaching a separate assembly ill_typed/type-mismatch refusal; it does not prove successful complete compilation, enum evaluation or IR-842/FR-014-AC-2 agreement.

Gate limitation: repository-wide formatting reports drift in 14 untouched files, and all-target Clippy reports 18 existing lints in tests/checked_package_v2.rs. That file is byte-unchanged by this PR. These are not new in-scope source defects, and this review neither suppresses them nor claims the required aggregate gate passed. The lead must run and report the unchanged repository gate honestly before any merge decision. No Cargo or aggregate gate was run by this reviewer.

## Published standalone colors input review

Candidate: 67847241f5b067a6845142dcf8c84b36e8d9a848.

**PASS for the in-scope change** — Rust/API review confirms exact-byte definition accessors, deterministic data guards and real source operations without unsafe code, fake admission or public semantic implementations.

The new acme/colors document is a complete independently identified Semantic IR model containing only the two nullary Color cases. It does not modify or prune the complex acme/shapes declaration set. The owner-required canonicalization descriptor, construct meaning, source identity, type identity, case identities and source origins agree. The four no-argument Boolean functions encode the two truth arms of equality and inequality: redEqualsRed=true, redEqualsGreen=false, redDiffersFromRed=false, redDiffersFromGreen=true.

The dependency-free library exposes exact authored input bytes with two documented include_str constants. They are justified consumer-facing definition accessors, not placeholder compiler APIs, copied validators or duplicate minting algorithms. No Cargo dependency, lockfile, schema, executable helper or QSL source changed. The second Rust guard checks the published model through its public accessor, requires one construct and one type, checks both exact nullary cases and their original source origins, and has an exact two-id TC-234/FR-154-AC-11 trace. The golden table reads the fixture under test and cannot pass missing or incorrectly cased entries.

Reviewed evidence at the exact committed pair: cmp succeeds for both private probe copies against the tracked model/source files. The real public QSL compile produced one model root, one variant-type declaration, two enum values, four pure functions, four binary applications and four checked equality decisions with enum member metadata. Four real native-run results are completed Boolean true/false/false/true for the functions named above. This is actual QSL compiler/value evidence, not generated-code or CG/QSL agreement evidence; the Rust producer guards do not themselves execute the compiler. The complex shapes fixture still reaches its separate downstream assembly refusal; nothing hides that result.

The two fixture guards and strict library/new-target Clippy pass in /tmp/ir842-colors-published-focused.log; changed source/test formatting and diff checks pass. Eight typed documents validate with zero grammar findings. The existing proposal README is plain untyped Markdown and is not claimed to pass typed artifact validation. The static matrix exit 0 now binds both guard functions to FR-154-AC-11; this remains partial producer evidence for that broad intake criterion.

The previously disclosed global formatting drift and 18 untouched legacy Clippy errors remain final-gate constraints. This reviewer ran no Cargo or aggregate gate and grants no waiver or unconditional merge-ready claim. No new in-scope finding was identified.

## New findings (disposition pass 3)

Candidate: b766811b1dafef40e6de7d1f9003db4e7c62878c. The aggregate gate exposed an in-scope regression that the earlier scoped review missed. Earlier PASS records are historical and are not rewritten.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | The new Native source asset is inside the JSON-only fixture collection, so the existing float-literal consumer parses it as JSON and the required Cargo test fails. | proposals/checked-package-v2/fixtures/model-enum-comparison.native; tests/checked_package_v2.rs:11064–11065 |

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
