---
id: FR-150
title: "Preserve original and effective model declarations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-035
    type: references
  - target: ix://agent-ix/quire-specification/FR-321
    type: references
---

# FR-150: Preserve original and effective model declarations

## Description

When normalizing the declarations of an admitted domain package, the model
checker SHALL retain each original declaration key, effective declaration
identity and ordered derivation provenance.

## Inputs

The original declarations that model intake (FR-154) builds from one domain
package, its ModelSelection (FR-321) and the selected
`quire.model.complete/v1` definition with its rule manifest
`quire.model.complete.rules/v1`.

## Outputs

An immutable retained-original inventory and a structural effective view,
carried as one `quire.model.effective-view/v2` value, or a typed refusal.
The structural view is checking input, not a validated checked package.

## Behavior

Each original declaration is keyed by `(domain package identity, IR node
identity)`. Supertypes, subsets and redefines are members of the
IR nodes, so every edge normalization reads is part of the admitted package.
QSL derives effective declarations. It never mints, renames or merges a
declaration key, and never derives an edge the package does not declare.
Normalization never replaces the original key or actual FCD origin. Every derived
member records its source declarations and rule. Equal normalized shapes from
distinct original declarations always remain distinct effective declarations;
no identity-equivalence declaration exists.

The model checker SHALL retain every phase-1 original declaration and its
actual FCD origin in the immutable selection inventory, including originals
whose kinds have no phase-2 qualify rule. It SHALL derive effective entries
only for the selected rules' kinds. A retained scalar, population, namespace, or non-`quire` clause
without such a rule SHALL NOT receive an invented qualify fact, effective
declaration or additional declaration charge. An operation's `Pre:` or `Post:` contract clause is not a declaration, so it is neither retained nor qualified. Effective entries and fact
inputs reference keys from the exact retained inventory. This inventory is
existing selection data, not an added member of an effective declaration identity.

The effective member key is `(owner effective type identity, original
declaration key)`. Paths that reach the same original declaration collapse to
one effective member that links to each direct parent reaching it; its
expansion lists every path. A source name
that resolves to distinct original declarations is refused
`ambiguous_declaration`/`ambiguous-name` at source resolution and, for a
model-owned `field` or `operation` member of a checked package, when a V2
reader resolves that member's name (FR-322 "Model-owned members" step 3);
the effective view keeps both members.

Each effective declaration identity is the structural
`{owner_effective_type, original}`, compared componentwise: `original` is the
original declaration key and `owner_effective_type` is `null` for an effective
type and the owning effective type's identity for a member. Nothing is hashed.
Its facts are `{ordinal, rule, inputs, parent}`, with `parent` the structural
identity of the one parent a fact links to; the effective view
(`quire.model.effective-view/v2`) is the value
`{model_selection, rules, declarations}` holding the ModelSelection, rule
manifest and every effective declaration with its facts, ascending by identity;
and the object universe of a connected supertype component
(`quire.model.object-universe/v2`) is the value `{model_selection, root_types}`.
The exact shapes and orders are defined by the selected
`quire.model.complete/v1` definition. A node id of the checked package is
`{package, path, ordinal}` (FR-322) and an effective declaration identity has
the different shape above, so no effective identity equals a node id. The
effective view is carried in the checked package as a `correspondence` node
with semantic form `model_correspondence`.

### Chain-link identity

An effective declaration's facts bind exactly its own content and a link to the
effective declaration of each direct parent: for a type, each direct supertype;
for a member `(T, m)`, the member `(P, m)` of each direct supertype `P` of `T`
that has it. The `parent` of an inherit fact is that parent's identity; every
other fact has `parent` `null`. A fact never contains an ancestor path
longer than one edge, so a diamond stores one inherit fact per direct edge and
the retained facts grow with the number of types and direct
edges, not with the number of ancestor paths. The selected
`quire.model.complete/v1` definition owns the exact fact list and order.

The full path-form derivation, every distinct inheritance path included, is
the deterministic expansion of the retained facts through the parent
links. A reader expands it on demand under its own limits; normalization
never encodes an expanded path. Equal facts have equal
expansions, so declarations that differ in owner, original key or expanded
facts differ, and one effective declaration has exactly one fact list. The
strict reader refuses a link of the wrong kind, owner or original.

## Normalization derivation

