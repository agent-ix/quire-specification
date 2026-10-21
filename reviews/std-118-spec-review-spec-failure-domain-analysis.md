---
id: SR-13666
title: "Failure-domain review of QSpec PR #320 (STD-118) replay selected item"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@b1c0365; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---

## Summary

Ticket: STD-118. Refusal order, identity confusion and edge cases of the new selected-item rules, read against FR-322 `source_map`, FR-322-AC-87, FR-340, FR-342, FR-375 and QSL ADR-017 PF-3/PF-4 and FR-116. The claim that a frame request's occurrence key is the frame node's `generated` occurrence matches ADR-017 PF-4 and FR-116 word for word.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-323 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-executor | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-AC-18 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| TC-278-RE-32 | spec/test-cases/TC-278-replay-envelope-members.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The refusal order is open on both sides. "Before consumption" places the both/neither/mismatch refusals nowhere relative to the byte-provision refusals ("before any recompile") and dependency rules 1 to 5, so a request with both members and a missing provision entry has two possible first refusals. The undeclared refusal resolves "against the recompiled package" but is not placed after rule 5's `package_id` check; QSL FR-116 resolves the operation only after the stale `package_id` rule, and a stale package makes the name lookup meaningless. State: member-shape refusals at decode, before the byte-provision rule; resolution after rule 5. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:137-142 |
| FND-002 | medium | "A `frame_operation` by `object` and `operation`, refusing ... when the package declares none" leaves the other outcomes of FR-342's operation resolution open: an `object` naming a subtype that only inherits the operation (FR-342 anchors it at the declaring type and refuses `invalid_model_binding`/`malformed-declaration`), a name two supertypes expose (`ambiguous_declaration`/`ambiguous-name`), an `object` naming a record value type or other non-declaring node, and an operation with no frame node (QSL FR-116 refuses `missing-name` for "no operation with a frame node"). Resolve by FR-342's operation resolution and its refusals, and add RE-32 cases for an inherited and an ambiguous operation. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:139-142 |
| FND-003 | medium | "The `generated` occurrence of the operation's frame node" assumes one such occurrence with a defined ordinal. FR-322's `generated` role requires a compiler-derived node with exactly one owner declaration node, the ordinal being its position among that owner's derived nodes; FR-322-AC-87 and FR-375 define that order only for refinement steps. A frame node has no operation node to own it (FR-322 l.255-256), and FR-342 l.53-57 shares one frame node among every operation of a declaring type with an equal frame, so neither the owner nor the ordinal is fixed and a shared frame may carry one generated occurrence per operation. Fix the frame's owner and ordinal in FR-340/FR-322 (or say the occurrence is the one at ordinal 0 of the declaring type's frames in node-id order) before a request can carry it. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:132-134 |

## Verdict

0 high and 3 medium finding(s) in this lens; not mergeable until the medium findings are fixed or dispositioned.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | Step 2, "the byte-provision refusals above, then dependency rules 1 to 5", newly orders two existing rules that FR-323 states only as "before any recompile": a request whose `dependencies` are misordered and whose provision lacks an entry now refuses `missing_import`/`missing-selection`, while QSL FR-098 checks dependency order "before it checks any source count or builds any dependency input", and TC-277 has no vector combining the two. Either keep step 2 as "the existing byte-provision and dependency rules, in their own order" or state the order and add the TC-277 vector. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:153 |
| FND-005 | medium | A frame request that carries `arguments` or an occurrence key has no stated outcome: AC-18 says a frame request carries neither, but the refusal list names no refusal for one that does, so an executor may ignore the extra member, which is the second carrier PF-4 removes. Refuse it at decode (for example `invalid_runtime_input`/`unknown-member` at `/replay/arguments` and at the occurrence key's pointer) and add a TC-278 row. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:139-141, 503 |
| FND-006 | low | RE-35's second case, "an operation no clause or attempt names (no frame node)", relies on QSL's rule that frame and anchor nodes exist only for an operation a clause or attempt names (QSL FR-105, ADR-017). QSpec FR-340 and FR-342 do not say when a frame node is emitted, so QSpec alone cannot build the case. Cite QSL FR-105 in the row or state the emission rule in FR-340. | spec/test-cases/TC-278-replay-envelope-members.md:102 |

## Dispositions

Round 1, head c0543ce (3470fee, c0543ce).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
