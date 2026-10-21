---
id: SR-13641
title: "QSpec PR #310 (STD-107) FR-322 dependency_reference signature, loci, supply order"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@d54caee35ed62feafe6019ef048ec50ea249132c vs main d080afa; spec/objects/interfaces/FR-322-checked-package-artifact.md, spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md, proposals/checked-package-v2/dependency-selection-vectors.json, proposals/checked-package-v2/README.md; context quire-contract-ir 38e9e34 dependency_references.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-233
    type: reviews
---

## Summary

Ticket: STD-107 (PR #310, head d54caee35ed62feafe6019ef048ec50ea249132c). Compared with the IR-289 reader (quire-contract-ir 38e9e34, `dependency_references.rs`): the step-7 loci (`/package`, `/node`, the term) and the per-entry supply order (presence then binding, first failing entry in lock order) match what the IR lane implemented. The signature definition does not: the IR reader starts its closure from the function's own `dependencies` list and every body `reference` target, which the PR explicitly excludes. The vectors change only adds a `schema_only` key; no digest or vector outcome changes, so nothing needed recomputing. `cargo test --test checked_package_v2 --test backend_provider_v1` passes (11 + 112).

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| FR-322-AC-84 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 supply order | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 signature type nodes | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| FR-322 step-7 loci | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined |
| README schema_only | proposals/checked-package-v2/README.md | examined |
| dependency-selection-vectors.json schema_only | proposals/checked-package-v2/dependency-selection-vectors.json | examined |
| TC-233 procedure | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |
| IR-289 reader | quire-contract-ir:crates/quire-contract-model/src/checked_package/v2/dependency_references.rs | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The signature definition contradicts the IR-289 reader, which is the implementation the PR says it follows. IR (dependency_references.rs:495-499) seeds the closure with the function's `dependencies` list and all body `reference` targets, so a dependency function whose body references a declared record is refused `ill_typed`/`operator-ineligible`; AC-84 requires that same function to be eligible. The PR picks the narrower, less conservative reading (more functions admitted) and calls it a judgment call, not a planner default, and names no IR follow-up ticket. Either flag it as a planner default with an IR ticket to narrow the reader, or keep IR's broader set | spec/objects/interfaces/FR-322-checked-package-artifact.md:519-523 |
| FND-002 | low | "`operator-ineligible` at the term itself (or, for a term standing outside a callee position, at that term)" says the same thing twice; drop the parenthetical | spec/objects/interfaces/FR-322-checked-package-artifact.md:539-544 |

## Verdict

Not mergeable: FND-001 (medium) plus the FR-322-AC-84 id collision with PR #311 (SR-13643 FND-001).

## Dispositions

Disposition pass 1, against head 55916ae23ed2cc7f018a482fe2d51ccd946ecc09.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | accepted-no-change | Narrower signature reading kept deliberately; FR-322-AC-86 now states it, and the PR body records that IR-289 over-refuses a function the spec admits and needs a fix (IR lane notified by the coordinator). No IR ticket id is named yet; the IR reader stays out of line with FR-322-AC-86 until that fix lands. |
| FND-002 | fixed | verified at 55916ae23ed2 |
