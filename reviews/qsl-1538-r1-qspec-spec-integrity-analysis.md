---
id: SR-20261103145406
title: "QSL-1538 r1 integrity analysis of quire-specification PR #610"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification; PR #610 head f7934212; FR-465 channel layout and AC-3/AC-4/AC-10; FR-464 Term shapes; FR-466 interval bounds; TC-947 CH13/CH13a/CH13b/CH14/CH14a/CH17 and Expected Results; protocol-node-vectors.json; protocol-node-mutation-manifest.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/TC-947
    type: references
  - target: ix://agent-ix/quire-specification/FR-464
    type: references
---

# QSL-1538 r1 integrity analysis of PR #610

## Summary

Ticket: QSL-1538. This analysis checks whether the new text agrees with the
rest of the node family and the test case.

- **AC-10 and TC-947.** AC-10 and CH13a, CH13b and CH14a list the same
  admitted and refused spellings. Expected Results 1, 3 and 5 cite the new
  vectors and AC-10.
- **AC-3.** The wording "one of the four delivery policy words or a delivery
  count interval" is correct.
- **Fixtures.** The vectors fixture and the manifest agree. CH13a is filed as
  `later`/`admit`/`needs_new_nodes` like CH13. CH13b is filed as
  `later`/`property`/`needs_property_assertions` like CH17. CH14a is filed as
  `pending` with the same Q-LAYOUT question and evidence as CH14 and CH15.

## Verdict

**MINOR GAPS.** Two low findings. One is a node-id claim in CH13a that
conflicts with CH13b. The other is an unexplained second interval encoding in
the node family.

The text choice is coherent with QSL FR-509, whose Outputs require "a
channel node whose delivery text is `[lo,hi]` with no space", and with IR
#855. It keeps argument 5 a single text term for every policy. But FR-465
does not say why it departs from FR-464's interval shape.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | CH13a expects "four distinct node ids, each distinct from CH13's". A channel is a declared node, and its node id is a function of its declaration path (FR-464 "node id"). Changing `delivery` changes the body, not the id. TC-947 Expected Results 5 says this for CH13b ("change the bodies stated and no declared node id"), so CH13a and CH13b disagree. The vectors fixture says "node keys" for the same row. CH13a copies CH13's existing wording. Use "four distinct channel bodies, each distinct from CH13's", as AC-10 does. | spec/test-cases/TC-947-checked-package-protocol-declaration-bodies.md:86,163; proposals/checked-package-v2/fixtures/protocol-node-vectors.json:2440-2445 |
| FND-002 | low | The node family now has two interval encodings. FR-464 "Term shapes" defines **interval** as a Group of `lower`/`upper`, and FR-466 refuses `lower > upper` with `ill_typed`/`type-mismatch`. The delivery interval is a text `[lo,hi]`, and FR-465 refuses `lo > hi` with `malformed_wire`. FR-465 calls that text "an enumerated text value (FR-464 'Term shapes')", but a grammar is not an enumeration. The choice is deliberate (FR-509 Outputs, IR #855), but a reader of FR-464 could expect the Group shape and the `type-mismatch` cause. Add one sentence to FR-465 saying `delivery` stays one text term so its shape does not vary by policy, and that FR-464's interval shape does not apply. | spec/objects/interfaces/FR-465-checked-package-protocol-declaration-bodies.md:155-160; spec/objects/interfaces/FR-464-checked-package-protocol-node-bodies.md:182,186-188; spec/objects/interfaces/FR-466-checked-package-protocol-control-bodies.md:132-138 |
