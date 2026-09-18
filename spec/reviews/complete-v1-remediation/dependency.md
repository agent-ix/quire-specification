---
id: SR-601
title: "Dependency review of complete V1 against its neighbouring specifications"
type: SpecReview
analysis: dependency
scope: "cross-repository dependencies on quire-spec-language, quire-contract-ir, quire-contract-codegen and filament-core-data"
review_set: subset
evaluated_revision: "fc5434b on task/63-full-v1-specification"
review_date: "2026-09-17"
---

## Summary

Read the four neighbouring specification trees against complete V1. Twenty-six
load-bearing references were added as structured edges. Five conflicts cross a
repository boundary and are posed as ADR-001 through ADR-005 rather than
decided here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-621 | high | Open: declaration identity in a lifted domain package is keyed by artifact id here and minted from the slugged title on the producing side. Retitling, member keys and underscored ids all diverge. | ADR-001; FR-154 |
| FND-622 | high | Open: complete V1 requires a semantic IR 2.0.0 domain package and a member set that the producing tree neither emits nor defines, and the producing tree refuses that version. | ADR-002; FR-154; AD-006 |
| FND-623 | high | Open: model intake and formal type system authority are each assigned in both directions between complete V1 and quire-spec-language. | ADR-003; AD-010; FR-154; FR-195 |
| FND-624 | high | Open: the native CLI has two exit status contracts, while FR-300 requires API and CLI to agree on disposition. | ADR-004; FR-301; FR-300 |
| FND-625 | medium | Open: Kani harness generation is claimed by three specifications, with two harness surfaces and three tool pinning contracts. | ADR-005; FR-196 |
| FND-626 | medium | Resolved: the normative tree carried no reference to contract IR, codegen, FCD or QSL artifacts although the contract IR tree already linked back to it. Twenty-six verified references added across eighteen artifacts. | FR-195; FR-196; FR-197; FR-154; AD-006; FR-301; FR-339 |
| FND-627 | low | Open: subsystems/index.md names subsystem owners as bare repository names rather than artifact targets, so no owner row arbitrates a contested boundary. | spec/subsystems/index.md |
