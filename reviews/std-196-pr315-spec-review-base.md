---
id: SR-13653
title: 'Spec review of QSpec PR #315: universe keys, instance order, bound reference type (STD-196)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@d870b40de; PR #315 diff origin/main...HEAD: FR-361, TC-290, backend-provider-v1/vectors.json; ticket STD-196'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-361
  type: reviews
- target: ix://agent-ix/quire-specification/TC-290
  type: reviews
---

## Summary

Ticket: STD-196. PR quire-specification#315, head d870b40de15daccf948556127fa2c1717f19030c (the author's push adding the universe-provision encoding; the first pass at 2e3e4a28 was superseded).

Base lens with EARS, integrity and failure-domain folded in. Gates: cargo tests pass (13 + 113); quire validate 134 failures as on main, FR-361 and TC-290 clean. Merges into main alone; conflicts with #314 in FR-361 (see SR-13650 FND-001). FR-361-AC-9..13 collide with nothing. The previously unsettled byte encoding is now defined (FR-361-AC-13), which AC-10's 'permuting changes the request identity' needs; its consistency with FR-181 is FND-003. Checked against FR-153 (coverage), FR-363 (cause selection) and FR-181 (typed canonical form).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-361 universe definition | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361 universe provision | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361 instances and order | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361 binding | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361 absent-key and guard | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361-AC-9 | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361-AC-10 | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361-AC-11 | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361-AC-12 | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| FR-361-AC-13 | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md | examined |
| TC-290 step 7 | spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md | examined |
| TC-290 expected 7 | spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md | examined |
| TC-290 expected 8 | spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md | examined |
| FR-153 coverage | spec/functional/type-model/FR-153-query-closed-model-environments.md | context_only |
| FR-363 refuting evidence | spec/functional/temporal/FR-363-settle-every-behaviour-temporal-verdicts.md | context_only |
| FR-181 typed canonical form | spec/functional/runtime/FR-181-simulate-finite-executions.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The parenthetical cites FR-153 ('a member type covers itself and every type that conforms to it') and concludes 'so a member type that is T, a subtype of T or a supertype of T covers it'. By FR-153 a member type S covers T only when T conforms to S, so a proper subtype of T does not cover T; the conclusion does not follow and contradicts the cited rule. Whether a population whose member types are only subtypes of T yields instances is therefore read two ways. Fix: define the selecting relation separately (for example 'some member type of p conforms to T or T conforms to it') instead of calling it FR-153 coverage, and add an AC for a subtype-only population. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:143-145 |
| FND-002 | high | FR-361 says the instance order 'changes neither the verdict label, basis nor cause'. Under FR-363 the result cause comes from the reported refuting evidence: `undefined-evaluation` for an undefined read, none for a false evaluation. If one instance refutes by an undefined read (AC-11's (`all`, `e`)) and another by a false evaluation, the first refuted instance in the order decides the cause, so permuting a universe list can change it. FR-361-AC-10 only asserts the label. Fix: say the cause and counterexample are those of the first refuted instance (order-dependent), or define the cause independently of the order; add the mixed case to AC-10. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:146-151 |
| FND-003 | medium | The provision is called 'each key in the FR-181 typed canonical form of the pair', but FR-181's tag list has no `key` tag, and FR-181 encodes a supplied input collection as `{"type":"sequence","elements":[...]}`, not as a bare JSON array. So the encoding is new and the FR-181 attribution is wrong. Fix: add the `key` form and the universe-provision document to FR-181 (in this PR, since FR-181 owns the typed forms), or drop the attribution and state why the provision is a bare array. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:130-134 |
| FND-004 | medium | Step 5 widened `counters` to {c, d}, but expected result 7 and FR-361-AC-9 list only (`counters`, `c`), with AC-9 saying `counters = [c]`. Step 7 then permutes '[c, d] and [d, c]' without naming the population, and checks 'a refuted claim' without naming it. A tester cannot build the vector as written. Fix: state step 7's universes explicitly (`counters = [c]`, `all = [c, e]`, or the widened lists), name the refuted claim, and say which list is permuted. | spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md:49 |
| FND-005 | low | 'Each instance binds x to the reference ... whose object_type is the population's canonical member type' and 'A state's object matches the binding by universe and identity alone' are normative rules that AC-12 tests, but they have no SHALL, unlike the ordering rule above them. Fix: write them as 'The analyzer SHALL bind ...' and '... SHALL match ...'. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:153-156 |

## Verdict

Not mergeable: FND-001 and FND-002 are internal contradictions with FR-153 and FR-363. FND-003 should be fixed here because the provision encoding is what this push added. FND-004 makes TC-290 step 7 buildable. The design choices (conservative mixed-population refutation, no type-test operator) are flagged as judgment calls in the PR body and are not findings.
