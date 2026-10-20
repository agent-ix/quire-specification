---
id: SR-13360
title: "Base review of QSpec PR #290 (IR-896): ContractPackage reader under FR-271/FR-272"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@595b874; spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/objects/interfaces/FR-330-contract-package.md, spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md, spec/test-cases/TC-937-contract-package-reader.md, proposals/quire-v1/definitions/native-diagnostics.md; ticket IR-896"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-271
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-272
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-330
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-461
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-937
    type: reviews
---

## Summary

Ticket: IR-896. PR: quire-specification#290 at 595b874, scoped to `git diff refs/remotes/origin/main...595b874` (merge base 9542172 = origin/main). Spec-only diff over six files. Base review of the new reader-to-FR-272 classification: FR-330 "Reader results as FR-272 diagnostics", FR-330-AC-11/12, FR-272-AC-21 and the payload contract edit, FR-271 "Reader-layer codes" and FR-271-AC-7, the FR-461 `contract_package` setting names, and the TC-937 mapping vectors. Every claim was checked against the sources, not the PR description: FR-322 (read vocabulary at line 607), FR-047, FR-271, FR-273, FR-274, QSL FR-285, and Contract IR main 000e461 (`crates/quire-contract-model/src/checked_package/shared.rs`, `v2/contract_package.rs`, `common.rs`).

Independently verified: FR-322's v2 refusal vocabulary has exactly 20 codes. Exactly 12 of them are rows of the FR-271 values table and the other 8 are not; the 12/8 split was checked against the extracted table. CG's `CheckedPackageRefusalCode` has 19 variants: the 20 minus `wrong_snapshot`. So luna's 18 matches neither source, and the author's 20 is right for FR-322 only. The `limit_kind` to cause table matches CG's six `CheckedPackageLimit` variants and the existing I2-reader mapping in the FR-272 and native rows. FR-272-AC-1 and AC-10 are literally unchanged: every closed cause set is byte-identical, and adding a surface sentence adds no variant. The at-limit/one-over vectors match CG's strict `>` checks for bytes, nodes and edges and the work meter's `consumed > limit`. Category incomplete (exit 22) for `stage_limit_exceeded` matches QSL FR-285-AC-5.

