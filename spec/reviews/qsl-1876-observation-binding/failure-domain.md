---
id: SR-202611101500005
title: "Failure-domain review of QSL-1876 observation binding derivation"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification; spec/objects/observation/FR-261-observation-binding.md (PR #648 diff)"
review_set: subset
---
## Summary

Ticket: QSL-1876. Failure-domain review of the repeat-occurrence derivation. The
rank rule has no correction exclusion and no tie or late-admission rule, so the
same record can get different ordinals, and a correction counts as an extra
occurrence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A correction (FR-296) keeps the predecessor's binding identity and subject and carries a new source ordinal, so it is a second admitted observation in the same group; the rank rule counts predecessor and correction as two occurrences instead of one | spec/objects/observation/FR-261-observation-binding.md:35-39, spec/objects/observation/FR-296-admitted-observation-identity-preimage.md:65-66 |
| FND-002 | medium | "the monitor's progress order for that clock family" is defined nowhere in spec/; it gives no rule for records at equal progress position (equal event position or timestamp), and it does not say that the rank is fixed for one selected admitted set, so late admission renumbers later occurrences | spec/objects/observation/FR-261-observation-binding.md:35-39, spec/objects/observation/FR-268-progress-and-late-data-authority.md |

## Verdict

FAIL. Identity confusion between instances is handled correctly by the FR-287
subject. Occurrence ordinals need a correction rule (rank over correction chains
or heads only) and a total order with ties named (for example progress position,
then source ordinal) over a named admitted set.
