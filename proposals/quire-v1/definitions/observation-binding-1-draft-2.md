# Observation binding definition candidate

Selection identity: `quire.observation.binding/v1`; revision: `1-draft.2`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This successor retains the binding, role-sharing, correlation, availability and
static-versus-assessment-input meanings of
[1-draft.1](observation-binding.md). It adds the immutable dependency selection
that `1-draft.1` intentionally left open. It is not a source profile, wire
schema, model schema, result store, adapter protocol, or executable interface.

## Required immutable dependencies

| Dependency | Identity | Revision/version | Exact bytes or source revision |
| --- | --- | --- | --- |
| Linked package interpretation | `quire.package.composed/v1` | `1-draft.1` | `sha256:0df176dbb831117bd27443f7ef8075d9a130bcbc225022b440acbce111c62b9d` |
| Temporal lifecycle/result facet | `quire.temporal.bounded-facet/v1` | `1-draft.3` | `sha256:e97b0065361736289c7a37fbdcd1fd0b9af73c8af38a8de1863066179c99e197` |
| Protocol result/conformance root | `quire.protocol.finite-global/v1` | `1-draft.4` | `sha256:721e1c0541eea01db4119c3ee990494b58af140ad90adec63ef6e4b2f6fc5270` |
| Producer interface | FCD Producer interface | `1.2.0` at `06a62c7624ea56bbe30120e8987d2ec4f551e77d` | `sha256:de838deae48b2b11c79019749b91057049e9ca5e80a483d9281cb1a7d2ab96ae` |
| Shared result rules | B FR-061 | reviewed `2d8372c0ce22f43b313dba139880d00890f02984` | `sha256:f56b11b7ee602a5d77ace8f3ac945661042d171f4935a82c1f7da184b6661c42` |

An assessment selects every listed dependency by the exact identity, revision
and bytes above. Missing, substituted, stale, or mutually inconsistent selections
refuse the dependent assessment. A compatible-looking current profile, package,
producer document, result, or local default is not a substitute.

## Retained binding boundary

Roles retain their owning declaration/source-region/kind identity. One explicit
binding may serve compatible roles, but equal local names never merge roles.
Each applied binding retains the selected producer relationship/population/window,
source/schema, anchor, unit, capture, and scope identities. Changed concrete
mappings, records or resource limits under this selected closure remain distinct
assessment inputs; changing any dependency above requires a newly linked subject.

The selected result interface retains assessment execution, independent
decision-scope and surrounding-execution closure, truth, settlement basis, exact
decision support, activation, participation, completeness, adequacy, provenance,
and linked supersession. This binding definition consumes those facts; it does
not define a second result/evidence family.

## Adoption boundary

A must index this successor's exact digest before a linked package selects it.
The prior `1-draft.1` bytes remain recoverable and cannot be rewritten to mean
this dependency closure.
