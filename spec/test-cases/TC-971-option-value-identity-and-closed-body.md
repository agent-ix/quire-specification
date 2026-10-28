---
id: TC-971
title: "A present option is an identity-bound value with a closed payload reference"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-971: A present option is an identity-bound value with a closed payload reference

## Description

Verify the closed body, computed application-node identity and refusal shapes
for the V2 `value`/`option_value` node.

## Test Procedure

For each positive vector OV-01 and OV-02 in
`proposals/checked-package-v2/option-value-vectors.json`, validate its body
against `$defs.OptionValueBody`, compute the RFC 8785 JCS SHA-256 of its whole
`preimage`, and compare it with the vector's `sha256`. Confirm the keys differ
when the referenced payload identity differs. Change only `semantic_type` to a
second option type and confirm the computed key differs again.

Validate each OV-R1..OV-R4 refusal body against `$defs.OptionValueBody`:
empty payload, wrong binding name, an inline literal instead of a reference,
and multiple payload bindings. Each fails the schema. Confirm the V2 schema
selects `OptionValueBody` exactly for `node_tag: "value"` and
`semantic_form: "option_value"`; it does not add an operation identity or
accept this body for `union_value`.

Tag the tests `#[trace("TC-971", "FR-322-AC-244")]`.

## Expected Results

Both positive bodies validate and their vector digests recompute. Changing the
option type or payload reference changes identity. Each listed refusal body is
rejected by the closed option-value body schema, and present-option construction
has no operator identity.
