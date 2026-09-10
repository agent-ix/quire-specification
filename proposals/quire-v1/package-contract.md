# Linked packages and assessment bindings

Proposed consumer contract for [FR-030](../../spec/functional/FR-030-bind-composed-definitions.md)
and [FR-035](../../spec/functional/FR-035-bind-ecosystem-subjects.md), for the
composed `1-draft` candidate. It does not register a wire format or canonicalizer,
adopt a profile, or replace D's model/configuration, F's observation or B's
request/result contracts.

## Static package subject

An explicit compiler input inventory supplies the complete native source-unit
set and exact locally supplied dependencies. It identifies the language/edition
definition, which must agree with every source header. Directory contents,
installed tools and retrieval locations cannot add units or select dependencies.
All units use one edition. Aliases are unit-local; native declaration names share
the closed package namespace. A missing source unit prevents establishing that
namespace. A missing model/profile refuses its dependents without erasing
independent declarations whose names and dependencies are established.

| Static component | Information retained and checked |
| --- | --- |
| Sources | Exact authority, identity, revision and byte digest per unit; original source regions and clause/declaration identities. Two editable authorities for one clause refuse. |
| Language | Exact language/edition and supplied definition artifact; no implementation-version default. |
| Profiles | Each declaration's selected root definition name/revision/byte digest, exact required definition closure and the meaning owned by each dependency. |
| Models | Compiled model document, permitted exports, authoritative declaration handles and exact manifest/lock/dependency closure under the selected producer contract. |
| Native dependencies | Resolved predicate, operation-contract and temporal-obligation references with their defining source, model and profile selections; typed anchors and capture requirements. |
| Binding requirements | Typed roles, selected model/observation contracts and clock/scope premises described below. Concrete runtime instances are not inserted here. |

The native `model` import digest continues to mean SHA-256 of the selected
compiled model document's exact bytes. Manifest/lock digests, the producer's
package fingerprint and source digests remain separate. Package identity/version
and exports must match that document and its validated closure. A transitively
reachable type is not thereby a directly imported export. D controls model
closure, presence, units, relationships and configuration; A consumes its exact
admitted view or refuses unsupported meaning. The old minimum-only ConfigVersion
domain and parent relationship remain admission controls, not implicit finite
bounds or fields.

Each profile definition names its exact required definitions and the forms,
value rules and interpretation it supplies. Transitive selections are validated
before dependent declarations are published as checked. Definition cycles,
conflicting content for one immutable definition identity, missing requirements
and incompatible requirements within one selected closure refuse. Installing a
profile cannot grant operators. Declarations can select different compatible
interfaces; a referenced predicate retains its own checked definition rather
than acquiring caller permissions. This definition-dependency rule does not
prohibit identity-bearing model graphs or redefine D's package dependency policy.

Partial compilation retains every requested declaration's disposition and
dependency cause. Only dependency-closed checked declarations reach a consumer;
partial output cannot claim whole-package admission. An unrelated backend's lack
of support is a request disposition, not a static source failure.

## Typed binding requirements

A required role is identified by its owning native declaration, original source
region and role kind. Its local name is retained for diagnostics, not used as a
global lookup key. Two `"refund-clock"` roles in different clauses remain distinct
even when an explicitly supplied clock satisfies both.

| Role information | Required interpretation |
| --- | --- |
| Type and authority | Exact model declaration or selected producer interface; kind distinguishes value input, invocation, workflow/participant, relationship, population and clock/progress. Equal field shapes do not substitute for identity. |
| Anchor | Current observation, exact invocation pre/post, temporal origin/activation, protocol occurrence, compensation registration or activation, as authored. |
| Scope premises | Workflow/role/relationship endpoints, snapshot or window/membership contract, clock kind/unit/history and completeness authority where applicable. |
| Dependencies | Prerequisite roles, supplied record correspondence and immutable captures. A clock, population and its progress record must describe the same declared scope. |

Roles derive from checked native declarations and imported authoritative
contracts. They introduce no field grammar, event schema or model store. The
linker retains their contract selections; it does not infer an adapter from a
binder's spelling. A semantic change to an imported binding contract requires a
new static selection. Switching between admitted concrete bindings satisfying
the same contract is an assessment input change.

The runtime request maps each required role to one binding under D/F/B's selected
contracts. Repeated entries for the same role refuse, even when equal; one
explicit concrete binding may serve several compatible roles. Missing or
incompatible mappings refuse the affected request. Missing observations under a
valid mapping remain incomplete when the obligation requires them. An absent
observation is not a model's optional-none value.

Binding a template does not require inventing future events. A recovery role can
be mapped before compensation registration or activation. Absence of a recovery
record cannot fail a prerequisite of an unactivated recovery obligation.
Activation, valid open-prefix pending, missing required evidence and inactivity
at closure remain distinct under B/E/F's rules. Captures bind once at the
authored anchor; mapping another role cannot retag them or supply another
workflow's values.

## Configuration and assessment identity

Retain the complete, exact configuration artifact consumed at each stage. Its
semantic selections must agree with source and linked requirements. A runtime
configuration cannot change a linked model, profile or binding-contract meaning:
such a request refuses and requires an explicitly newly linked subject. Concrete
adapters, populations, traces, clocks, progress, implementations, requested claims
and resource controls remain identified assessment inputs.

The static subject is compared through the components above. Build provenance
retains the full compile configuration and implementation; assessment provenance
retains the full runtime configuration and inputs. Changing only a resource limit
changes configuration and assessment input identity while preserving static
meaning, consistent with D's proposed configuration requirements. Changing a
selected model/profile or required binding meaning changes the static subject
as well. A projected configuration cannot stand in for the original input.

Exact serialized bytes, static language subject and assessment inputs have
distinct identities. Reordering a set in a permitted encoding may preserve its
subject while changing raw bytes. Source-byte changes alter source correspondence
even if a typed-expression canonicalizer establishes equal expression meaning.
Canonical identity is optional until its exact domain/version/algorithm is
selected and qualified; structural comparison or ordinary JSON hashing cannot
claim it. Existing ArtifactRef/SemanticRef, producer fingerprint and IR/JCS domains
retain their selected contracts. Definition digests cover exact supplied bytes;
an artifact need not contain its own byte digest.

## Admission examples and remaining producer agreement

Use a three-family order package with two typed per-observation inputs and a
refund-clock role. Bind O1 and O2 through distinct workflow/relationship instances
even when they share a provider and clock authority. Swapping O2's progress into
O1's population must fail the scope check. Raising the run budget preserves the
static subject; changing the temporal boundary profile requires a new linked
subject. Unactivated compensation needs no fabricated refund record.
[IT-010](../../spec/integration/IT-010-bind-linked-package-assessments.md) specifies
the producer/consumer controls; none has executed yet.

D's inspected [FCD PR #97](https://github.com/agent-ix/filament-core-data/pull/97)
at `fa78dc7165b4d8ddc2f49da026b372f205f23902` supplies proposed presence,
relationship, population and explicit configuration requirements. It does not
yet supply the standard-facing FS02 contribution. F's FR-110–114 supplies
observation obligations but not a frozen binding record. Their exact wire/type
correspondence and composed profile definition artifacts require agreement
before these integration cases become executable contracts.
