---
id: TC-970
title: "The corpus replay adapter maps closed requests and byte provisions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-336
    type: verifies
---

# TC-970: The corpus replay adapter maps closed requests and byte provisions

## Description

Verify the corpus-owned replay adapter fixture against the published schema,
including FR-323 source and target forms, structural nested types, and the
relative byte-provision reference rule.

Scope: FR-336-AC-6, FR-336-AC-7, FR-336-AC-8.

## Test Procedure

Validate `proposals/conformance-corpus-v1/schema.json` as Draft 2020-12 and
validate `fixtures/replay-request.json` against `$defs.ReplayRequest`. Confirm
that the request has exactly the 15 `ReplayRequestWire` members and that each
nested object and tagged variant is closed. Mutate one request at a time to
add an unknown member to a structural object, replace a `WitnessValue` or
`DomainKey` tag with an unknown `kind`, replace an integer decimal with a JSON
number or a noncanonical decimal string, and use a stage-limit name outside
FR-255; each mutation is refused by the schema or the FR-255 consumer decoder
as specified. Confirm the `source` accepts the FR-323 Witness and Input arms,
the `replay_target.members` accepts exactly one FR-323 target member, and the
expected result retains the FR-323 `results` envelope with one FR-072 replay
result.

For `byte_provision`, construct replay rows for both a source document (four
QSL FR-001 identity labels) and a domain-package document (`identity` is
`{"domain_package": true}`), each with a listed `bytes` file and matching
relative `bytes_path`; verify that the adapter reads raw bytes and preserves
each row and selector at the same ordered position, without a digest key.
Then separately use an absent, unlisted, absolute and parent-traversing path
and verify refusal before `ReplayRequestWire` construction.

The QSpec schema test is tagged `#[trace("TC-970", "FR-336-AC-6")]` for
the closed provision-row shape. QSL consumer adapter tests are tagged
`TC-970` with FR-336-AC-7 and FR-336-AC-8 for request decoding and path
resolution.

## Expected Results

The published replay fixture validates, all documented source and target
forms map to their named FR-323 members, structural values reject unknown
fields and tags, and byte paths admit only listed sibling `bytes` files whose
raw bytes are paired with the row's source-label selector or domain-package
marker in the ordered provision. Provision rows have no digest or
digest-domain key.
