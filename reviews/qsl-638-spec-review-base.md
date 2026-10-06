---
id: SR-1299
title: "Spec review of quire-specification PR #190 (QSL-638): checked-package v2 nodes carry their owner"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@0b4f87346d8e46724cfe2e0ee09d77b7f0ca20f5; PR #190 diff against main ece259a; FR-322 (derived-shape key stage, owner member, Owner member schema, Model-owned members step 2, AC-29, AC-50 to AC-54), FR-340, FR-342, FR-370, TC-233, TC-280; checked against quire-spec-language PR #648 at 3260821e (FR-092, FR-093, FR-094, TC-416)"
review_set: subset
---
# Spec review of quire-specification PR #190

## Summary

Ticket: QSL-638. The PR puts each node's identity-preimage `owner` on the v2 wire, so a reader can re-derive every structural key, in-group keys and group digests included, from the package alone. Declared structural nodes carry a `SourceOwner`. Model declaration nodes and clause functions carry a `ModelOwner`. The derived-shape key stage joins each owner to the lock and keys from it. Step 2 becomes a lookup by the wire owner. `correspondence`/`abstraction_relation` is left to FR-451. New ACs 51 to 54 are added, with TC-233 and TC-280 fixture specs.

The review covers the base checklist plus the failure-domain lens (soundness, meaning whether a tampered owner can pass) and the integrity lens (whether every keyed node is covered exactly once, and cross-repo consistency). It was run at PR head 0b4f873. That head includes the plan-lead rulings commit made after the briefed 72d1b3c.

Examined:
- FR-322 derived-shape key stage, the owner join and the abstraction_relation exclusion (examined)
- FR-322 `owner` member and "Owner member schema" (OwnerPresenceRule) (examined)
- FR-322 "Model-owned members" step 2, Declaration lookup (examined)
- FR-322-AC-29, AC-50, AC-51, AC-52, AC-53, AC-54 (examined)
- FR-340 field entries, FR-342 step 4, FR-370 step 2 wording (examined)
- TC-233 owner and abstraction procedure and expected results (examined)
- TC-280 MW-01 to MW-09, MA-05 (examined)
- FR-451 identity and target steps (context_only)
- TC-281 conforming_reference procedure (context_only)
- proposals/checked-package-v2/schema.json DeclarationTagRules, PackageLock, SourceOwner, ModelOwner (context_only)
- QSL FR-092, FR-093 (emission owner rule, AC-21, AC-22), FR-094 (M1 to M5, C1 to C6) (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322-AC-52's second sentence requires a reader to admit a package whose declared node's `declaration` occurrence lies in a different lock source than its `SourceOwner`. The `owner` member says a declared node carries "the `SourceOwner` of the source unit that declares it", so the AC admits a package that breaks FR-322's own rule. The emitted owner and the source map can then disagree undetected, so go-to-declaration lands in a unit other than the owner. Reading the owner only from `owner` does not stop the reader from checking that the two are coherent. Refuse the mismatch, for example as `invalid_package` at the occurrence, and make AC-52 assert that refusal. AC-51's owner-change mutation already shows that the key comes from `owner`. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1001, :386 |
| FND-002 | medium | The owner join checks a `ModelOwner` only by `identity` against `model_selections`. Its `node` is resolved only by step 2, and only when a member resolution, frame, anchor or reference conformance reaches the node. A clause function's `ModelOwner.node` is never resolved at all. A package rekeyed end to end can therefore carry a self-consistent model declaration node whose owner names no declaration, or names a declaration of the wrong kind (MW-07's shape), under a `Reference<T>` that nothing dereferences. It can also carry a clause function owned by a member that does not exist, and all of these are admitted. Resolve every `ModelOwner` in the join: a model declaration node's `node` must be a declaration of the kind its tag and form name, and a clause function's `node` must be an operation member, or an object type for an invariant, of the selected document. Refuse otherwise with `missing_declaration`/`missing-selection`, and add an AC with mutations for both node kinds. | spec/objects/interfaces/FR-322-checked-package-artifact.md:249, :597 |
| FND-003 | low | FR-322-AC-54 says "the same node with its key tampered, node id kept". The node id is the key, so the mutation contradicts itself. TC-233 instead edits the FR-451 body with the node id kept. Restate the AC as a body edit that keeps canonical order and `dependencies` (for example one `rust_type` segment changed), so that FR-451 step 1 (Identity) is the check that fails. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1003, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:210 |
| FND-004 | low | The PR rewrites step 2 from owner recovery to declaration lookup and aligns FR-340, FR-342 and FR-370. Two citing specs still use the removed wording. FR-451 step 3 says "an owner the reader cannot recover refuses with FR-322 step 2's own refusal", and TC-281 says "recover both owners by FR-322 step 2". Align both with the lookup wording. | spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md:104, spec/test-cases/TC-281-model-reference-equality-and-field-projection.md:45 |
| FND-005 | low | "Structural" now has two meanings. The derived-shape stage excludes `correspondence`/`abstraction_relation`, but OwnerPresenceRule's "structural" (no nominal preimage, body term not `application`) includes it. The result is correct only because schema.json's DeclarationTagRules forbids `declaration` on every `correspondence` node. Say so in "Owner member schema", or exclude the form there by name, so a reader does not infer that an abstraction relation needs an owner. | spec/objects/interfaces/FR-322-checked-package-artifact.md:549 |
| FND-006 | low | TC-233 says that every emittable fixture node "is real QSL emitter output". That is a provenance claim no test step can check. The checkable oracle is the independent key recomputation plus admission, so state the emitter run as the fixture-production step and keep key recomputation as the check. A hand-authored node of a kind QSL never writes is to be "keyed by FR-092/FR-094 exactly as emitter output would be", but FR-092 and FR-094 give no rule for kinds QSL never emits. Name FR-322's structural-node rule as the key for those nodes. Regenerating the fixtures depends on QSL #648's emitter writing `owner`, which QSL FR-093 says it does not do yet. Record that dependency as a Linear blocking edge. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:154 |
| FND-007 | low | The ruling that a declared application-keyed node is owner-free (the same value under two owners shares one id) is stated in FR-322's `owner` member, but no AC pins it. Add a declared application-keyed node, such as a declared value whose body root is an application, to the two-owner fixtures. AC-53 can then assert that its id is equal across `a` and `b`, as the `Integer` row does, and QSL FR-093-AC-22 can pair with it. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1002 |

