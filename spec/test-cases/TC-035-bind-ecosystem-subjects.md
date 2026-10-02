---
id: TC-035
title: "Retain explicit ecosystem scope identities in assessment requests"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-035
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-287
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-288
    type: verifies
---
## Description

Planned acceptance controls for [FR-035](../functional/foundation/FR-035-bind-ecosystem-subjects.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Bind O1 and O2 with distinct workflow identities but a shared external provider. Link O1's payment effect to a refund and two shipments. Substitute a delivery or trace identity for the effect. Bind a declared population window, then change its identity and separately make required membership unavailable.

For the same linked subject, map two distinct compatible clock roles explicitly
to one clock binding. Independently omit a role, repeat its mapping, substitute
another type with identical fields, and supply O2's progress for O1's window.
Each adverse control must identify the affected role and dependent request;
the original shared-clock mapping must be accepted.

Change only the runtime resource ceiling and compare the retained exact
configuration, assessment inputs and static components. Then change the runtime
configuration's selected model, temporal boundary profile and binding-contract
meaning independently; each conflicts with the original linked requirements.

Bind a valid compensation template before activation, without a refund record.
Observe no missing-recovery-input failure for that unactivated obligation. Then
activate it and request an evaluation that requires the recovery observation;
withhold that observation. Distinguish this incomplete evidence from a complete
observed model value that legitimately has an optional member absent.

Use the template linked without runtime observations in TC-030. First supply
all required runtime scope selections and their valid declaration keys.
Then independently remove a required scope selection, withhold a required record
under the otherwise valid mapping, and substitute a declaration key from a
different domain package with the same display name. The first and third mutants
refuse binding; the second remains incomplete evidence. Prior static success
cannot discharge any missing runtime prerequisite or authorize an override.

For one concrete runtime subject, mutate the authority identity, subject kind,
subject declaration key and local object identity bytes one at a time. Remove
and empty each member, reuse the unchanged complete key with incompatible
binding relationships, permute a finite subject collection, and change only
display, locus, payload, trace, timestamp, arrival position and transport
status.

For one linked protocol definition, mutate the authored source-document key,
admitted declaration handle, formal requirement/clause and each semantic
dependency selection one at a time. Offer only its declaration name, source
span or package-local table index without the admitted package qualifiers.
Then change only external rendering metadata/path/build provenance, runtime
observations and resource-only controls while preserving the source-document
key, and lexicographically sort a finite definition set. Separately edit a
display name inside the authored source and retain the resulting changed raw-
byte source digest.

## Expected Results

Declared relationships and workflow identities survive binding. Substitutions refuse, scope changes remain visible in request identity, and unavailable membership cannot produce aggregate success. Subject kinds and relationships come from the checked package's domain declarations and observation representations from F's reviewed contract, not a duplicated test model.

Compatible explicit role sharing is admitted; missing, duplicate and mismatched
role mappings refuse. Runtime configuration changes remain identified and cannot
override linked meaning. Missing future recovery records do not manufacture an
activated obligation, while evidence required by an activated assessment remains
incomplete. Assertions inspect typed causes, references and obligation identities.
Every key-member mutation changes the canonical key; a semantic comparison
across authority or kind refuses. Missing/empty members and incompatible
rebinding refuse without a default; presentation/transport-only
changes preserve identity; and collection permutation preserves canonical
lexicographic order without adding causality.
Every protocol-definition key-member mutation changes its identity. An
unqualified declaration offer refuses, external presentation/runtime-only
changes that preserve the source key preserve the definition key, an authored
source edit changes it, and sorting establishes no control, delivery, retry or
temporal order.
