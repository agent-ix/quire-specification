---
id: SR-850
title: "Base review of PR #175: probabilistic properties and dense time"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@d7919e0e5510d1e8f2e5830b4acb5138beaad0e5; PR #175 diff against origin/main a18859d; FR-405..FR-421, FR-090, FR-161, FR-181, FR-193, FR-250, FR-251, FR-252, FR-255, FR-271, FR-272, FR-290, FR-331, FR-349, interface_013, TC-110, TC-210, TC-266, TC-271, TC-282, TC-350..TC-374, shared-grammar.md, native-diagnostics.md, simulation-sampler.md, spec/spec.md, spec/complete-v1/tests.md, spec/temporal/tests.md; design inputs QSL ADR-024, ADR-026, ADR-028"
review_set: subset
---
# Base review of PR #175

## Summary

Ticket: STD-137, with STD-139. This pass checks the QSpec half of QSL-371
and QSL-373 against QSL ADR-024, ADR-028 and ADR-026 and against the owner
rulings. It also covers the new value objects FR-409, FR-410, FR-412, FR-413
and FR-418.

Every ruling is applied. Confidence parameters are read only by statistical
checks (FR-407, FR-411-AC-9). Fair schedulers follow Baier and Kwiatkowska
(FR-406, FR-411). The scheduler picks after the draw (FR-406). The weighted
long-run fraction is kept (FR-407-AC-7, FR-411). A workload that gives every
delay makes a DTMC (FR-420, FR-421). A delay with no `delay` member is
nondeterministic and is decided by minimum or maximum (FR-405, FR-420,
FR-421). An undefined evaluation settles `refuted` with `undefined-evaluation`
in the exact and timed engines (FR-411, FR-416). The statistical engine
settles it `measured: rejected` (FR-408), which matches ADR-024 SV-11.

No fixed caps were found. Every limit is a request setting with a published
default. No compatibility paths were found. The digest and pin removals
(FR-090, FR-250, FR-252, FR-331, FR-290, the sampler definition, TC-210,
TC-271, TC-282) delete ceremony and add no replacement.

The worked numbers were checked by hand: TC-351 SP-01/02, TC-352 ES-01,
TC-355 Okamoto `N`, TC-366 WA-03 and TC-372. The new TC-210 and TC-354
sampler vectors were recomputed: digest `d5160380…`, indices 1, 3, 3, 4, 4,
and `v mod 4` = 1, 0, 0, 2, 3 at choice 1.

## Verdict

