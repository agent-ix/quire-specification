---
id: SR-13632
title: "Base review of STD-155 FR-154 parameter-name row (PR #307)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-154, TC-280, proposals/checked-package-v2/fixtures/adverse.json, proposals/quire-v1/definitions/model-complete.md (PR #307 head d586f0d)"
review_set: subset
date: 2026-10-21
relationships:
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-specification/TC-280
    type: references
---

# Base review of STD-155 FR-154 parameter-name row (PR #307)

## Summary

Ticket: STD-155 (blocks IR-571). I recomputed all 13 `parameter_name_cases` independently in Python (scratchpad rv305/recompute307.py) against domain-package-acme-orders.json under FR-154 as written. 12 match. `parameter-name-repeated` does not (FND-001). The regex is the same as FR-154's artifact-id regex. The last-segment rule matches the owner-nested `<operation identity>/<param>` form (FR-154-AC-12) and every existing param fixture. Treating names as case-sensitive matches the exact-byte comparison of variant names and case-sensitive identities. `cargo test --test checked_package_v2 --test backend_provider_v1 --test model_member_types --test checked_package_model` passed with its own target dir (11+4+112+24). quire validate: 134 failures on main and on the PR head, none in a touched file.

Units examined:

- `FR-154-AC-14` (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `FR-154` (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `FR-154` (examined): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `FR-154-AC-13` (context_only): spec/functional/type-model/FR-154-admit-domain-package-model.md
- `TC-280` (examined): spec/test-cases/TC-280-model-owned-member-types.md
- `TC-235` (context_only): spec/test-cases/TC-235-business-construct-meaning-binding.md
- `model-complete.md` (examined): proposals/quire-v1/definitions/model-complete.md

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Vector `parameter-name-repeated` expects `expected_identity` `ix://acme/orders/Order/scaled`, the operation. A repeated name is a shared identity, and FR-154's shared-identity row, AC-13 ("naming the shared identity at the later member's path") and TC-235 M33 run 4 all name the shared parameter identity. My recompute gives `ix://acme/orders/Order/scaled/n` at `TypeDefinition.operations[1].params[1]`. The reference reader returns the operation identity for this branch, so TC-280 passes on the wrong value. AC-14's repeated clause should also say which identity the refusal names. | proposals/checked-package-v2/fixtures/adverse.json:142 |
| FND-002 | high | Checklist hash/pin/ledger rule: AC-14's last sentence ("The published `adverse.json` `parameter_name_cases` hold ten refusals and three admitted entries.") and `assert_eq!(cases.len(), 13)` at tests/model_member_types.rs:1965 count fixture entries. They describe the fixture file, not system behaviour. Remove both. | spec/functional/type-model/FR-154-admit-domain-package-model.md:270 |

## Verdict

Not mergeable. One published vector names the wrong identity (FND-001), and AC-14 counts fixture entries (FND-002). The rule itself is sound, and the model-complete.md row agrees with FR-154.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
