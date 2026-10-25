---
id: SR-16360
title: "Base review of the closed refinement checked-package wire (IR-1016)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification PR 438 at e757b1a4; FR-375, FR-322, TC-310, and checked-package-v2 schemas"
review_set: base
relationships:
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/TC-310
    type: references
---

# Base review of the closed refinement checked-package wire

## Summary

Independent review of QSpec PR 438 at e757b1a4 against FR-375, FR-322,
TC-310, the two checked-package-v2 schemas, and the existing dependency and
identity rules. No applicable `AssuranceProfile` was found in the repository's
authored spec tree. The review covered the PR diff and its direct normative
context; it did not claim to qualify an IR decoder implementation.

The closed `RefinementKey` shapes distinguish a local `NodeRef` from a direct
dependency `PackageNodeKey`. A current-package `package_id` is forbidden in the
body, avoiding a package-identity cycle. The selected profile is a
`DefinitionRef`, not an alias or graph node. The six arrays and their row
members are closed, and `{term: "open"}` is confined to step arguments. The
preimage schema permits `group_reference` in expressions while the package
schema uses `SemanticTerm`.

Both changed JSON schemas pass Draft 2020-12 metaschema validation and all
local `$ref`s resolve. Quire validation passes for FR-375 and TC-310. FR-322
still reports its pre-existing missing `id` and typed-fields defects; neither
was introduced by this PR. `git diff --check origin/main...HEAD` passes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolved at e757b1a4: the first draft's `{version, body}` node preimage omitted the concrete model carried by `semantic_type`, allowing equal node IDs for distinct row-empty concrete subjects. The revised `RefinementNode` requires `semantic_type`, FR-375 binds it to the emitted node, and TC-310 mutation 24 checks the distinct IDs. | FR-375 checked-package node and AC-7; `node-identity-preimage.schema.json` `RefinementNode`; TC-310 mutation 24 |
| FND-002 | high | Resolved at e757b1a4: the first draft and existing AC-11 required an external abstract protocol clause in local `dependencies`, conflicting with FR-322's local-only rule. FR-375 and AC-11 now distinguish local `NodeRef` dependencies from directly selected dependency-package keys; TC-310 mutations 13 and 25 cover both. | FR-375 Protocol subject, checked-package node, AC-11; FR-322 dependencies; TC-310 mutations 13 and 25 |
| FND-003 | medium | Resolved at e757b1a4: the first draft's dependency sentence named only row keys, omitting a local abstract header key when rows are empty. FR-375 and FR-322 now require the digest-sorted local body-key set, including local `abstract` and `protocol`, and TC-310 mutation 24 checks the empty-row case. | FR-375 checked-package node; FR-322 dependencies; TC-310 mutation 24 |

## Verdict

PASS for the revised spec and schema diff. The three review findings were fixed
within PR 438. This verdict covers the authored wire contract; producer and
reader implementation qualification remain separate work.
