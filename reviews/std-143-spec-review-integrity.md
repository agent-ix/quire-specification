---
id: SR-860
title: "Integrity review of PR #176: lifecycle, depth, witness value path, replay packet owner and digest staleness retirement"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@fac70325054b02f9fdbed031781b0be9132fad89; PR #176 diff against origin/main (a18859d); FR-146, FR-150, FR-154, FR-201, FR-207, FR-272, FR-300, FR-301, FR-305, FR-306, FR-321, FR-322, FR-323, FR-339, FR-351, FR-352, FR-354, FR-460, FR-461, FR-462, NFR-073, AD-014, AD-016, IT-010, IT-077, interface_004, TC-180, TC-191, TC-195, TC-217, TC-220, TC-221, TC-225, TC-226, TC-234, TC-254, TC-255, TC-269, TC-277, TC-280, TC-425..TC-429, proposals/checked-package-v2, native-diagnostics, model-complete, tests/*.rs; checked against quire-spec-language ADR-029 (spec/390-lifecycle), ADR-030 (spec/381-depth) and ADR-031 (spec/389-witness)"
review_set: subset
---
# Integrity review of PR #176

## Summary

Ticket: STD-143 (depth rule and v2 body grammar), with STD-141 (lifecycle,
CLI, plugins, cache) and STD-144 (witness value path). This pass checks the
PR's requirements against each other, against the files they cite, and
against ADR-029, ADR-030 and ADR-031.

The PR follows the three ADRs and the recorded rulings closely:

- Depth is never a limit kind. `nesting-depth-exceeded` is gone from both
  catalogs, there is no depth setting, and FR-461-AC-4 refuses one.
- `stage_limit_exceeded` names the limit, its value and its setting
  (FR-461, FR-272 `setting`).
