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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | high | FR-361:64-67 and FR-361-AC-8 still say an item has zero instances, and settles `no-instance`, when no object of the universes conforms to T. Under the new overlap rule an item has one instance per key of every overlapping population, whatever the object under the key, and keys are untyped. Example: `over (x: Counter)` with only `all: Entity = [e]`, e not a Counter: the instance rule gives instance (`all`, `e`), which an unguarded read refutes `undefined-evaluation` (as AC-11 does), while AC-8 says `inconclusive`/`no-instance`. This was already latent at round 0 (supertype populations), and #314's no-instance sentence repeats the 'no conforming object' wording. Fix: restate zero instances as 'no overlapping population has a key' in the paragraph and in AC-8, and have #314's rebase use that wording. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:64-67 |

## Dispositions

Round 1, head f9118b199.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Fixed in f9118b199: Instances now come from populations that overlap T (either direction of conformance), defined in FR-361 itself, with a subtype-only population in AC-9. |
| FND-002 | fixed | Fixed in f9118b199: The first refuted instance now decides reported instance, counterexample, basis and cause; AC-10 adds the mixed-cause vector (cause none at (all, c) under [c, e], undefined-evaluation at (all, e) under [e, c]), which I checked against TC-290's fixture. |
| FND-003 | fixed | Fixed in f9118b199: The provision form is now FR-361's own closed {universe, identity} key, with FR-181 cited only for the hex spelling; vectors recomputed without type. |
| FND-004 | fixed | Fixed in f9118b199: Step 7 now states its universes (counters = [c, d], all = [c, e], specials = [s]), names the refuted claim and the permuted list; AC-9 matches. |
| FND-005 | fixed | Fixed in f9118b199: Binding and matching are now SHALL statements. |

Round 2, head bb0e3d947.

| FND | outcome | reason |
| --- | --- | --- |
| FND-006 | fixed | Fixed in bb0e3d947: Zero instances is now 'no population overlaps the over type, or every overlapping population has an empty universe'; a non-conforming key is still an instance; AC-8 adds the all: Entity = [e] case; TC-294 step 7 adds the unrelated-population subject and TC-290 step 7 the all = [e] subject, both consistent with the fixtures (TC-293 ConfigVersion; TC-290 Counter/Entity). Re-checked AC-8, AC-9, AC-11 for supertype, subtype-only, mixed and empty-universe populations: consistent, apart from FND-007 and FND-008. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The zero-instance sentence now defines everything by overlap, but its requires-bound clause still says 'a matching population with no universe'. 'Matching' is no longer defined anywhere. Fix: 'an overlapping population with no universe'. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:67-69 |
| FND-008 | medium | FR-361 names `lookup<T>(p, x)` being `some` as the guard, with `p` the instance's population, but a clause cannot name the instance's population when its instances span several overlapping populations. TC-290 step 7 / AC-11 guard with `lookup<Counter>(all, x)`: for (`counters`, `d`) and (`specials`, `s`) that lookup is `none`, so the guarded `always holds(x.value >= 0)` holds for them without ever reading their value. The expected 'the guarded one holds' is true but is reached vacuously, so a negative `d.value` would also be reported as holding. Fix: state that over several overlapping populations the guard is the disjunction of `lookup<T>(p_i, x)` over every overlapping population (or add a population-free membership form), and make TC-290's guarded check use it. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:175-179 |

Round 3, head 350a1b48b.

