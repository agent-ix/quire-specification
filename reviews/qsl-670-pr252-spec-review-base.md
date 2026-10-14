---
id: SR-9103
title: "Base spec review of PR #252: chain-link model identity and snapshot invocation member"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@ecc84cdc31095a6e531361a671864f7a95a86833; PR #252 diff against origin/main: model-complete.md, value-accounting.md, model-complete-rules.json, FR-012, FR-046, FR-143, FR-150, FR-201, FR-204, FR-321, AD-006, AD-016, interface_003, TC-046, TC-195, TC-196, TC-198, composed-foundation/tests.md, checked-package-v2 README"
review_set: subset
---
# Base spec review of PR #252 (QSL-670)

## Summary

Ticket: QSL-670. The PR replaces path-form derivation facts with one-edge
facts that link to the direct parent's identity (`/v2` effective-declaration,
effective-view and object-universe domains; inherit, subset and redefine rules
at `/v2`; selections, rule manifest and accounting identities unchanged), names
the optional snapshot `invocation` member in FR-012 and FR-046, and fixes the
FR-012-AC-7 example.

Independent recomputation (my own script, built from the model-complete.md
rules, not from the published preimages):

- TC-195 N01: all four declaration digests, view `76d04f1e`, universe
  `af5934e1`, JCS lengths 366/666/491/695, universe 335, view 3101, total 5654;
  the hashed-bytes incomplete run's consumed 2553. All match.
- TC-195 N02: all eight digests, view `ecdf3d6d`, universe `6763207e`,
  declarations 5625, view 7128, total 13088; 13 facts, 40 work units. Match.
- TC-195 N09: n01v2 view `f81504ef`, universe `1164b658`. Match.
- TC-195 N13: chain 149894 B / 199 facts / 600 units; ladder 209400 / 296 /
  794; dense 108988 / 165 / 392; `4V + 2E + 2` holds for all three. Match.
- TC-195 N19: 10,001 types, 20,001 facts, 15,104,845 hashed bytes, 60,006
  work units. Match. The QSpec key is 83-87 bytes because TC-195 fixes the IR
  node identity as `ix://test/orders/X` (and TC-234's test asserts the
  `ix://` prefix). The QSL design note's 66-70 byte key (14,084,413 B) uses a
  bare `T0` node; the QSpec shape is the correct one.
- TC-196 D01: f = 1, 3, 2, 2 for A, D, C, B; 16 candidate + 6 dominance + 4
  subtype units = 26. D02: 3 x (1 + 2 + 2 + 3) = 24. Subtype order A < D < C <
  B by digest. Match.
- TC-198 L04, L09: universe and type digests and order match the vectors.
- Schema: `parent-on-qualify-fact`, `cross-domain-declaration-key` and
  `owner-as-declaration-key` are refused by the schema and `duplicate-parent`
  and `mis-wired-link` are schema-valid (so semantic), as labelled.

AC numbering: FR-150 on main has AC-1..11 and AC-18, so AC-12..16 do not
collide. #209 (`spec/670-identity-cost`) is CLOSED, and #252 does not depend
on it. Every new or changed AC has a TC: FR-150-AC-12 N11, AC-13 N12, AC-14
N13, AC-15 and AC-16 N19, AC-4 and AC-6 N01/N02/N07, FR-046-AC-8 TC-046, and
FR-012-AC-1/AC-7 TC-231. The QSL FR-106 text on QSL main agrees with FR-012
and FR-046 (four labels, no digest, absent = unbound, `wrong-invocation`).

Paired QSL #830: same preimage, fact shape, `/v2` domain set and expansion
rule, and the same strict-reader refusals and charges. Differences: (1) QSL
TC-1024 expects "about 14.1 million" hashed bytes for the 10,000-edge chain
because its fixture key has no `ix://` node prefix; QSpec N19 states
15,104,845. Both pass conditions are at most 16,777,216, so nothing
contradicts, but the QSL figure does not match the QSpec key convention.
(2) QSL keeps a phase-3 type-hash exception to charge-before-hash. QSpec's
`normalize.hash` row has no exception, because QSpec orders phase 4 by
declaration key. (3) QSL FR-081-AC-12 adds a 524,288-byte package-encoding
bound that QSpec AC-15 does not have.

## Verdict

Changes needed: two high findings. Both are wrong or contradictory vectors in
TCs. Recorded, not blocking: no Rust test applies `invalid_mutations`
(medium); TC-196 R01 charge order under the new direct-edge-then-exploration
rule is not pinned (medium); FR-046 does not say whether a PreCall checks the
`invocation` member (medium); complete-v1/tests.md has no rows for
FR-150-AC-12..16, the same as AC-9..11 and AC-18 (low); N19 sits between N13
and N14 (low).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | TC-196 D03 still lists the `no-applicable` refusals for subtypes `C`, `A`, `D`, `B` in that order. The PR reorders the effective type identities to `A` < `D` < `C` < `B` and updates D01 to that order, and `dispatch.subtype` charges each closed subtype ascending by effective type identity. D03's order is the old digest order and is now wrong; it should read `A`, `D`, `C`, `B`. | spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md:63 |
| FND-002 | high | TC-195 N12's fourth change, "`D/x4.redefines` removed" from N06's admitted model, does not produce identities. It turns the model back into N06's first case: `B/x2` and `C/x3` both redefine `A/x` at `D`, neither owner descends from the other, so normalization is refused `derivation-conflict` with no effective view. The vector's expected result (identities that change for exactly the affected declarations) cannot hold for that change, and FR-150-AC-13's "one phase-4 fact" clause has no working case. Use a phase-4 change that stays admitted, for example removing `C/x3.redefines` (`D` descends from `B`) or adding a `subsets` entry. | spec/test-cases/TC-195-model-normalization-provenance.md:65; spec/test-cases/TC-195-model-normalization-provenance.md:59; proposals/quire-v1/definitions/model-complete.md:455 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | The new N12 figures for "`C/x3.redefines` removed" give 64 work units, which the coordinator lists as 8 records + 24 facts + 4 cycle-checks + 13 declarations + 15 hashes. My recompute confirms 13 declarations, 24 facts (28 before) and the four changed identities. The 64 leaves out the phase-4 charges in value-accounting.md. `normalize.redefinition-check` charges `m + r` per redefining member: `B/x2` is 2 + 0 and `D/x4` is 4 + 1, so 7. `normalize.conflict-check` on `(D, A/x)`, reached by `c = 2` redefiners (`B/x2`, `D/x4`), charges f(B) + f(D) = 2 + 3 = 5. Normalization work is therefore 76 (75 if `r` counts only same-type redefiners), not 64. The stated figure is wrong. | spec/test-cases/TC-195-model-normalization-provenance.md:65; proposals/quire-v1/definitions/value-accounting.md:602-603 |

Recorded (medium, not blocking): N12 says `(D, C/x3)` changes "through its link to `(C, C/x3)`". It also loses its own phase-4 redefine fact `[C/x3, A/x]`, because rule 4 puts that fact on every `(T, m)` where `m` redefines `t`. The set of four changed identities is correct.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 33474c9 |
| FND-002 | fixed | 33474c9 |
