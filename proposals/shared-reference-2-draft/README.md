# Verification reference amendment

This private amendment, merged by specification PR #8 at `8ab058b`, addresses
B's conditional review of that PR and is accepted by FS05 as an internal
interchange contract. Independent consumer-specific qualification and public
promotion remain separate.
The state-core 1-draft schema is a separate version.

## Selected versions and scope

The new envelope has exactly wireVersion, kind and value. wireVersion is
`ix.shared-reference/2-draft`; kind is artifact or semantic. This version covers
these two forms only. Existing source-map and migration envelopes keep their
1-draft selection; this amendment supplies no inferred migration, decoder
fallback or general extension bag.

An artifact value selects `ix.artifact-ref/3-draft`. Its five mandatory fields
are refVersion, kind, authority, identity and wire. Wire has exactly identity
and version (nonempty strings). Authority and identity are nonempty strings.
Every record is closed.

The thirteen prior kinds remain: source, model-package, linked-package,
executable-projection, property, binding, snapshot, invocation,
generated-artifact, run-artifact, environment, model-manifest and
dependency-closure. The six additional kinds have these exact roles:

| Kind | Artifact selected |
| --- | --- |
| trace | Recorded ordered trace supplied as verification input. |
| fault-model | Authored model of faults/mutations selected for a verification method. |
| review-procedure | Authored procedure governing a manual review. |
| oracle | Definition or executable artifact selected as the oracle; its role alone does not establish independence or correctness. |
| observation | Retained observation contributed to an assessment; its role alone does not establish truth or independence. |
| review-disposition | External disposition artifact about an exact reviewed core; separate from its procedure and observations. |

The consumer selects its expected role explicitly. A structurally valid nearby
kind does not match that role and is refused. No generic-kind fallback is permitted.
Artifact identity is the exact tuple of kind, authority, identity and
wire under its selected refVersion. Equal bytes alone cannot join different
roles. Changing versions requires a separately accepted
mapping; an amendment's existence is not that mapping.

## Semantic reference

A semantic value has exactly artifact, language, semanticProfile and
requiredFeatures, preserving the prior field meanings.
All are required; only language and semanticProfile may be
null. A null profile requires a null language and no required
features. A non-null profile has exactly identity and version. A non-null language has identity and edition.

requiredFeatures is a duplicate-free decoded set of nonempty Unicode scalar
strings. Producers order it by UTF-8 bytes; readers accept reordered unique sets
and reject duplicate decoded members before comparison. Unknown required features
refuse interpretation. This rule does not sort arrays inside the referenced
artifact.

The selected semantic profile defines what a validated value means. Input intake must reject
duplicate decoded keys, invalid Unicode and non-finite numbers.

## Review disposition and its core

The disposition's ArtifactRef uses review-disposition. A nearby artifact kind refuses
correspondence.

B owns all report-core fields, validation, reviewer/procedure/provenance checks,
missing-disposition incompleteness and final result policy. The fixture here uses
an explicitly synthetic fixture profile to demonstrate the disposition
shape without claiming to define B's report payload or record a real review.
B qualifies the same rule against its actual report schema under IT-003-SC-03.

## Qualification boundary

FR-017/486 and TC-001, TC-002 and TC-975 define positive and adverse vectors; IT-003 records
producer structure and the later independent
strict reader as distinct observations. Structural schema validation is only
one layer. Readers retain the strict bounded intake, source/authority and
artifact checks from the adopted packet. Exhaustion yields incomplete without
partial acceptance. No network schema retrieval is authorized by schema IDs.

The accepted schema and vectors are first-party authored data. Existing document
and reusable-artifact publication terms remain unresolved; the packet stays
private. Any new executable qualification source is Rust under AGPL-3.0-or-later,
with original dependency grants retained. No hosted workflow is dispatched.
