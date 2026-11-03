---
id: SR-20261103145408
title: "QSL-1538 r1 gap analysis of quire-specification PR #610"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR #610 head f7934212; FR-465-AC-3, FR-465-AC-4, FR-465-AC-10; FR-426 duplicate/lose row and FR-426-AC-3; TC-947; TC-378; tests/protocol_node_mutation_manifest.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-465
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: references
---

# QSL-1538 r1 gap analysis of PR #610

## Summary

Ticket: QSL-1538. This is a manual criteria-to-tests check. Quoin was
available, but this repo's executable coverage for these ACs is the mutation
manifest, so the manifest was read directly.

- **FR-465-AC-10** is traced to TC-947 CH13a, CH13b and CH14a.
- **No executable row in this repo.** CH13a and CH13b are `later`, and CH14a
  is `pending` behind the Q-LAYOUT schema gap. That matches its siblings:
  CH13 and CH17 are `later`, and CH4, CH14 and CH15 are `pending`. It is not
  new debt from this PR.
- **Executable coverage elsewhere.** AC-10 runs in Contract IR PR #855
  (`protocol_channel_delivery_admits_canonical_count_interval`). That PR is
  reviewed separately.
- **Partition check.** It was recomputed in Python with the test's own
  `ids_of` rule (entries with `sub_case_of` skipped). 409 vector ids give 409
  manifest entries across rows, later and pending, with no repeat, and the
  two sets are equal.
- **Manifest schema.** The manifest validates against
  `protocol-node-mutation-manifest.schema.json` (Python jsonschema, 0
  errors) and ends with a newline.

## Verdict

**MINOR GAPS.** One medium finding: FR-426's new interval clause has no
acceptance criterion and no test vector. FR-465-AC-10 is fully traced to its
vectors.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-426 `duplicate`/`lose` row now says an interval "admits neither". FR-426-AC-3 still says only "`duplicate` and `lose` are enabled exactly under the delivery policies that admit them", and TC-378 has no interval-channel vector. No AC or test asserts that an interval channel offers no `duplicate`/`lose` step. Add the interval case to FR-426-AC-3 and one TC-378 vector, for example a `[1,1]` channel where no execution loses or duplicates, mirroring FR-509-AC-5's last clause. If FND-001 of the spec review is fixed, the same vector covers the d-entry send. | spec/functional/protocol/FR-426-apply-protocol-steps.md:99,221; spec/test-cases/TC-378-channels-under-delivery-policies.md |
