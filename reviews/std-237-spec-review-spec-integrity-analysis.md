---
id: SR-14740
title: "spec-integrity analysis of STD-237 (QSpec PR #347)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@6d280af1f597c7cc709cb2f1e673d2e5f467e7ae; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, proposals/checked-package-v2/README.md, proposals/checked-package-v2/dependency-call-vectors.json, proposals/checked-package-v2/fixtures/positive-dependency-library.json, proposals/checked-package-v2/fixtures/positive-dependency-call.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---
# SR-14740: spec-integrity analysis of STD-237 (QSpec PR #347)

## Summary

Ticket: STD-237. Scope is `git diff origin/main...6d280af`. The review
checked the changed FR-322 graph-shape sentence, FR-322-AC-37, FR-322-AC-85,
TC-233's procedure and expected results, the README entry and the three new
published files.

Independent checks, by a script that does not use the harness:

- Every node key of both fixtures recomputes: 18 library nodes and 4 importer
  nodes. Each is the JCS SHA-256 of its `quire.structural-node/v1` preimage
  (`semantic_type` null when self-typed, `owner` present when carried) or,
  when the body holds an application, of its `quire.application-node/v1`
  preimage. The same script reproduces every key of the merged positive
  fixtures except `union_value` and `abstraction_relation` nodes, which have
  their own preimages, so the rule it applies is the published one.
- Both `identity_projection`s equal the nodes without `occurrences`, every
  other identity-preimage member equals the lock member, and both recorded
  `package_id`s recompute (`4376612918ef...` for the library and
  `a423525e9cde...` for the importer).
- Each `dependencies` list is the body's `reference` targets in ascending
  order, plus the bounded type for the `bounded_domain` node. This also
  holds under the rules that open PR #348 adds.
- Every vector digest names the node or package it claims: `ebe64b...` is
  undeclared `value`/`parameter`, `b3f295...` is `Lib::Point`, `fa983d...`
  is `Lib::make_point`, `0deec3...` is `Lib::any_point` (`Set<Point>`),
  `14ef99...` is `Lib::first_small` (a tuple holding `Point`), `f3a9c2...`
  is `Lib::is_open` (`Reference<acme/orders::Order>`), and `09e949...` is
  the `package_id` of `positive-all-families.json`, which the importer does
  not select.

The added locus (`/semantic_graph/nodes/{n}/dependencies`) matches FR-322's
existing `/dependencies` loci for the protocol carrier and the population
link, and FR-272's `invalid_semantic_graph` payload, which carries a
`document_pointer`. No other statement in `spec/` or `proposals/` gives
another locus or calls AC-37 prose-only. No new ids were added. No other open
QSpec PR edits AC-37, AC-85, TC-233's dependency-call paragraphs or the new
files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The fixtures, node keys, `package_id`s and vector targets recompute
independently. The changed normative text agrees with the rest of FR-322 and
FR-272. `quire validate --scope . spec/**/*.md` reports 134 failing
documents, the baseline, and FR-322 and TC-233 have the same diagnostics as
on main.
