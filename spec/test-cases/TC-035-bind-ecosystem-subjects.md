---
id: TC-035
title: "Retain explicit ecosystem scope identities in assessment requests"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-035
    type: verifies
---
## Description

Planned acceptance controls for [FR-035](../functional/FR-035-bind-ecosystem-subjects.md).
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
all required runtime scope selections and their valid producer/native relations.
Then independently remove a required scope selection, withhold a required record
under the otherwise valid mapping, and substitute a producer relation for a
different model/profile with the same display name. The first and third mutants
refuse binding; the second remains incomplete evidence. Prior static success
cannot discharge any missing runtime prerequisite or authorize an override.

## Expected Results

Declared relationships and workflow identities survive binding. Substitutions refuse, scope changes remain visible in request identity, and unavailable membership cannot produce aggregate success. Concrete producer/observation representations come from D/F's reviewed contracts, not a duplicated test model.

Compatible explicit role sharing is admitted; missing, duplicate and mismatched
role mappings refuse. Runtime configuration changes remain identified and cannot
override linked meaning. Missing future recovery records do not manufacture an
activated obligation, while evidence required by an activated assessment remains
incomplete. Assertions inspect typed causes, references and obligation identities.
