# Native protocol, compensation and result contract

Draft PT01/PT02 contribution to the composed Quire v1 baseline. It defines
semantic representation and consumer contracts. Agent A owns the single native
grammar and integrates the eventual protocol surface; this document does not
create another authored language, parser or evidence system.

## Selected v1 boundary

The v1 protocol profile represents a finite global protocol over explicitly
identified workflow instances. Its admitted structure consists of finite named
roles and role instances, typed messages, explicit channels, distinct send and
receive events, authored causal order, role-owned labeled choice, bounded
parallel branches and join, bounded iteration and declared termination. A
protocol package also states delivery and duplication premises and the facts
visible to the role that owns each choice.

The evaluator checks supplied observations. It never sends a message, invokes a
business operation, registers a real compensator or performs recovery. Global
finite replay and conformance are selected v1 capabilities. Projection,
realizability, refinement and assume-guarantee composition remain independent
requested capabilities with their own premises and may be unavailable while
global conformance remains useful.

The representation consumes A-owned source/profile/predicate identities,
D-owned component, relationship and population identities, E-owned clock and
closure semantics, and F-owned observation/correlation/completeness bindings.
It preserves those authorities rather than copying their models.

## Identity model

These identities remain distinct and typed:

| Subject | Meaning |
| --- | --- |
| Protocol definition | Static authored global behavior under an exact language edition and profile definition. |
| Workflow instance | One assessed occurrence of a protocol; related instances remain separate occurrences. |
| Role / role instance | An authored responsibility / its configuration-bound participant in one workflow instance. |
| Message | The authored typed communication obligation. |
| Send / receive | Distinct protocol events; neither one implies the other. |
| Delivery | A transport observation carrying a message attempt; repeated delivery need not be a new effect. |
| Attempt | One invocation try under an authored operation and retry relation. |
| Effect | A declared successful business-state transition, identified separately from attempts and receipts. |
| Compensation registration | An obligation created only after its paired effect succeeds. |
| Compensation attempt / effect | A recovery try / its successful business effect. Success does not imply restoration unless the recovery relation establishes it. |
| Commit | An authored boundary after which specified compensations are no longer permitted. |
| Result | Facts about one exact request and subject, with execution, truth, activation, completeness and adequacy separated. |

Identity substitution refuses. A telemetry trace identifier, endpoint, queue,
timestamp or equal display name cannot stand in for a workflow, role, message,
attempt or effect identity. Related order, shipment, payment-attempt, payment-
effect and refund instances use explicit typed relationships. Population
obligations retain their declared snapshot or event-time window plus membership,
progress and completeness dependencies; unknown required membership cannot
establish success.

## Control and observability

Sequence contributes explicit causal edges. Parallel branches admit every
interleaving consistent with their branch edges and join condition; input order
or wall-clock sorting does not add causality. An iteration has an authored finite
maximum and a progress edge. Termination names the required branch/workflow
closure condition.

Every choice has one owning role, labels, branch guards and a visibility set.
The owner must be able to distinguish the branch from information available at
the decision point. A choice whose branches are distinguishable only by a hidden
fact is valid source syntax but refused by the selected protocol profile. This
is an admission/conformance premise; it does not claim a general projection
theorem.

Channel premises state participating roles, message types, ordering policy,
delivery cardinality and whether duplication can occur. V1 does not silently
assume exactly-once delivery, FIFO across channels, reliable delivery or globally
synchronized clocks. A trace is conformant only relative to the exact premises
selected by the request.

## Compensation semantics

A compensation definition binds a forward effect, a compensating operation,
the role that owes it, eligible failure/abort conditions, a retry relation, a
commit boundary and an authored recovery relation. Registration occurs after the
forward effect is established. An untriggered registered compensation is
inactive; an eligible but unobserved compensation is incomplete or pending under
the supplied progress/closure authority; an attempted compensation can succeed,
violate, fail or time out.

