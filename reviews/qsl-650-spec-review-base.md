---
id: SR-1304
title: "Spec review of PR #194 (QSL-650): quire.backend-provider/v1 wire schema and package bytes"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@045b37b3bd08d2f5a55b65b424274e7ac435d922; PR #194 diff against origin/main 65e816f: FR-331 (Wire schema section, request bullet, AC-18 to AC-20), FR-462 (frame table, failure table, AC-7), TC-428 PW-07, TC-431, proposals/backend-provider-v1/{README.md,schema.json,vectors.json}; cross-checked against AD-016 arrow 6, TC-274, TC-276, FR-290, FR-379, FR-380, FR-385, FR-391, FR-428, FR-439, FR-461, QSL ADR-014 (quire-spec-language origin/main dac0d90) and qsl-foundation/src/bound.rs"
review_set: subset
---
# Spec review of PR #194 (QSL-650)

## Summary

Ticket: QSL-650. This PR publishes the normative `quire.backend-provider/v1`
schema with its vectors, the package-bytes member (`package` as unpadded
base64url plus `package_id`), and FR-331/FR-462 text that defines 19 meanings
the owning FRs left silent. Each of the 19 was checked against its owning FR.
The review also checked that every `$ref` into `urn:quire:checked-package:v2`
resolves, that the schema is closed where it says it is, that the Kani table
matches AD-016 and TC-274/TC-276, that the extent form matches QSL ADR-014 §4,
and that nothing is copied from another repo and no version or file tracking
is added.

