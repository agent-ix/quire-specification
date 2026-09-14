---
id: TC-142
title: "Declare population and completeness"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-112
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-263
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-264
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-265
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-266
    type: verifies
---
## Description

Planned OB01 population controls. No executable test or completed coverage is claimed.

## Test Procedure

Assess a finite snapshot and an event-time window under Producer interface 1.2.0;
change one identity; remove one required member; remove the external
payment-provider record; remove closure input; and independently mutate producer
version, membership digest, configuration digest, and closure digest.
Repeat with adjacent half-open windows: a member at the first window's `start`,
one exactly at its `end`, and the second window starting at that same instant.
Repeat the selected correspondence for an event-position sequence, a
fixed-sample clock with exact rational epoch/period/unit, and RFC 3339 timestamp
coverage. For each family, retain member-object identity separately from record
identity and position/sample/timestamp-anchor identity; attempt to substitute
one for another and to infer a UTC instant for an event position.
At each declared event-size and population/window-member limit, record the
admitted event size and retained population/window-member count; repeat one
byte or member over that exact limit. The one-over input must not be retained or
allocated beyond its declared bound merely because the final disposition is a
refusal.
Derive the explicit-members, snapshot and population identities independently
from their exact RFC 8785 preimages. Permute presentation order and then mutate
each semantic key component one at a time. Exercise empty, unknown, duplicate,
unsorted-on-wire and one-over-limit member sets; exercise snapshot-only,
window-only, neither and both selection forms; substitute a JCS digest where a
raw-artifact digest is required and vice versa.

## Expected Results

Scope identity changes the subject. Missing membership, provider evidence, and
closure remain explicit incomplete/refused reasons. Every producer-selection
mismatch is refused without an ambient default and cannot establish aggregate
success. The start member occurs once in the first window; the boundary member
occurs only in the second window and cannot be double-counted.
Each clock family preserves its own half-open start/end coverage. Event positions
do not gain timestamps; fixed samples retain exact rational values; and one
object with two observations remains two records. Identity substitution and
cross-clock inference refuse rather than manufacturing coverage.
Exact limits retain at most their declared event and population/window-member
state. One-over results retain no excess state and identify the affected subject
and exceeded bound as incomplete/refused.
Independent derivations agree byte-for-byte. Presentation permutation preserves
identity after canonical admission, while every semantic key mutation changes
the applicable identity. Unknown membership is not empty membership; duplicate,
unbounded, invalid selection-cardinality and foreign-domain inputs refuse before
a partial population or identity is exposed.
