---
id: SR-9134
title: "Integrity review of the TL-46 rulings (role internality, choice-visibility carrier, await false-constraint truth)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification PR #649 at 921f42368a59fb6adb31c6743beec46d78acef95; diff origin/main...HEAD: FR-049, FR-050, FR-061, FR-172, FR-211, FR-217, FR-479, TC-050, TC-061, TC-174"
review_set: subset
review_date: "2026-11-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-172, type: references }
  - { target: ix://agent-ix/quire-specification/FR-479, type: references }
  - { target: ix://agent-ix/quire-specification/FR-049, type: references }
  - { target: ix://agent-ix/quire-specification/FR-061, type: references }
---

## Summary

Tickets: QSL-1872, QSL-1874, QSL-1871. Cross-file consistency of the four
rulings against the unchanged FRs they lean on (FR-061, FR-227, FR-288,
FR-464, FR-465, FR-466, FR-479) and the sibling repos (QSL FR-384-AC-7,
FR-519; quire-protocol FR-151, FR-019). Ruling 1 (internality as a binding
fact) is consistent with FR-288 (runtime configuration values are not key
members), FR-465 / FR-465-AC-8 and QSL FR-384-AC-7 / FR-519. Ruling 4 is
consistent with FR-061. The defects are in ruling 2 (the carrier's reach path
names a choice the activation relation row does not hold, and the package
carrier of the FR-227 record is unstated) and ruling 3 (open-progress
`pending` conflicts with FR-061's open-scope decisive-counterexample row).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-172-AC-6 / carrier section reach the visibility record "through the choice's occurrence in the activation relation row", but the FR-464 row is (clause key, `origin` or `each`+trigger value/parameter node key, temporal clause key) and FR-479 derives the control occurrence from clause and temporal keys only; no choice is in the row, so which choice's record an activation preserves is undetermined. | spec/functional/protocol/FR-172-bind-choice-obligation-captures.md:46-60, spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:209,256-259, spec/objects/protocol/FR-479-obligation-activation-binding.md:69-74 | wrong-requirement |
| FND-002 | medium | FR-172 says the consumer reads the FR-227 "Visibility evidence contract" record (owner, choice occurrence, distinguishing fact, producing declaration/message, causal path) "from the admitted package", but the checked package choice body (FR-466) carries only `owner` and a `visible` expression set; no interface FR gives the package carrier of the full record. The "definition whose choice lacks the record refuses" clause is unreachable for an admitted package, since FR-053/FR-466 refuse such a choice at admission. | spec/functional/protocol/FR-172-bind-choice-obligation-captures.md:46-60, spec/objects/protocol/FR-227-owned-choice.md:108-115, spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:59,144-153 | wrong-requirement |
| FND-003 | medium | FR-049 paragraph and FR-049-AC-14 force `pending`/unavailable under open progress, but the FR-061 "Truth is violated" row permits `violated` under an open decision scope with a decisive counterexample preserved by every admitted continuation, and the PR presents the falsifying response as that counterexample. Either state why it is not preserved while progress is open (for example another response may still bind before the deadline) or allow the FR-061 open-scope row. "never `violated` by timeout" also reads two ways. | spec/functional/foundation/FR-049-bind-native-choreography-syntax.md:80-90,108, spec/functional/protocol/FR-061-report-orthogonal-results.md:211 | wrong-requirement |
| FND-004 | low | FR-479 now cites FR-172 "Choice visibility carrier" as the definition of the visibility evidence it excludes, but its frontmatter `relationships` carry no FR-172 or FR-227 edge. | spec/objects/protocol/FR-479-obligation-activation-binding.md:1-22,62-67 | wrong-requirement |

## Verdict

Ruling 1 and ruling 4 are internally consistent. QSL FR-384-AC-7 and FR-519
already say a role carries no internality, matching the PR. quire-protocol
FR-151 is stale beyond AC-7: line 35 asserts upstream admission refuses a
checked-package role with absent internality (FR-217) and AC-3/the premise
table read internality as a package fact; FR-019 cites FR-300 (now
expose-rust-lifecycle-api) where FR-479 is meant. Both are downstream
quire-protocol edits, not defects of this PR. FND-001 to FND-003 need a fix
round; FND-004 is a nit.
