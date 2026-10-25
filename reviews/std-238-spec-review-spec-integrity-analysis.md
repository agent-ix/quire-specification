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

## New findings (disposition pass 1)

Re-checked at head 7c3223e172b5436adbf9d461d920fda8e4cd72fb against merge
base bb6d08aeedc333716f12bb66f65472a8724bc3ba. The head does not contain
origin/main 73e610b (nor 6b26194, which the author named); it merges with it
cleanly and the two test targets pass on that merge (118 + 23).

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | Major. The split moved FR-411 and FR-413 out but left the FR-412 policy witness work, which is the same out-of-ticket class as FND-002. In PR 200 the FR-412 `policy` row becomes "one QSL FR-202 WitnessEntry per named complete product key" with randomized choices "where the actual fairness/derived TS-5 mode admits them". FR-412 adds a TS-5 consumer paragraph ("For a policy WitnessEntry, the operator SHALL use the actual exact weighted choice distribution ... derived TS-5 admissibility ... QSL FR-201 / ADR-028 §13a"). FR-331 adds the WitnessEntry wire paragraph ("Each policy `choice` SHALL carry QSL FR-202's closed WitnessEntry"). The schema replaces the policy `choice` `TransitionIdentity` with `ProbabilityWitnessEntry`/`ProbabilityWitnessChoice`. FR-331-AC-27 ends "a policy choice that is open, empty or an ordinary transition identity is refused", and TC-431 BP-16 and TC-363 EC-10 test it. This is a wire-breaking change to certificate evidence, not a `certificate-rejected` spelling, and STD-238 does not ask for it. The row keys moving to `ProbabilityProductKey` belong with the `{product_key}` locus and FR-412-AC-8; the choice shape and the TS-5 text do not. Fix: move the policy choice shape, the TS-5 paragraph, the FR-331 WitnessEntry paragraph, the two schema definitions, the AC-27 policy clause, the BP-16 policy part and EC-10 to PR 354, or get an owner ruling that brings them into STD-238. | spec/objects/interfaces/FR-412-probability-certificate.md:38; spec/objects/interfaces/FR-412-probability-certificate.md:120; spec/objects/interfaces/FR-331-backend-provider-envelope.md:234; spec/objects/interfaces/FR-331-backend-provider-envelope.md:575; proposals/backend-provider-v1/schema.json:904; spec/test-cases/TC-363-probability-certificate-checker.md:42; tests/backend_provider_v1.rs:921 |
| FND-006 | medium | Minor. PR 200's product-key text needs FR-411 accumulator text that only PR 354 carries. PR 200's FR-411 still says the accumulator is "steps, reward or weighted sums since activation, saturating one unit above the threshold". PR 200's FR-331 requires "both unreduced fraction sums". FR-412 requires "Distinct unreduced fraction sum pairs ... SHALL remain distinct". TC-363 EC-09 tests "steps activation/above-bound/censored" and "count-fraction (1,2) versus (2,4)". QSL FR-196 encodes an amount above the bound as `["above-bound"]`, and only PR 354's FR-411 states that, along with keeping fraction sums unsaturated and unreduced. A reader of PR 200's FR-411 would saturate fraction sums at c+1, while FR-412's key keeps them exact. Fix: carry PR 354's accumulator sentences (above-bound, censored, fraction sums kept) in PR 200's FR-411, or move EC-09's accumulator variants and the unreduced-sum clauses to PR 354. | spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:53; spec/objects/interfaces/FR-331-backend-provider-envelope.md:218; spec/objects/interfaces/FR-412-probability-certificate.md:61; spec/test-cases/TC-363-probability-certificate-checker.md:41 |
| FND-007 | low | Minor. The fix commit's rewording left a broken sentence in FR-331: "two existential state keys (`n = 0`, `n + m >= 2`, ADR-023 HC-2). The When the universal and existential tuple arities ... differ ...". The leftover "The" is from the old "The reader SHALL check ..." sentence. Fix: delete it. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:184 |

## Dispositions

Round 1, checked at head 7c3223e. Severity labels: major is high, minor is
medium or low.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The State locus now says that under `malformed` the record "may be any FR-201 digest domain", and the schema adds `MalformedStateLocus` (any non-empty domain, `HexDigest`), paired only with `malformed`. Every other State rule keeps `ModelStateDigest`. A separate jsonschema 4.26.0 probe admits `malformed` at `package-fingerprint`, which is what QSL state_graph.rs `malformed()` writes per TC-644. The probe refuses `bad-state`, `rank-broken` and `initial-missing` there, and refuses an empty domain. qsl-spellings result 17 is admitted, and `model-state-other-domain` (rule patched to `bad-state`) and `model-state-malformed-empty-domain` are refused. |
| FND-002 | fixed | FR-413 and `ProbabilisticStep` are back to main, and the FR-411 accumulator and TS-5 text is in draft PR 354. Only the FR-411 product-key paragraph stays, and the PR body names it. The FR-412 policy and TS-5 residue is a new finding, FND-005, not this one. |
| FND-003 | fixed | The schema `QueryPart` and the FR-331 table now say "`step/<r>` is the spelling QSL FR-338 specifies and FR-468's per-step refinement certificate supplies the row". The test is now `tc_431_certificate_loci_are_the_catalogued_spellings`. |
| FND-004 | still-open | Minor. Nothing in PR 200 addresses it, and an ix-board search found no QSL follow-up ticket that cites FR-331's catalog in QSL FR-338 or makes the QSL reader refuse unlisted pairs. File the QSL ticket, then defer to it. |
