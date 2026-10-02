---
id: SR-861
title: "EARS review of PR #176: lifecycle, depth, witness value path and plugin wire requirements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-specification@fac70325054b02f9fdbed031781b0be9132fad89; PR #176 new and edited requirement statements: FR-146, FR-300, FR-301, FR-305, FR-306, FR-352, FR-354, FR-460, FR-461, FR-462, NFR-073"
review_set: subset
---
# EARS review of PR #176

## Summary

Ticket: STD-143, with STD-141 and STD-144. This pass checks each new or
edited requirement statement for EARS form, one subject and one requirement
per statement. FR-300, FR-305, FR-352, FR-354, FR-460, FR-461 and FR-462 are
event-driven EARS statements with a named subject. NFR-073 is a ubiquitous
statement with a subject. `quire validate` raised no EARS warning on these
files. FR-306 combines two requirements in one statement.

## Verdict

PASS with one low finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-306's statement holds two requirements with two triggers, execution parity and the cache key, joined by "; and when" | spec/functional/tooling/FR-306-compile-and-cache-exactly.md:26-30 |

## Applied review

Followed /reviewer with /quoin:spec-review and the spec-ears-analysis
sub-analysis over the PR's diff only.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 646c033 |
