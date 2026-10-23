---
id: SR-16271
title: "Spec-review of the exact binomial method in FR-408 (quire-specification PR 420)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f40e02f5b8c175cb569213b532a3b941e07ef7a7; spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md; spec/objects/interfaces/FR-410-statistical-verdict.md; spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/test-cases/TC-355-okamoto-and-sprt.md; proposals/backend-provider-v1/schema.json"
review_set: subset
date: 2026-10-23
relationships:
  - target: ix://agent-ix/quire-specification/FR-408
    type: references
  - target: ix://agent-ix/quire-specification/FR-410
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/TC-355
    type: references
---

# Spec-review of the exact binomial method in FR-408 (quire-specification PR 420)

## Summary

Ticket: STD-429 (QSpec half; QSL half quire-spec-language#1373, SR-16270). Head f40e02f5. origin/main
is 1 commit ahead and is not an ancestor. The merge is clean.

Gates run by the reviewer: `quire validate --scope . 'spec/**/*.md'` gives exactly 86 failure lines, and none
of the failures is new. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 24 + 134. QSL
parses neither TC-355 nor the schema (grep of QSL crates), so no QSL conformance run is needed.

Hand derivation (exact rationals, Python `fractions`, `math.comb`), with alpha = beta = 1/100,
m = 1, iota = 1/10, theta = 9/10:

- N = ceil(ln 100 / (2 * 0.01)) = ceil(230.26) = 231.
- G(k, 9/10) = Pr(X >= k): k=219 0.0063428 (<= 1/100, accept), k=218 0.0127723 (no).
- L(k, 9/10) = Pr(X <= k): k=196 0.0087720 (<= 1/100, reject), k=197 0.0146424 (no).
- So the method accepts at k >= 219, rejects at k <= 196 and is undecided at 197..218, which matches the
  vector. Okamoto: accept iff k/231 - 1/10 >= 9/10 (k = 231 only); reject iff k/231 + 1/10 < 9/10 (k <= 184).
  That matches too.
- The error bounds hold. G(k, p) rises in p, so Pr_p(accept) <= G(k*, theta) <= beta' for p < theta, and the
  same argument bounds rejection. Both tails can hold only when alpha' + beta' > 1, because G + L = 1 + pmf(k).
  Clopper-Pearson decides wherever Okamoto does (Hoeffding).
- Tail precision: 8(N+1) roundings at 2^(1-P) with P = 96 + ceil(log2(N+1)) give a relative error of at most
  2^-92 (< 2^-90). The statement is sound.
- At k = 219 the exact Clopper-Pearson ends (exact-rational bisection, 60 halvings) are lower 0.90351472792271070918... and upper 0.97621175445884490925...

- SC-10: N = ceil(ln 100 / (2 * 10^-4)) = 23,026. With p = 24233/25000 ≈ 0.9693, G(k, 0.95) is about 13 sd
  out, so the method accepts. The lower end exceeds 0.95 because G(k, 0.95) < beta', and the width is about
  0.0053 < 0.02. Consistent.
- The bisection rule moves the lower end up only when G's upper bound is below beta', so the reported lower
  end never passes the exact root. The rule is outward-safe.

FR-408-AC-13/14 are unique. The SC-10 to SC-13 ids repeat only in TC-967 (#404), a different TC. The PR
conflicts with open #415 in FR-331's method list, as its body says: keep #415's text and add `clopper-pearson`.
It merges clean with #419 and #410.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | [soundness] FR-408-AC-13 says that 'the reported interval contains [0.9035147279, 0.9762117545] and lies within 2^-64 of it at each end', and TC-355 SC-11 says the same. The exact ends are 0.90351472792271071 and 0.97621175445884491. FR-408's outward bisection returns ends within 2^-64 of those, about 0.9035147279227 and 0.9762117544588. That interval lies inside the stated pair, so it does not contain it, and each end is about 1e-11 from it. A correct provider fails both clauses. Fix: state the exact ends to 20 digits (lower 0.90351472792271070918, upper 0.97621175445884490925) and require the reported lower end <= the exact lower end and > it minus 2^-64, and the same for the upper end. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:327; spec/test-cases/TC-355-okamoto-and-sprt.md:43 | wrong-requirement |
| FND-002 | medium | [untestable-ac] SC-13 expects 'a fixture whose G(k, θ) lies within the rounding error above β' is undecided'. When the exact G is above β', no correct or incorrect tail gives acceptance, so the vector cannot tell the conservative rule ('decide only on a tail's upper bound') from a rule that decides on the lower bound or the midpoint. The vector has to put the exact G just below β', by less than the bound's error, so that the upper bound exceeds β'. It must then expect undecided, as SC-09 does for SPRT. Fix: change SC-13's fixture to place G below β'. | spec/test-cases/TC-355-okamoto-and-sprt.md:45; spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:328 | wrong-requirement |
| FND-003 | low | [other] The description now names three methods, but the title and heading still read 'Okamoto and SPRT decisions'. Fix: name the exact binomial method in the title and heading. | spec/test-cases/TC-355-okamoto-and-sprt.md:3; spec/test-cases/TC-355-okamoto-and-sprt.md:11 | missing-requirement |

## Verdict

Not mergeable. The method, its error argument, the tail-bound arithmetic, the schema enum and the FR-331/FR-410 additions are correct and agree with QSL #1373. The AC-13/SC-11 interval clause fails for a correct provider, and SC-13 cannot detect a non-conservative tail rule. Fix with #1373 and resolve the FR-331 conflict with #415.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-004 | low | [ambiguous] The exact upper end is 0.976211754458844909955841... The text writes it as `0.97621175445884490996…`, rounded up in the 20th digit, while the lower end `…070929…` is truncated. Where the literal serves as the threshold ('at least `0.97621175445884490996…`'), a correct reported upper end in [exact, …0996) fails it. Fix: write `0.97621175445884490995…`, truncated like the lower end, or give the digits `0.976211754458844909955841…`. | spec/test-cases/TC-355-okamoto-and-sprt.md:43; spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:327 | wrong-requirement |

## Dispositions

Round 1, against head e3bc134b. Gates at e3bc134b: `quire validate` exactly 86 failure lines; `cargo test --test checked_package_v2 --test backend_provider_v1` 24 + 134 passed. Exact Clopper-Pearson ends at k = 219 (one-sided levels: lower = least p with G(219, p) >= beta' = 1/100, upper = greatest p with L(219, p) >= alpha' = 1/100; exact-rational bisection, 90 halvings): lower in [0.90351472792271070929237711031, 0.90351472792271070929237711113], upper in [0.97621175445884490995584141752, 0.97621175445884490995584141834]. The author's 0.90351472792271070929... and 0.97621175445884490996... are right to within the rounding of the last digit. The first-round SR's ...070918 / ...490925 were wrong: they were the lower ends of a 60-halving bracket, not the exact values. The 64-halving dyadic bracket ends that a conforming bisection reports are lower 0.90351472792271070929182724685 (exact minus 5.5e-22) and upper 0.97621175445884491000524824100 (exact plus 4.9e-20), both within 2^-64 = 5.42e-20.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
