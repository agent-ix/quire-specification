---
id: SR-13066
title: "Dependency review of QSpec PR #285 (STD-199)"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-specification@4d0b395; spec/functional/analysis/FR-375-declare-model-refinement.md, spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md, spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md (context); QSL FR-147/FR-135 at b41d0a9 (context)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-432
    type: reviews
---

## Summary

Ticket: STD-199. Checks the edges the new internal-list input creates (FR-375 to FR-432, FR-322, FR-323, FR-272, FR-307) and compares with the QSL side merged in quire-spec-language #1016 (FR-147, FR-135 at b41d0a9).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-375-internal | spec/functional/analysis/FR-375-declare-model-refinement.md | examined |
| FR-307 | spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md | context_only |
| FR-432-spell | spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md | context_only |
| FR-375-AC-14 | spec/functional/analysis/FR-375-declare-model-refinement.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-375 makes a per-library `refinement_internals` value an input to each supplied library's compile (so it changes that library's `package_id` and the importer's lock selection), but FR-307, which owns supplied libraries, still defines a supplied library as "one library's source, offered under its identity `L` alone" compiled "through the same stages". FR-307 is neither amended nor linked, so a reader of FR-307 cannot learn the library compile has a second input. Amend FR-307's supplied-library definition (and FR-375's frontmatter) or state the library list there. | spec/functional/analysis/FR-375-declare-model-refinement.md:144-156; spec/functional/tooling/FR-307-package-reusable-semantic-libraries.md:55-62 |
| FND-002 | low | New prose dependencies are not recorded as edges: FR-375's Dependencies section adds FR-272 and its text now relies on FR-323 replay, but its frontmatter has no FR-272 or FR-323 relationship; FR-432's Dependencies and References do not mention FR-375 as the owner of the host/lock/replay rule or STD-199; FR-322's frontmatter has no edge to FR-375 or FR-432 although its new lock member cites both. Blast-radius queries on FR-272/FR-323 miss FR-375. | spec/functional/analysis/FR-375-declare-model-refinement.md:1-25,318-322; spec/functional/protocol/FR-432-map-every-protocol-step-in-refinements.md:192-205; spec/objects/interfaces/FR-322-checked-package-artifact.md:1-70 |

## Verdict

Mergeable on this lens after FND-001; FND-002 is bookkeeping. QSL consistency: the shape, the pointer loci, absent-is-empty, byte ordering, the list-vs-row `conflicting-binding` rule, the three support-set feature names and their spans all match QSL FR-147/FR-135. Follow-ups for the QSL lane once this merges (not defects of this PR): QSL FR-147's spelling table lacks timeout and fence nodes, the `cattempt` template path and fork/join/cend/spawn/retire/activate, and FR-147-AC-6 still expects `spawn Spec::Twice::r` to be `malformed-declaration` where QSpec now spells `spawn` (it becomes `missing-name` when no role `r` exists); FR-147's "Remaining work" (host, node member, lock/replay) and FR-135's "QSL-proposed pending QSpec" become settled; FR-147's "not by a binding refusal" rule for a model-side list holds only while `refinement-internal-list` is unbuilt, since QSpec refuses a model-side list `malformed-declaration` once the feature exists.
