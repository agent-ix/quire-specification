---
id: TC-032
title: "Refuse unsupported required features"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-032
    type: verifies
---
## Description

Portable controls for [FR-032](../functional/foundation/FR-032-preserve-language-evolution.md).

## Test Procedure

Present a reader with sources that each select one required feature outside
its supported set: a new operator, a new declaration form, a new profile and a
new semantic family. Then present a source whose required features the reader
supports only in part: knowing the new artifact bytes, parsing its source or
supporting its core subset does not establish full support. Assert the actual
requested feature scope in each refusal. Then present a source whose optional
metadata the selected wire contract permits the reader to ignore.

## Expected Results

Each unsupported required feature refuses with a located unsupported-selection
refusal naming the feature; no reader drops the feature or evaluates a smaller
subset as the whole subject. Permitted optional metadata is ignored and the
subject is interpreted under its selected definitions.
