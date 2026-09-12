---
id: TC-042
title: "Select pre-state reads without retagging captured values"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-042
    type: verifies
---
## Description

Planned controls for [FR-042](../functional/foundation/FR-042-select-pre-state-reads.md).
No executable test or observed coverage is claimed.

## Test Procedure

Bind an explicit synthetic operation whose same object has version 1 in pre and 2 in post, with immutable delta and a valid pure predicate P. Exercise each composite/nested/alias example. Independently attempt parameter/result-only selection, the wrong clause anchor, cross-observation presence justification and retagging of a captured reference.

Include the positive alias control `let s = pre(self) in s.version`, which
captures the context inside the selector and retains its pre qualification.

## Expected Results

Successful cases use the specified observation and original source binding. Forbidden cases refuse before lowering with no Boolean result. Equal scalar/object labels never conceal a change in provenance; historical 0-draft pre(parameter) behavior is not used as replacement evidence.
