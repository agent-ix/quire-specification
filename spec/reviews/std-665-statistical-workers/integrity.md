---
id: SR-202611141457004
title: "Integrity review of the STD-665 statistical.workers registration"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #654 at a5e9aa28, diff origin/main...HEAD: spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md (Inputs, Workers section, FR-408-AC-15..17), spec/functional/foundation/FR-461-name-the-limit-value-and-setting.md (Behavior 5), spec/objects/interfaces/FR-331-backend-provider-envelope.md (request member list), spec/test-cases/TC-356-regenerative-long-run.md (WK-01..04); compared with QSL FR-191 and FR-255 on agent-ix/quire-spec-language main"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-408, type: references }
  - { target: ix://agent-ix/quire-specification/FR-461, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
  - { target: ix://agent-ix/quire-specification/TC-356, type: references }
---

## Summary

Ticket: STD-665. Structural consistency of the amendment: new AC ids FR-408-AC-15..17 are unique
(`acceptance_criterion_ids` test passes), each names TC-356 and TC-356 carries a matching vector,
the TC-356 description is updated, and the FR-331 member list agrees with FR-408 Inputs.

## Verdict

PASS with two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-408 states the worker semantics as its own SHALLs and also says "QSL FR-191 'Workers' is the authority for these semantics", while QSL FR-191 and FR-255 say QSpec FR-408 owns the statistical settings; with two normative texts that already differ (the look-ahead bound, base FND-002) nothing says which one governs | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:330-332 |
| FND-002 | low | The "same results" bullet requires equal findings and verdict, but FR-408-AC-15 and WK-01 list decision, counts, estimates, witnesses, provenance, certificates and stop only, so findings and verdict equality is untested | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:324-326,358 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 86ac612d: the "authority" bullet is replaced by a rule that this section governs the statistical settings where the texts differ, consistent with QSL FR-191/FR-255 saying QSpec FR-408 owns them (the wording of that rule is ears-conformance FND-002). |
| FND-002 | fixed | 86ac612d: FR-408-AC-15 and WK-01 now include findings and verdict. |
