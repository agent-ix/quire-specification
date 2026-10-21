---
id: SR-13679
title: 'EARS review of QSpec PR #321: FR-322 owner join population identity'
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@8602df3; git diff origin/main...8602df3 (PR #321, branch qspec-population-identity): spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-280-model-owned-member-types.md, proposals/checked-package-v2/model-owner-form-vectors.json, tests/model_member_types.rs; ticket QSL-869"
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
---

## Summary

Ticket: QSL-869. EARS conformance of the requirement text changed by PR #321: the FR-322 'Population references' owner-join bullet and FR-322-AC-82. The rest of the bullet list (Node, Declaration key, Identity, Use) uses the same indicative style as before; this pass judges only the changed obligations. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `cargo test --test model_member_types --test checked_package_v2 --test backend_provider_v1 --test checked_package_model` passes all: model_member_types 24, checked_package_v2 114, backend_provider_v1 14, checked_package_model 4. `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, identical to main apart from one FR-322 warning shifting 9 lines; the touched files add none. quoin 0.28.3, quire 0.36.2 (engine 0.50.2).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322 Population references: Owner join | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322-AC-82 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new obligations are stated in the indicative, with no SHALL: 'The reader builds the population's node from it', 'the node's `owner.node` is compared with the entry's `identity`', 'An entry that carries a `node` member and no `identity` declares no population'. FR-322 uses SHALL elsewhere (for example :1212, 'The `population` node SHALL resolve...'). Fix: one SHALL statement per obligation, e.g. 'The owner join SHALL compare `owner.node` with the `identity` of each `populations` entry of the selected document and SHALL NOT read a `node` member of a population entry.' | spec/objects/interfaces/FR-322-checked-package-artifact.md:1274-1285 |
| FND-002 | low | Phrases written as corrections of the former text: 'unchanged', 'carries no `node` member', 'so no `node` member is read from the domain document'. A reader who never saw the `{node}` wording has no reason to expect a `node` member. TC-280:68 ('and carry no `node`') does the same. Fix: state the positive rule only ('each entry is an FCD population record, keyed by `identity`') and drop the 'no `node`' contrasts. | spec/objects/interfaces/FR-322-checked-package-artifact.md:1276-1284 |

## Verdict

Mergeable with respect to EARS once the two low findings are fixed; neither blocks on its own.
