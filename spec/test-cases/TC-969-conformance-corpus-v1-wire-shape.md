---
id: TC-969
title: "The conformance corpus v1 schema admits the fixture and refuses out-of-domain shapes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-336
    type: verifies
---

# TC-969: The conformance corpus v1 schema admits the fixture and refuses out-of-domain shapes

## Description

Verify the published structural schema admits its fixture and rejects missing,
unknown and out-of-domain wire values before consumption.

Scope: FR-336-AC-4.

## Test Procedure

Load `proposals/conformance-corpus-v1/schema.json` and
`proposals/conformance-corpus-v1/fixtures/complete-v1.json`. Validate the
schema as Draft 2020-12, compile it, and validate the fixture. Then independently
mutate the fixture by removing a required vector member, adding an unknown
member, and replacing the vector class, subject consumer type and expected
disposition with values outside their vocabularies. Validate each mutated
document against the same schema.

Tag the tests `#[trace("TC-969", "FR-336-AC-4")]`.

## Expected Results

The schema is valid, the fixture is admitted, and every mutated document is
refused by the schema.
