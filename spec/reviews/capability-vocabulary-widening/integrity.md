---
id: SR-608
title: "Integrity review of the FR-290 capability-kind vocabulary widening"
type: SpecReview
analysis: integrity
scope: "quire-specification #134 diff 818f555..5d3aa25: FR-290, FR-331, interface_013, FR-270/271/272, native-diagnostics 1-draft.5, FR-349, FR-355, FR-060/061/240/291/292, AD-016, TC-266/270/271, test matrices"
review_set: all
evaluated_revision: "5d3aa25+wt"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

REVISE. Most of the diff holds together. The six `invalid_capability` causes
and the two new `unsupported_projection` causes are the same in
native-diagnostics `1-draft.5`, FR-271 and FR-272. The revision bump is
applied the same way in the README, FR-270 and the header test. The
tool-absence rule reads the same in FR-290, FR-331, FR-272 and TC-271. AD-016's
frame row matches the claim-form assignment table. The new interface_013
invariant matches FR-290's candidate-set rule. Every new AC traces to TC-271.

One contradiction is high severity. FR-290 refuses any carrier that does not
declare `quire.capability-kind/v1`, but no carrier this repo owns (FR-331,
FR-240, FR-291, FR-292) has a member that declares it. As written, FR-290-AC-1
refuses every existing carrier. Four medium findings each leave a rule with
more than one reading or no observable subject:

- multi-kind items;
- the empty-candidate warning payload;
- the SMT discharged-form set;
- protocol tests that still enumerate every member.

Settled rulings are taken as given and not reopened: FR-290 is the single
vocabulary, absence settles `unsupported` (warned), CG `negotiate_*` is the one
negotiation point, the old vocabulary has no compatibility path, and extent
belongs to QSL #222.

## Review set

