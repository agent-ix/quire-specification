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

Scope: FR-336-AC-4, FR-336-AC-5, FR-336-AC-6.

## Test Procedure

Load `proposals/conformance-corpus-v1/schema.json` and
`proposals/conformance-corpus-v1/fixtures/complete-v1.json`. Validate the
schema as Draft 2020-12, compile it, and validate the fixture. Then independently
mutate the fixture by removing a required vector member, adding an unknown
member, and replacing the vector class, subject consumer type and expected
disposition with values outside their vocabularies. Then set
`input.files[0].kind` to each member of FR-336's input-file-kind vocabulary
and validate each document; replace it once with `unknown` and validate again.
Compile the published `$defs.InputValue`, `$defs.CallInput` and
`$defs.ReplayProvisionPath` and validate
each tagged value, a call with all four value tags, and invalid tag, member,
decimal, identity, malformed accounting-decimal and replay digest mutations.
Verify that call arguments are a positional array of tagged values and that a
`{parameter, value}` argument object is refused by the schema.

Tag the tests `#[trace("TC-969", "FR-336-AC-4", "FR-336-AC-5", "FR-336-AC-6")]`.

## Expected Results

The schema is valid, the fixture and every listed input-file kind are admitted,
the four tagged input-value forms and positional call-file shape are admitted,
and invalid mutations, including a named `{parameter, value}` call argument,
are refused.
