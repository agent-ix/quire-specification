---
id: SR-16187
title: "Spec review of quire-specification PR #387: STD-333, stale EN-1 closure and component certificates"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@39b923752; PR #387 (git diff origin/main...39b923752): spec/objects/interfaces/FR-331-backend-provider-envelope.md, spec/test-cases/TC-431-backend-provider-wire-vectors.md, proposals/backend-provider-v1/vectors.json, tests/backend_provider_v1.rs"
review_set: subset
evaluated_revision: "39b923752"
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
---
# SR-16187: Spec review of quire-specification PR #387: STD-333, stale EN-1 closure and component certificates

## Summary

Ticket: STD-333. Reviewed `git diff origin/main...HEAD` at 39b923752 against QSL FR-282 (Behavior bullets 1-2, FR-282-AC-9), FR-338, FR-339 and FR-127 at QSL main 2c5d82d37 (which includes #1271, STD-296, on FR-338), and FR-331's existing `stale-dependency` scope and BP-26. The FR-331 wording is behaviour-only and the lead defaults are flagged, including the open question on the other checkers. The vector and the `without-requested` adverse vector are well formed, and the two `#[trace("TC-431", "FR-331-AC-36")]` attributes sit on the tests that iterate the vectors. Ids FR-331-AC-36 and BP-32 are unique across main and every open PR (#384 adds no FR-331 AC). Gates at the head: validate 86 failure lines (main 86); cargo 23 + 119 passed. QSL conformance `cargo test -p qsl-package --lib checked_v2` (QSL main 2c5d82d37, QSPEC_DIR at this head): 43 passed, 3 failed, the same three `extension_definition_*`/`published_extension_definitions_*` tests that fail identically with QSPEC_DIR at QSpec main (control run), caused by QSpec #365's absent file; QSL reads no backend-provider-v1 file. Cleanest merge order of the three wire PRs: #384, #382, #387. Merge: conflicts textually with #382 (vectors.json tails, TC-431 table tail, trace attributes in tests/backend_provider_v1.rs) and with #384 (vectors.json, TC-431). The FR-331 hunks of #384 and #387 merge cleanly.

Method: quoin spec-review (base), with integrity, EARS, criterion-strength (by hand: no Jev client) and failure-domain checks folded in; code-review of the `tests/` change where present is folded in here. Gap-analysis does not apply (spec, vectors and trace tags only).

Units examined:

- `FR-331 stale-dependency scope` (examined): spec/objects/interfaces/FR-331-backend-provider-envelope.md
- `FR-331-AC-36` (examined): spec/objects/interfaces/FR-331-backend-provider-envelope.md
- `TC-431 BP-32` (examined): spec/test-cases/TC-431-backend-provider-wire-vectors.md
- `TC-431 BP-26` (context_only): spec/test-cases/TC-431-backend-provider-wire-vectors.md
- `QSL FR-282 Behavior` (context_only): agent-ix/quire-spec-language:spec/functional/FR-282-check-analyze-certificates-in-the-qualified-core.md

## Verdict

**NOT MERGEABLE as written**: one high finding (the QSL half must land first or together), one medium.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-331 now states that the EN-1 closure (QSL FR-338) and component (QSL FR-339) certificates each name a `package_id` and that their checkers compare it before any other rule. QSL main contradicts this: FR-282 says 'The FR-338 and FR-339 certificates carry no package_id of their own and are bound through their CertificateRequest; the settlement of a package mismatch in those checkers is not stated', and FR-338/FR-339 define no such member or rejection. No open QSL PR touches FR-338 or FR-339, and no QSL ticket for the QSL half is named. Merging this alone puts the two repos in direct contradiction about QSL-owned types. Land it together with, or after, the QSL half the PR body lists (package_id on ClosureCertificate, the checkers' package-mismatch outcome, FR-127's settlement row, FR-282's sentence). | spec/objects/interfaces/FR-331-backend-provider-envelope.md:524-531, agent-ix/quire-spec-language:spec/functional/FR-282-check-analyze-certificates-in-the-qualified-core.md:62-69 |
| FND-002 | medium | FR-331-AC-36's behaviour (the comparison runs before any other rule, the result is never `certificate-rejected` or `proved`, `requested` is the certificate's id) is verified only by TC-431 BP-32, a wire-schema vector. Its two results have the same shape as BP-26's `stale-dependency` entry, and nothing in them marks a closure or a component certificate, so the vector passes whatever any checker does; only the final clause ('the wire entry is admitted') can fail. Either restrict AC-36 to wire admission and put the checker behaviour in a QSL AC (for example, next to FR-282-AC-9 in TC-1141), or name a test that runs the EN-1 checkers. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:706, spec/test-cases/TC-431-backend-provider-wire-vectors.md:BP-32 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | QSL agent-ix/quire-spec-language#1281 (head 44b7a66c, open) states the QSL half the finding asked for: a package_id on ClosureCertificate, carried once by ComponentCertificate through its closure; check_closure and check_components return PackageMismatch{certificate, recompiled} before any rule; FR-127 settles it Inconclusive(StaleDependency) with requested the certificate's package_id; FR-282's 'carry no package_id' sentence is replaced. Checked against this PR's FR-331 text: consistent. The contradiction remains on QSL main until #1281 merges, so #387 stays held until then. |
| FND-002 | fixed | see after_excerpt |

Disposition pass 1 verdict (head 2378f02f): **NOT MERGEABLE yet: hold until QSL #1281 merges**, then rebase and merge last. FND-002 fixed in 2378f02f (AC-36 limited to wire admission; checker behaviour cited to QSL FR-338-AC-11, FR-339-AC-4, FR-282-AC-10, FR-127-AC-13, which #1281 adds). The head conflicts with current main (#384) in vectors.json and TC-431, and will conflict with #382: rebase as main plus this PR's BP-32 rows after #382 lands.