Gates: `cargo test --test checked_package_v2 --test backend_provider_v1` passes on 595b874 (9 and 105 pass). `quire validate --scope .` (quire 0.36.2, engine 0.50.2) on the six touched files fails only in FR-271 (missing `id`, three kebab-case `extraction-*` cells), FR-272 (missing `id`) and native-diagnostics.md (missing `type`). Its output, with only the duplicate-archetype warnings filtered out, is byte-identical on origin/main 9542172, so these failures pre-exist, as the author said. Method `code-review` was not run: no code or test file changed. The Cargo gates above stand in for it.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-330-AC-11 | spec/objects/interfaces/FR-330-contract-package.md | examined |
| FR-330-AC-12 | spec/objects/interfaces/FR-330-contract-package.md | examined |
| FR-272-AC-21 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-271-AC-7 | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | examined |
| FR-271-reader-layer-codes | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | examined |
| FR-272-payload-contract | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-330-reader-results | spec/objects/interfaces/FR-330-contract-package.md | examined |
| FR-330-setting-derivation | spec/objects/interfaces/FR-330-contract-package.md | examined |
| FR-461-behavior-2 | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md | examined |
| TC-937-mapping-vectors | spec/test-cases/TC-937-contract-package-reader.md | examined |
| FR-272-AC-1 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-272-AC-10 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | examined |
| FR-047-AC-2 | spec/functional/foundation/FR-047-emit-typed-located-causes.md | context_only |
| FR-271-AC-2 | spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md | context_only |
| FR-274-values | spec/objects/foundation/FR-274-diagnostic-location-kind.md | context_only |
| FR-330-invalid-body-locus | spec/objects/interfaces/FR-330-contract-package.md | context_only |
| FR-322-read-vocabulary | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Carrying the eight reader-layer codes verbatim with no FR-271 row and no second catalogue contradicts FR-047-AC-2 ("every emitted machine code resolves to the selected catalog; an uncatalogued code cannot be published as a stable producer outcome") and FR-271-AC-2 and Absence (a code that does not resolve in the selected catalog compares to absence and refuses interpretation). It also contradicts FR-330's own unchanged line 383 ("FR-272 catalogued diagnostics at `package_read`"). FR-271's scope section knows only producer codes (in the table) and consumer codes (in the consumer's own namespace). A reader-layer code that no catalog holds is a third, undefined kind, so a conforming reader must treat a `malformed_wire` diagnostic as uninterpretable. FR-272-AC-1's closed sets are not violated, because no variant changes. The defect is that these codes resolve nowhere. Owner decision needed: add FR-271/FR-272 rows for the eight; or map them onto existing families (for example `unknown_contract_version` to `unknown_wire`, and `malformed_wire`, `duplicate_member` and `unknown_member` to `invalid_package` with `malformed-json`, `duplicate-member` and `unknown-member`, which FR-330 already does for nested duplicates); or select a separate reader catalog through FR-270, as FR-283 does for projection refusals. | spec/objects/interfaces/FR-330-contract-package.md:488 |
| FND-002 | high | `locus` is mis-typed. The new FR-272 prose defines a reader's locus as "the pointer or path where a refusal or limit stop was reached" and puts it in the FR-274 location, and FR-330 adds "The reader's locus is the diagnostic's FR-274 location". But FR-330 line 194 and CG `CheckedPackageRefusal.locus` define `locus` as a `CheckedNodeId` `{domain, digest}`, and the RFC 6901 pointer is the separate `path` member. A node id is none of FR-274's three kinds (`source_span`, `json_path`, `runtime_value`), so it cannot ride in the location slot. Meanwhile the refused result's own `path` is given no destination; only the incomplete `path` goes to the location. With TC-937's "Require that `locus` appears in no payload", the node id is dropped. That is lossy, against IR-896's lossless-preservation goal for the CG payload `code, path, cause, locus, contract_version, document_pointer, expected, actual`. Fix: map the refused `path` to the FR-274 `json_path` location, as for incomplete, and either keep `locus` as a payload name or add a node-id location kind. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:208 |
| FND-003 | medium | The blanket "category refusal" for every `refused` result conflicts with QSL FR-285 ("`Refused` takes the category of its cause, so a profile-gated construct is unsupported (21)") and with QSpec TC-413 SV-04/SV-10 (`unsupported_construct`/`not-yet-implemented` has verdict unsupported, exit 21). FR-330 says the reader's codes are FR-322's whole vocabulary, which includes `unsupported_construct` and `unknown_profile`. CG's ContractPackage reader also reaches `validate_body`, which emits `unsupported_node_tag`. Either restrict the rule to the codes the reader can emit and state their categories, or say that the category follows the code as FR-285 does. The owner's relay chose "category refusal", so confirm it was meant to cover the unsupported-class codes too. | spec/objects/interfaces/FR-330-contract-package.md:445-452 |
| FND-004 | medium | The `contract_package.*` names follow FR-461's stage.counter form: the counters are from its list, and `input_bytes` and `work_units` match `intake.input_bytes` and `s3.work_units`. But they are "raised through the reader's limits argument" only. FR-461 Behavior 3/4 and AC-2 require every stage limit's name to work through the library setter, the replay `stage_limits` (FR-323) and `--limit` (FR-301). FR-460 Behavior 5 requires a published default, and the FR-271 row says these limits are "carried by the stage limits (FR-323)". The PR therefore either carves an undeclared exception out of FR-461 or creates unstated CLI and replay obligations. Also, the reader takes CG's shared `CheckedPackageReadLimits`, the I2 reader's type, whose six identical limits still have no FR-461 names (FR-322 line 645 promises "its setting name"). The same field would then be named under one reader only. The setting stage `contract_package` also differs from the diagnostic stage `package_read`. State the entry points and the default, or the exception, and name the I2 reader's settings in the same pass or say why they differ. | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:54 |
| FND-005 | medium | The `diagnostics` vector cannot be built. A ContractPackage has no diagnostics collection, FR-330 never says what the reader counts as diagnostics, and CG's ContractPackage reader charges only Bytes (`common.rs` `read_typed_prefix`), Nodes, Edges, Occurrences and Work: `CheckedPackageLimit::Diagnostics` appears nowhere in `v2/contract_package*`. So "for each of the six limits, read an input one over the limit" has no input for `diagnostics`, and FR-330-AC-12 and FR-272-AC-21 cannot be met for that row. Also, for `bytes` the incomplete record has no `path` (FR-330 forbids it), so "the location the `path`" must read "absent location" for that row. Either state what the reader counts under `diagnostics`, or drop that row from the vectors and say why. | spec/test-cases/TC-937-contract-package-reader.md:192-200 |
| FND-006 | low | "The driver derives `setting`" names an actor that QSpec does not define in FR-330, FR-272 or FR-461. FR-461 puts the obligation on the outcome ("names the limit, its value and its setting"). Say which party produces the FR-272 diagnostic, for example "the party that reports the reader result as an FR-272 diagnostic", so a non-driver consumer knows it owns the derivation. | spec/objects/interfaces/FR-330-contract-package.md:468-472 |
| FND-007 | low | FR-272-AC-21 joins the incomplete-result mapping with two unrelated payload-contract rulings ("`contract_version` is a canonical payload name and `locus` is not"). `contract_version` only serves `unknown_contract_version`, a refused, reader-layer code with no FR-272 row, so an incomplete-result test cannot exercise it. Split the payload-name ruling into its own criterion or move it into FR-330-AC-11. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:198 |

## Verdict

Not mergeable. Two high findings: the reader-layer codes resolve in no catalog, against FR-047-AC-2 and FR-271, and `locus` is mapped as if it were a pointer. Three medium: blanket category refusal against FR-285, unstated FR-461 entry points and defaults, and an unbuildable `diagnostics` vector. Two low. Sound: the 20-code count and 12/8 split, the six-row limit_kind/cause table, the bound/actual/path mapping for incomplete, the at-limit/one-over semantics, the unchanged FR-272-AC-1/AC-10 closed sets, the incomplete 22 / refusal 20 exit alignment, and the pre-existing validation failures.

## New findings (disposition pass 1)

Against head 6c9a7ea. Every claim of the fix was re-checked against the sources. FR-322's schema `DiagnosticCausePairing` (proposals/checked-package-v2/schema.json) maps every cause to a code, and none of those codes is one of the eight. The Diagnostic `code` enum holds none of the eight either. CG's emitters build all eight with `cause: None` (`common.rs` `refused`, `unknown_contract_version`). So "no cause variant" holds for FR-322. CG's defensive nested-refusal validator (`v2/contract_package.rs`:1416-1417) does admit `unknown_contract_version`/`unsupported-wire` and `unknown_required_capability`/`unsupported-feature`, which FR-322 pairs with `unknown_wire` and `unknown_required_feature`. That divergence is on the CG side and outside this PR. `cargo test --test checked_package_v2 --test backend_provider_v1`: 9 and 105 pass at 6c9a7ea. `quire validate` on every changed file reports only the failures that already exist on main (FR-271 id and extraction cells, FR-272 id, native-diagnostics type). `quire matrix`: FR-271-AC-7 and FR-272-AC-22 are untagged.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | FR-272-AC-22 and FR-271-AC-7 are `Test (TC-233)`, and TC-233 now describes parsing the eight rows. But no test carries their tags: `quire matrix --scope .` reports both untagged, while the sibling row checks FR-271-AC-6 and FR-272-AC-10 are tagged in `tests/checked_package_v2.rs`, at 8561 and 8270. QSpec runs TC-233 in its own Rust tests, so the claim that they are "tested in TC-233" does not hold yet. Add `#[trace("TC-233", ...)]` row-parse tests beside the AC-6 and AC-10 tests. | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:207 |
| FND-009 | medium | Some TC-937 refusal vectors cannot be built for the ContractPackage reader, and some contradict FR-330: - FR-330 (lines 389-396) says the reader reports a duplicate or unknown member as `invalid_package` with cause `duplicate-member` or `unknown-member`. CG does the same: `admit()` remaps the `DuplicateMember` code, and line 1044 refuses unknown members as `invalid_package`. So the `duplicate_member` and `unknown_member` code cases cannot come from this reader. - FR-330 has no capability check, so `unknown_required_capability` cannot come from it either. - The required `unsupported_construct`/`not-yet-implemented` case, also in FR-330-AC-11 and the FR-330 prose, names a pair FR-322 never admits: `not-yet-implemented` is in neither the FR-322 schema's cause enum nor CG's `CheckedPackageRefusalCause`. Limit the vectors to the codes this reader can return, and use a reachable pair, or none, in the category example. | spec/test-cases/TC-937-contract-package-reader.md:183-190 |
| FND-010 | low | The `setting` is "the `CheckedPackageReadLimits` member for bytes" (and so on), with no exact string given. One implementer may write `bytes` and another `CheckedPackageReadLimits.bytes`. FR-272's payload contract and both `stage_limit_exceeded` rows still define `setting` as the FR-461 setting name "that raises the limit at every entry point". Give the exact spellings (`bytes`, `nodes`, `edges`, `occurrences`, `work`), and say in the FR-272 row that this reader's `setting` is a limits-member name pending FR-461 registration. | spec/objects/interfaces/FR-330-contract-package.md:467-482 |

## Dispositions

Round 1, against head 6c9a7ea.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | deferred | FR-461 registration (CLI and replay entry points, FR-460 defaults) of the shared CheckedPackageReadLimits members, for this reader and the I2 reader together, is out of scope by owner decision and is the IR-896 follow-up. The PR removed the invented contract_package.* names and the FR-461 edit, and states the exclusion in FR-330. FND-010 covers the residual spelling ambiguity. |
| FND-005 | fixed | - |
| FND-006 | fixed | - |
| FND-007 | fixed | - |
