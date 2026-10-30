---
id: SR-2026103003
title: "Base review of the derived request item key rule"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; spec/objects/interfaces/FR-322-checked-package-artifact.md (Derived request items, AC-276); spec/functional/temporal/FR-366, FR-368, FR-417; spec/functional/protocol/FR-429; spec/functional/memory/FR-438; spec/test-cases/TC-431 (BP-51); tests/backend_provider_v1.rs (tc_431_a_derived_item_is_keyed_by_its_anchor_node_and_kind)"
review_set: base
---

## Summary

Ticket: none (no tracker id in branch or title); PR quire-specification#541.
Reviewed the diff of `origin/main...origin/spec/package-level-item-node-rule`
at head 4f1079710.

(a) The rule agrees with FR-368 "Claim and anchor of a derived or refinement
item" and FR-368-AC-14: model subjects anchor on the one `model`/`model_root`
node at `generated` 0, protocol subjects on the `protocol`/`protocol_clause`
node at `declaration` 0, and the unanchored cases (FR-368-AC-22, AC-24) write no
request item. It agrees with FR-331 "Items", where `claim` names the anchor
node and occurrence and is never absent. In the `derived-items-request`
vector, items 3 and 4 claim `{acme/orders, [model, model_root], 0}` at
`generated` 0, and items 5 and 6 claim `{example, [Example, Handshake], 0}` at
`declaration` 0. The fixture holds both nodes with exactly those
tag, form and occurrence values.

(b) The vector half of BP-51 is not vacuous. Moving item 4's claim off item 3's
claim, or pointing item 3 at a node of another tag, fails it. The retarget
half checks the test's own `anchor_of` predicate. This repository has no
production reader, and the FR-368-AC-19 test uses the same pattern. The three
retargets each fail that predicate for a different reason (tag, form,
absence). See FND-002 and FND-003 for what the test does not reach.

(c) No digest, pin, stored hash, count pin or prose inventory was added.

(d) The new ids FR-322-AC-276, FR-366-AC-11, FR-429-AC-17, FR-417-AC-7,
FR-438-AC-9 and TC-431 BP-51 are each the next free id on `main`. #539 adds
FR-322-AC-275 and #540 adds no new FR-322 AC, so there is no collision. No other open PR
adds any of these ids.

`cargo check --tests` at 4f1079710 in a detached worktree finishes with exit 0 and no warnings. `cargo test` was not run.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-322 calls (anchor `node_id`, `item_kind`) the key of a derived item and says replay "resolves the item by the same pair", but the same subsection says two subjects can share an anchor (two sets of initial states over one model). Their deadlock-freedom items then have the same pair. The key is not unique, and replay by the pair is ambiguous. Either state the key as (subject, anchor `node_id`, `item_kind`), as the obligation identity of FR-366/FR-368 already does, or say the pair resolves the anchor node and the item comes from `request_index` | spec/objects/interfaces/FR-322-checked-package-artifact.md:1061-1062,1077-1079,1095; spec/functional/temporal/FR-366-check-deadlock-freedom.md:108-112 | wrong-requirement |
| FND-002 | medium | FR-322-AC-276 (and FR-366-AC-11, FR-429-AC-17, FR-417-AC-7, FR-438-AC-9) open with "adds no node to the checked package" / "no `node_tag`, `semantic_form` or derived path", but BP-51 never checks that. It looks up claim nodes only. A fixture or producer that also minted a node for a derived item, for example a node whose path names `deadlock-freedom`, passes BP-51 unchanged. Add an assertion over the package's nodes, or drop that clause from the ACs | tests/backend_provider_v1.rs:1743-1800; spec/objects/interfaces/FR-322-checked-package-artifact.md:3263 | correct-requirement-no-evidence |
| FND-003 | low | FR-322 "Derived request items" says the reader also checks that `claim.occurrence` is the occurrence FR-368 chooses, but no AC states that refusal and none of BP-51's three retargets changes the occurrence: all three keep `generated` 0 and fail on the node. A reader that skips the occurrence check passes every published case | spec/objects/interfaces/FR-322-checked-package-artifact.md:1088-1089,3263; spec/test-cases/TC-431-backend-provider-wire-vectors.md:89 | correct-requirement-no-evidence |

## Verdict

The rule is right and consistent with FR-368, FR-368-AC-14, FR-331 "Items" and
the `derived-items-request` vector. The five new ACs are testable, the IDs do
not collide with #539 or #540, and nothing ceremonial was added. FND-001 is a
wording fix that makes the stated key unique. FND-002 is the one real evidence
gap: the "no node of its own" clause, which is the point of the rule, is not
asserted. FND-003 is a small coverage gap. Not mergeable until FND-001 and FND-002 are
fixed. FND-003 can be fixed in the same round.