**Changes needed.** Two high, six medium and three low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-290's evidence-kind section says the statistical arm settles an every-scheduler claim `ExactOnlyForm`. FR-408 Admission, FR-408-AC-1 and TC-353 RT-02 settle it `EveryScheduler`. The two requirements give one item two different causes. | spec/objects/protocol/FR-290-protocol-claim-kind.md:173-176; spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:53,183 |
| FND-002 | high | FR-413 says `scheduler` is absent exactly under a workload. Under a workload with free delays, FR-421 and FR-420 leave each free delay to the scheduler, and the bound is read on its minimum or maximum. ADR-028 TA-6 says the witness scheduler then chooses those delays. So a refutation in that case has no place to carry its delay choices. FR-412's objective (`minimum`, `maximum` or `workload`) cannot name a workload with free delays either. | spec/objects/interfaces/FR-413-probabilistic-counterexample.md:56,122; spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:101; spec/objects/interfaces/FR-412-probability-certificate.md:33 |
| FND-003 | medium | FR-411's verdict table and FR-193's verdict map put `failed` in the FR-331 value column for a budget stop. FR-331 lists `incomplete` as a result value. Sibling FR-363 (PR #170) maps a stopped run to FR-331 `incomplete`, with `failed` as the FR-360 label. Readers of the two PRs would record different FR-331 values for the same stop. | spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:177-178; spec/functional/analysis/FR-193-analyze-continuous-hybrid-models.md:71; spec/objects/interfaces/FR-331-backend-provider-envelope.md:42 |
| FND-004 | medium | Result causes and proof bases are spelled two ways. Some are PascalCase: `ReplayParity`, `UndecidedSuccessor`, `NotMarkov`, `CertificateRejected`, `MissingConfidence`, `ExactValue`, `ZoneCertified`, `Exhaustive`. Others are kebab-case: `undefined-evaluation`, `backward-induction`, `finite-sample`, and FR-363's `replay-parity`, `undecided-successor` and `exhaustive`. No closed wire list of the new unsupported, inconclusive and undecided causes exists in FR-331 or FR-272, although ADR-028 QS-4 asks for one. Two producers would encode the same cause differently. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:42; spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:172-180 |
| FND-005 | medium | FR-408 does not fix the rounding direction for the SPRT per-sample increments `ln(p1/p0)` and `ln((1−p1)/(1−p0))`. It does not fix it for the composed regenerative half-width either: it rounds `N`, the thresholds and the half-width, but not the inputs. "Within `2^-64`" with no direction lets two conforming implementations reach different decisions near a threshold. That breaks FR-408-AC-7's identical-decision rule. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:124-147,189 |
| FND-006 | medium | FR-414 says a window with too few values makes the clause "undefined" and stops there. It never states how the monitor result settles. The owner ruling settles every undefined evaluation `refuted` with `undefined-evaluation`, but ADR-024 WA-4 still says O-16 undefined. State the monitor outcome explicitly. | spec/functional/temporal/FR-414-evaluate-window-aggregates.md:69-74 |
| FND-007 | medium | TC-374 DG-06, the only vector for FR-421-AC-6, removes `timeout`'s delay. But `timeout`'s window at the start is the single point 3 ms, so the scheduler has no real choice. The expected result also names no value and no verdict ("the maximum … is compared with 0.01"). The vector can never fail. | spec/test-cases/TC-374-stochastic-and-probabilistic-timed.md:41; spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md:118 |
| FND-008 | medium | The TC vectors define their subjects by unqualified "ADR §7.2", "ADR §15.1" and similar citations, for example `Service`, `Health`, `Avail`, `Link`, `Coin` and `Retx`. QSpec has its own ADR-001..006, so the citation is ambiguous, and the models are not defined in QSpec. TC-359 and TC-369 define their models inline; the others do not. | spec/test-cases/TC-351-step-probability-and-markov-condition.md:26; spec/test-cases/TC-360-exact-finite-horizon.md:23 |
| FND-009 | low | FR-405, FR-420 and FR-421 cite a "Team-leader decision for this lane" for nondeterministic delays. It is an owner ruling recorded on the ticket. Lane jargon does not belong in public spec text. | spec/functional/analysis/FR-405-declare-probabilistic-models.md:142 |
| FND-010 | low | QSpec normative text names QSL internals. FR-181 Dependencies asserts that the QSL sampler (QSL FR-101, FR-188) matches TC-210, and FR-405 names "QSL's `NegativeReward`". | spec/functional/runtime/FR-181-simulate-finite-executions.md:199-201; spec/functional/analysis/FR-405-declare-probabilistic-models.md:80 |
| FND-011 | low | The new FRs trace to US-011 ("interchangeable exact methods"), US-008 ("bounded temporal obligations") and US-009. None of these stories states probabilistic, statistical or real-time intent, and a `measured` result is not an exact method. | spec/functional/analysis/FR-405-declare-probabilistic-models.md:6 |

## Verdict detail

The semantics match the ADRs closely, including the fair-end-component
refinement, the quantile transform, the undefined path, the digital-clock
route and the sampler revision. The two high findings are contradictions
between requirements, and each needs a one-line fix.

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-specification@9ed6fab8b33842c1fef83d918d683f84d2b4de04.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | high | FR-413's path-set rule cannot carry the refutation that FR-411-AC-13 and TC-360 EF-06 require. For a quantile claim it asks the activated, violating mass to "exceed `1 − q`", and the general rule asks the sum to pass the bound strictly. For `quantile q of M >= c` the violating paths are those with `M < c`, and the strict reduced bound fails when that mass reaches `q`, equality included. On EF-06 the mass is exactly `1/2 = q`, so replay's sum check rejects the evidence and the item settles `inconclusive`, `replay-parity`, not `refuted`. The rule also ignores the activation mass: it is exact only when activation has probability 1. | spec/objects/interfaces/FR-413-probabilistic-counterexample.md:77,84; spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:253; spec/test-cases/TC-360-exact-finite-horizon.md:38 |
| FND-013 | medium | FR-410's `estimate` records `Pr(M <= c \| activated)` for every quantile claim. FR-408 decides `quantile q of M >= c` on `Pr(M < c \| activated)` against `q` (FR-407's strict reduction), so for that form the recorded estimate and its interval are of a different probability from the one the decision reads. | spec/objects/interfaces/FR-410-statistical-verdict.md:38; spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:96-99 |
| FND-014 | medium | FR-408's zero-variance rule uses half-width `1` while the regenerative sample variance is 0. A run whose cycles are all identical (for example a deterministic cycle) then never decides and always stops at `max_samples` with `incomplete`. Team-leader decision: adopt the Chow-Robbins half-width, `z · sqrt(S² + 1/n) / (L̄ · √n)`, so a degenerate sample has half-width `z / (L̄ · n)` and the run still stops; QSL #572 uses this form and cites QSpec FR-408. FR-408-AC-12 and TC-356 RG-05 change with it. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:136-147,239; spec/test-cases/TC-356-regenerative-long-run.md:35 |

