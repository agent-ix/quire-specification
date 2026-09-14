---
id: NFR-051
title: "Preserve deterministic observation assessment inputs"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-specification/FR-110
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-114
    type: constrains
---
## Statement

The observation service SHALL make each settled assessment reproducible from its immutable admitted observations, bindings, captures, progress/closure inputs, and selected semantic revisions.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Differing settled dispositions for identical admitted inputs | 0 | 0 | metamorphic-testing |
| Assessments missing an input identity | 0 | 0 | contract-testing |

## Verification

Repeat batch and incremental assessment with byte-identical admitted inputs and
compare structured dispositions and identities. Mutate one retained identity at
a time and verify that it is visible in the resulting subject/provenance or
refusal. Omit each required identity while holding any otherwise equal truth
constant; incomplete provenance must fail this metric rather than pass because
the disposition matches.

## Status

Accepted by FS04 for immutable replay/reproducibility inputs and outcomes.
Cross-repository conformance remains separately tracked.
