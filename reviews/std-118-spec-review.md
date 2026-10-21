---
id: SR-13663
title: "Spec review of QSpec PR #320 (STD-118) replay selected item"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@b1c0365; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---

## Summary

Ticket: STD-118 (CG packet member split to STD-245). Base checklist over the FR-323 selected-item paragraph, the executor refusal paragraph, FR-323-AC-6 and the new FR-323-AC-18, and TC-278's description, precondition, RE-02 and the new RE-30..RE-32, read against merged QSpec FR-322, FR-340, FR-341, FR-342, FR-271/FR-272 and QSL main (4639aa67d) ADR-017 PF-3/PF-4/Q-4, ADR-013 O-26 and FR-116. The replay request has no JSON schema under proposals/ and no test under tests/ (grep for `frame_operation`, `TC-278`, `FR-323` and `native-runtime` over proposals/, tests/ and src/ finds none), so every TC-278 row, old and new, is prose only and nothing could be mutation-tested; `cargo test --test checked_package_v2 --test backend_provider_v1` passes (114 + 14) and is unaffected. `quire validate --scope . 'spec/**/*.md'` gives 134 failures on both main and the head, identical modulo line numbers; FR-323's frontmatter/record failures are pre-existing and the touched files add none. No open QSpec PR touches FR-323 or TC-278, and FR-323-AC-18 and RE-30..RE-32 are free.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-323 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-executor | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-AC-6 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-AC-18 | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| FR-323-selection | spec/objects/interfaces/FR-323-native-runtime-envelope.md | examined |
| TC-278-description | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-RE-02 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-RE-30 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-RE-31 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-RE-32 | spec/test-cases/TC-278-replay-envelope-members.md | examined |
| TC-278-expected-results | spec/test-cases/TC-278-replay-envelope-members.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The mismatch refusal has nothing to compare against. AC-18 and the executor paragraph refuse "the member that does not match the counterexample's kind", but no FR-323 `replay` member carries that kind: the request carries an opaque originating counterexample identity and expected verdict, and FR-331's `CanonicalCounterexample` (backend-provider-v1 schema `$defs/CanonicalCounterexample`) has no frame arm at all. The executor can only infer the kind from the selected item itself, which makes the mismatch case unobservable, and RE-31's third case (a function counterexample's request carrying `frame_operation`) cannot be built as a mismatch rather than as an ordinary frame request. Name the member that fixes the kind (for example the obligation identity's record kind or an explicit counterexample-kind member) or drop the mismatch clause. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:137-139, 481 |
| FND-002 | high | "`arguments` are keyed by the operation's parameter node ids" names ids that do not exist uniquely. FR-322 (l.253-256) gives a model member, an operation included, no node of its own; a `value`/`parameter` node is a state-clause binder whose key hashes its `name`, `level` and `semantic_type` (FR-341 l.129-139). For an operation with a result, a parameter is level 1 in its precondition and level 2 in its postcondition, so it has two node ids; an operation that no clause names has no parameter nodes at all (QSL ADR-017: frame and anchor nodes exist only for an operation a clause or attempt names). QSL FR-116 also carries the frame invocation's arguments in the `invocation` document, so keyed arguments are a second carrier of one identity, the G-2 hazard PF-4 removes. Either say which binder's nodes key them, or state that a frame request's arguments come from the QSL/CG invocation and carry none here. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:133-135 |
| FND-003 | medium | `object` is "the `QualifiedName` of the model object type that declares the operation", but FR-340's `semantic_type` and FR-342's `context` admit a `model`/`process` or `model`/`state_machine` declaring type, and FR-342 gives a process or state-machine operation its own anchor and frame node. As written a counterexample of a process or state-machine operation's frame cannot be named. Say "the object type, process or state machine that declares the operation" (QSL FR-116 has the same narrowing, so QSL-331 should follow). | spec/objects/interfaces/FR-323-native-runtime-envelope.md:130-132 |
| FND-004 | medium | The PR defines the italic *selected item* as the `replay` member, while `results` (l.109, "one ordered typed disposition per selected item") and FR-323-AC-1 ("Every selected item yields exactly one ...") already use "selected item" for `selection` entries, and `selection` stays "ordered clause/function identifiers" with no rule for what a frame replay request selects. QSL ADR-017 Q-4 and STD-118 ask for FR-323's `selection` for a frame packet. Rename the new term (for example *replay target*) and state what `selection` holds on a frame replay request, or state that the replay target replaces it there. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:94, 109, 124 |
| FND-005 | low | RE-01 and RE-02 still name the member "`QualifiedName`"; the member is now spelled `function`, and removing it from a function request is now AC-18's "neither" case. Say `function` in both rows. | spec/test-cases/TC-278-replay-envelope-members.md:68-69 |
| FND-006 | low | Expected Results describes RE-01..RE-29 and says nothing of RE-30..RE-32. Add one sentence for the admitted frame request, the three member refusals and the undeclared operation. | spec/test-cases/TC-278-replay-envelope-members.md:101-115 |
| FND-007 | low | FR-323 now depends on FR-340 (frame node), FR-342 (operation resolution) and QSL FR-116 / ADR-017 PF-4 (the frame packet), but its `relationships` and Dependencies list none of them. Add FR-340 and FR-342 as `references` and cite QSL FR-116 and ADR-017 PF-4 in Dependencies. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:6-12, 486-504 |

## Verdict

Not mergeable: two high findings (the mismatch refusal has no carrier for the counterexample's kind; the frame request's argument keys do not exist uniquely) and two medium. Correct as written: the two-member split keeps the function request's spelling, the absent-not-null rule, AC-6's edit, the RE-30..RE-32 binding to AC-18, the `generated` occurrence matching QSL ADR-017 PF-4 and FR-116, and `missing_declaration`/`missing-name` being an FR-272 pair. Outside this PR: QSL ADR-013 O-26 (l.893), T-8 and FR-070/FR-098 still say "the selected function's `QualifiedName`"; QSL-331 should follow once this lands.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | Leftover of the dropped frame members: the `replay` property bullet (l.121-122) still lists "the failing node's occurrence key" for every replay request, and FR-323-AC-6 still lists arguments and the occurrence key unconditionally, while the new paragraph and FR-323-AC-18 say a frame request carries neither. TC-278's Description (l.15-17) repeats AC-6. Condition both members on a function request in the bullet, AC-6 and the TC-278 Description. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:121-122, 491 |

## Dispositions

Round 1, head c0543ce (3470fee, c0543ce).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | still-open | The RE-30..RE-35 sentence was appended under `## Status` (TC-278 l.133), not `## Expected Results`, which still describes RE-01..RE-29 only. Move it into Expected Results. |
| FND-007 | fixed | - |
