---
id: SR-2679
title: "Base review of union-value preimage selection"
type: SpecReview
analysis: base
scope: "FR-322 application and derived-shape selectors; FR-440 union-value preimage and AC-9/10; TC-395 N14-N16; spec/spec.md FR-440 index entry"
review_set: base
---

# Base review of union-value preimage selection

## Summary

The QSL-679 amendment reconciles the two QSpec selectors with the paired
QSL FR-092 and FR-319 amendment. Every `value`/`union_value` uses the existing
application preimage while keeping its aggregate body. Nullary and
reference-payload values take the same selection; record and tuple selection,
case arm order, recursion encoding and reader refusal precedence retain their
existing rules. The canonical identities bind checked semantic content to
its proof boundary; no tracking digest or new identity domain is introduced.

This is a specification review, not completed executable qualification.
The requirements specify fixed independent byte/key oracles and separate
producer and strict-reader checks. TC-395 N14-N16 makes their positive,
negative and regression obligations explicit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Executable coverage remains outstanding: the computed matrix marks AC-9 and AC-10 `untagged`. Existing QSpec application-preimage qualification selects bodies containing applications and does not qualify the union-value exception. Producer vectors and strict-reader admission/refusal must execute before QSL-679's qualification is complete. | [FR-440](../../objects/interfaces/FR-440-checked-package-union-and-case-nodes.md), [TC-395](../../test-cases/TC-395-checked-package-union-and-case-nodes.md) |
| FND-002 | low | Full scoped Quire validation is blocked by the pre-existing FR-322 value-object shape: the resolved schema reports missing `id` and `fields`. Both errors reproduce on the unchanged main artifact. The selector amendment introduces no new validation failure. | [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) |

## Validation

On terra, `/home/peter/.local/bin/quire` reports CLI 0.36.2 and engine
0.50.2; `/home/peter/.local/bin/quoin` reports 0.28.3. Scoped validation of
the four edited artifacts exits 3 for FR-322's two errors. Validation of
the unchanged main FR-322 reproduces them. FR-440, TC-395 and the master
index pass. `quire matrix --scope . --format tsv` reports the new criteria
as `untagged`. `git diff --check` passes.

The review found no additional contradiction in the changed selectors:
FR-322 explicitly includes union values in the application-node key stage
and excludes them from the derived-shape stage; FR-440 supplies the exact
preimage and its aggregate body. The paired QSL requirements select the
same representation. FND-001 remains a qualification gate; FND-002 remains
a baseline validation limitation. Neither is a claim of a completed test.
