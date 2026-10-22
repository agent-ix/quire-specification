---
id: SR-14653
title: 'Spec integrity analysis of PR #344: an operation redefines through Operation.redefines (STD-104)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-specification@f756c29c6; git diff origin/main...f756c29c6 (PR #344, branch std-misc1-std-104): FR-154 redefinition paragraph and FR-154-AC-16, FR-322 "Model-owned members" step 3 and FR-322-AC-30, TC-280 preconditions, MT-08 and MA-16..MA-18, against proposals/checked-package-v2/domain-package-acme-orders.json, tests/model_member_types.rs project_document, filament-core-data@0de623c schema/semantic/v1/semantic-ir.schema.json and FCD FR-068; ticket STD-104'
review_set: subset
date: '2026-10-22'
relationships:
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-280
  type: reviews
---

## Summary

Ticket: STD-104. PR: quire-specification#344, head f756c29c6. FR-154-AC-16 is new and unique. Main ends at FR-154-AC-14, and open PR #267 adds a second AC-14. MA-16 to MA-18 are unused on main and on the other open PRs. The lead default, option (a), matches the existing spec. The model-complete.md Redefines row (line 168) says that `redefines` names "a field or operation of the package". The missing-name row of the FR-154 refusal table (line 84) already covers a dangling `redefines`. QSL intake reads an inline operation `redefines`: qsl-semantics/src/model/intake.rs:2760-2765, `OperationMemberRecord.redefines`, in the qsl-lane2 checkout at QSL 2e6c83637. The published fixture and the FCD contract do not match the new text, though. Gates: `quire validate --scope . spec/**/*.md` reports 134 failed documents, as on main, and the diagnostic multiset is identical. FR-154 and TC-280 have 0 diagnostics and FR-322 has 6, the same as on main. `cargo test --test checked_package_v2 --test backend_provider_v1` passes 117 and 14.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-154 redefinition paragraph | spec/functional/type-model/FR-154-admit-domain-package-model.md:151-162 | examined |
| FR-154-AC-16 | spec/functional/type-model/FR-154-admit-domain-package-model.md:287 | examined |
| FR-154 intake chain step 3 (Lift) | spec/functional/type-model/FR-154-admit-domain-package-model.md:50-52 | context_only |
| FR-322 Model-owned members step 3 | spec/objects/interfaces/FR-322-checked-package-artifact.md:932-947 | examined |
| FR-322-AC-30 | spec/objects/interfaces/FR-322-checked-package-artifact.md:1784 | examined |
| TC-280 Preconditions (redefinition carrier) | spec/test-cases/TC-280-model-owned-member-types.md:36-50 | examined |
| TC-280 MT-08..MT-10, MA-16..MA-18 | spec/test-cases/TC-280-model-owned-member-types.md:130-132,149-151 | examined |
| domain-package-acme-orders.json Sub and SubSub extensions | proposals/checked-package-v2/domain-package-acme-orders.json:1198,1319 | examined |
| project_document | tests/model_member_types.rs:1384-1388 | context_only |
| FCD semantic-ir operation schema | filament-core-data@0de623c schema/semantic/v1/semantic-ir.schema.json ($defs.operation) | context_only |
| FCD FR-068 required-extension row | filament-core-data@0de623c spec/functional/FR-068-decide-and-report-ir-admissibility.md:113 | context_only |
| model-complete.md Redefines row | proposals/quire-v1/definitions/model-complete.md:168 | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Major. The PR's normative text contradicts its own published fixture and TC-280. The TC-280 Preconditions now say that "Each operation redefinition is the redefining operation's `Operation.redefines` member: `Sub/size` and `SubSub/size` name `Order/size`, and `Sub/scaled` names `Order/scaled`". At this head, `domain-package-acme-orders.json`, the selected document whose digest every published selection carries, has no `Operation.redefines`. It declares all three redefinitions only in the `ix://acme/orders/extension/redefines-operation` extension of `Sub` and `SubSub` (`required` false). That is exactly MA-17's input, which the new FR-154 paragraph says "redefines nothing". Under this PR, then, MT-08, MT-09 and MT-10 over the published document must refuse `ambiguous-name` but record "Resolves `Sub/size`" and the others. The `domain_package` projection that TC-280 says "FR-154 intake reads, derived from the document (a test derives it and compares)" also records redefinitions that FR-154 now says the document does not declare. tests/model_member_types.rs `project_document` derives them from `extensions[].payload.redefines`. The fixture move and its re-key belong in this PR. Otherwise the Preconditions and MT-08 to MT-10 must not claim the new carrier. | spec/test-cases/TC-280-model-owned-member-types.md:43-45,130-132; spec/functional/type-model/FR-154-admit-domain-package-model.md:155-158; proposals/checked-package-v2/domain-package-acme-orders.json:1198,1319; tests/model_member_types.rs:1384-1388 |
| FND-002 | high | Major. `Operation.redefines` is not a member of the contract the document is written in. FR-154's intake chain (step 3) says the FCD extraction frontend lowers declarations into "one semantic IR 2.0.0 domain package". At filament-core-data 0de623c, `$defs.operation` in `schema/semantic/v1/semantic-ir.schema.json` has no `redefines` property and `additionalProperties: false`, and `$defs.field` does carry `redefines`. A conformant producer therefore cannot emit the admitted half of FR-154-AC-16 or the operation-redefinition resolution of FR-322-AC-30, and a document that carries `Operation.redefines` is not valid semantic IR. FR-154 records no dependency on an FCD change, and the PR body says no FCD ticket exists. The lead default leaves AC-16's positive case and MT-08 unbuildable by any conformant producer until the FCD change is filed and lands, and the spec should state that dependency. | spec/functional/type-model/FR-154-admit-domain-package-model.md:50-52,151-154,287 |
| FND-003 | high | Major (confidence medium). FR-154 says an extension payload declares no redefinition "whatever its identity or its `required` flag", and MA-17 expects `ambiguous_declaration`/`ambiguous-name` for the `required: true` variant. FCD FR-068 makes a `required: true` extension that the consumer policy does not admit an admissibility refusal (`UNKNOWN_REQUIRED_EXTENSION`). QSpec FR-133-AC-3 states the same principle for language extensions: "A consumer that lacks a required extension refuses it". A reader that does not implement `redefines-operation` should therefore refuse the `required: true` document at intake, not admit it and resolve ambiguously. Either MA-17's `required: true` case should expect that intake refusal, or FR-154 should state why domain-package intake ignores the `required` flag. | spec/functional/type-model/FR-154-admit-domain-package-model.md:155-157; spec/test-cases/TC-280-model-owned-member-types.md:150; spec/functional/source-package/FR-133-extend-language-uniformly.md:49 |
| FND-004 | low | Minor. The new rows MA-16 to MA-18 sit between MA-09 and MA-10 in TC-280's vector table, out of id order. | spec/test-cases/TC-280-model-owned-member-types.md:149-151 |

## Verdict

Not mergeable. Option (a) is the right default and matches model-complete.md, FR-151 and QSL intake. As written, though, the PR makes TC-280 contradict the published document it is evaluated over (FND-001). It relies on a member that the FCD semantic IR contract forbids, with no recorded dependency (FND-002). Its `required: true` extension case conflicts with FCD admissibility (FND-003). FND-004 is ordering hygiene.

## Resolution

FND-001: `domain-package-acme-orders.json` now declares the `Sub/size`, `Sub/scaled` and `SubSub/size` redefinitions as `Operation.redefines` and no extension payload. The document digest, the package ids that bind it and the dependency-selection vectors are re-keyed, and `project_document` reads `Operation.redefines`. FND-002: FR-154 Dependencies records that the filament-core-data Semantic IR contract owns the `operation` member set and has to carry `redefines`. FND-003: FR-154 and FR-154-AC-16 state that a consumer that does not admit a `required` extension refuses the document before intake (FR-133-AC-3), and MA-17 expects the ambiguous result for `required` true when the consumer admits the extension. FND-004: MA-16 to MA-18 follow MA-15 in the TC-280 table.
