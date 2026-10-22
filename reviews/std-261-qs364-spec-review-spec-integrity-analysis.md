---
id: SR-15450
title: "integrity review of STD-261 FR-323 state_clause replay target (QSpec PR #364)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@b5a2d15c0155235a398ec330d8892e19f4919973; spec/objects/interfaces/FR-323-native-runtime-envelope.md, spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-278
    type: reviews
---
# SR-15450: integrity review of STD-261 FR-323 state_clause replay target (QSpec PR #364)

## Summary

Ticket: STD-261. Diff reviewed: `git diff origin/main...b5a2d15c0` (FR-323 and TC-278 only), read
together with QSL PR #1211 (f23314833) and QSL PR #1212 (4fabd79f9), which consume it.

Checks:

- Decision (b), a third `state_clause` member, against option (a). FR-323 tells request kinds apart only
  by which target member is present, and rule 1 refuses a request with no target at decode. Under (a) a
  state-clause request would need a new discriminator and an exception to AC-19. The rejection of (a)
  is sound, and (b) is the smaller surface: one member, one resolution sentence, and the existing
  decode, rule-5 and frame-precedent rules unchanged.
- Decode pointers: the "last member present in the order function, frame_operation, state_clause" rule
  keeps the existing function+frame_operation pointer, and every pair with `state_clause` gives
  `/replay/state_clause`. AC-19, RE-38 and RE-39 agree with the prose.
- Resolution: `state_clause` resolves after rule 5, a function's name refuses `missing-name`, and
  envelope clause staleness is left to QSL FR-122. This matches QSL FR-122 at #1211's head and FR-109's
  single name table.
- An operation's pre/postcondition selects by `state_clause`, not `frame_operation`, which matches FR-341
  (separate `state_clause` and `frame` nodes).
- New ids FR-323-AC-21 and RE-37..RE-41 are unique on main and across the open QSpec PRs (#267, #207, #59
  touch neither file).
- Behavior-only language: there is no status or changelog text.
- QSL test parsing: QSL tests read QSpec FR-322, TC-195 and the `proposals/` fixtures. They do not read
  FR-323 or TC-278.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | minor: `state_clause` is "the `QualifiedName` a state clause ... is declared under". That can be read as the name of the context the clause is declared under (its object type or operation), not the clause's own name. QSL FR-106's `ClauseSelection` and FR-122 name the clause itself (`VersionUnchanged`). Say "the state clause's own declared `QualifiedName`". | spec/objects/interfaces/FR-323-native-runtime-envelope.md:140-146 |

## Verdict

Mergeable. One minor wording finding. Option (b) is sound, and the decode order, pointers and resolution agree with QSL FR-122 (#1211) and with FR-070, FR-071 and FR-098 (#1212).
