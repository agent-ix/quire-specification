---
id: SR-2453
title: "Gap analysis of the certificate-rejected locus wire candidate (FR-331-AC-24, TC-431 BP-11, tests/backend_provider_v1.rs)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@cff80d04fa88ecae0831be2ef2fb70adabed1c98 against origin/main a865694e5db4daeb292c83070ecc2f12e0c9656c; spec/objects/interfaces/FR-331-backend-provider-envelope.md (AC-24); spec/test-cases/TC-431-backend-provider-wire-vectors.md (BP-11); proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json; tests/backend_provider_v1.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# Gap analysis of the certificate-rejected locus wire candidate

## Summary

Ticket: none resolved; no tracker id on the branch or the commit. Planless gap
analysis: plan completion was not assessed.

This analysis maps FR-331-AC-24 to TC-431 BP-11 and then to the Rust tests.

- **Positive clause** (the three query loci and the indices `"0"`, `"41"` and
  `"18446744073709551615"`). The vector `certificate-rejected-loci` is admitted
  by `tc_431_positive_vectors_are_admitted_with_owned_members`. That test also
  checks that every `/results/n/cause/at` owner pointer exists and names
  FR-331.
- **Adverse clause** (11 cases). One adverse vector exists for each. Each is a
  single patch of `certificate-rejected-loci` and is refused by
  `tc_431_adverse_vectors_are_refused`. The refusal must lie at the patched
  member, under it, or at a non-root ancestor of it. Each adverse case listed
  in the AC has its vector, and each vector is listed in BP-11.
- **Exact-range clause.** This is `tc_431_proof_step_index_admits_exactly_the_u64_range`.
  Its oracle is independent of the regex: `str::parse::<u64>` plus a
  print-back check. It is not a tautology, because the oracle never consults
  the schema. Its samples match the AC and BP-11.
- **Reachability through a real reader.** QSpec has no provider reader: the
  test header says reader behaviour beyond the schema is not claimed.
  FR-331's reader SHALL is evidenced only at the schema level, as AC-18 to
  AC-23 already are.

  On the QSL E4 branch (`qsl-490-e4`, unmerged, outside this review), the
  current serializer refuses to write `ProofStep`. So no implemented producer
  emits the positive proof-step vector yet. That is the pending E4 coder's
  work, as the author's report states.
- **Not stubbed.** No production code changed; only schema, vectors, spec and
  test changed.

Focused test evidence is the author's: 10 passed and fmt clean at `cff80d0`,
with the log at `logs/certificate-locus-focused-test.log`. I did not rebuild,
because no finding needed a run. I independently confirmed the regex with a
non-Rust oracle; see the integrity review SR-2454.

## Verdict

**One low finding.** AC-24 is fully exercised. Its vector clauses, however,
are reached only by tests tagged with other ACs.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-24's positive and adverse vector clauses are exercised by `tc_431_positive_vectors_are_admitted_with_owned_members` and `tc_431_adverse_vectors_are_refused`. Those tests carry `#[trace]` tags for FR-331-AC-19 and AC-23 (and AC-20), not for AC-24. The only AC-24 tag is on the U64String range test, which never loads a vector. The computed matrix therefore shows AC-24 covered by a test that verifies one of its three clauses. AC-23 is the precedent: its vector clauses are traced by adding AC-23 to both runners. Fix: add `#[trace("TC-431", "FR-331-AC-24")]` to both runner tests. | tests/backend_provider_v1.rs:219-222; tests/backend_provider_v1.rs:262-266; tests/backend_provider_v1.rs:626 |
