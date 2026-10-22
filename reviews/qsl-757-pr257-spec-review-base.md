---
id: SR-11188
title: "base review of quire-specification PR 257 (same-identity collisions of one owner)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@35efd91f75a36fe1d361f43039bbea7c3352c1e5; spec/functional/type-model/FR-154-admit-domain-package-model.md, spec/test-cases/TC-235-business-construct-meaning-binding.md"
review_set: subset
---

## Summary

Ticket: QSL-757. One-pass spec review of PR 257 at head 35efd91: the extended FR-154
"Two nodes share one identity" row, the new variants-prose bullet, FR-154-AC-13, and
TC-235 M27 run 2, M33 and M34. Checked against the unchanged FR-154 text, FR-154-AC-11,
FR-208 (Variant types and FR-208-AC-15), and the behaviour of QSL PR 861 (its
tc757_same_identity_collisions.rs and variant_type.rs tests and its amended FR-056,
TC-996 step 4 and TC-1003 step 5). Manual AC walk over QSL-757 items 3 and 4 (QSpec side).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The unchanged FR-154 sentence after the variants refusal list still says a package whose cases carry equal names under one identity is refused before intake by the reader or the lift with no declaration. That contradicts the new bullet ("whatever their names" refuse `conflicting-binding` at the shared identity), FR-154-AC-13 and the amended TC-235 M27 run 2, which is exactly that input (two `Gold` cases sharing `variant/Tier-Gold`). Fix: rewrite that sentence to the `conflicting-binding` outcome (keeping only the lift's `UNSLUGGABLE_NAME` note if wanted) or delete it. | spec/functional/type-model/FR-154-admit-domain-package-model.md:139 |
| FND-002 | high | In TC-235 M33 run 6 (`[Silver, Gold]`, `Silver` carrying `variant/Tier-Gold`) the mis-minted case is the earlier `Silver` at `variants[0]`. The new FR-154 bullet exempts only "that case", the later one. The mis-minted-identity row of the declaration table, the identity bullet above it, FR-154-AC-11 and FR-208's Variant types all still refuse every case whose identity is not its slot identity `malformed-declaration` at `Variant.identity`, and FR-154 reports every failing check. As written, FR-154 gives run 6 two refusals, but M33 run 6 and QSL PR 861 give exactly one, because the reader-stage collision stops intake. FR-154-AC-13's "for it" does not settle this. Fix: say in the bullet (and in AC-13) that a same-identity collision of one owner stops the per-case identity, origin and payload checks of both cases, or that it refuses the package before intake checks run, as QSL FR-056 says ("Intake's other checks do not run"). | spec/functional/type-model/FR-154-admit-domain-package-model.md:113-120; spec/test-cases/TC-235-business-construct-meaning-binding.md:112 |
| FND-003 | low | M33 run 1 and M34 run 1 say `Money` is given "a second field", but K2's `Money` already has two fields, so the added field is the third (`fields[2]`, as the expected result says). | spec/test-cases/TC-235-business-construct-meaning-binding.md:112-113 |

## Verdict

Two blocking contradictions with the FR text (FND-001, FND-002). Otherwise sound:

- The row extension matches QSL PR 861's paths and keys: `fields[2]`, `operations[2]` for
  the operation in a field and operation pair (`fields` sorts before `operations`),
  `operations[0].params[1]` and `variants[1]` in both orders, each naming the shared identity.
- The "equal names and distinct identities" row and the one-owner scope are unchanged.
- FR-154-AC-13 is bound by concrete M33 and M34 runs that can fail.
- The M27 run 2 change matches QSL TC-996 step 4, TC-1003 step 5 and FR-056-AC-26/AC-30
  as amended in PR 861.
- The PR body's claims hold.
- The M34 charge counts recompute from K2's declaration counts: fields, operations and
  variant cases are declarations and parameters are not (K2 is 21; M29 is 25 with one
  added operation and parameter). That gives 21+1=22, 21, 24+1=25 and 21+4=25, all correct.
- M33 and the new M27 run 2 state no charge count. QSL refuses these at the reader stage.
  That is acceptable but differs from the neighbouring rows.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | bdb19a2 (FR-154 sentence now says equal names under one identity are refused once as that collision, `conflicting-binding` at the shared identity, with no declaration) |
| FND-002 | fixed | bdb19a2 (the variants bullet says the collision refuses before the per-case checks and suppresses the identity, origin and payload checks of both colliding cases, naming the `[Silver, Gold]` case; FR-154-AC-11, FR-154-AC-13 and FR-208-AC-15 carry the same minimal exception, which applies only to a case already refused by a collision) |
| FND-003 | fixed | bdb19a2 (M33 run 1 and M34 run 1 now say "a third field") |