## New findings (disposition pass 2)

Reviewed at agent-ix/quire-specification@addbd0986dcee88165668b4bb956d8f33569e816.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-015 | low | FR-408-AC-12 and TC-356 RG-05 assert that the `Avail` claim `long_run_fraction holds(v.up) <= 0.9995` is not settled `measured: rejected` on seed `20261001`. Nobody computed that outcome, and the method does not imply it. The true value `1800/1801` is about `5.5e-5` below `0.9995`, inside the indifference region (`ι = 0.0002`), so the method's error bounds say nothing about this claim and a conforming run may reject it on that seed. The vector can then fail a correct implementation. The Chow-Robbins mechanism the AC means to test is deterministic: with `S² = 0` the half-width is `z/n`, so the run does not stop at `min_cycles` 1000, and an all-identical-cycle fixture stops at the first `n` with the rounded-up `z/n <= ι` (about 11,632). Assert that, or move the threshold out of the indifference region. | spec/functional/analysis/FR-408-settle-probabilistic-claims-statistically.md:235; spec/test-cases/TC-356-regenerative-long-run.md:35 |

## New findings (disposition pass 4)

Reviewed at b6d7d8914e67169121f72054146a0b1c1bdc8063 (commit b6d7d891, caller cancellation gets its own `incomplete` cause). Limited to lines that commit changed.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-016 | low | FR-411 now says a caller cancellation stops the exact engine with `incomplete`, `cancelled`, naming the cancellation source, but no FR-411 AC or TC tests it. Only FR-408-AC-8 and TC-356 RG-06 cover `cancelled`, and both are for the statistical engine. | spec/functional/analysis/FR-411-settle-probabilistic-claims-exactly.md:233-234 |
| FND-017 | low | FR-331's closed `incomplete` row adds `cancelled`, "naming the cancellation source", but unlike `limit-reached{limit, value, setting}` and `precision-budget{lower, upper}` it names no member that carries the source. Two implementations would encode the source differently. Give the member, for example `cancelled{source}`. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:60 |

## Dispositions

Round 1, reviewed at 9ed6fab8b33842c1fef83d918d683f84d2b4de04 (fix commits 96a98644 and 9ed6fab8).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 96a98644 |
| FND-002 | fixed | 96a98644 |
| FND-003 | fixed | 96a98644 |
| FND-004 | still-open | One PascalCase cause is left: FR-406:63 still says `NotMarkov{state, transition, post_states}`. Every other cause and basis is kebab-case, and FR-331 now has the closed wire table. |
| FND-005 | fixed | 96a98644 |
| FND-006 | fixed | 96a98644 |
| FND-007 | fixed | 96a98644 |
| FND-008 | fixed | 96a98644 |
| FND-009 | fixed | 96a98644 |
| FND-010 | fixed | 96a98644 |
| FND-011 | fixed | 96a98644 |

Round 2, reviewed at addbd0986dcee88165668b4bb956d8f33569e816 (rebased onto main ae02648b; commits b56235f8 and addbd098).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | addbd098 |
| FND-012 | fixed | addbd098 |
| FND-013 | fixed | addbd098 |
| FND-014 | fixed | addbd098 |

Round 3, reviewed at b2cf33d49e7a173e46f1f117e0a79619807c1118 (commits 98ef379c and b2cf33d4).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-015 | fixed | 98ef379c |

Round 4, reviewed at b6d7d8914e67169121f72054146a0b1c1bdc8063. No finding had a still-open or missing outcome before this round.