- The v2 body grammar fixes package JSON depth by schema (FR-322 "Body
  grammar"), and both schemas carry it.
- Plugins are trusted programs and carry `trusted` (FR-305, FR-354, NFR-073).
- Undefined exits 10 and keeps its label (FR-301, matching ADR-029 RU-1).
- Only `native-run-result/2` exists. FR-352-AC-2 refuses `/1`.
- Model selection makes one content check (MS-03, `content-mismatch`).
- The catalog rev/SHA pin, the dependency `version` key and the
  `revision-mismatch` path in FR-322/FR-323 are removed.
- AD-016 moves the replay packet to QSL `WitnessEnvelope`, and IR names no
  QSL type.

Every new AC has a behaviour TC row. `cargo test --locked` passes at the
head. `quire validate` reports no errors in the PR's new files. The errors it
reports in the edited `objects/` files are main's existing value-object
errors.

The PR cannot merge as it stands, for three reasons:

- Two test rows still expect an altered domain-package document to be
  admitted, which contradicts the MS-03 content check.
- FR-207 adds value-path kinds while it still defers their order to FR-047,
  and FR-047 was not updated.
- It depends on #174 for the `content-mismatch` cause and the FR-307
  version removal.

## Verdict

FAIL. FND-001 to FND-003 are high. The PR also has to merge after #174 (FND-004, FND-005).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TC-234 I02 expects K1 with one field value changed, supplied under S1's digest, to be admitted; FR-154-AC-2, FR-322-AC-43 and MS-03 refuse it `stale_dependency`/`content-mismatch` | spec/test-cases/TC-234-domain-package-model-intake.md:45 |
| FND-002 | high | TC-195 N05 expects F1 with `A/x`'s multiplicity changed under the unchanged selection to be admitted; FR-150 refuses it `stale_dependency`/`content-mismatch` | spec/test-cases/TC-195-model-normalization-provenance.md:59 |
| FND-003 | high | FR-207 adds the `population` and `built` subjects and the object-reference step, and orders them, but still says the orders "are fixed by FR-047 and are not asserted here"; FR-047 still defines five step kinds and two subject kinds | spec/objects/foundation/FR-207-runtime-value-path.md:63,119-121; spec/functional/foundation/FR-047-emit-typed-located-causes.md:60,78-81 |
| FND-004 | medium | `content-mismatch` is not in the closed cause vocabularies at this head: native-diagnostics row, FR-272 row, and the V2 schema Diagnostic `cause_tag` enum and pairing. #174 adds the two markdown rows; neither PR adds it to the schema | proposals/quire-v1/definitions/native-diagnostics.md:80; spec/objects/foundation/FR-272-typed-diagnostic-cause.md:62; proposals/checked-package-v2/schema.json:144,150 |
| FND-005 | medium | FR-322 binds a dependency by identity and `package_id` "as FR-307 states", but FR-307 at this head still keys imports by version and refuses a version difference `revision-mismatch`; #174 carries the FR-307 change | spec/objects/interfaces/FR-322-checked-package-artifact.md:91-93; spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md:54-75 |
| FND-006 | medium | FR-321 still keys a ModelSelection by (`identity`, `version`, `digest_domain`, `digest`) although intake no longer checks the version (MS-04, FR-154-AC-2 "whatever its version label"); an unchecked label stays in the selection identity and the package identity preimage | spec/objects/interfaces/FR-321-model-selection-artifact.md:30 |
| FND-007 | medium | FR-460 applies to every wire format reader, but TC-003, IT-003 and TC-154 still prescribe depth limits (1 MiB/depth 64; "node, depth, work ... limits") | spec/test-cases/TC-003-verification-jcs.md:27; spec/integration/IT-003-verification-reference-amendment.md:37; spec/test-cases/TC-154-refuse-output-mapping-mismatch-and-exhaustion.md:24 |
| FND-008 | low | The edited `stage_limit_exceeded` rows keep catalog revision annotations (`1-draft.6`, `1-draft.7`), which is version tracking in requirement text | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:79; proposals/quire-v1/definitions/native-diagnostics.md:97 |
| FND-009 | low | FR-301 still says "Machine output is versioned" and "versioned input/output formats", and its summary list of distinguished statuses omits undefined and inconclusive, which its new table adds | spec/functional/tooling/FR-301-expose-complete-cli.md:33-41 |

## Merge items (not findings)

- The `tc_195_*` Rust tests (complete-model lock and rule manifest) are slated
  for deletion by #174. They pass at this head.
- #176 and #174 both edit FR-322, FR-323, FR-352, FR-354, FR-272, AD-016,
  IT-077, schema.json, native-diagnostics.md, model-member-type-vectors.json,
  complete-v1/tests.md and tests/*.rs. Merge #174 first, then rebase #176.
- FR-322-AC-28 (ModelOwner preimage carries `version`) is #174's STD-145
  content-only ModelOwner change, so this review raises no finding for it.

## Applied review

Followed /reviewer with /quoin:spec-review: integrity and consistency,
cross-reference and ADR-conformance checks. Rulings came from the wave-b and
wave-c briefs. Other repos were measured on their pushed branches:
quire-spec-language spec/390-lifecycle, spec/381-depth and spec/389-witness,
and quire-specification spec/wave-b-q9-ceremony (#174).

## New findings (disposition pass 1)

Reviewed at 44ca4adfef42a297a5c5b5d6bc5604d3e98793e2, rebased onto main ae02648b.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-010 | medium | FR-150 still says each derived fact records a "rule revision", but c94bae0 removed `revision` from the closed `RuleRef` and the rules object of `model-effective-declaration.schema.json` and from the rule manifest, so a fact that records a revision is refused by the schema | spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md:85; proposals/checked-package-v2/model-effective-declaration.schema.json:17 |
| FND-011 | medium | FR-462 and FR-305 bind a plugin by backend identity only, and its `hello` frame carries its FR-331 manifest, but FR-331 still gives each descriptor a manifest digest, a version and a pinned tool identity, keys candidates by (backend identity, manifest digest) and refuses on a tool-lock mismatch, and FR-271's `unsupported_projection` row keeps "a routed backend whose pinned tool is unavailable" (both already on main). These are version and pin checks that the identity-only plugin rule removes | spec/objects/interfaces/FR-331-backend-provider-envelope.md:27-29,40; spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:62; spec/functional/tooling/FR-462-speak-the-plugin-wire.md:69,75-79 |
| FND-012 | low | The catalog no longer has a revision (08bebe3, and FR-270 keys it by interpretation identity only), but FR-047's statement and FR-270's title and statement still call it a "versioned diagnostic catalog" | spec/functional/foundation/FR-047-emit-typed-located-causes.md:25; spec/objects/foundation/FR-270-versioned-diagnostic-catalog.md:3,14,18 |
| FND-013 | low | The PR deletes the dependency-selection "mutation inventory is exact" assertion, but the same function keeps the "order inventory is exact" assertion, and seven other exact-name inventory assertions over fixture lists remain. Each fails when a vector is added and asserts nothing the per-vector behaviour loop does not already check | tests/checked_package_v2.rs:3916,4047,4297,4316,4676,4687,4900,5938 |

## New findings (disposition pass 2)

Reviewed at e678bb5f04db7e361cdafc10bd396cc52f1c2dd6, rebased onto main aa031866. Delta: the PR's files, 44ca4adf to e678bb5f, limited to lines the delta changed. `cargo test --locked` passes at e678bb5f.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-014 | low | FR-013's Behavior charges "each field write's grant test and each `redefines` link it follows" at `frame.write` and `frame.redefines-link` but does not say which is charged first. FR-013-AC-7 and TC-236 Vector 5 depend on `frame.write` coming first: with `work_units` = k, they expect the stop "at the last `frame.redefines-link`" with `consumed = k`. Charged links-first, the same run stops at `frame.write`. State the order in the Behavior sentence. | spec/functional/foundation/FR-013-validate-operation-frames.md:27, :39; spec/test-cases/TC-236-frame-grant-population-scope.md |
| FND-015 | low | The AD-001 risk row edited by this PR still mitigates by "preserv[ing] the original minimum-only ConfigVersion design gate and historical definition bytes". That keeps superseded bytes as a compatibility record. Drop the clause. | spec/assurance/AD-001-composed-native-language.md:194 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 646c033 |
| FND-002 | fixed | 646c033 |
| FND-003 | fixed | 646c033 |
| FND-004 | fixed | 646c033 |
| FND-005 | fixed | d07b11f (#174 on main) |
| FND-006 | fixed | 646c033 |
| FND-007 | fixed | d07b11f (#174 on main, TC-003 and IT-003); 646c033 (TC-154) |
| FND-008 | fixed | 08bebe3 |
| FND-009 | fixed | 646c033 |

Round 2, reviewed at e678bb5f04db7e361cdafc10bd396cc52f1c2dd6.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-010 | fixed | 0085926: FR-150: each derived fact records its declaration-key inputs, rule identity and ordinal. |
| FND-011 | fixed | dbc5834: FR-331 drops manifest digest, version, pinned tool and tool lock; FR-271's row reads "a routed backend whose tool is unavailable". |
| FND-012 | fixed | dbc5834: FR-270 is titled "Diagnostic catalog"; FR-047 says "its diagnostic catalog". |
| FND-013 | fixed | 0085926: No "inventory is exact" assertion remains in tests/checked_package_v2.rs. |
