---
id: SR-555
title: "Dependency review of FS06 output-mapping acceptance"
type: SpecReview
analysis: dependency
scope: "FS06 prerequisites, target standards, optional observers and qcir implementation routing"
review_set: subset
evaluated_revision: "task/7 working tree based on 0dd4384; qcir #55/#56/#57 open"
review_date: "2026-09-14"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-122, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-123, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-124, type: reviews }
  - { target: ix://agent-ix/quire-contract-ir/issue-55, type: references }
  - { target: ix://agent-ix/quire-contract-ir/issue-56, type: references }
  - { target: ix://agent-ix/quire-contract-ir/issue-57, type: references }
---
# Dependency review of FS06 output-mapping acceptance

## Summary

PASS. FS01–FS05 provide the accepted source, identity and semantic prerequisites.
FS06 can accept target mappings before exporter implementation because the
downstream tickets may implement but may not redefine these correspondences.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | low | No acceptance blocker remains: FS01–FS05 are merged and the output profiles explicitly depend on their accepted meanings. | QS #2–#6; FR-120–125 |
| FND-014 | medium | Resolved: FS06 selects OCL `formal/14-02-03`, SysML Language `formal/26-03-02`, KerML `formal/26-03-01` and FRET `v3.1.0`. | sources-and-rights; FR-297 |
| FND-015 | medium | Resolved: target runtimes/libraries could become undeclared qualification dependencies. They are optional downstream observers. | NFR-061; TC-155 |
| FND-016 | low | Qcir #55/#56/#57 are independent post-acceptance implementation lanes. Their completion gates TM-008/IT-060 evidence, not the accepted QSpec semantics. | Plan-007; IT-060 |

## Dependency graph

```text
accepted FS01–FS05 -> common FS06 boundary -> OCL / SysML-KerML / FRETish profiles
                                                  |       |          |
                                               qcir55  qcir56      qcir57

optional parser/tool -> structural observation only
```

No foreign runtime is required to generate or qualify the pure Rust mapping
package. Rights ambiguity refuses only the optional observation.
