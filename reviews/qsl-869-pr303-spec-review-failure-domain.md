---
id: SR-13604
title: 'Spec review of QSpec PR #303: FR-322 population reference operand, failure domain'
type: SpecReview
analysis: failure-domain
scope: 'agent-ix/quire-specification@ab6ff70; git diff origin/main...ab6ff70 (PR #303, branch qspec-fr322-popref): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-280-model-owned-member-types.md, tests/checked_package_v2.rs, tests/model_member_types.rs, proposals/checked-package-v2/model-owner-form-vectors.json; ticket QSL-869'
review_set: subset
date: '2026-10-20'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: QSL-869. PR: quire-specification#303, head ab6ff70. Failure-domain lens: identity confusion, unstated failure modes and purity of the new node. Identity: the key is a function of (package identity, IR node) only, with no digest, version, U, N, clause or operation in it. FR-321 allows at most one selection per identity, so a key cannot name two documents. Two `relation`/`population` nodes with one owner have one node_id, which graph id uniqueness refuses. Population node ids are `ix://<package>/<artifact-id>` (FR-154), so a type and a population cannot share an owner.node. Confusing the two population-family operand forms is ruled out: the reader distinguishes by the referenced node's tag and form, a parameter typed by a `relation`/`population` node fits no operand because a relation node has no type family, and the reaches population link refuses the reference form. Failure modes covered: missing selection, wrong-kind owner, undeclared owner, population-owned other forms, ownerless (malformed_wire) and every non-operand-0 position (ill_typed/operator-ineligible). Purity: the node carries nothing a document change could make stale, apart from the owner.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-population-references-owner-join | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-identity | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-use | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-relation-to-population | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-population-references-quantifier-domain | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-model-owned-step-1 | spec/objects/interfaces/FR-322-checked-package-artifact.md | context_only |
| TC-280-PR-01..PR-06 | spec/test-cases/TC-280-model-owned-member-types.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The owner join admits the node when 'that row's admitted document declares `owner.node` as a population (`quire.meaning.model.population/v1`)'. Nothing says where in the document a population is declared. 'Model-owned members' step 1 (F322:802-812) defines the selected document as `contractVersion`, `package`, `constructs` and `types`, and its projection as 'a derived view that holds no member the document does not'. Every vector and test adds a `populations: [{node}]` member to the projection (model-owner-form-vectors.json `acme/runs`; tests/checked_package_v2.rs:13026; tests/model_member_types.rs:141). The document as specified has no such member, and the meaning id is never checked: an entry carries only `node`. A reader written against a real FCD document cannot run this join, and two readers would guess different locations (a top-level node list, or a `types` entry whose construct meaning is population). PR judgment call 5 says this is ungrounded and needs the FCD owner. Fix: name the FCD location of population nodes, the ones FR-154 intake binds by construct meaning, in step 1 and in the projection, and key the join on it. Or, if the FCD shape cannot be settled in this PR, state the projection member `populations[].node` as the join's input, bound at FR-154 intake by meaning, and open the FCD owner question as a follow-up ticket. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1249-1254 |

## Verdict

One medium finding: the owner join reads a document member that FR-322's own definition of the selected document does not have (FND-001). This can be deferred to a follow-up with the FCD owner if the spec says the projection member is the interim input. Nothing else found: identity, refusal coverage and operand disambiguation hold.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The owner-join bullet now names its interim input, the step-1 projection's `populations: [{node}]` member, and says FR-154 and FCD own the IR location. The FCD-owner question is written up in the PR body's 'Open follow-up for the planner'. Still open for that follow-up: step 1 (F322:802-812) still lists only `contractVersion`/`package`/`constructs`/`types` and says the projection holds no member the document does not, and the meaning id is not checked. The planner should file this as a Linear ticket, which has no id yet. It does not block this PR. |

Round 1 at c58cd9f: FND-001 deferred to the FCD-owner follow-up. The interim join input is now stated.
