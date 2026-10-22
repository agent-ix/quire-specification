---
id: SR-13644
title: "Failure-domain review of QSpec PR #310 (STD-107) dependency supply order"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@d54caee35ed62feafe6019ef048ec50ea249132c vs main d080afa; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
---

## Summary

Ticket: STD-107 (PR #310, head d54caee35ed62feafe6019ef048ec50ea249132c). The supply rule fixes per-entry order (presence, then binding) and reports one entry. The case that separates that rule from a two-pass rule (all presence checks, then all binding checks) is not in the AC or the TC.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-84 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 supply order | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 procedure | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-84 claims "supplied-package presence then binding" per entry, but its example (two unsupplied entries) and TC-233's (first supplied, second not) both pass under a two-pass reader that checks all presence first. The discriminating case is missing: entry 0 supplied with a stale package and entry 1 unsupplied must refuse `stale_dependency`/`content-mismatch` at entry 0, not `missing_import`/`missing-selection` at entry 1. Add it to AC-84 and TC-233 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1753 |

## Verdict

Medium finding; fix in this PR.

## Dispositions

Disposition pass 1, against head 55916ae23ed2cc7f018a482fe2d51ccd946ecc09.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | verified at 55916ae23ed2 |
