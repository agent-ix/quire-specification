---
id: FR-047
title: "Classify producer diagnostics from typed causes"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-005
    type: implements
  - target: ix://agent-ix/quire-specification/FR-203
    type: references
  - target: ix://agent-ix/quire-specification/FR-270
    type: references
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-273
    type: references
  - target: ix://agent-ix/quire-specification/FR-274
    type: references
  - target: ix://agent-ix/quire-specification/FR-207
    type: references
---
## Description

When reporting a native producer failure, the producer SHALL derive its machine-readable classification from a typed cause in its diagnostic catalog rather than diagnostic message text.

## Inputs

A failure observed by one exact selected producer interface, carrying one member
of that interface's closed stage vocabulary and its available source/subject
context.

## Outputs

A stable catalogued classification, failure stage and structured cause details with exact available loci; human-readable wording is presentation.

## Behavior

Every emitted code belongs to the selected catalog; no ad-hoc literal or English-message match determines its kind. Cause details distinguish materially different refusals sharing a broader category, including missing versus conflicting definitions, wrong anchor versus missing observation, unknown syntax versus prohibited recognized construct, and unproved definedness versus runtime logical violation. Diagnostic formatting/localization cannot change classification. Wire extension/version behavior follows the existing shared interchange contract; this requirement creates no competing result envelope.

Every producer stage shall bind the exact producer-interface identity plus one
exact member of that interface's closed stage vocabulary.
When the interface or stage member is unknown, then the reader shall refuse the
diagnostic interpretation without substituting a stage from another interface or
from the six failure concerns this requirement covers.

A runtime object or value path is the location kind that names a value inside a
validated runtime subject, and it is distinct from an authored source span and
from a JSON field/index path. Such a path is a root together with an ordered,
possibly empty sequence of traversal steps.

The root is a selected observation identity together with a subject read under
that observation. Presence and value facts never cross observation identity, so
the observation is part of the root and not a step. FR-207 is the one source of
the subject kinds, the traversal step kinds, path equality and the path
comparison order, and this requirement reports paths under exactly those rules.
Rendered path text, display name, message wording, locale and reported source
span never participate in equality or order. The comparison is a total order
over every set of paths whose roots share one observation identity, which is
the set a single diagnostic reports. It carries no causal, severity or
precedence meaning.

An unavailable path is explicitly absent and compares only to absence; an empty
step sequence denotes the subject itself and is never an absent path, and a
name-search match is never a path. An absent observation or an absent subject
yields no path at all. This requirement fixes the structure, equality and
comparison of such a path and fixes no serialized spelling, discriminator tag or
separator for it; the existing shared interchange contract remains the only
authority on the wire.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-047-AC-1 | Changing/localizing human-readable messages leaves the same typed failure's stage, code and cause details unchanged. | Test (TC-047) |
| FR-047-AC-2 | Every emitted machine code resolves to the selected catalog; an uncatalogued code cannot be published as a stable producer outcome. | Test (TC-047) |
| FR-047-AC-3 | Distinct anchor, observation, profile and definedness failures retain structured discriminating causes without requiring message parsing. | Test (TC-047) |
| FR-047-AC-4 | Original source/model/clause loci and identities remain attached where available; diagnostics do not reconstruct a location by searching for a repeated name. | Test (TC-047) |
| FR-047-AC-5 | Per-request resource/unsupported failures keep their actual stage and prerequisites rather than becoming another clause's Boolean result. | Test (TC-047) |
| FR-047-AC-6 | A runtime object or value path locating an unavailable nested value distinguishes a field selection, an option-value selection and a member selection; two paths differing only in step kind compare unequal, and an empty step sequence is not reported as an absent path. | Test (TC-047) |
| FR-047-AC-7 | Two paths with identical subjects and steps under different selected observations compare unequal, and paths sharing one observation are totally ordered. | Test (TC-047) |
| FR-047-AC-8 | Every retained stage resolves in its selected producer interface; unknown interface/member pairs and labels copied from another interface refuse without replacing the originating stage. | Test (TC-047) |
| FR-047-AC-9 | A path rooted at the receiver binding `self` of an invocation is a valid invocation-binding subject, compares unequal to a path with equal steps rooted at a parameter binding of that invocation, and orders after it. | Test (TC-235) |

## Dependencies

- [Detailed draft contract](../../../proposals/quire-v1/shared-foundation.md).
- [Shared grammar](../../../proposals/quire-v1/shared-grammar.md).
- [FR-207](../../objects/foundation/FR-207-runtime-value-path.md) types the runtime
  object or value path this requirement defines.

Proposed composed-v1 obligation. Historical definitions and their acceptance
remain separate; this artifact does not establish implementation coverage.
