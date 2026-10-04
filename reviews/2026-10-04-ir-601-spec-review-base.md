---
id: SR-1292
title: "Base review of PR #188: division checks only the exposed member"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f0002b92095dee386759dd17530db0f757f285d6; PR #188 diff against origin/main 57e51d6c: FR-147, FR-007, FR-271, FR-272, TC-192, spec/complete-v1/tests.md, proposals/quire-v1/definitions/native-diagnostics.md, proposals/quire-v1/definitions/value-accounting.md, proposals/state-core/state-semantics.md, tests/checked_package_v2.rs"
review_set: subset
---
# Base review of PR #188 (IR-601)

## Summary

Ticket: IR-601. The owner ruling is MEMBER-ONLY: `q` and `r` are computed
exactly under the selected profile, and only the member the expression exposes
must be in the consumer's domain. I checked the diff against that ruling, the
author's four choices, every place the pair rule or the old code, cause and
charge points appeared, the signed-minimum cases, TC-192's oracles and the
FR-271/FR-272 catalog rows.

- Ruling: FR-147's operator table checks `q` for `div`, `r` for `rem` and the
  Euclidean `r` for `mod`, and says the unexposed member is never checked,
  retained or exposed. FR-147-AC-10 and TC-192 DIV-16 carry the ticket's two
  examples: `10 div 5` over `Int[1,10]` returns `2`; `-10 rem -1` over
  `Int[-10,5]` returns `0`. I recomputed both under truncating, floor and
  Euclidean: the results do not depend on the profile.
- Leftovers: `git grep` on the head, excluding `reviews/`, finds no
  `division_pair_out_of_domain`, `both-outside-domain`,
  `integer-division.domain-pair`, `integer-division.result-pair`, or pair or
  atomic wording for `div`/`rem`.
- Signed minimum: MIN `div` -1 is `2^63` and refuses in a signed-64 domain
  with `quotient-outside-domain`. MIN `rem` -1 is `0` under all three profiles
  and completes (DIV-04, DIV-05, FR-147, FR-007).
- TC-192 oracles: I recomputed the profile table, DIV-08 (two separate
  expressions, `result_units: 1`, denial at `integer-division.result-retain`
  with `work_units` limit 3 and consumed 3), DIV-15 (`10 div 3` = 3,
  `10 rem 7` = 3, `10 rem 5` = 0, which refuses) and DIV-16. All are correct.
- Catalog: the FR-271 values row, FR-271-AC-6, the FR-272 row and AC-11, and
  the native-diagnostics row and code lists all spell `division_out_of_domain`
  with exactly `quotient-outside-domain` and `remainder-outside-domain`.
  `spec/complete-v1/tests.md` maps FR-147-AC-10 to TC-192.
- `quire validate --scope . 'spec/**/*.md'` gives 145 failed documents on the
  head and 145 on a clean origin/main 57e51d6c. The sorted failure lists are
  byte-identical once paths are normalized, so the PR adds no failure.

Author choices:

1. `division_out_of_domain`: keep. It fits the `<thing>_out_of_domain`
   family (`decimal_`, `modulo_`, `integer_`, `rational_`). quire-exact#2
   (head 45c000f1) still spells `division_member_out_of_domain` and must
   change to this spelling.
2. `mod` on `modulo_out_of_domain`/`outside-domain`: keep. `mod` checks only
   the Euclidean remainder, so it already meets the ruling's "remainder for
   rem and mod". It had its own code before this PR, and the ruling replaces
   only the `div`/`rem` pair check.
3. `integer-division.domain` (`value_occurrences=1`) and
   `integer-division.result-retain` (`result_units += 1`): keep. They match
   the `integer-modulus.*` points. quire-exact#2 still names
   `integer-division.domain-pair`/`result-pair` and must change.
4. `div`/`rem` in the examples: keep. Integer `/` is refused
   (shared-grammar.md:1071) and is rational division otherwise, so the
   ticket's `10 / y` would be wrong as a spec example.

## Verdict

Approve with one low finding in this method. The integrity review (SR-1293)
has the medium state-core finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-192 DIV-06 asks for "a `div` whose quotient is one below `MIN` or one over `MAX`". With signed-64 operands and a nonzero divisor, no profile gives a quotient below `MIN`: `abs(q) <= abs(a) <= 2^63`, and the floor or Euclidean quotient of a positive divisor is at least `-2^63`. Only "one over `MAX`" can happen, and its example `MIN div -1` is the DIV-05 case again. Drop the "one below `MIN`" half, or say it is unreachable. | spec/test-cases/TC-192-integer-division-profiles.md:45 |
