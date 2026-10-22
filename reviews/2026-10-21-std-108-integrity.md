---
id: SR-14680
title: "integrity review of STD-108 FR-290 scalar operation claim form (QSpec PR #339)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@43c4295af9b7507a827bd7f1244513870a8bc674; spec/objects/protocol/FR-290-protocol-claim-kind.md, spec/test-cases/TC-271-capability-kind-candidate-negotiation.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-290
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-271
    type: reviews
---
# SR-14680: integrity review of STD-108 (QSpec PR #339)

## Summary

Ticket: STD-108. Diff reviewed: `git diff origin/main...43c4295` (2 files).
The PR adds two rows to the FR-290 claim-form table, a defining paragraph
after it, a clause to FR-290-AC-5 and the TC-271 vector CK-09.

Checked against QSL FR-057 "Claim forms and their kinds" and FR-062
"Requirement records of a value function" (QSL PR #456, merged). The rows,
the per-occurrence rule, the measure exclusion and the narrowing rule match
QSL. The lead default (the claim sits beside FR-146 admission and does not
replace it) matches QSL FR-062 "The claim": `check` proves definedness and
narrow ranges, so an admitted body never yields a claim that fails.

CK-09 is buildable as written: the `function f using V (...) : R pure { e }`
form matches FR-146; under FR-149 a `convert<T>` operand gets no expected
type, so `x + 1` is `Integer` and `convert<Int[0, 10]>` of it is the range
narrowing `quire.op.numeric.narrow`; and `x + 1` over `Int[0, 9]` lies in
`[1, 10]`, so both functions are admitted under FR-146.

No other open QSpec PR edits FR-290 or TC-271. `CK-09` is new in TC-271.
Gates: `quire validate --scope . spec/**/*.md` reports 134 failing documents,
the same as main; FR-290 has the same 8 diagnostics as on main and TC-271 has
none. `cargo test --test checked_package_v2 --test backend_provider_v1`
passes, 117 + 14.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The scalar operation application is defined by owning FR ("integer (FR-044, FR-147), rational (FR-149), decimal (FR-140) ... numeric conversion, or text or enumeration (FR-141)"), not by catalog identity prefix as QSL FR-062 does. That leaves the boundary open for identities QSpec's catalog holds: `quire.op.string.eq`/`.ne`, `quire.op.uuid.*` and `quire.op.timestamp.*` (not in the exclusion list, and not clearly "text"), `quire.op.rational.narrow` (a rational operation under QSL, so a claim, but read as a "narrowing" it would not be), and `quire.op.quantity.convert`. Two implementers can assign different claim sets. Name the families by `quire.op.<family>` prefix as QSL does: `proposals/checked-package-v2/schema.json` lists `quire.op.decimal.*` and every other prefix. | spec/objects/protocol/FR-290-protocol-claim-kind.md:136-140 |
| FND-002 | low | "for every argument assignment under which its path condition holds" uses "path condition", which QSpec does not define. QSL FR-062 defines it as the enclosing `if` and `and`/`or`/`implies` guards. QSpec's nearest term is FR-146's guard facts and their propagation. Define the term here or refer to FR-146's facts. | spec/objects/protocol/FR-290-protocol-claim-kind.md:114 |
| FND-003 | low | CK-09 is inserted between CK-07 and CK-08, out of id order. CK-05 still says it derives "every row of the FR-290 claim-form table", but it does not derive the two new rows. CK-09 covers them, so CK-05's "every row" no longer holds. | spec/test-cases/TC-271-capability-kind-candidate-negotiation.md:42-46 |
| FND-004 | low | The PR body says "QSpec has no `quire.op.decimal` identity, so the families are named by the FRs that own them". `proposals/checked-package-v2/schema.json` enumerates eleven `quire.op.decimal.*` identities. The stated reason for FND-001's wording is false. | PR #339 body |

## Verdict

The new rows and paragraph are consistent with FR-146 and with QSL. No
contradiction or unbuildable criterion was found. FND-001 is the one
substantive issue: the family boundary is ambiguous for five catalog
families. FND-002 to FND-004 are wording and hygiene fixes.

## Resolution

FND-001: the claim families are named by `quire.op.<family>` prefix in the FR-290 paragraph after the claim-form table (`integer`, `rational` including `rational.narrow`, `decimal`, `ieee`, `quantity` including `quantity.convert`, `numeric` other than `numeric.narrow`, `text`, `enum`); `string`, `uuid` and `timestamp` operations carry no claim.
FND-002: the claim-form row defines the path condition as the enclosing `if` guards and the left operand of an enclosing `and`, `or` or `implies`, as FR-146 propagates guard facts.
FND-003: CK-09 follows CK-08, and CK-05 now excludes the two scalar-operation rows, which CK-09 covers.
FND-004: the PR body no longer claims that QSpec has no decimal identity.