Examined:
- FR-331 Wire schema: Stages, Package bytes, Items, Result entries, Kani results, Artifacts/counterexamples/accounting (examined)
- FR-331-AC-18, FR-331-AC-19, FR-331-AC-20 (examined)
- FR-331 request bullet (line 58) and Identity and validation (examined)
- FR-462 Inputs, frame-body table, failure-defect table, FR-462-AC-7 (examined)
- TC-428 PW-07, TC-431 BP-01 to BP-07 (examined)
- schema.json, every `$defs` entry (examined)
- AD-016 arrow 6 `KaniOutcomeKind` table and the paragraphs under it (context_only)
- TC-274 VC-01 to VC-03, TC-276 RU-01 to RU-05 (context_only)
- FR-290 tool absence, item-level causes, registration causes (context_only)
- FR-379, FR-380, FR-385, FR-391, FR-428, FR-439, FR-461 cause and setting text (context_only)
- QSL ADR-014 §1 B-4 row, §4, §11 F `bound` (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Kani results table has no row that settles `tested` or `failed`. It opens with "A Kani run settles exactly one result from its AD-016 `KaniOutcomeKind`", and its nine rows produce only `proved`, `inconclusive`, `refuted`, `declined`, `unsupported` and `incomplete`. AD-016 says a backend `tested` result keeps the value `tested` and an IR mapping invariant break yields `failed`. FR-290 says a tool that becomes unavailable after a passing probe records `failed`, and FR-462 adds `tool-failure`. AC-20 says the vectors cover "each row of the Kani results table, so together they hold each of the eight results", which the rows do not give. The vectors and the test require a Kani `tested` and a Kani `failed`, so the table is the part that is wrong. Add rows for `tested` (backend success that is not proof), for `failed` with `tool-failure{defect}` (FR-462), and for `failed` with diagnostic `unsupported_projection`/`tool-unavailable` after a passing probe (FR-290). Each row names its owner. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:220-234, spec/objects/interfaces/FR-331-backend-provider-envelope.md:311 |
| FND-002 | medium | FR-331 and the README say an item whose "kind, extent or probabilistic evidence kind is absent or outside its vocabulary is admitted on the wire". For extent that is false. `Extent` is a closed `oneOf` on `classification` `bounded`/`unbounded`, so an unknown classification, an unknown `FiniteBound` type or a missing `finite_bound_available` is refused by the schema. The adverse vectors `extent-bound-unknown-kind` and `unbounded-extent-without-availability` rely on that refusal. FR-290 has an item-level cause only for an absent extent classification (`absent-extent`) and none for an out-of-vocabulary one, so the schema behaviour is the consistent one and the sentence is what needs fixing. A reader built from the prose would admit an unknown classification and have no cause to settle it. Reword to "an absent `kind` or a `kind` outside the vocabulary, an absent `extent`, and an absent or unknown evidence kind". | spec/objects/interfaces/FR-331-backend-provider-envelope.md:157-160, proposals/backend-provider-v1/README.md:69-72, proposals/backend-provider-v1/schema.json:264-271 |
| FND-003 | medium | The extent wire form cites QSL ADR-014 §4 for a `FiniteBound` of `cardinality`, `integer-range`, `depth` or `variants{members}`. ADR-014 defines three variants (§4 table; §11: "`FiniteBound{Cardinality, IntegerRange, Depth}`"). `Variants` comes from QSL FR-358, and qsl-foundation `bound.rs:92-95` calls it "a harness-bound kind only, never a B-4 substitution kind". `extent.bounds` holds substituted B-4 proof bounds, yet the schema admits `variants` there. Two smaller drifts from the cited types: `depth.maximum` admits 0, where QSL's is `NonZeroU64`, and `DomainKey.path[]` and `population.ordinal` admit values up to 2^53-1, where ADR-014 has `u32`. Either drop `variants` from the B-4 `extent.bounds` and cite FR-358 wherever harness bounds use it, or cite the owner that makes it a substitution kind. Tighten `depth.maximum` to `PositiveCount` and bound `path` and `ordinal` to `u32`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:147-153, proposals/backend-provider-v1/schema.json:243-263 |
| FND-004 | medium | The FR-331 `request` bullet still says the request carries "contract-package items, requested claims, domains and limits", and QSL ADR-014 §1 row B-4 puts proof bounds "On the wire: QSpec FR-331 request `domains`". The closed `Request` schema has no `domains` and no request-level `limits`. Proof bounds are item `extent.bounds`, and limits are item `temporal.limits` and `probabilistic.limits`. The Identity sentence ("claim/domain/bounds/options") reads the same way. Restate the bullet and the identity sentence in terms of the schema members (`items[].claim`, `items[].extent.bounds`, `items[].options`, item limits, `deadline_ms`). Then raise the ADR-014 B-4 wording with QSL, which owns that record. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:58, spec/objects/interfaces/FR-331-backend-provider-envelope.md:285, proposals/backend-provider-v1/schema.json:201-215 |
| FND-005 | medium | A `hello` that advertises an unknown kind or mode, or `probabilistic-satisfaction` without evidence kinds, now has two outcomes. FR-290 refuses that registration alone with `invalid_capability`/`unknown-kind`, `unknown-mode` or `unknown-evidence-kind`, keyed by backend identity. The manifest-stage schema encodes those vocabularies (`Kind` and `Mode` enums, the `evidence_kinds` if/then), so the same hello fails its stage schema, and FR-462's new table settles a body the stage schema refuses as `tool-failure` `malformed-body`. The adverse vectors `manifest-unknown-kind`, `manifest-unknown-mode` and `manifest-probabilistic-without-evidence-kinds` name FR-290 as owner, which points to the registration cause. State which applies. One way: FR-462's `malformed-body` row excludes a hello whose only defect is an FR-290 registration cause, and that hello is reported with the FR-290 registration refusal. | spec/functional/tooling/FR-462-speak-the-plugin-wire.md:125, proposals/backend-provider-v1/schema.json:22, proposals/backend-provider-v1/schema.json:71-75, proposals/backend-provider-v1/schema.json:195 |
| FND-006 | low | Two payload definitions differ from their owners. `instance-bound-reached{value, states}` carries no member naming what raises the bound, but FR-428-AC-1 requires the cause to name `max_live_instances` as "the member that raises it". `memory-bound-reached` adds a dotted FR-461 `setting`, but FR-439 names "the request member that raises it" (FR-439-AC-2: `max_store_buffer`), which `limit` already carries. ADR-025 calls these bounds method bounds, not FR-461 resource limits. Give `instance-bound-reached` the raising member and drop `setting` from `memory-bound-reached`. If you keep `setting`, cite the owner that makes these FR-461 settings. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:215-216, proposals/backend-provider-v1/schema.json:649-650 |
| FND-007 | low | FR-331 says every proof basis and cause is "spelled exactly as written: lower-case words joined by `-`". The new Result entries text adds AD-016's `kani_vacuous_proof`, spelled with underscores, to the closed inconclusive set. The spelling rule was scoped to probabilistic, timed and real-time items. The new sentence merges the Kani cause into the same closed set without saying the rule does not cover it. Say that `kani_vacuous_proof` keeps the AD-016 spelling and is the one exception, or scope the rule explicitly. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:69-71, spec/objects/interfaces/FR-331-backend-provider-envelope.md:216-217 |
| FND-008 | low | The spec does not say how items settle when the plugin refuses a `request` body: a non-canonical `package`, or a decoded `package_id` that differs from the body's (`stale_dependency`/`content-mismatch`). FR-462-AC-7 says the body "is refused before any item runs", but not what the plugin sends or which terminal record each routed item gets. FR-331 requires exactly one record per item. Name the settlement, for example each routed item `failed` with `tool-failure`, or a `fault` frame and its host-side settlement. | spec/functional/tooling/FR-462-speak-the-plugin-wire.md:72-78, spec/functional/tooling/FR-462-speak-the-plugin-wire.md:140 |

## Verdict

Most of the wire is sound. All nine `$ref` targets into
`urn:quire:checked-package:v2` exist in that schema's `$defs` (`HexDigest`,
`Identifier`, `IntegerString`, `NodeId`, `NonNegativeIntegerString`,
`Occurrence`, `PackageId`, `PositiveIntegerString`, `Rational`). The checks
referenced them and did not copy them. Closure holds. Adding an unknown member
at every object node of every positive vector is refused everywhere except
limit-keyed maps (`bounds`, `limits`, tool `bounds`) and the owner-shaped
leaves the README names (FR-272 `detail`, FR-412 `objective`). Nothing is
vendored from another repo. The only digests are the FR-364 provision content
address and the FR-201 certificate identity, both functional, and the PR adds
no version or file tracking.

