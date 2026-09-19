---
id: SR-613
title: "EARS conformance review of the FR-290 capability-kind widening"
type: SpecReview
analysis: ears-conformance
scope: "quire-specification #134 diff 818f555..5d3aa25: FR-290, FR-331, FR-349, FR-355, FR-060, FR-061, FR-240, FR-291, FR-292, FR-270, FR-271, FR-272, TC-271"
review_set: all
evaluated_revision: "5d3aa25+wt"
review_date: "2026-09-19"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-290, type: references }
  - { target: ix://agent-ix/quire-specification/FR-331, type: references }
---

## Summary

REVISE. The engine check covers the 13 changed requirement and test documents and finds all 13 grammar-clean, with 0 `[ears:*]` findings. The semantic pass finds one high finding, six medium findings and two low findings. In FR-349, AC-2 and the Behavior section disagree on when the SMT negotiation arm settles `unsupported`. FR-290 adds candidate-set, advertised-mode, version and tool-absence behaviour only as declarative prose, with no EARS statement and no named subject. The single FR-290 SHALL statement still covers only claim matching. The FR-060, FR-061, FR-240, FR-291 and FR-292 changes narrow those requirements to the `protocol` family and are clean.

## Review set

This review uses the EARS-conformance lens only. It covers the normative statements and acceptance criteria that the diff adds or changes. The rulings below are settled, and no finding reopens them: FR-290 is the single capability-kind vocabulary and is widened in place; QSL's old four-kind vocabulary gets no compatibility; specs state what is. Engine run: `quire validate --summary` over the 13 changed documents reported `13/13 docs grammar-clean (100%); 0 grammar finding(s)`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-349-AC-2 settles `unsupported` for an obligation form *outside* the four named forms. The Behavior section settles it for any form the arm does not *discharge*, and TC-266 SMT-01b tests an induction item, one of the four forms, settling `unsupported`. The two rules decide different settlements for a plain quantifier-free clause and for an undischarged form among the four. Fix: state one rule, such as "If an item's obligation form is not one the SMT negotiation arm discharges, then the arm SHALL settle it `unsupported` with a warning naming the form", and align AC-2 and SMT-01b to that rule. | FR-349; FR-349-AC-2; TC-266 |
| FND-002 | medium | The widened FR-290 obligations have no EARS statement and no named subject. These are candidate-set computation, the advertised-mode dispositions, refusal of an unknown kind or mode at registration, and refusal of a carrier with an unsupported version. Each is written as declarative prose ("The registry computes", "refuses the backend's registration", "is refused"). The one SHALL in the Description ("When a claim is requested of a backend, the dispatcher SHALL match…") has a trigger that does not cover registration or carrier admission. It also names a "dispatcher" that FR-290 does not otherwise identify now that the vocabulary is no longer protocol-only. Fix: add one SHALL statement per obligation with its owner as subject. Example: "When a request item is submitted, the `quire-spec-language` registry SHALL compute its candidate set…"; "When a backend registers, the registry SHALL refuse an advertised kind or mode outside this table…"; "If a carrier declares a vocabulary identity other than `quire.capability-kind/v1`, or none, then the reader SHALL refuse it without reading its labels." | FR-290 |
| FND-003 | medium | FR-290 states tool absence as an Identity-and-comparison bullet, in passive voice and with no SHALL ("an item … records the FR-331 result `unsupported`"). It is an unwanted condition with no named actor, and the scope of "when the adapter probes it before the run" is ambiguous: it may qualify both "missing" and "does not match", or only the second. Fix: "If the adapter's probe after routing and before the run finds the pinned tool missing or not matching the descriptor's pinned identity, then the adapter SHALL record the FR-331 result `unsupported` with a warning naming the kind, backend and expected tool identity." | FR-290; FR-290-AC-8; FR-331-AC-6 |
| FND-004 | medium | The FR-290 empty-candidate row names "every required kind that … every registrant leaves unadvertised". When an item requires several kinds, each kind can be advertised by some registrant while no registrant advertises all of them. The set is then empty, and the warning names no kind, so the response is not verifiable. FR-272's `unsupported-requested-capability` payload ("every required FR-290 kind no registrant advertises") has the same gap. Fix: define the warning payload for this case, for example each registrant with the required kinds it lacks, or the full required-kind set, in FR-290 and FR-272. Add a TC-271 vector with two registrants that each advertise one of the two required kinds. | FR-290; FR-272; TC-271 |
| FND-005 | medium | FR-290 Description and FR-290-AC-5 give each claim exactly one kind. FR-290's candidate-set rule, FR-331 `request` ("its required FR-290 kinds") and FR-272 ("every required FR-290 kind") give each item a set of kinds. The spec does not say whether an item carries one claim or several, so the cardinality of the `request` item's kinds field is ambiguous. Fix: state the relation in FR-290. Either one item carries one claim and one kind, and the plurals become singular, or an item carries several claims and their kinds form the required set. | FR-290; FR-290-AC-5; FR-331; FR-272 |
| FND-006 | medium | FR-355-AC-6 requires a corpus member "for every obligation form the registered SMT backend discharges under FR-349". After this diff, FR-349 no longer advertises or declares its discharged obligation forms; the negotiation arm settles them per item. The AC's reference set therefore has no declared source, and TC-270 DP-01 cannot enumerate it. Fix: require FR-349 registration to declare its discharged obligation-form set, or name the fixed four forms in FR-355-AC-6. | FR-355; FR-355-AC-6; FR-349; TC-270 |
| FND-007 | medium | The FR-349 Description packs three responses into one SHALL: register, advertise, and "settle … which of the … obligation forms it discharges". The third response is not observable as written, and pairs are described as things the backend "discharges". Fix: split the Description into three statements. Write the advertisement as "advertise the FR-290 (kind, mode) pairs whose claims it discharges". Express the arm settlement as the If/then from FND-001. | FR-349 |
| FND-008 | low | FR-290-AC-4 still reads "A claim kind whose required capability has no registered backend". Under the widened vocabulary, the kind is the capability, so the subject is circular. Fix: "A claim whose required capability kind no registered backend advertises settles `unsupported` with a warning naming that kind", which matches FR-349-AC-5. | FR-290-AC-4 |
| FND-009 | low | The FR-271 `unsupported_projection` meaning omits the case where the one candidate's negotiation arm does not discharge an obligation form. FR-272-AC-9 maps that case to this code. The row's "(revision `1-draft.5`)" tag also sits on the tool-unavailable clause alone, although the FR-271 revision prose says `1-draft.5` also adds `unbounded-extent`. Fix: add the undischarged-obligation-form case to the meaning and place the revision tag on both `1-draft.5` clauses. | FR-271; FR-272-AC-9 |

## Round 2 dispositions

Evaluated against the uncommitted working tree on 5d3aa25. Paths are relative to the repository root.

| Finding | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | resolved | FR-349 Description, body and AC-2 all settle a form outside the declared obligation-form set `unsupported`, `spec/functional/ir-proof/FR-349-register-smt-backend.md:31-34,57-60,81`; SMT-04 (TC-266:40) |
| FND-002 | resolved | Named-subject SHALL statements now cover the reader (`spec/objects/protocol/FR-290-protocol-claim-kind.md:75-82`), registry registration (FR-290:114-117,152-153) and `negotiate_*` (FR-290:127-132,170,180). The Description's "dispatcher" (FR-290:22) remains; see FND-010. |
| FND-003 | resolved | Tool absence is an If/then SHALL with the adapter as subject, FR-290:213-217 |
| FND-004 | resolved | One kind per item, FR-290:86; FR-290:176; FR-272 row; native-diagnostics:96 |
| FND-005 | resolved | FR-290:86-88; FR-331:28 |
| FND-006 | resolved | FR-349:57; FR-355-AC-6 |
| FND-007 | open | low. The response is now observable, but the FR-349 Description (FR-349:28-34) packs four responses into one SHALL: register, advertise, declare and settle. Fix: split it into a registration SHALL and an If/then for the arm settlement. |
| FND-008 | resolved | FR-290:232 |
| FND-009 | resolved | `spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md:62` |
| FND-010 (new) | open | low. The FR-290 Description (FR-290:22-24) still makes an unidentified "dispatcher" the subject for all ten kinds. Fix: "Each requested item SHALL require exactly one member of this closed vocabulary", which matches FR-290:86. |

Round 2 verdict: ACCEPT WITH FINDINGS