`review_set: all` over the diff. Consumers of FR-290 were found by grepping
`FR-290`, `claim kind` and `six` across `spec/`, `proposals/`, `docs/` and
`tests/`. Design context was checked against QSL ADR-012 draft (QSL PR #234)
§1.1, §7.1–§7.4.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-290 refuses any carrier that declares no vocabulary identity, and says each format's owner assigns the member that carries it. No format this repo owns does so. FR-331 `manifest`/`request`, the FR-240/FR-061 protocol result and FR-291/FR-292 have no such member, so under FR-290-AC-1 every one of them is refused. TC-271 CK-03 tests a member that no spec defines. Fix: add a vocabulary-identity member (for example `capability_vocabulary: "quire.capability-kind/v1"`) to the FR-331 manifest and request, with an FR-331 AC. Either do the same for FR-240/291/292, or state that `contract_version` implies the identity for each. | FR-290; FR-331; FR-240; FR-291; FR-292; TC-271 CK-03 |
| FND-002 | medium | Multi-kind items have two readings. The FR-290 Description says the dispatcher matches "exactly one member". FR-331 (`required FR-290 kinds`), the candidate-set rule ("every kind the item requires") and the FR-272 payload all treat required kinds as a set. The advertised-mode table compares the extent with the candidate's modes "for that kind", but gives no rule when two required kinds settle differently (for example one `bounded`+`unbounded`, one `bounded` only). Fix: state that an item carries a non-empty set of required kinds, one per claim form, and give the rule that combines per-kind mode dispositions (for example: any `unsupported` gives `unsupported`, otherwise any `requires-bound` gives `requires-bound`). | FR-290 Description, Advertised mode; FR-331 `request`; FR-272 |
| FND-003 | medium | The empty-candidate warning payload does not agree across three places. The FR-290 table names the kinds that "the named backend, or when none is named every registrant, leaves unadvertised". FR-272 and native-diagnostics name "every required FR-290 kind no registrant advertises", which is wrong when a backend is named. ADR-012 §7.3 says "every unmet capability kind". Under split advertisement (X advertises only A, Y only B, the item needs A and B) the set is empty but no kind is unadvertised by every registrant, so the warning names nothing. Fix: define unmet kinds once in FR-290. With a named backend, they are the required kinds it does not advertise. With none named, they are the required kinds no registrant advertises; when that set is empty, retain each registrant's unmet kinds. Mirror the text in FR-272 and native-diagnostics. | FR-290 Candidate set; FR-272 `unsupported_projection`; native-diagnostics |
| FND-004 | medium | The SMT discharged obligation-form set is not declared anywhere. Registration no longer advertises forms, since FR-290 moves them into the negotiation arm. FR-355-AC-6 and TC-270 DP-01 still require corpus coverage of "every obligation form the backend discharges", but nothing states which forms those are, so neither can be checked. FR-349-AC-2 covers only forms outside the four named techniques. It misses a named form this backend does not discharge, which is the TC-266 SMT-01b case. Fix: have FR-349 declare the SMT arm's discharged-form set as a fixed, inspectable part of its negotiation arm (not a capability kind). Reword AC-2 to "a form outside its declared discharged set settles `unsupported`, warned, naming the form". | FR-349 AC-2; FR-355 AC-6; TC-266 SMT-01b; TC-270 DP-01 |
| FND-005 | medium | Protocol consumers were not updated. FR-060/061/240/291/292 now say "protocol-family", but TC-060 ("Enumerate every FR-290 member") and TC-061 ("Enumerate every FR-290 claim kind") still range over all ten members, including kinds that a protocol result must not carry. No AC states that a non-protocol kind on a protocol result, fragment or backend is refused. FR-290-AC-1 (ten labels plus carrier identity) is still traced to TC-060/TC-061, which test neither. Fix: narrow the TC-060/061 enumerations to protocol-family members. Add a refusal vector for a non-protocol kind, backed by FR-061-AC-17 and FR-291/FR-292 wording. Trace AC-1's ten-label and identity clauses to TC-271 only. | TC-060; TC-061; FR-061-AC-17; FR-291; FR-292; spec/protocol/tests.md |
| FND-006 | low | Items with no required kind have no candidate rule. ADR-012 §7.2 gives such an item the named backend or every registrant, and every requested item reaches `negotiate_*` once. FR-290's rule is vacuous for zero kinds, so an unnamed item with two or more registrants settles `invalid-request`. FR-290-AC-5 covers only the admission-time `case` obligation. Fix: add one sentence to FR-290's candidate-set section for items with no required kind, or state that every requested item requires at least one kind. | FR-290 Candidate set, AC-5; ADR-012 §7.2 |
| FND-007 | low | FR-271's `unsupported_projection` meaning is narrower than its causes. "No registered backend supports" leaves out a named backend that does not advertise the kind, and an obligation form the one candidate does not discharge. Only the tool clause is tagged `1-draft.5`, although `unbounded-extent` is also new. Fix: reword to cover every case and tag both new clauses. | FR-271; FR-272 |
| FND-008 | low | The matrices disagree. TC-271 is P0 in spec/complete-v1/tests.md and P1 in spec/protocol/tests.md. The V1-BACK-017 row (FR-290, Amend) still lists only TC-060 and TC-231. Fix: pick one priority, and add TC-271 to V1-BACK-017. | spec/complete-v1/tests.md; spec/protocol/tests.md |
| FND-009 | low | FR-331 `candidates` "ascending identity order" does not define the comparison. It also omits the manifest digest, which ADR-012 §7.1 uses in the candidate member and sort key `(identity, manifest digest)`. A request `extent` outside `bounded`/`unbounded` has no named cause. `unknown-mode` covers registrations only, and the generic FR-331 out-of-domain refusal has no typed cause. Fix: define the order (bytewise UTF-8 identity, then digest) and the candidate member shape. Name the refusal cause for a bad request extent. | FR-331; ADR-012 §7.1; FR-272 |
| FND-010 | low | FR-290-AC-4 still reads "a claim kind whose required capability has no registered backend". After the widening the kind is the capability, and the AC leaves out the named-backend empty case. Fix: "An item whose candidate set is empty settles `unsupported` with a warning naming its unmet kinds (as defined per FND-003)." | FR-290-AC-4 |

## Consistency checks that pass

- Catalog: the six `invalid_capability` causes, their retained payloads, and
  "no cause maps a refused label to an FR-290 member" match in
  native-diagnostics, FR-271, FR-272 and FR-272-AC-9. The `1-draft.4` or
  earlier non-emission rule is stated. The README, FR-270 and
  `tests/checked_package_v2.rs` agree on `1-draft.5`.
- Negotiation: the FR-290 candidate table, FR-331-AC-4, interface_013
  (`negotiate` input plus invariant) and TC-271 NG-01..NG-09 agree, and the
  advertised-mode rows match ADR-012 §1.1.
- Tool absence: FR-290, FR-331 `results` and AC-6, the FR-272
  `tool-unavailable` cause and TC-271 TA-01..TA-03 agree. The probe runs after
  routing, with no hold, fallback or downgrade.
- Claim-form table: it agrees with AD-016 (frame gives `operation-contract`)
  and with FR-146 (`case` exhaustiveness is settled at admission as
  `undefined_expression`/`unproved-exhaustiveness`).
- Traceability: FR-290-AC-5..8, FR-331-AC-4..6 and FR-272-AC-9 each trace to
  TC-271, and TC-271 `verifies` FR-290, FR-331 and FR-272.

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | `spec/objects/protocol/FR-290-protocol-claim-kind.md:65-77`; `spec/objects/interfaces/FR-331-backend-provider-envelope.md:26,40,52`; FR-240:142-143; FR-291:27; FR-292:30 |
| FND-002 | resolved | Each item has exactly one kind, FR-290:86; FR-331:28; so no rule for combining per-kind dispositions is needed |
| FND-003 | resolved | FR-290:176; FR-272 `unsupported_projection` row; native-diagnostics:96. All three name the item's kind and any named backend. |
| FND-004 | resolved | Declared obligation-form set, `spec/functional/ir-proof/FR-349-register-smt-backend.md:31-34,57-60`, AC-2 (FR-349:81); FR-355-AC-6; TC-270 DP-01 and DP-03 |
| FND-005 | resolved | TC-060:21; TC-061:67-70; FR-061-AC-17 (FR-061:237); FR-290-AC-1 traces to TC-271 |
| FND-006 | resolved | Every requested item requires exactly one kind, FR-290:86; the `case` and FR-353 rows request no backend, FR-290:94,101 |
| FND-007 | resolved | FR-271:62 covers the empty set, the named backend and the obligation form, and tags both `1-draft.5` causes |
| FND-008 | resolved | `spec/protocol/tests.md:259` and `spec/complete-v1/tests.md:1154` are both P0; V1-BACK-017 at `spec/complete-v1/tests.md:217` |
| FND-009 | resolved | Candidate member, order and digest, FR-331:29 and FR-290:147-150. A missing classification is `absent-extent` (FR-290:127-128). An out-of-domain classification falls under FR-331's refusal before consumption (FR-331:39-40). |
| FND-010 | resolved | FR-290:232 |
| FND-011 (new) | open | medium. FR-290:180-181 says "Registration order, display names, advertised modes and ambient state never change a candidate set, its order or a disposition". The advertised-mode table at FR-290:134-143 makes the disposition depend on advertised modes (for example `requires-bound` versus the arm's disposition). Fix: remove "advertised modes" from that list, or scope the sentence to "a candidate set or its order". |
| FND-012 (new) | open | low. FR-349:57 and FR-355-AC-6 read "the backend descriptor's declared obligation-form set". The FR-331 manifest descriptor at FR-331:27 lists identity, digest, version, (kind, mode) pairs, domains, options, bounds and pinned tool, with no obligation-form set. Fix: add "declared obligation forms (FR-346/347/348/147 subset)" to the FR-331 manifest descriptor, or state in FR-349 that it travels in `options`. |
| FND-013 (new) | open | low. native-diagnostics:97 says every `invalid_capability` cause retains "the request item index, or for a registration cause the backend identity". `unsupported-version` refuses the whole carrier and has no item index. Fix: add "`unsupported-version` retains the carrier, not an item index". |

Round 2 verdict: ACCEPT WITH FINDINGS
