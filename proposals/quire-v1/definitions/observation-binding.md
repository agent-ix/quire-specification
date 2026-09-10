# Observation binding definition candidate

Selection identity: `quire.observation.binding/v1`; revision: `1-draft.1`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This definition selects the semantic observation-binding contract consumed by a
linked package. It is not a native source profile, wire schema, model schema,
result store, adapter protocol, or executable interface. A package selects this
definition by its exact identity, revision, and definition bytes in addition to
the native linked-package selection and every source/schema selection used by
the applied bindings.

## Selected meaning

The selected meaning is the [observation binding and assessment-input
contract](../observation-contract.md): declared role identity consists of the
owning native declaration, original source region, and role kind; local names
are diagnostic only. Each required role has exactly one applied binding, except
that one explicit concrete binding may serve several compatible roles. The
selection preserves source/schema, subject, signal/value/unit, anchor,
scope/progress, capture, and record identities without heuristic substitution.

The definition carries no field grammar of its own. It consumes A's
package/source/profile identities, D's model and relationship identities, E's
clock/history/closure meanings, and B's result dispositions through their
selected contracts. A signal name, trace identifier, endpoint, compatible field
shape, or adapter version is not an alternative identity.

## Selection boundary

Changing this definition's identity, revision, or selected meaning requires a
new linked subject. Changing only a concrete role mapping, admitted record set,
or declared resource limit under this same definition/revision creates a new
identified assessment input and does not alter the static subject. A selected
definition does not fabricate a required record, causal relationship, future
compensation, completeness premise, or result disposition.

Progress/closure authority is scoped to its declared population/window and
clock meaning; O2 progress cannot settle O1. A compensation role may be mapped
before activation without manufacturing a refund record. Once an assessment
requires an unavailable record or premise, its availability remains incomplete
or refused under the selected result contract.

## Adoption boundary

This candidate must be indexed with its exact byte digest in A's shared
definition catalog before a composed baseline can select it. Structural text
validation, target-language parsing, or a concrete binding mapping does not
adopt this definition or qualify an observation implementation.