## Verdict

The design is sound at its core.

**Tampered owners.** `owner` sits in each node's `identity_projection` entry, so it enters `package_id`. With the node id kept, a changed owner fails the first pass at that node. In a group it also changes the group digest. An owner that is not in the lock fails the join before either pass. AC-51 to AC-53 test all three.

**Coverage.** Every node is keyed exactly once:
- nominal nodes by the nominal rule
- application nodes by `quire.application-node/v1`
- `correspondence`/`abstraction_relation` by FR-451's identity step
- every other node by this stage, model declaration nodes and clause functions now included

The OwnerPresenceRule table agrees with QSL FR-093's emission rule ("exactly when the preimage carries one"), with FR-094's M1 to M5 and C1 to C6 owners, and with FR-093-AC-22 pairing with AC-53. ACs 51 to 54 and MW-01 to MW-09 are each testable as written, apart from FND-003's wording.

**Removing the undeclared-model/relation exclusion is right.** Before this PR, those nodes were keyed only lazily by the step-2 search, so a model declaration node that no member reached was never checked. Now every one is keyed eagerly in the stage from its wire owner, with the closed ModelDeclarationNode shape check. The remaining gap is that the owner's `node` is still resolved lazily (FND-002).

**Excluding only `abstraction_relation` is right.** `quire.abstraction-relation-node/v1` is the only non-structural preimage version among the correspondence forms. On main, the stage re-keyed that node by the structural preimage, so every valid package carrying one was refused. The other correspondence forms have no preimage of their own, so excluding them would leave them unkeyed.

**The author's open questions.**
1. Owner-free application keys are acceptable. The application preimage still hashes `declaration`, cross-package references are package-scoped, and nothing can forge an owner into or out of a key that does not contain one. Pin the behaviour with a test (FND-007).
2. The ruling (emitter output for emittable kinds, hand-authored only for kinds QSL never writes, TC-280 wire cases hand-authored) satisfies QSL-635's intent. The oracle must stay QSpec's own key rule, not provenance (FND-006).
3. Only `abstraction_relation` is excluded, as argued above.

`quire validate` on the six changed files reports the two FR-322 and FR-340 structural errors (a `value_object` `id` and a `fields` record) that main already reports, so this PR did not cause them.

**Mergeable once FND-001 and FND-002 are fixed.** The low findings are fixed in the same PR.
