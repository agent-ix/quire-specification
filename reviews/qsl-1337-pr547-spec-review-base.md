---
id: SR-1337003
title: "Spec review of quire-specification PR #547 (QSL-1337): ContractPackage id frames"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@bd5bba31 (PR #547, contract-package-id-frames) vs origin/main; FR-322 Operand index segments Scope bullet, FR-330 Properties, feature_accounting, source_map and reader sections, FR-330-AC-7, FR-330-AC-14, TC-934"
review_set: base
date: "2026-10-31"
---

## Summary

Ticket: QSL-1337. FR-322 and FR-330 state the same two-frame ruling for
`source_map` without contradicting each other. Output `nodes[].node_id`
resolves in the output graph. Requested, refusal and occurrence ids resolve in
the source graph. The protocol-reference binding ("exact local node_id",
FR-330:59) is an output-node body reference, so the ruling's "any other id
naming an output node" covers it.

Two ContractPackage id families still have no frame or no reader that can
resolve them: `feature_accounting`, and the source frame as a whole.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The source frame cannot be resolved by the ContractPackage reader. Its inputs are the ContractPackage and one admitted package per `dependency_selections` entry. `package.source` is "provenance, not a second package", so the source checked package is not supplied. The reader still "checks ... node_ids and references" and source-map totality and correspondence. TC-934 joins each occurrence to "its graph node". A source-frame id's index segments name a graph the reader does not have, so those checks cannot rebuild source-frame keys or join an occurrence `node_id` to its output `nodes[].node_id` (different frames, so wire values differ). FR-330 must either make the source checked package a reader input, or say exactly which checks treat source-frame ids as opaque wire values and how the cross-frame node/occurrence correspondence is decided. | spec/objects/interfaces/FR-330-contract-package.md:36-40,294-300,318-330,363-367; spec/test-cases/TC-934-contract-package-source-map.md:24-27 |
| FND-002 | high | `feature_accounting` ids are assigned no frame. A record's `node_id` is "the requested exact `node_id`" (source frame by the ruling), yet for `lowered` it "is the root node_id in `semantic_graph`" (output frame). For a derived root these are now different wire values, so the two rules contradict each other. The payload ids `unsupported_node_id`, `unbounded_type`, `body_node_id` and `refusal.locus` also have no frame. The accounting/request bijection and the `refusals` pointer "by the exact requested id" depend on the answer. | spec/objects/interfaces/FR-330-contract-package.md:153-193,288-290; spec/objects/interfaces/FR-322-checked-package-artifact.md:1112-1118 |
| FND-003 | medium | FR-330-AC-14 is only partly testable as written. "Resolves it against the source package's graph rather than the output graph" is not observable at the ContractPackage reader, which has no source graph (FND-001). TC-934's new case does not require the absent requested id to hold an index segment, so a string-only id passes whichever frame is used. The AC and TC should require a derived absent id with at least one index segment, carried byte-equal to the source package's spelling, and admitted with `invalid_input` and an empty output graph. | spec/objects/interfaces/FR-330-contract-package.md:452; spec/test-cases/TC-934-contract-package-source-map.md:51-57 |

## Verdict

Not mergeable until FND-001 and FND-002 are resolved. Clean units: the
FR-322 Scope bullet's ContractPackage sub-bullet, which agrees with the FR-330
two-frame paragraph; the protocol-reference binding at FR-330:59; and the
absent-key paragraph's `invalid_input` with an empty output graph.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | high | FR-330's reader never checks the output graph's index segments, yet its earlier stages already need the output keys rebuilt. The ContractPackage reader does not run FR-322's reader, and no FR-330 rule refuses an out-of-range or cyclic `{"node": i}` segment in an output `semantic_graph` node id or body reference. The new cross-frame rules compare rebuilt output keys at the "source-map totality" and "accounting/request consistency" stages: `nodes[].node_id` against occurrence keys, and a `lowered` root against its source key. Both stages run before "graph references", the only later stage. On an output graph with a dangling or cyclic segment, those comparisons are undefined and no refusal or locus is stated. | spec/objects/interfaces/FR-330-contract-package.md:350-356,392-407,262-268 |
| FND-005 | high | The new rule makes a source-key reference "of any other shape (an extra member, a missing `source_key`, a negative, fractional, string or exponent-form index)" `malformed_wire`, and FR-330-AC-15 asserts the same. FR-330's own closed-shape rule (line 366) says a missing, unknown, wrong-kind or out-of-domain member is `invalid_package` with `missing-member`/`unknown-member`/`wrong-value-kind`/`invalid-value`. FR-330-AC-11 says "a member outside the closed schema is `invalid_package`". The closed-shape stage also runs before the `source_keys` stage, so the same input gets two different codes and loci. | spec/objects/interfaces/FR-330-contract-package.md:366-372,402-405,492,496; spec/test-cases/TC-937-contract-package-reader.md:189-198 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (411496f7): the ContractPackage is self-contained. Source-frame ids are `{"source_key": i}` into the new required `source_keys` table, and FR-322 Scope says no ContractPackage id resolves against the source graph. The reader can now rebuild every source-frame key from its own inputs. |
| FND-002 | fixed | Round 1: `feature_accounting[].node_id` and the payload ids `unsupported_node_id`, `unbounded_type`, `body_node_id` and `refusal.locus` are source-key references. A `lowered` record's source key and output root must rebuild to one structural key (FR-330-AC-16). |
| FND-003 | fixed | Round 1: FR-330-AC-14 and TC-934 require a derived absent request with at least one index segment, carried in `source_keys` and admitted with `invalid_input` and an empty output graph. |
