# Observation binding definition

Selection identity: `quire.observation.binding/v1`.
**The selection role is accepted by FS01 and this semantic revision is accepted
by FS04. Serialization, canonicalization and executable qualification remain
separate.**

Accepted definition: local rule and dependency links select this checkout.
Git preserves drafting history; FS04 freezes this coherent baseline.
This editing policy does not relax immutable runtime selections or digest checks.

This definition selects the observation-binding contract consumed by a linked
package, including role sharing, correlation, availability, independent result
axes and the explicit correspondence for all three clock families. It is not a
native source profile, wire schema, model schema, result store, adapter protocol
or executable interface.

## Required definitions and rules

| Dependency | Identity | Current normative contract |
| --- | --- | --- |
| Linked package interpretation | `quire.package.composed/v1` | [Package reference](package-reference.md) |
| Observation-range correspondence | `quire.observation.range/v1` | [Range correspondence](observation-range.md) |
| Observation binding rules | `quire.observation.contract/v1` | [Observation contract](../observation-contract.md) |
| Temporal lifecycle/result facet | `quire.temporal.bounded-facet/v1` | [Temporal meaning](temporal-common.md) |
| Protocol result/conformance root | `quire.protocol.finite-global/v1` | [Protocol meaning](protocol-finite.md) |
| Domain declarations | Checked package `model_selections` domain package | [Domain declaration selection](../observation-contract.md#domain-declaration-selection) |
| Shared result rules | B FR-061 | [Orthogonal results](../../../spec/functional/protocol/FR-061-report-orthogonal-results.md) |
| Observation result rules | F FR-115/116 | [Result contribution](../../../spec/functional/observation/FR-115-report-activation-participation-and-adequacy.md) and [consumer-result boundary](../observation-output-mapping-contract.md) |
| Target output-mapping rules | `quire.output.mapping/v1` | [Common and target profiles](../output-mappings/README.md) |

FS04 selects only the temporal/observation and shared result/closure rules
reachable from this definition. FS06 separately selects the non-authoritative
target-output mapping contract; that contract consumes the qualified result
axes without changing this binding. Neither acceptance adopts protocol
declaration or global-conformance semantics, whose owning work remains independent.

An actual package/assessment selects its required dependency artifacts by exact
identity, revision and bytes. Missing, substituted, stale or mutually inconsistent
selections refuse the dependent request. A local draft link does not authorize
an evaluator to replace its selected artifact with current working-tree bytes.
Declaration, raw document, definition and result digests retain their
declared domains; equal digest prefixes do not make them interchangeable.

## Retained binding boundary and range selection

Roles retain their owning declaration/source-region/kind identity. One explicit
binding may serve compatible roles, but equal local names never merge roles.
Every required role maps exactly once; repeated or incompatible mappings refuse.
Each applied binding retains the selected relationship and population declaration keys, window,
source/schema, subject, signal/value, anchor, unit, capture, scope and range
identity. Internal/external visibility remains explicit. A signal name, trace
identifier, endpoint, compatible field shape or adapter version cannot replace
the selected model, relationship or observation authority.

Changing the selected binding meaning requires a newly linked subject. Changing
only a concrete role mapping, admitted record set or resource limit under the same
selected meaning changes assessment-input identity without changing the static
subject. Concrete ranges and observations are assessment inputs, not static
prerequisites for parsing or type-checking a future template.

A compensation role may be mapped before activation without manufacturing a
refund record. A valid mapping cannot fabricate a causal relationship, future
event, completeness premise or result. Once an assessment requires an unavailable
record or premise, it retains an incomplete/refused disposition with the actual
cause instead of false or optional-none. O2 progress cannot settle O1.

The selected range names exactly one clock family. Event-position records retain
sequence/position identity without an invented timestamp; fixed samples retain
exact epoch/period/unit/index; timestamped records retain their timestamp anchor.
Member-object identity, record identity, and position/sample/timestamp-anchor
identity remain independently readable. Start is included once and end is
excluded in every selected range. A timestamped native inclusive
deadline at the range end needs a separately selected covering range; no
closure label, membership list, or adjacent range may substitute.

The selected result interface retains assessment execution, independent
decision-scope progress and closure, surrounding-execution progress and closure,
complete-global-conformance closure, truth, settlement basis, exact decision support, activation, participation,
completeness, separate observation/protocol adequacy, assumptions, claim strength,
provenance and linked supersession. Omitting or cross-wiring a required retained
axis refuses; completed assessment execution cannot supply a closure fact.
A witness or counterexample inside exact admitted support may settle truth while
unrelated coverage remains incomplete. Compatible surrounding-execution changes
preserve that truth/support, and decision-scope closure cannot establish complete
global conformance without its own required execution/branch/workflow closure
record and premises. The global record retains B's explicit state; non-global
claims retain `not-required` with no global premise set. An absent required record,
duplicated or foreign premise, or a cross-wired closure refuses. A later contradiction retains earlier result bytes and produces only
linked supersession or invalidation under the corrected input identity.
This definition consumes the shared result contract and creates no second
result/evidence family.
