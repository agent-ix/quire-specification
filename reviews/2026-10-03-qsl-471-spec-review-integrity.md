---
id: SR-1287
title: "QSL-471/QSL-395 spec review of quire-specification PR #186, imports and supplied libraries select by identity only"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@137820a015374b9e0a23d6fec018244d924d6f7d; PR #186 diff against origin/main (2f846f8): proposals/quire-v1/package-contract.md, proposals/quire-v1/shared-foundation.md, spec/assurance/AD-006-model-graph-state-contracts.md, spec/functional/foundation/FR-002-resolve-owned-declarations.md, spec/functional/foundation/FR-030-bind-composed-definitions.md, spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-227-reusable-semantic-library-resolution.md, spec/test-cases/TC-277-replay-byte-provision.md; leftover sweep over spec/, proposals/, docs/ (reviews/ excluded)"
review_set: subset
---
# QSL-471/QSL-395 spec review of quire-specification PR #186

## Summary

Ticket: QSL-471 (with QSL-395). PR: quire-specification#186, branch
task/fr307-identity-only, head 137820a. Spec-only.

Decided rulings checked against the diff:

- An import is `import "L" [as a];` with no version or digest. FR-307 states
  it, and a leftover `version`/`digest` token is
  `invalid_syntax`/`unexpected-token` at the token (both in the FR-272 /
  native-diagnostics catalog).
- A supplied library is identified by identity only; at most one per
  identity; a second, different package is
  `invalid_package`/`conflicting-definition` retaining the identity and both
  `package_id`s. FR-307, FR-307-AC-6 and TC-227 L09 state it.
- The lock binds the recomputed `package_id`. FR-307 and FR-322
  `dependency_selections` state it.
- Replay dependency entries are `{identity, package_id, sources}` (FR-323,
  unchanged). An edited dependency refuses naming its identity: FR-323 now
  orders the entry-id check (rule 4) before the proved-id check (rule 5),
  matching QSL ADR-015 D-4 (rules 6 and 7 before rule 5).
- A model selection is identity plus `sha256-jcs` digest with no version:
  package-contract, shared-foundation, AD-006, FR-002 and FR-030 updated.

Sub-analyses run, scoped to the diff plus a leftover sweep:

- **Integrity / consistency.** Leftover version/digest-on-import and
  version-on-selection text swept across spec/, proposals/ and docs/. Found
  the grammar production (FND-001), the known boundary.md line (FND-002),
  IT-070 (FND-003) and FR-002's package version (FND-007). Model `model`
  production (shared-grammar.md:109, state-core/profile.md:49) and FR-030's
  "exact import digest" are model-import digests and correct. The
  `revision-mismatch` uses in FR-181/FR-433/FR-439/FR-413 are unrelated
  counterexample replay.
- **Cross-repo consistency with QSL** (worktree qsl-a2-versions at e8a8993:
  ADR-015, FR-099, FR-098, FR-087). Replay codes, causes and order agree:
  QSL rules 1/3/4/6/7/5 map to FR-323 rules 1/2/2/3/4/5 with the same codes
  and causes. Missing-selection, cycle and syntax refusals agree. One payload
  and trigger-point disagreement on the duplicate supply (FND-004).
- **Failure domain.** The new "rule 4 catches an edited dependency" claim is
  shadowed by rule 2 for interface-changing edits (FND-005).
- **Validation.** `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md'
  'reviews/2026-*.md' --summary` on the PR head and on origin/main 2f846f8:
  both 145 documents failed structural validation and 7 grammar findings.
  The only differences are one line-number shift in an existing FR-322
  `ac:non-canonical-shape` warning (849 to 850) and property-extractable
  criteria 1148 to 1147. No new failure.

## Verdict

Changes requested: one high, four medium and two low findings. The FR-307,
FR-322, FR-323, TC-227 and TC-277 edits implement the rulings correctly and
agree with QSL ADR-015 on every refusal code, cause and order. The high
finding is the complete-V1 grammar, which FR-307 names as the owner of every
source form and which still requires `'digest', string` in `import-decl`.
FND-004 is a cross-repo disagreement whose fix may belong on the QSL side,
since the QSpec text is the decided ruling. Dated reviews/ files were left
alone as records.

