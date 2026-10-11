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
missing source-map entry; accounting/request mismatch; stale identity; wrong
identity domain; and a semantically equivalent but noncanonical wire encoding
with reordered members or insignificant whitespace. For the last case, preserve
the original digest so the only defect is noncanonical bytes. For every refusal,
capture the typed result before invoking a consumer. For each nested defensive
payload, repeat with a missing required member, duplicate member, unknown
member, wrong member kind, and `null` in place of an omitted optional member; recompute identity for
each shape mutation so the reader reaches nested-schema validation. Also construct a valid
ContractPackage whose source lock has one `dependency_selections` entry and
whose graph has a `dependency_reference` to that selected package. Supply the
matching admitted dependency package keyed by its library identity, then repeat
with that input omitted, with a supplied admitted package whose library
identity differs from the selection, and with an admitted package whose
identity matches but whose `package_id` differs from the selection. Add a
second, unselected admitted dependency input that contains a node matching the
reference; it must not satisfy or redirect the reference.

## Expected Results

The valid value returns the existing typed `admitted` envelope. Each mutated
value returns the FR-330 typed `refused` result using FR-322's generic
wire/canonical checks and FR-330's own contract-version rule, with the
applicable FR-322 code/cause mapping and FR-272 diagnostic identifying its
stage and available loci. Malformed nested payloads are refused as
`invalid_package` with the applicable `missing-member`, `duplicate-member`,
`unknown-member`, `wrong-value-kind`, or `invalid-value` cause and the
ContractPackage pointer.
The wrong identity domain is refused as
`invalid_package`/`invalid-value` at `/identity/domain`, using FR-322's
existing typed pair; `digest_domain_mismatch` remains for a wrong domain on an
input `PackageId`. Noncanonical wire bytes are refused as `noncanonical_wire`
at `package_read`, with no cause or pointer, before identity recomputation.
The correctly supplied dependency admits the ContractPackage, and its
reference resolves only against that package. Omitting the selected dependency
or supplying a value with a different library identity refuses
`missing_import`/`missing-selection` at the selected lock pointer before the
reference is checked. Supplying an admitted package with the selected identity
but a different `package_id` refuses `stale_dependency`/`content-mismatch` at
that pointer and retains the selected and supplied package identities. The
extra unselected input is ignored. No consumer receives or reparses a refused value. No alternate
reader accepts a value refused by the owner.
