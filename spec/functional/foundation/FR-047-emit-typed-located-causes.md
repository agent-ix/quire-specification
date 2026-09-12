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

When reporting a native producer failure, the producer SHALL derive its machine-readable classification from a typed cause in its versioned diagnostic catalog rather than diagnostic message text.

## Inputs

A failure at source recognition, identity/linking, type/profile checking, input binding, capability selection or resource-limited execution, with its available source/subject context.

## Outputs

A stable catalogued classification, failure stage and structured cause details with exact available loci; human-readable wording is presentation.

## Behavior

Every emitted code belongs to the selected catalog; no ad-hoc literal or English-message match determines its kind. Cause details distinguish materially different refusals sharing a broader category, including missing versus conflicting definitions, wrong anchor versus missing observation, unknown syntax versus prohibited recognized construct, and unproved definedness versus runtime logical violation. Diagnostic formatting/localization cannot change classification. Wire extension/version behavior follows the existing shared interchange contract; this requirement creates no competing result envelope.

A runtime object or value path is the location kind that names a value inside a
validated runtime subject, and it is distinct from an authored source span and
from a JSON field/index path. Such a path is a root together with an ordered,
possibly empty sequence of traversal steps.

The root is a selected observation identity together with a subject read under
that observation. Presence and value facts never cross observation identity, so
the observation is part of the root and not a step. The subject is exactly one
of a declared object identity, or a named parameter or result binding of one
declared invocation. Two subjects of different kinds are never equal.

Each step is exactly one of three kinds: a field selection, naming a declared
field of the current value by its exact declared spelling; an option-value
selection, naming the contained value of an option and carrying no name of its
own; or a member selection, naming a declared model member by its exact declared
spelling. A field selection and a member selection are never equal to one
another even when they carry the same name, and an option-value selection is
never a field of the option it traverses.

Two runtime object or value paths are equal when their roots are equal
componentwise, their step sequences are of equal length, and each corresponding
step pair agrees on step kind and on that kind's name where it carries one.
Rendered path text, display name, message wording, locale and reported source
span never participate.

Such paths are compared for collection comparison only, by root first and then
step by step. Root comparison is by observation key, then by subject kind
ordering a declared object identity before an invocation binding, then by
subject key. Step comparison orders a proper prefix before every path that
extends it, orders differing step kinds as field before option-value before
member, and orders steps of equal kind by their exact name. That comparison is a
total order over every set of paths whose roots share one observation identity,
which is the set a single diagnostic reports; it extends to a total order across
observations only where the selected observation identity itself supplies a key
order. It carries no causal, severity or precedence meaning.

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
| FR-047-AC-2 | Every emitted machine code resolves to the selected versioned catalog; an uncatalogued code cannot be published as a stable producer outcome. | Test (TC-047) |
| FR-047-AC-3 | Distinct anchor, observation, profile and definedness failures retain structured discriminating causes without requiring message parsing. | Test (TC-047) |
| FR-047-AC-4 | Original source/model/clause loci and identities remain attached where available; diagnostics do not reconstruct a location by searching for a repeated name. | Test (TC-047) |
| FR-047-AC-5 | Per-request resource/unsupported failures keep their actual stage and prerequisites rather than becoming another clause's Boolean result. | Test (TC-047) |
| FR-047-AC-6 | A runtime object or value path locating an unavailable nested value distinguishes a field selection, an option-value selection and a member selection; two paths differing only in step kind compare unequal, and an empty step sequence is not reported as an absent path. | Test (TC-047) |
| FR-047-AC-7 | Two paths with identical subjects and steps under different selected observations compare unequal, and paths sharing one observation are totally ordered. | Test (TC-047) |

## Dependencies

- [Detailed draft contract](../../../proposals/quire-v1/shared-foundation.md).
- [Shared grammar](../../../proposals/quire-v1/shared-grammar.md).
- [Planned acceptance matrix](../../composed-foundation/tests.md).
- [FR-207](../../objects/foundation/FR-207-runtime-value-path.md) types the runtime
  object or value path this requirement defines.

Proposed composed-v1 obligation. Historical definitions and their acceptance
remain separate; this artifact does not establish implementation coverage.
