---
id: SR-202611141457005
title: "Dependency review of the STD-665 statistical.workers registration"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-specification; PR #654 at a5e9aa28, diff origin/main...HEAD: spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md (Inputs, Workers section, FR-408-AC-15..17), spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md (Behavior 5), spec/objects/interfaces/FR-331-backend-provider-envelope.md (request member list), spec/test-cases/TC-356-regenerative-long-run.md (WK-01..04); compared with QSL FR-191 and FR-255 on agent-ix/quire-spec-language main"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-408, type: references }
  - { target: ix://agent-ix/quire-specification/FR-461, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
  - { target: ix://agent-ix/quire-specification/TC-356, type: references }
---

## Summary

Ticket: STD-665. The diff adds no `relationships:` edge. FR-408 already references FR-331 and is
referenced from FR-331; the cross-repo citations of QSL FR-191 and FR-255 are prose, as is house
practice for QSL citations. The amendment is feature work (a request setting and its semantics);
it has no enablement prerequisite in this repo other than the schema amendment base FND-001 names.
The dependent QSL-1917 is blocked on this ticket, as the ticket records.

## Verdict

PASS with one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-461 now cites FR-408 "Workers" as the source of its new sentence but has no `references` edge to FR-408, while it carries edges for the other FRs its behaviors cite (FR-300, FR-301, FR-323) | spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md:1-17,87 |
