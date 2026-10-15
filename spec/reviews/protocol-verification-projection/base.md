---
id: SR-209
title: "Base review of the protocol-result verification projection"
type: SpecReview
analysis: base
scope: "FR-061, FR-062, TC-061, TC-064, TM-004, IT-021 and the protocol contract"
review_set: all
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-062, type: reviews }
---
## Summary

Base-plus-seven QUOIN review of the B-owned canonical protocol-result and E02
projection amendment found four blocking design defects during drafting. All are
resolved at `07da692`; the canonical result and its narrower verification
projection now have separate ownership, exact version/identity rules, complete
state mappings, typed refusals and planned discriminating tests.

## Verdict

**PASS for integration into the composed specification.** Implementation remains
truthfully gated by adoption of the composed baseline, authorization of private
setup ticket #33 and the accepted Engineering Assurance interface. This review
claims no implementation or executed coverage.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved: a prose-only mapping with different closed state vocabularies was not rebuildable; FR-062 now owns one total projection table and refuses unrepresented source meaning. | FR-062; TC-064 |
| FND-002 | high | Resolved: assumptions and observation adequacy were absent from the canonical source while FR-007 required both, and protocol adequacy could collide with E02 observation adequacy; FR-061 now separates all three facts and FR-062 prohibits cross-wiring. | FR-061; FR-062-AC-1/AC-3 |
| FND-003 | high | Resolved: the first draft placed a digest over the bytes inside those same bytes; FR-061 now puts the recomputed RFC 8785 digest in the enclosing artifact reference and TC-061 mutates the bytes independently. | FR-061-AC-3; TC-061 |
| FND-004 | medium | Resolved: the projection lacked exact source/target versioning, a reproducible derived identity, source-byte retention and stable refusal codes. | FR-062-AC-2/AC-4; TC-064 |

## Checklist result

- IDs are unique and sequential within FR-061, FR-062 and their test cases.
- Both FRs implement US-006; FR-062 depends on FR-061 and the external FR-007.
- Every acceptance criterion has a concrete TC row in TM-004.
- Positive, invalid, boundary, state, identity, digest, lineage, version and
  semantic-collision cases are explicit.
- Inputs, outputs, ownership, failure codes and downstream prerequisites are
  stated without claiming a runner, evidence store or implemented adapter.

## Review set

All seven QUOIN analysis lenses accompany this base review in the same folder.
