---
id: SR-020
title: "Failure-domain review of the native-source ruling amendment"
type: SpecReview
analysis: failure-domain
scope: "Native-source-1-draft and FR-019 at bd6f5ea"
review_set: all
---
## Summary

Checked extension failure, identity, purity and topology against the carried-
forward admission rules. No additional unstated failure contract was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings | - |

## Checks

Unknown/deferred constructs refuse without dead-code erasure or profile fallback.
Exact source and definition identities prevent reusing old meaning; duplicate
authorities and mismatched anchors refuse before lowering. Pure native control
does not read application state. Recursive containment, scalar lookups and
closure remain refused; existing runtime exhaustion cannot become false. Source
presence is not inferred from option/cardinality, and unsupported distinctions
refuse at their declaration. SR-019 retains the known definition/reference gates;
this lens does not claim those gates are resolved by defensive refusal.