Of the 19 defined meanings, 14 are real gaps filled correctly. They are the
stage split, `request_index`, `claim`/`locus`, `candidates`, `options`, the
manifest `domains`/`options`/`bounds`, `dependencies`/`provision`/`deadline_ms`,
the `declined` causes (consistent with AD-016 rows 3-5 and TC-276), the
`tool-failure` defects, `witnessed{sources}`, the artifact members,
counterexample positions, accounting, the disposition diagnostic as its
warning, and the FR-410/412/418 sub-member spellings. Four carry findings:
the extent form (FND-003, FND-004), the Kani table (FND-001), and two payloads
(FND-006). The out-of-vocabulary admission sentence (FND-002) and the hello
refusal path (FND-005) are contradictions between the new text and an owner
or the schema.

The `measured` result makes nine values, while TC-274 VC-03 and TC-276 RU-05
still say "exactly the eight existing values". That drift was already on
main, so it is not this PR's. AC-20's "eight results a Kani run can settle"
is consistent with nine overall.

Not mergeable as is. Fix FND-001 to FND-005 in this PR. FND-006 to FND-008 are
cheap and belong to the same text. Once they are fixed it is mergeable.

## Dispositions

Round 1, reviewed at 346fa49 (fix range 276a7e0..346fa49). vectors.json was reformatted, so it was diffed semantically: every original vector is unchanged, and the fix adds 4 positive results and 10 adverse vectors. FND-004 was checked against FR-290, FR-368, FR-408, FR-411 and FR-461. None defines a request-level `domains` or `limits` on the FR-331 request. FR-368 puts proof bounds in the temporal `subject.proof_bounds` and limits in the item's `limits`. FR-411 names the item's `ExactProbLimits`. FR-461's `stage_limits` belongs to the replay request, not this envelope. FND-005 was checked against FR-290 lines 140-145 and 185-191: FR-290 owns the registration refusal and its causes, keyed by backend identity.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 346fa49 |
| FND-002 | fixed | 346fa49 |
| FND-003 | fixed | 346fa49 |
| FND-004 | fixed | 346fa49 |
| FND-005 | fixed | 346fa49 |
| FND-006 | fixed | 346fa49 |
| FND-007 | rejected | Plan ruling: `kani_vacuous_proof` is an existing cross-repo wire value (AD-016, TC-274). The fix adds only a parenthetical stating the exception; the schema, vectors and AD-016 spelling are unchanged. |
| FND-008 | fixed | 346fa49 |
