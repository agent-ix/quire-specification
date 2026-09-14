---
id: TC-155
title: "Separate structural target observation from preservation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-122
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-123
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-124
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-125
    type: verifies
  - target: ix://agent-ix/quire-specification/NFR-061
    type: verifies
---
## Description

Accepted FS06 portable determinism, source-authority and observer controls.
TM-008 remains Planned.

## Test Procedure

Repeat equal mapping requests across path, time, locale and optional-observer
changes. Attach parser acceptance, parser rejection, missing tool, changed tool/
library revision and ambiguous-rights observations. Attempt to feed generated
text as native source, use FRET formalization as TL correspondence, or promote a
conditional/unrepresented/refused record after target acceptance.

## Expected Results

Equal admitted mapping inputs yield byte-identical packages. Observer facts are
separate, downstream and non-semantic. Missing/ambiguous observers refuse only
the observation. Generated text never becomes native source, truth, preservation
or cross-target input.