Note for the QSL side, not a QSpec defect: QSL FR-099:188 and ADR-015 D-1 say
"Remaining work: QSpec FR-307 still states a library version"; this PR
removes it, so that sentence goes stale when #186 merges.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The complete-V1 grammar still requires a digest on every import: `import-decl = 'import', string, 'digest', string, [ 'as', ident ], ';' ;`. FR-307 now says the import is `import "L" [as a];` and that a `digest` token is `invalid_syntax`/`unexpected-token`, and its Dependencies section says the complete-V1 grammar "owns every source form used by this requirement". As written, the owning grammar rejects the valid form and accepts the refused one. Fix: `import-decl = 'import', string, [ 'as', ident ], ';' ;`. | proposals/quire-v1/shared-grammar.md:288; spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md:51-54 |
| FND-002 | medium | boundary.md still says "Resolve imports by domain package identity, exact version and `sha256-jcs` digest". This is the known leftover. It also says "Do not choose ... newest version", which assumes versions. Fix: "Resolve imports by domain package identity and `sha256-jcs` digest", and drop the version clause. | proposals/state-core/boundary.md:12-14 |
| FND-003 | medium | IT-070-SC-02 expects the checked package's `model_selections` to "name the domain package by identity, version and `sha256-jcs` digest". FR-322-AC-35 refuses a `ModelRef` carrying a `version` member as `unknown_member`, and the ruling is identity plus digest. The scenario's expected result is one the reader refuses. Fix: "by identity and `sha256-jcs` digest". | spec/integration/IT-070-artifact-bundle-to-checked-package.md:50-51; spec/objects/interfaces/FR-322-checked-package-artifact.md:866 |
| FND-004 | medium | QSpec and QSL disagree on when the duplicate-supply refusal fires and what it retains. FR-307, FR-307-AC-6 and TC-227 L09 refuse "a second, different package of the same identity" retaining "the identity and both packages' `package_id`s". QSL ADR-015 D-1 and FR-099-AC-3 refuse "a second library supplied under an identity already supplied" while building the dependency input, before any compile, "naming both" libraries. At that point no `package_id` exists, and QSL refuses whether or not the two differ. The code and cause agree; the trigger and payload do not. Fix: QSL compiles both supplied libraries before refusing and retains their `package_id`s (QSL side, matching the decided ruling), or QSpec states that the refusal retains the two supplied sources. | spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md:65-68; FR-307-AC-6; spec/test-cases/TC-227-reusable-semantic-library-resolution.md:40; quire-spec-language ADR-015 D-1, FR-099-AC-3 |
| FND-005 | medium | FR-323 rule 4 says "this is the rule that catches an edited dependency", and BP-07 expects every `package_id`-changing edit to refuse by rule 4. An edit that removes the export the proved package calls, or changes its parameter or result type, makes the recompile itself refuse (`missing_declaration` or `ill_typed`, in the importing unit) at rule 2, before rule 4 runs. QSL D-4 has the same order (rule 4 recompile before rules 6 and 7), so the repos agree; the QSpec claim and the vector are too broad. Fix: BP-07's edit changes a declaration the proved package does not call, or the body of the called function, and the rule 4 sentence says it catches an edit the recompile admits. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:52-57; spec/test-cases/TC-277-replay-byte-provision.md:44 |
| FND-006 | low | TC-227 L04 says "A and B each import L with the same `package_id`". An import no longer carries a `package_id`; in one compile both imports select the one library supplied under `L`. Fix: "A and B each import L, supplied once, and call `l::f`". | spec/test-cases/TC-227-reusable-semantic-library-resolution.md:36 |
| FND-007 | low | FR-002's Inputs still list "Package identity, exact version". No package version exists in FR-322's identity preimage or the package contract, and the PR edited this line for the domain package version but kept this one. Fix: drop "exact version". | spec/functional/foundation/FR-002-resolve-owned-declarations.md:17 |

## Dispositions

Round 1, reviewed at 11fd601 (fix commit 11fd601 over 137820a). FND-004
follows the lead's ruling: QSpec adopts QSL's spine rule (ADR-015 D-1,
FR-099-AC-3). `quire validate` on 11fd601 against origin/main 2f846f8: no
new failure (145 failed documents on both; the added review file is
grammar-clean, 951 documents against 950).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 11fd601 |
| FND-002 | fixed | 11fd601 |
| FND-003 | fixed | 11fd601 |
| FND-004 | fixed | 11fd601 |
| FND-005 | fixed | 11fd601 |
| FND-006 | fixed | 11fd601 |
| FND-007 | fixed | 11fd601 |
