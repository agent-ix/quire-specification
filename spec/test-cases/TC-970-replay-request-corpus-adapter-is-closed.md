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

Scope: FR-336-AC-7, FR-336-AC-8, FR-336-AC-12.

## Test Procedure

Validate `proposals/conformance-corpus-v1/schema.json` as Draft 2020-12 and
validate `fixtures/replay-request.json` against `$defs.ReplayRequest`. Confirm
that the request has exactly the 14 `ReplayRequestWire` members, `package` among them, and that each
nested object and tagged variant is closed. Remove `package`, and replace it
with an object that is not a checked package; each is refused. Confirm that every
node id the request names (`replay_target`, `obligation_identity`,
`declared_domains`, `source`) is the id of a node in the embedded package's
graph and that `dependencies` has one entry per dependency the package's lock
selects. Mutate one request at a time to
add an unknown member to a structural object, replace a `WitnessValue` or
`DomainKey` tag with an unknown `kind`, replace an integer decimal with a JSON
number or a noncanonical decimal string, and use a stage-limit name outside
FR-255; each mutation is refused by the schema or the FR-255 consumer decoder
as specified. Confirm the `source` accepts the FR-323 Witness and Input arms,
the `replay_target.members` accepts exactly one FR-323 target member, and the
expected result retains the FR-323 `results` envelope with one FR-072 replay
result.

For `obligation_identity.model`, confirm that the function replay fixture
carries `null`, matching the absence of an owning model. Validate that request
and a separate request with a non-empty owning-model string. Map the two
member values to `Option<String>` and confirm `None` and `Some` of the exact
original string, respectively; neither invents a package or function name as
the model. Independently remove `model`, replace it with an empty string,
Boolean, number, array or object, and add an unknown obligation-identity
member; each mutated request is refused by the closed schema. These checks
exercise the corpus adapter encoding and do not change FR-071's request wire.

For `byte_provision`, construct a replay vector with one listed `bytes` file
and a matching relative `bytes_path`; verify that the adapter reads the file's
raw bytes and maps them to that provision entry. Then separately use an absent,
unlisted, absolute and parent-traversing path and verify refusal before
`ReplayRequestWire` construction.

Tag the request-shape and provision tests with the applicable
`#[trace("TC-970", "FR-336-AC-7", "FR-336-AC-8")]` criteria, and the
owning-model checks with `#[trace("TC-970", "FR-336-AC-12")]`.

## Expected Results

The published replay fixture validates, all documented source and target
forms map to their named FR-323 members, structural values reject unknown
fields and tags, and byte paths admit only listed sibling `bytes` files whose
raw bytes supply the located provision.
The owning-model member preserves absence as `None` and a non-empty model
identity as `Some` of the exact supplied string; missing or malformed members
are refused rather than defaulted.
