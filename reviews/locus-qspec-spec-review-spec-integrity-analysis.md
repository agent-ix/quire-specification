---
id: SR-2454
title: "Spec integrity review of the certificate-rejected locus wire candidate (FR-331, TC-431, backend-provider-v1 schema)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@cff80d04fa88ecae0831be2ef2fb70adabed1c98 against origin/main a865694e5db4daeb292c83070ecc2f12e0c9656c; spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json; context: spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md, spec/objects/interfaces/FR-412-probability-certificate.md, spec/objects/interfaces/FR-418-zone-certificate.md, spec/functional/ir-proof/FR-354-record-proof-trust-basis.md, spec/test-cases/TC-363-probability-certificate-checker.md, spec/test-cases/TC-371-zone-certificate-checker.md, spec/test-cases/TC-269-proof-trust-basis.md, agent-ix/quire-spec-language@84fba37c8a342218e99aa0f2cb66b89204e3e879 FR-149, FR-163, FR-169, FR-314, FR-338, FR-339"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# Spec integrity review of the certificate-rejected locus wire candidate

## Summary

Ticket: none resolved. The branch `spec/certificate-locus-wire` and the commit
subject "FR-331: type the certificate-rejected locus" carry no tracker id. No PR
exists yet; publication waits for owner approval. Reviewer: claude-opus-5-5, an
independent reviewer that did not author the candidate.

The candidate closes `certificate-rejected.at` from any nonempty string to two
shapes. A query locus is `unrolling`, `base` or `step`. A proof-step locus is
the closed object `{part, index}`, where `index` is a canonical decimal u64
string. FR-331 gains one paragraph that ends "No other locus has a wire
spelling, so a reader SHALL refuse any other `at` before consumption", and it
gains AC-24 and TC-431 BP-11.

Examined, with no finding:

- **The `U64String` pattern.** An independent oracle compared Python
  `re.fullmatch` against an integer range check, with no Rust involved. It ran
  over 372,059 samples: every digit position of `u64::MAX` with each next digit
  and each fill digit, the boundaries near 2^53, 2^63 and 2^64, and 400,000
  random strings of 1 to 22 digits. It found 0 mismatches. Each alternative was
  also read by hand: the digit counts sum to 20, and skipping the zero digits
  at positions 9 and 13 is correct.
- **RFC 8785.** A u64 above 2^53-1 is not exact as a JSON number, so the
  string form is required. The JSON-number index is refused
  (`proof-step-index-number`).
- **Closed shapes.** The `oneOf` branches are disjoint: a string enum and a
  closed object. The extra-member, missing-member, empty-object and
  empty-string cases are each refused.
- **The existing positive locus is kept.** `certification-outcome` result 3
  (`query-mismatch` at `unrolling`) is still admitted.
- **The positive vector agrees with QSL FR-314 Behavior.** `shape-mismatch` is
  at `base`, the first part an `Inductive` basis expects.
- **Structural validation.** `quire validate` 0.36.2 (engine 0.50.2) gives the
  same two FR-331 errors (`value_object` `id`, `semantic.record-invalid`
  `fields`) and one warning as the author reported. The frontmatter and the
  Properties block are unchanged by the diff, so these errors predate the
  candidate. TC-431 passes, and both documents are grammar-clean.

Not checked here: QSL FR-314's step-index definition is owned by the QSL
counterpart and its own reviewer. The questions it raises are listed in the
review evidence report, not as QSpec findings.

## Verdict

**Changes needed.** One high, one medium and two low findings.

