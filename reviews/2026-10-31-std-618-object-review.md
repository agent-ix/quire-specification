---
id: SR-2026103013
title: "Object review — STD-618 FR-322 selected operation witness"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-322 operation member and NodeRef identity, AC-25/30/290, SelectedOperationWitness schema"
review_set: all
---

## Summary

The object review is recorded under the valid `base` analysis value because the installed SpecReview schema does not include `object-review`; the separate Linear marker below preserves the method identity.

## Object review

The operation member is correctly represented as a declaring type reference plus original member name, respecting the domain rule that it has no graph node. The domain object's declaring-type admissibility is contradictory across AC-25, AC-30 and AC-290.

## Verdict

**FAIL** — the object identity contract does not name one consistent set of declaring node forms.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-290 admits relation declaring nodes that model-member rules refuse | FR-322-AC-290 |