| FND | outcome | reason |
| --- | --- | --- |
| FND-007 | fixed | Fixed in 350a1b48b: 'matching' is now 'overlapping'. |
| FND-008 | fixed | Fixed in 350a1b48b: The guard is now the disjunction of lookup<T>(p, x) over every overlapping population. Hand-run of the new AC-11 fixture (inc disabled; c.value = 1, s.value = 1, d.value = 0; e a non-Counter Entity; instances (all,c), (all,e), (counters,c), (counters,d), (specials,s)): the disjunction guard is true for c, d, s and false for e, so the guarded always holds(x.value >= 1) is false only at (counters, d) and the item is refuted there; the all-only guard is false for (counters,d) and (specials,s), so every instance holds and the item is proved. The fixture discriminates. |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | Procedure step 7 still checks `always holds(x.value >= 0)` 'guarded by `lookup<Counter>(all, x)` being `some`', but expected result 7 now reads 'the claim guarded by the disjunction holds'. Both hold, but the procedure and the expected result name different guards. Fix: say 'guarded by the disjunction over `all`, `counters` and `specials`' in the procedure. | spec/test-cases/TC-290-model-behaviours-as-temporal-traces.md:49 |
| FND-010 | medium | FR-395 now gives one instance per key of every overlapping population, but the following sentences still say 'the product of their universes', 'a clause one of whose parameters' populations has an empty universe ... has zero instances' and 'A parameter whose population has no universe is requires-bound'. FR-399:89 ('an empty universe for any one of its parameters') and FR-399-AC-11 have the same problem. With two overlapping populations, one empty and one with keys, these say `no-instance`, while the instance rule and FR-361's zero-instance rule give instances. The state-graph family is not aligned either: FR-390:55-57 still has 'one object of a population ... one instance per key of that population's universe'. Fix: state zero instances per parameter as FR-361 does (no overlapping population, or every overlapping population empty), and align FR-390. | spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md:68-76 |
| FND-011 | medium | FR-361 makes an instance a (population, key) pair, and a key can sit in two overlapping populations (AC-9: (`all`, `c`) and (`counters`, `c`)). A refutation must name its instance, but the wire forms do not carry the population. FR-364's `over_binding` is the bound reference, which distinguishes the two only through the canonical member type and not at all when two populations share it. FR-393 `Instance`, FR-400 `Instance` and FR-402's obligation identity carry only parameter and key. So two instances are named identically. Fix: add the population to the instance identity and to those wire forms, or rule that instances with equal keys across populations are one instance. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:150-165 |

Round 4, head a31fd81f6.

| FND | outcome | reason |
| --- | --- | --- |
| FND-009 | fixed | Fixed in a31fd81f6: TC-290 step 7 now guards the >= 0 check with the disjunction over all, counters and specials, matching expected result 7. |
| FND-010 | fixed | Fixed in a31fd81f6: FR-395, FR-399, FR-399-AC-11 and FR-390 now state zero instances as no key in any overlapping population and requires-bound for an overlapping population with no universe (residual wording in FR-399:88 is FND-013). |
| FND-011 | fixed | Fixed in a31fd81f6: Lead ruling (owner-level default, flagged in the PR body): an instance is its key; a key listed by several overlapping populations is one instance at its first occurrence, so the wire forms keyed by parameter and key stay sufficient. Re-ran AC-9/10/11: instances c, e, d, s; AC-10 [c, e] -> c first, cause none; [e, c] -> e first, undefined-evaluation; the AC-11 guard fixture still discriminates (disjunction refutes at d, all-only guard proves). The type component of those wire references is FND-012. |

## New findings (disposition pass 4)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | high | x is bound to (universe, object_type, identity) with object_type the canonical member type of the first overlapping population that lists the key. FR-204 defines that component as the object's most-specific type, never a static supertype, and makes reference equality componentwise over all three components. In the AC-9 fixture, c is listed first by `all`, so x = (U, Entity, c), while the reference to c from `counters` or `allInstances<Counter>` is (U, Counter, c). Under FR-204 they compare unequal, but FR-361-AC-12 asserts the equality holds. FR-361 only drops the type component when matching a state's object, not in `=`. Which population sorts first by identity text (so a rename) then decides both the equality outcome and the bytes of the reference that FR-364 `over_binding` and the FR-393/FR-400/FR-402 instance keys carry: on the wire those keys are TypedValue references, whose `object_type` is required. So the claim that nothing in FR-364/393/400/402 depends on population is true for structure but not for these bytes. The same conflict already held at round 0 for a subtype object under its population's member type. Fix: bind object_type to the object's most-specific type (FR-204; FR-153 makes it unique per key), or state that reference equality on an over binding ignores the type component and say which object_type the wire carries. | spec/functional/temporal/FR-361-read-model-behaviours-as-temporal-traces.md:166-171 |
| FND-013 | low | FR-399:88 says 'one instance per key of the universe of each population overlapping its parameter's type', which reads as one instance per listing. FR-361 and FR-395 now say a key listed by several overlapping populations is one instance. Fix: add that clause, or cite FR-361's instance definition. | spec/functional/analysis/FR-399-settle-hyperproperty-results.md:88 |
