---
id: SR-14810
title: "Spec integrity review of the certificate-rejected rule and locus catalog (QSpec PR 200, STD-238)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@9c60fe7e8347315e5726475ef78d63cc0faae4a5 against merge base 97bfd65e7193d71253f7b84d0ded6218ef8569a5 (PR 200, ticket STD-238); spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/objects/interfaces/FR-412-probability-certificate.md; spec/objects/interfaces/FR-413-probabilistic-counterexample.md; spec/objects/interfaces/FR-418-zone-certificate.md; spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md; spec/test-cases/TC-363-probability-certificate-checker.md; spec/test-cases/TC-371-zone-certificate-checker.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json; tests/backend_provider_v1.rs; context: agent-ix/quire-spec-language@0f34179e270472da428677ccf4d0afefb957baab qsl-replay/src/certificate.rs, certificate/closure.rs, certificate/simulation.rs, certificate/state_graph.rs, spec FR-149, FR-169, FR-338"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-411
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-418
    type: reviews
---
# Spec integrity review of the certificate-rejected rule and locus catalog

## Summary

Ticket: STD-238. PR 200 (`spec/certificate-locus-wire`, head 9c60fe7) makes
FR-331's `certificate-rejected{rule, at}` a closed catalog. It covers 24 rule
spellings, a locus table, schema definitions (`QueryPart`, `ProofStepLocus`,
`ProductStateLocus`, `ModelStateDigest`, `EntryLocus`, `StepVerdict`,
`CertificateRejected` and others), the vectors `certificate-rejected-loci`,
`-rule-completion`, `-hyper-identities` and `-qsl-spellings`, and TC-431
BP-11, BP-12 and BP-24. The diff against the merge base also carries the
earlier commits on the branch. Those add FR-412 and FR-413 product-key
transport, the FR-413 witness step and scheduler shapes, and FR-411 text.

Each claimed QSL form was checked by reading the qsl-replay serializers on QSL
origin/main 0f34179. A scratch script also built every form those serializers
write and ran it through the PR's `CertificateRejected` definition with
Python `jsonschema` 4.26.0. The script is independent of the Rust test.

Examined with no finding:

- The rule set. The QSL `CertificateRule` enum has 20 variants, and their
  kebab spellings are a subset of the schema's 24 rules. The other four are
  `inequality-fails` and `threshold-missed` (FR-412) and `undefined-member`
  and `violating-member` (FR-163, hyper). The schema union holds exactly 24.
- The product state. `ProductStateRef` serializes as
  `{model_state: {domain, digest}, automaton_state: <base64url no pad>}`, as
  the table says. The rules closure.rs and scc.rs emit there are
  `shape-mismatch`, `initial-missing`, `successor-missing`, `bad-state`,
  `witness-fails`, `not-partition` and `backward-edge`. That is exactly the
  Product state row.
- The model state and entry. `DigestWire` writes `{domain, digest}` with
  lowercase hex, and `Entry{index}` writes `{entry: "<u64>"}` with only
  `malformed`. The state_graph.rs rules at ModelState match the State row.
  The domain is the exception (FND-001).
- The simulation step. `SimulationStep` always writes `position`, `edge` and
  `verdict`, with nulls. `malformed`, `initial-mismatch`, `successors-differ`
  and `bad-state` carry `edge` null and `verdict` null. `post-differs` carries
  an edge and a null verdict. `step-fails` carries an edge and a verdict. All
  of these were admitted.
- The step verdict. `FailedStep::wire` uses `kind` tags with u64 decimal
  strings. Every kind `check_step` can return was admitted:
  `InitialNotAbstract`, `StutterChanged`, `NoAbstractMatch`, the three
  `AbstractStepRejected` causes, `UndefinedEvaluation` and both
  `Undetermined` evaluations. The engine-only `{kind: "Undefined"}` form is
  refused, as FR-331 states, and the serializer comment confirms that
  `check_step` never returns it. FR-379 is unchanged, as the lead default
  says.
- The query part and proof step. QSL writes `unrolling`, `base` and `step`,
  and writes `{part, index}` with the index as a decimal string.
- Ids. FR-331-AC-24..27 and TC-431 BP-11/12/24 are unused on origin/main
  8d59511 and in the 14 other open QSpec PRs.
