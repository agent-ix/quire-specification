---
id: SR-13678
title: 'Spec review of QSpec PR #321: population owner join keyed by identity, base'
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@8602df3; git diff origin/main...8602df3 (PR #321, branch qspec-population-identity): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-280-model-owned-member-types.md, proposals/checked-package-v2/model-owner-form-vectors.json, tests/model_member_types.rs; ticket QSL-869"
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/TC-280
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
- target: ix://agent-ix/quire-specification/FR-154
  type: reviews
---

## Summary

Ticket: QSL-869 (related STD-217, STD-165). PR: quire-specification#321 (private), head 8602df3f72d07c72d6366aaa4aad675d6d4a38b6, branch qspec-population-identity, one commit, 4 files, +106/-13. The PR restates the FR-322 'Population references' owner join. The selected document's `populations` entries are raw FCD population records keyed by `identity`, the join compares `owner.node` with that `identity`, and the node keys do not change. It amends FR-322-AC-82, adds TC-280 PR-07 and puts the real shape in the owner-form vectors. The direction is right and unblocks both sides. Cross-repo check: FCD's real population shape (filament-core-data crates/semantic-ir/src/schema.rs:818-830, POPULATION_MEMBERS) is exactly `identity, displayName, kind, members, extent, origin`, required, plus optional `symmetric`, with extra members forbidden. `kind` is `{module, name}` and `origin` is `{source: {sourceIdentity, path, startLine, startColumn}}` or `{generated}`. The vector entries match this exactly. QSL emits that shape (quire-spec-language examples/config-version/model.semantic-ir.json; intake read_population, qsl-semantics/src/model/intake.rs:3906). IR-937 (quire-contract-ir 2140725, crates/quire-contract-model/src/checked_package/v2/model_members.rs:2233-2238) reads `text(population, "node")?` and refuses malformed when it is absent. Reproduced: QSL main with Cargo.lock at IR-937 fails 132 of 585 qsl-analyze `it` tests with InvalidModelBinding/MalformedDeclaration at /lock/model_selections/0. With that one IR line changed to read `identity`, all 585 pass. So the amended FR-322 matches what QSL emits, and IR can conform with a one-line reader change. IR's own tests then need updating: 5 checked_package_v2_owners tests fail because they inject `{node}` populations (tests/it/checked_package_v2_owners.rs:141, :623). Node keys: P7 and P8 are unchanged and recompute from their preimages, whose `owner.node` equals each entry's `identity`. Sweep: no other QSpec spec text says a domain-document population carries `node`. FR-154, FR-208, FR-451 (population entries target the reference node) and TC-198/TC-234 speak of a population's IR node identity only. FR-341 and FR-343 do not mention populations. The positive-operation-identities fixture's operand 0 is the `census` `value`/`parameter` binding typed by `Population<Order>[2]`, which reads no document population, so it still reads correctly. The full-reader harness and its published cases are not consistent with the amended spec (SR-13677 FND-001, FND-001 here). Open PRs: none of #267, #207, #200 or #59 touches FR-322, TC-280, the owner-form vectors or the tests, and FR-322-AC-82 and PR-07 are not used elsewhere. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `cargo test --test model_member_types --test checked_package_v2 --test backend_provider_v1 --test checked_package_model` passes all: model_member_types 24, checked_package_v2 114, backend_provider_v1 14, checked_package_model 4. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, identical to main apart from one FR-322 warning shifting 9 lines; the touched files add none. quoin 0.28.3, quire 0.36.2 (engine 0.50.2).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 Population references: Owner join | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-280 PR-07 | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| TC-280 owner-form projection description | spec/test-cases/TC-280-model-owned-member-types.md | examined |
| FR-322 Population references: Identity | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| TC-233 population_reference_cases | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| README checked-package-v2 population bullet | proposals/checked-package-v2/README.md | examined |
| FR-154 identity rule | spec/functional/type-model/FR-154-admit-domain-package-model.md | context_only |
| FR-451 Keys | spec/objects/interfaces/FR-451-checked-package-abstraction-relation-body.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The full-reader admitted cases depend on a selected document that declares `counters` and `archive`, but no published artifact contains that document. domain-package-acme-orders.json declares no populations, adverse.json:79-81 carries only `owner_node`, and TC-233 says only 'a population of the selected document'. Each consumer has to build the population entries itself. QSpec's harness injects `{node}` (tests/checked_package_v2.rs:13027) and IR copied it (quire-contract-ir checked_package_v2_owners.rs:623). That is how IR-937 shipped a reader no real document satisfies, and this PR does not close it: the corrected shape is published only in the owner-form projection, which the full reader does not read. Fix: publish the record shape for these cases, either as `populations` in domain-package-acme-orders.json (re-pinning the selection digests that depend on it) or as a document patch in adverse.json, and have TC-233 name it, so that a reader joining on `node` fails a published case. | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md:290-297 |
| FND-002 | low | The README still lists the owner cases as `PR-01` to `PR-06`; the vectors now hold PR-07. Fix: 'the owner cases `PR-01` to `PR-07`'. | proposals/checked-package-v2/README.md:141-144 |

## Verdict

Not mergeable as is. The core change is correct, and QSL and a one-line IR fix both conform to it (reproduced 132 -> 0 qsl-analyze failures). Before merging: fix the full-reader harness and publish the population records its cases read (SR-13677 FND-001, FND-001 here), and resolve the PR-07 / FR-154 conflict (SR-13681 FND-001). The other findings are wording fixes.
