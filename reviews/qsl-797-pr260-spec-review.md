---
id: SR-11267
title: "Spec review of PR #260: link change in the native-run-result/2 frame witness"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@7fd97f95ce6962f5a0ac403c4e4b2b38e581428a; spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md (FR-352-AC-9), spec/test-cases/TC-255-native-run-result-v2-wire.md (NR-11)"
review_set: subset
---
# Spec review of PR #260 (QSL-797)

## Summary

Ticket: QSL-797. The PR amends FR-352-AC-9 so the frame witness can carry a
relationship `link` change, and adds TC-255 NR-11, which reuses QSL TC-1037
rows 1, 2, 4, 10 and 11 by reference. Integrity, EARS and object-consistency
checks ran in this one pass.

Examined against the merged QSL spec (quire-spec-language origin/main):

- FR-267 frame witness wire `link` row: members `relationship` (declaration
  key), `difference` (`added`|`removed`), `source` and `target` (closed
  `{population, key}`), `modifies`; top-level `population` is the source's.
  FR-352-AC-9 names the same members and the same population rule.
- FR-267-AC-12 reader edits (drop each link member, drop a `source`/`target`
  member, unknown member, `difference` other value or `null`, bare-key end).
  NR-11 runs a subset of them; each listed edit is a refusal under FR-267's
  reader rules, so each expected "located wire diagnostic" is reachable.
- FR-115 `Link` witness and step 5 order, FR-115-AC-8 and AC-10: row 2 is
  `Link { Pump-housing-Tank, removed, {Q,p1}, {Q,t1}, [] }`, row 4 is
  `added` with source `{Q,p3}`, so top-level `population` is `Q` for both.
  Rows 1, 10 and 11 report `success`, exit 0; FR-115 lists all three
  (modified relationship, end at a deleted object, relationship not complete
  in both snapshots) as links step 5 leaves out. NR-11's expected results
  match.
- FR-116-AC-7: replay of the link change; NR-11 makes no replay claim, no
  conflict.
- QSpec FR-013-AC-10 and FR-340-AC-11: a relationship `modifies` entry is one
  node key that grants every link; deleted-object ends are ignored; a
  relationship not complete in both snapshots is not compared. Consistent
  with AC-9's "left out" sentence.
- TC-255 NR-01 to NR-10: NR-10 keeps its forbidden-parent `field_write`
  fixture; NR-11 is a separate row, so nothing existing is widened or
  contradicted.

No tracking, hash or version ceremony is added.

## Verdict

PASS. No blocking finding. Two low findings are recorded.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-352-AC-9 cites "FR-106 check 11.5" unqualified, while the same AC qualifies "QSL FR-115" and "QSL FR-267". QSpec has no FR-106, so the reference does not resolve inside QSpec; it should read QSL FR-106. | spec/functional/foundation/FR-352-mint-native-run-result-witness-wire.md:123 |
| FND-002 | low | NR-11 runs only part of the FR-267-AC-12 reader edit list: it omits removing `key` from `target` and adding an unknown member to the change. NR-10's extra-member edit and FR-352-AC-8 cover unknown members in general, so this is a coverage nit and not a contradiction. | spec/test-cases/TC-255-native-run-result-v2-wire.md:38 |