- `quire validate` (0.36.2, engine 0.50.2) reports 134 failing documents.
  The touched files' messages are identical to the merge base.
- No stale contradiction in QSpec. TC-269, FR-354 and FR-468 still say
  "naming the part/step" generically, which agrees with the catalog.

## Verdict

**Changes needed.** Two high findings: a QSL main form that the catalog
refuses, and unticketed normative changes to FR-411/FR-413. Two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The catalog refuses a rejection that QSL main writes. The State locus fixes `domain` to `quire.simulation.state-key/v1` (`ModelStateDigest` const). FR-331-AC-24 and BP-24 `model-state-other-domain` refuse any other domain. But `state_graph.rs` `malformed()` rejects a closure digest outside the state-key domain with `Malformed` at `CertificateLocus::ModelState(*state)`, which carries the foreign record. Its TC-644 unit test pins `ModelState(foreign)` for `DigestDomain::PackageFingerprint`, so QSL writes `{"domain": "package-fingerprint", ...}`. The probe confirms the schema refuses that `malformed` cause. A QSL main rejection therefore cannot be carried. This contradicts the lead default that the QSL main form is the authority where QSL serializes. Fix (owner choice): admit any FR-201 digest domain for `malformed` at a model state, or open a QSL ticket so FR-169's Malformed reports such an entry another way, for example as `Entry{index}`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:140; spec/objects/interfaces/FR-331-backend-provider-envelope.md:150; spec/objects/interfaces/FR-331-backend-provider-envelope.md:602; proposals/backend-provider-v1/schema.json:747; proposals/backend-provider-v1/vectors.json:3485 |
| FND-002 | high | The PR changes normative content outside STD-238, with no AC and no mention in the PR body. FR-411 changes the bounded product: the accumulator saturating one unit above the threshold becomes the single `["above-bound"]` amount, fraction sums are retained unreduced, and a digital-timed TS-5 admissibility paragraph is added. No FR-411 AC changed and no TC row covers these. FR-413 and the schema's `ProbabilisticStep` now require `action`, `post_key` and `elapsed`, allow `transition`/`random` null, and replace the scheduler and choice shapes. These are wire-breaking changes to counterexample evidence, not certificate-rejected spellings. STD-238 asks only for the rule/at catalog. The PR body describes only that catalog and FR-418. Fix: split the FR-411/FR-413 witness and product-key work into its own ticket and PR, or get an owner ruling that brings it into STD-238 and give the new FR-411 behaviour ACs and TC rows. | spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:55; spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:79; spec/objects/interfaces/FR-413-probabilistic-counterexample.md:193; proposals/backend-provider-v1/schema.json:597 |
| FND-003 | low | `step/<r>` is described as a form QSL writes, but QSL main has no serializer for it. QSL main's `QueryPart` has only `Unrolling`, `Base` and `Step`, and no production code builds a row-carrying part. `step/<row>` is a QSL FR-338 spec convention only. Yet the schema's `QueryPart` description says "as QSL writes it", the test is named `tc_431_certificate_loci_are_the_spellings_qsl_writes`, and the PR body lists `step/<row>` among the forms "qsl-replay serializes on QSL main". The FR-331 table's owner column also names only QSL FR-338/FR-314, while the QSpec per-step refinement certificate with one step part per row is FR-468. Fix: describe `step/<r>` as QSL FR-338's specified spelling, not a serialized one, and cite FR-468 for the row. | proposals/backend-provider-v1/schema.json:697; spec/objects/interfaces/FR-331-backend-provider-envelope.md:136; tests/backend_provider_v1.rs:1247 |
| FND-004 | low | The QSL and QSpec halves disagree after merge, and QSL PR 672 (the paired change) closed unmerged. QSL FR-338 still says "FR-331's `certificate-rejected` `at` is a non-empty text and defines no finer spelling, so this is QSL's convention". Also, QSL main's reader test `certificate_rejections_keep_every_rule_and_complete_typed_locus` admits all ten of its rules at every query part and proof step. This catalog refuses those pairs, for example `initial-missing` at `step`. Fix: file a QSL follow-up ticket to cite FR-331's catalog in FR-338 and to refuse unlisted rule/locus pairs in the reader. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:108 |
