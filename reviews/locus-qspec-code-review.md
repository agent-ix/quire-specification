---
id: SR-2452
title: "Code review (with Rust lane) of the certificate-rejected locus wire candidate: tests/backend_provider_v1.rs, backend-provider-v1 schema and vectors"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification@cff80d04fa88ecae0831be2ef2fb70adabed1c98 against origin/main a865694e5db4daeb292c83070ecc2f12e0c9656c; tests/backend_provider_v1.rs; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json"
review_set: subset
---
# Code review of the certificate-rejected locus wire candidate

## Summary

Ticket: none resolved; no tracker id on the branch or the commit.

This review covers the Rust lane (`rust-review`) and the data changes.

- **`tc_431_proof_step_index_admits_exactly_the_u64_range`** (+49 lines).
  - It is first-party Rust and adds no dependency.
  - It reuses the existing `definition()` helper and the pinned `jsonschema`
    0.55.0. Pattern matching uses `fancy-regex`, where `$` is end of text, as
    ECMA-262 requires.
  - The oracle (`parse::<u64>` plus print-back) is independent of the
    pattern, so the assertion can fail. `+1` is correctly treated as
    non-canonical: Rust parses it, but it does not print back.
  - It has no `unwrap` outside test assertions, no lossy cast
    (`usize::from(bool)`), and no I/O.
  - Its trace tag is correct for the clause it verifies.
- **Schema.**
  - `U64String` is anchored. Independent verification found 0 mismatches over
    372,059 samples (SR-2454).
  - `QueryPart` is a closed enum, and the `CertificateLocus` `oneOf` branches
    are disjoint.
  - The object branch is closed (`additionalProperties: false`, both members
    required).
  - The `certificate-rejected` arm's other members are unchanged.
- **Vectors.**
  - The JSON is valid, and each vector name is unique; the runner asserts this.
  - Every adverse vector is one operation on `certificate-rejected-loci`.
  - The owner map covers each new `at`.
  - The new vector's `dispositions` and `accounting` are consistent with its
    six results.
- **No new antipattern.** No pin, digest or tracking record is added. The
  existing `jcs_sha256` and provision digests are untouched by this diff.

I did not rebuild. The author's locked focused run passed 10 tests at
`cff80d0`, and the review needed no further run.

## Verdict

**Clean for code.** The Rust test and the schema and vector data are correct
and idiomatic. The spec-level defects are recorded in SR-2454 (integrity) and
SR-2453 (gap analysis).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