The recovery relation compares the declared post-recovery state with its
authored target. A partial refund can satisfy a partial recovery obligation while
failing full restoration. Two refund attempts, two transport receipts and two
refund effects are three different cardinalities. Retrying under the same effect
identity cannot manufacture another successful business effect. Commit before a
required registration, compensation before its forward effect, and compensation
after a forbidding commit are protocol violations.

## Shared assessment result

Each requested subject/capability pair produces one retained result even when it
cannot execute. The minimum orthogonal dimensions are:

| Dimension | Closed meanings |
| --- | --- |
| Execution | completed, refused, unsupported, resource-incomplete, failed |
| Truth | satisfied, violated, pending, unavailable |
| Activation | triggered, inactive, activation-unknown |
| Participation | exact participating and required role-instance sets, or unknown with reasons |
| Completeness | complete, open, incomplete, with missing inputs and authority/progress identities |
| Adequacy | demonstrated, not-demonstrated, inapplicable, unknown, with the exact obligations and cases considered |
| Claim | global-conformance, monitorability, local-projection, realizability, refinement or composition |
| Provenance | source/profile/model/protocol/request/binding/observation/progress/backend identities and derived artifact digests |

The combination is constrained. A refused or unsupported execution has no truth
verdict. A logical success requires complete inputs for every fact on which that
truth depends. An inactive obligation is not a satisfied obligation and cannot
demonstrate a compensation path. Missing observations, required roles, population
members or negative cases cannot improve truth, adequacy or package-level status.
Package-level complete success requires every selected required request to have
an eligible successful result; independent supported results remain available
when another request is unsupported.

The result is a portable semantic contract for A/E/F and future Engineering
Assurance consumers. Quoin retains evidence ownership. The protocol evaluator
does not introduce a second evidence store, execution runner, report registry or
verification-method catalog.

## Distinguishing corpus

The planned corpus uses orders O1 and O2, shared external payment provider P,
inventory, split shipments S1/S2, payment attempts A1/A2, payment effect E1 and
refunds R1/R2.

Accepted cases:

1. Payment success registers one refund obligation; concurrent S1/S2 branches
   join before order completion.
2. A failed shipment activates a refund; one retry shares the refund obligation
   but retains a distinct attempt; a single full refund effect satisfies the
   recovery relation.
3. Two related partial refunds sum exactly to the authored amount under a closed
   declared population and satisfy partial recovery.
4. O1 and O2 share provider P while retaining distinct workflow/effect subjects.
5. Global replay completes while local projection is explicitly unavailable.

Refused, violating or incomplete controls:

1. A choice owned by fulfillment depends only on a payment-provider fact hidden
   from fulfillment: refused as unobservable.
2. The same transport delivery is relabeled as a second payment effect: refused
   identity binding or duplicate-effect violation.
3. Compensation appears before its forward effect, registration occurs after a
   forbidding commit, or refund occurs after that commit: violation.
4. A triggered refund fails, times out, or lacks closure/progress needed to
   decide: failed, pending or incomplete remain distinct.
5. A refund belongs to O2 while its compensation obligation belongs to O1:
   correlation refusal; equal trace/provider identifiers do not repair it.
6. One expected partial-refund population member is missing or membership is
   unknown: no aggregate success.
7. Mutual local assumptions admit a circular story without a causal global
   strategy: no realizability or composition success.
8. A satisfiable but unprojectable global trace remains eligible for global
   replay while the projection request is unavailable or refused.

Every case pins finite branch, iteration, message, active-instance and observation
bounds in its fixture. Example cardinalities do not become semantic maxima.

## Reuse and claim limits

PT01/PT02 may adopt a Rust library only after E03 records exact version, source,
license, runtime boundary, admitted algorithm/profile and discriminating adverse
cases. Similar terminology does not transfer a theorem. General projection,
realizability and composition are not prerequisites for the selected global
replay/conformance slice.

The maintained architecture and its library list are research inputs. Earlier
FRETish-source and foreign-runtime recommendations are superseded by the owner
ruling: native Quire is the sole editable formal-clause language, external forms
are export-only, and production or possible qualification integrations are Rust.