The wire shapes and the u64 pattern are correct. But closing `at` makes the
`certificate-rejected` outcomes that FR-331 requires of every non-SMT core
checker impossible to write on the wire. FR-331 still requires those outcomes,
as do FR-412, FR-418, TC-363 and TC-371, and QSL FR-149, FR-163, FR-169,
FR-338 and FR-339. No existing vector carries one of these loci, but that does
not make them unused: the requirements themselves demand them.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-331 still says a rejected certificate settles `certificate-rejected{rule, at}` "for every core checker". Its Certification table lists core checkers whose loci are not SMT query parts: the `exhaustive` closure and component certificates, `exact-value` and `value-bounds` (FR-412), `zone-certified` (FR-418), `closed-form` and `witnessed`. The new paragraph admits only the SMT query and proof-step loci, and makes a reader refuse every other `at`. So FR-412-AC-3 (rejection "naming the failed rule and its state"), FR-418-AC-2 ("its node or component"), TC-363 EC-02/03, TC-371 ZC-02 to ZC-04, FR-411's "naming the failed check and its state", and the QSL checker rejections cannot be emitted on the FR-331 wire. Those QSL rejections are FR-338/339 `ProductState`, FR-149 `SimulationStep`, FR-163 `HyperProductState`, and FR-169 `ModelState` (FR-169-AC-10, V-6 row, which "settles" onto "the FR-331 terminal record"). A conforming producer would have to break FR-331-AC-16, or a conforming reader would have to refuse a required outcome. Before this change these outcomes could be written, because `at` was `Nonempty`. Deferring them to E8a turns them from representable to refused, which breaks the current contract. Fix options, for an owner ruling: (a) also give the other loci a typed wire form now; (b) keep the closed SMT forms and add one typed generic or tagged form for the loci that are not yet specified; (c) amend FR-331's "for every core checker" and every dependent FR/AC/TC at the same time. Option (c) removes required outcomes, so it needs an explicit owner decision. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:80-90; spec/objects/interfaces/FR-331-backend-provider-envelope.md:75; spec/objects/interfaces/FR-412-probability-certificate.md:83-85; spec/objects/interfaces/FR-418-zone-certificate.md:88-89; proposals/backend-provider-v1/schema.json:652-658 |
| FND-002 | medium | No requirement defines the wire spelling of `rule`. FR-331 says only "`rule` the check rule that failed". The wire-spelling table covers proof bases and causes, not the `rule` payload, and the QSL FR-314/FR-338 counterpart names Rust variants (`QueryMismatch`, `ShapeMismatch`, `ProofStepInvalid`, `NotRefutation`). The candidate's normative positive vector introduces three new kebab spellings with no owning text: `shape-mismatch`, `proof-step-invalid` and `not-refutation`. Two producers could write `QueryMismatch` and `query-mismatch`, both admitted by `Nonempty`, and a reader matching on rule would split them. Fix: state in FR-331, or in FR-314 by reference, that `rule` is the kebab-case spelling of the `CertificateRule` variant. Say whether the set is closed (for the SMT rules at least) or open. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:80-82; proposals/backend-provider-v1/vectors.json:1479-1540 |
| FND-003 | low | FR-331 and the schema's `CertificateLocus` description cite "QSL FR-314's `CertificateLocus`". The type is declared in QSL FR-338, as `pub enum CertificateLocus { ProductState, Query, ProofStep }`, and extended by FR-149, FR-163 and FR-169. FR-314 only lists the SMT variants. FR-331 now delegates normative meaning to QSL FR-314 (the step index), but it has no `relationships` edge to it, while it does carry other cross-repo edges (quire-contract-ir FR-029). Fix: cite FR-338 for the type and FR-314 for the index, and add a `references` edge to QSL FR-314. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:84-88; proposals/backend-provider-v1/schema.json:653 |
| FND-004 | low | FR-331 does not say whether a reader refuses a rule paired with the wrong locus shape. Examples are `query-mismatch` or `shape-mismatch` at a proof step, or `proof-step-invalid` or `not-refutation` at a query part. QSL FR-314 Behavior fixes the pairing for the producer only. The schema admits every pairing, and FR-331's reader rule ("refuse any other `at`") is silent on it. Readers can therefore differ on a mismatched pair. Fix: either state that the pairing is not checked on the wire, or add it once `rule` has a defined spelling (FND-002). | spec/objects/interfaces/FR-331-backend-provider-envelope.md:84-90; proposals/backend-provider-v1/schema.json:664 |
