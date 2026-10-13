---
id: SR-9042
title: "Base review of QSpec PR #251 (QSL-721 per-declaration specialization-cycle refusals) against QSL behaviour"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@82cba376a03882c3cac383cfc9c880b4964727eb; proposals/quire-v1/definitions/model-complete.md, spec/test-cases/TC-196-conformance-redefinition-and-dispatch.md, spec/test-cases/TC-235-business-construct-meaning-binding.md; compared with agent-ix/quire-spec-language@9a184a1e062b4479e239a4968829542d07fa6393 (impl/business-model-meanings, includes #787)"
review_set: subset
---

## Summary

Ticket: QSL-721. The PR changes four places so that a specialization cycle is
refused once per declaration on the cycle: TC-235 M17 run 3, TC-235 M20 run 4,
TC-196 R01 and the model-complete phase 3 cycle paragraph. This matches QSL
FR-082 ("refuse specialization for every declaration on the cycle"). It also
matches the QSpec precedent for variant types in TC-195 N14 (one refusal per
node on the cycle).

Checked against the QSL engine:

- M17 run 3 (record value types). QSL #787 retains one typed intake refusal
  per cyclic record declaration (`business_generalization_refusal`,
  qsl-semantics/src/model/intake.rs:2071-2120). Normalization reports both
  refusals after 24 `normalize.record` charges
  (`tc235_m17_record_value_specialization_vectors`). The order is
  `SpecializationCycle{ancestor: Money, via: TaxedMoney}` then
  `{ancestor: TaxedMoney, via: Money}`, which is declaration-key order, with
  each refusal at its own declaration. The 24-charge count matches the fixture:
  K2 has 21 declarations, plus `TaxedMoney`, its field `tax_minor` and the
  clause `NonNegative`.
- M20 run 4 (process) and TC-196 R01 (object type). QSL still reports one
  refusal per cycle edge set. `normalize.rs` keeps the first candidate per
  `directed_edges` set (:2852-2867), citing the old model-complete "exactly
  once" text, and the QSL tests assert `len() == 1`
  (`r01_a_closing_generalization_cycle_names_the_full_rotated_chain`,
  model_normalization.rs:2468; `tc235_m20_*`, tc235_record_event.rs:363 and
  :569). This is a QSL gap, not a defect in this PR. QSL FR-082 already
  requires a refusal per declaration, so the QSL engine contradicts the QSL spec
  as well. The charge counts do not change: M20 run 4 stays at 23 (K2's 21,
  plus `ExpressFulfilment` and `PF`; steps are not declarations, as with
  `Fulfilment`'s two declarations), and R01 keeps its six cycle-check charges.
  Both match the QSL charge pins.
- Including M20 run 4 was right. The rule is meaning-independent
  (model-complete phase 3 and FR-208 Generalization), so leaving one process
  cycle refusal would contradict the edited paragraph.
- Cause shape. FR-272 and native-diagnostics say `specialization-cycle` retains
  "the cycle's type declaration keys in path order", and model-complete rotates
  that path to start at the least key. "Each listing [Money, TaxedMoney]" is
  therefore faithful to QSpec. QSL is not consistent with it. Its typed cause is
  `SpecializationCycle{ancestor, via}`, two keys only, so a cycle of three or
  more keys cannot be retained in typed form. The full listing exists only in
  `detail` prose. On the record path that prose is not rotated: the
  `TaxedMoney` refusal's detail lists `[TaxedMoney, Money]`. QSL's M17 test
  asserts only `ancestor`/`via`. This is a QSL gap against FR-272. It needs a
  ruling on whether the typed cause gains a cycle-key list; it is not a PR
  defect.
- M17 run 2 (wrong-meaning supertype, unchanged). FR-154 says intake reports
  declaration-table refusals after its per-declaration `normalize.record`
  charges, and only admission checks run before the first charge. TC-234 I03
  ("after three `normalize.record` charges, two refusals") and TC-235 M11 (21
  charges for the same `supertypes[0]` meaning refusal) follow that rule. The
  24-charge claim is therefore supported by QSpec. QSL refuses at the reader
  (`read_refusals`) and asserts no charge count, which is a QSL gap.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The new model-complete text says "every declaration on a cycle is refused ... once, at its own declaration, in declaration-key order" and "each refusal lists the cycle's type keys". It does not cover a declaration that lies on two or more distinct cycles. Examples are A↔B plus A↔C, or the two directed cycles over {A, B, C} that QSL's `distinct_directed_cycles_with_the_same_nodes_are_not_deduplicated` exercises. One reading refuses such a declaration once in total, which leaves undefined which cycle's keys it lists. The other reading refuses it once per cycle. The text also leaves the order of refusals across several cycles undefined: key order inside each cycle in detection order, or one global key order. Two implementers would emit different refusal lists. | proposals/quire-v1/definitions/model-complete.md:340-347 | missing-requirement |

## Verdict

One medium finding: per-declaration cardinality and ordering are undefined when
a declaration lies on more than one cycle. The four edits are otherwise correct
and consistent with QSL FR-082. Including M20 run 4 was right, and the M17
(24), M20 (23) and R01 (six cycle-check) charge counts are correct. Four QSL
gaps follow from this PR and need a QSL ticket: normalize.rs per-edge-set
dedup (M20 run 4, TC-196 R01), the typed cause without a cycle-key list, the
unrotated record-path detail, and M17 run 2 refused at the reader without
charges. They are recorded in the summary, not as findings against this spec.
