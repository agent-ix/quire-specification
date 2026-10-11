---
id: TC-937
title: "ContractPackage reader admits only a complete canonical value"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-9
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-4
    type: verifies
---
# TC-937: ContractPackage reader admits only a complete canonical value

## Description

Verify that the single ContractPackage reader enforces the closed wire
contract before returning a typed value to any consumer.

## Test Procedure

Read one independently constructed valid canonical ContractPackage. Repeat
with one mutation per case: wrong FR-330 contract version; a checked-package
version substituted in the outer tag; missing, duplicate or unknown member;
wrong member kind; unresolved or incompatible graph reference;
missing source-map entry; accounting/request mismatch; an `identity.digest`
that differs from the recomputed projection; wrong
identity domain; and a semantically equivalent but noncanonical wire encoding
with reordered members or insignificant whitespace. For the last case, preserve
the original digest so the only defect is noncanonical bytes. For unresolved
or incompatible graph references, recompute identity so the reader reaches
graph-reference validation after identity. Also combine a
wrong outer version with a duplicate nested member. For every refusal,
except for duplicate-member and noncanonical-wire cases, submit each valid-JSON
mutation as canonical RFC 8785 bytes before reading; submit those two cases as
raw bytes;
capture the typed result before invoking a consumer. For each nested defensive
payload, repeat with a missing required member, unknown member, wrong member
kind, and `null` in place of an omitted optional member; do not recompute
identity for these shape mutations because nested schema is validated first.
Separately submit a raw nested object with a duplicate member name;
do not canonicalize or recompute identity for that input. For each `limit` and
`consumed` field in `failed` and `body_incomplete`, test `"08"`, `"+1"`,
`"-1"`, `""`, `"١"`, and
`"18446744073709551616"`; also replace a counter string with the JSON number
`8`. Construct two valid `failed` controls: one with `limit` and `consumed`
both `"0"` at a denied zero-limit charge, and one with both
`"18446744073709551615"`. These are reader wire-acceptance controls, not claims
that the values are produced by a particular measured charge. Recompute each
identity from the string-encoded projection.
Do not recompute identity for malformed counter values; the reader must reject
them before identity validation. Also test a valid
`body_incomplete` with `limit_kind: "bytes"` and no `path`, then mutate it to
add `path` and to add `path: null`; test a non-byte kind with `path`, then
remove that member. For
`invalid_body.refusal`, test a missing
`contract_version` for `unknown_contract_version` and a present
`contract_version` (including `null`) for a different `code`. Also give
`refusal.code` a wrong JSON kind and an unknown string while
`contract_version` is present, and give `incomplete.limit_kind` a wrong JSON
kind and an unknown string while `path` is present.
Also admit separate valid typed refusals: one `invalid_package`/`invalid-value`
with `path` equal to the empty string (the root pointer) and
`document_pointer` absent, and one `stale_dependency`/`content-mismatch` with
`document_pointer` equal to the empty string and `path` set to
`/lock/model_selections/0/digest`. Admit a valid non-byte incomplete value
whose typed `path` is the empty string; this is distinct from TC-933's producer
check for omitted typed pointers.
For cause validation, test an unknown cause string, a known FR-272 cause paired
with a code that FR-322 does not admit it for, and a wrong-kind cause value.
Also construct a valid
ContractPackage whose source lock has one `dependency_selections` entry and
whose graph has a `dependency_reference` to that selected package. Supply the
matching admitted dependency package keyed by its library identity, then repeat
with that input omitted, with an admitted package supplied under the selected
key but carrying a different library identity, and with an admitted package whose
identity matches but whose `package_id` differs from the selection. Add a
second, unselected admitted dependency input that contains a node matching the
reference; it must not satisfy or redirect the reference.

## Expected Results

The valid value returns the existing typed `admitted` envelope. Each mutated
value returns the FR-330 typed `refused` result using FR-322's generic
wire/canonical checks and FR-330's own contract-version rule, with the
applicable FR-322 code/cause mapping and FR-272 diagnostic identifying its
stage and available loci. Malformed nested payloads are refused as
`invalid_package` with the applicable `missing-member`, `unknown-member`,
`wrong-value-kind`, or `invalid-value` cause and the ContractPackage pointer.
A duplicate nested JSON member, including with a wrong outer version, is
refused as `invalid_package`/`duplicate-member` during strict JSON parsing,
before version checking, canonical-byte comparison, or identity validation.
Noncanonical wire bytes are refused as `noncanonical_wire`
at `package_read`, with no cause or pointer, before identity recomputation.
Counter strings with leading zeros, a sign, empty content, or a value above
`u64::MAX` are refused as `invalid_package`/`invalid-value`; a JSON number in
place of a required decimal string is refused as
`invalid_package`/`wrong-value-kind`; non-ASCII digits are also
`invalid_package`/`invalid-value`. A bytes incomplete record with `path` is
`invalid_package`/`unknown-member`, even when its value is `null`, while a non-byte record without `path` is
`invalid_package`/`missing-member`. `unknown_contract_version` without its
required member is `invalid_package`/`missing-member`; a different refusal
code with `contract_version` is `invalid_package`/`unknown-member`, even when
its value is `null`. A wrong-kind discriminator is
`invalid_package`/`wrong-value-kind`, and an unknown-string discriminator is
`invalid_package`/`invalid-value` at the discriminator before conditional
members are checked. An unknown cause string and a known but unpaired
`(code, cause)` combination are refused as `invalid_package`/`invalid-value`
at the cause pointer; a wrong-kind cause is `invalid_package`/`wrong-value-kind`
at that pointer. Empty-string root pointers are admitted as present pointer
values when the identity matches; omission of an optional pointer is checked
against its typed source only at emission, as TC-933 specifies. Both valid
`failed` controls (`0`/`0` and `u64::MAX`/`u64::MAX`) are admitted.
The wrong identity domain is refused as
`invalid_package`/`invalid-value` at `/identity/domain`, using FR-322's
existing typed pair; `digest_domain_mismatch` remains for a wrong domain on an
input `PackageId`.
The declared identity mismatch refuses `invalid_package`/`invalid-value` at
`/identity/digest`; this tests the ContractPackage's own recomputed identity.
It is distinct from the selected-dependency case below, whose different
`package_id` is `stale_dependency`/`content-mismatch` under FR-322's
dependency-selection rule.
The correctly supplied dependency admits the ContractPackage, and its
reference resolves only against that package. Omitting the selected dependency
or supplying an admitted value under the selected key with a different library
identity refuses
`missing_import`/`missing-selection` at the selected lock pointer before the
reference is checked. Supplying an admitted package under the selected key with
the selected identity but a different `package_id` refuses
`stale_dependency`/`content-mismatch` at that pointer; the FR-272 diagnostic
names the selected `package_id` as `expected` and the supplied `package_id` as
`actual`. The extra unselected input is ignored. No consumer
receives or reparses a refused value. No alternate
reader accepts a value refused by the owner.