The model checker applies the manifest's rules in this fixed phase order:
(1) `quire.model.normalize.intake/v1` and `quire.model.features/v1`, domain
package admission, declaration building and feature checks (FR-154);
(2) `quire.model.normalize.qualify/v1`, qualified-identity formation for
object types, record value types, event types, state machines, processes,
persistence interfaces, variant types, interfaces, variant cases (owned by
their variant type), fields, relationships, operations (owned by a type,
interface or persistence interface), `quire` invariant clauses owned by a type,
parts, ports (owned by their part's owning type), connections and allocations
(owned by each end's end-owner type);
(3) `quire.model.normalize.inherit/v2`, inheritance along each direct declared
supertype edge; (4) `quire.model.normalize.subset/v2` and
`quire.model.normalize.redefine/v2`, declared subsetting and redefinition; and
(5) `quire.model.normalize.canonicalize/v1`, identity computation. Each derived
fact records its declaration-key inputs, rule identity, ordinal and parent link. A later phase may reference but cannot erase an earlier derivation. No
rule declares an override relation, and no precedence exists between rules.

Two phase 4 redefinitions of the same member reaching one effective type
conflict unless one redefining owner is a proper descendant of every other;
subsetting never conflicts with redefinition because it derives no replacement.
A conflict is refused `invalid_model_binding`/`derivation-conflict` with both
derivation paths and exposes no effective member for either. A redefinition
whose target is not an inherited member is refused
`invalid_model_binding`/`redefinition-target`. A supertype cycle is refused
`invalid_model_binding`/`specialization-cycle`. A ModelSelection row is `{identity}` and carries no digest or version (FR-154). A model `variant` type
referenced by source is admitted (FR-208). Canonical ordering
affects serialization only; it never changes a declaration key or resolves
semantic ambiguity.

Normalization and later conformance share the one operation-owned meter under
`ModelNormalizationLimitsV1` of the selected
`quire.value.accounting/v1` definition at its named record, fact, cycle-check,
redefinition-check, conflict-check and declaration charge
points. `derivation_facts` counts the retained facts, not the path-form
facts of their expansion, and each direct supertype edge is charged once,
however many paths reach its target. Each normalization phase checks
exhaustively and reports every refusal it exposes in charge order; a phase that
reports a refusal ends checking. Exhaustion ends checking as
`resource_exhausted`/`insufficient-next-charge` with no effective view.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-150-AC-1 | An inherited/redefined member links its normalized identity to every contributing original declaration and rule. | Test (TC-195) |
| FR-150-AC-2 | Two equal effective shapes from distinct original identities remain distinguishable. | Test (TC-195) |
| FR-150-AC-3 | A conflicting phase 4 derivation refuses normalization with its named cause, and a supplied document is admitted at the normalization boundary under the identity its lock row records. | Test (TC-195) |
| FR-150-AC-4 | Replaying the ordered derivation from the same domain package reproduces the `quire.model.effective-view/v2` value by structural equality, independent of IR node order. | Test (TC-195) |
| FR-150-AC-5 | Conflicting derivations report both rule paths and expose no chosen effective member. | Test (TC-195) |
| FR-150-AC-6 | Diamond paths reaching one original declaration collapse to one effective member with one inherit fact per direct parent, whose expansion lists every path, | Test (TC-195) |
| FR-150-AC-7 | Declaration keys are fixed by the domain package identity and IR node identity: two documents of one package with equal nodes yield equal effective declaration identities and equal effective views. | Test (TC-195) |
| FR-150-AC-8 | A normalization run at its exact `ModelNormalizationLimitsV1` bound completes, and the one-less run is incomplete at the named charge point. | Test (TC-195) |
| FR-150-AC-9 | An admitted scalar without a phase-2 qualify rule remains retrievable by its byte-identical original key and origin, with no scalar qualify fact, effective entry or extra effective-declaration charge; all effective/fact-origin keys belong to the retained intake inventory. | Test |
| FR-150-AC-10 | Transfer from structural normalization to checked-fact admission and conformance preserves the same model counters; a denied next charge follows no work and exposes no validated effective view or unfinished-stage failure, including failures already collected from another selected model. | Test |
| FR-150-AC-11 | A walk following n supertype edges admits at ancestor_steps=n; at n-1 it returns exactly incomplete(ancestor_steps,n-1,n-1,1,model.ancestor-edge), and zero denies its first edge. An unavailable owning work charge wins before a simultaneous edge denial; previously admitted axis work is never refunded. | Test |
| FR-150-AC-12 | Expanding the retained facts of every effective declaration through its parent identities yields the path-form facts of an independent enumeration over the original declarations, in phase, input and rule order with multiplicity, for a chain, a diamond and a hidden redefinition; a link naming a declaration of the wrong kind, owner or original is refused. | Test (TC-195) |
| FR-150-AC-13 | Changing one parent link or one phase-4 fact changes the facts of the declaration that holds it and of no other declaration, and no two distinct effective declarations of a fixture set share an identity. | Test (TC-195) |
| FR-150-AC-14 | Retained facts and work units of a chain, a width-2 diamond ladder and a dense multi-parent shape equal the closed figures of TC-195 N13; a ladder whose path count is exponential is admitted at the default limits. | Test (TC-195) |
| FR-150-AC-15 | For a single-inheritance chain of 10,000 edges (10,001 effective types), normalization at the default limits eagerly forms every effective declaration and the complete view with 20,001 retained facts and at most 100,000 work units. | Test (TC-195) |
| FR-150-AC-16 | The FR-150-AC-15 case, run in an isolated release-profile process on the reference host Terra, has a peak resident set of at most 21,474,836,480 bytes and a wall time of at most 30 seconds, each measured from package input admission through retention of the complete result. | Test (TC-195) |
| FR-150-AC-18 | A dispatch link whose family walks (the redefinition-family walk and every effective-precondition walk, together) follow n distinct `redefines` edges, each charged once however many walks follow it, admits at `family_steps = n`; at n-1 it returns exactly `incomplete(family_steps, n-1, n-1, 1, model.family-edge)` and no dispatch table or ambiguity result, and zero denies its first edge. An unavailable earlier counter's charge wins before a simultaneous edge denial. | Test |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
