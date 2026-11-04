---
id: FR-210
title: "Protocol subject vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-049
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-050
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-051
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-035
    type: specifies
---
# FR-210: Protocol subject vocabulary

## Description

When a protocol requirement names a subject, the specification reader SHALL resolve that subject as a distinct typed kind rather than as a synonym recoverable from a transport record, a trace identifier or an equal label.

## Values

Each member below is named as a distinct subject by at least one requirement.
The citation records which requirement names it. These are semantic kinds, not
one universal producer wire enum: FR-287 qualifies the exact producer-local
kind discriminator by its selected authority. Requirements may name the subset
they consume without claiming another requirement's list is incomplete.

| Value | Description |
| --- | --- |
| `message` | The authored typed communication obligation. Named distinct by FR-051 ("preserve message, send, receive, delivery, attempt and effect as distinct typed subjects") and carried as "the single authored message/effect obligation" by FR-054-AC-1. |
| `send` | A protocol event. Named distinct by FR-051 and by FR-049-AC-2. A send alone does not establish a business effect (FR-051 behavior). |
| `receive` | A protocol event held distinct from `send`. Named distinct by FR-051 and by FR-049-AC-2. A receive alone does not establish a business effect (FR-051 behavior). |
| `delivery` | One concrete delivery subject. Named distinct by FR-051, FR-049-AC-2, FR-111 and FR-035 (as "message deliveries"). A second delivery key for one send increases delivery cardinality but establishes no second effect. |
| `receipt` | A distinct observation/provenance record that refers to a delivery. Replaying or adding receipt records for one delivery key does not mint a delivery, receive occurrence or effect (FR-051). |
| operation_attempt | Wire spelling `operation-attempt`. One forward-operation invocation try. Named `attempt` by FR-051/FR-111/FR-035 and `operation attempt` by FR-049-AC-2. An attempt with no established effect remains an attempt (FR-051-AC-3). |
| business_effect | Wire spelling `business-effect`. A declared successful business-state transition, named `effect` by FR-051/FR-049/FR-111/FR-035. It is never inferred from a send, receive, successful transport status, repeated delivery, trace identifier or equal payload alone. |
| compensation_attempt | Wire spelling `compensation-attempt`. One attempt against a compensation registration, distinct from `operation-attempt` under FR-058. |
| compensation_effect | Wire spelling `compensation-effect`. One business effect of a compensation attempt, distinct from both its attempt and a forward `business-effect` under FR-058. |
| `commit` | A concrete observation of an authored commit-boundary occurrence under FR-057; it is distinct from the static boundary object. |
| `compensation` | Named distinct by FR-049-AC-2 ("Send, receive, delivery, operation attempt, effect and compensation references remain distinct"). FR-051, FR-111 and FR-035 do not name it in their subject lists. |
| workflow_instance | Wire spelling `workflow-instance`. One assessed occurrence. Named distinct by FR-111 and FR-035; FR-050 binds it to its exact typed declaration and configuration identity. FR-051 does not name it in its subject list. |
| role_instance | Wire spelling `role-instance`. Named distinct by FR-035 ("Workflow instances, role instances, message deliveries, attempts and effects remain distinct subjects") and kept distinct from its static definition by FR-050. FR-051, FR-049-AC-2 and FR-111 do not name it in their subject lists. |

## Identity and comparison

- **Identity kind** — *value*. A subject kind label has no identity of its own
  and is compared componentwise as an exact kind. The identity of an occurrence
  of a subject belongs to that subject's own object artifact, not to this
  vocabulary.
- **Key** — the subject kind itself. Two references of different kinds are
  unequal. FR-050-AC-2 refuses substituting an endpoint or channel identity for
  a role and a component identity for a role instance; FR-051-AC-4 refuses a
  transport trace ID substituted for an authored workflow, attempt or effect
  identity. Display name, source path, span, message text, locale and rendering
  never participate: FR-035 states that "a trace ID, timestamp, endpoint or
  matching textual name alone does not establish a business relationship", and
  FR-049-AC-7 requires the exact declared relationship and endpoint direction
  "even when they share a provider, trace ID or equal scalar labels".
- **Key order** — the exact subject-kind representation supplied by its selected
  authority is compared under that producer contract as one FR-287 component.
  This table's row order is presentational and supplies no ordinal or precedence.
- **Causal order** — not applicable to the vocabulary. Causal order over
  protocol nodes is the authored partial acyclic order of FR-052, which "SHALL
  construct a finite causal graph ... without deriving causal edges from source
  order or timestamps" (FR-052) and is never derived from key order, timestamp
  or source position (FR-052-AC-2).
- **Absence** — a reference whose subject kind is absent compares only to
  absence. It is never defaulted to `delivery`, to `effect` or to any other
  member; FR-051 forbids inferring an effect from a transport fact alone.

## Not members of this vocabulary

A noun that the specification types as an object is not thereby a protocol
subject kind. The four subject lists above are the whole membership question, and
the following are named by none of them.

| Noun | What the specification does with it | Why it is not a member |
| --- | --- | --- |
| channel | Typed as [FR-215](FR-215-protocol-channel.md), a linked object with sender, receiver, message types, ordering policy, delivery cardinality and duplication premises (FR-054). | FR-050 keeps roles "distinct from components, endpoints and channels" — a channel is something a subject is bound through, not a subject assessed. FR-051, FR-049-AC-2, FR-111 and FR-035 do not name it. |
| compensation registration | Typed as [FR-233](FR-233-compensation-registration.md) (FR-056). | A registration is an obligation record paired with an effect. `compensation` is already the member FR-049-AC-2 names; the registration is not a second subject kind. |
| commit boundary | Typed as [FR-237](FR-237-commit-boundary.md) (FR-057). | FR-057 requires "separate attempt, effect, commit and recovery facts" — facts checked about a compensation, not subjects of a conformance claim. |
| recovery relation | Typed as [FR-238](FR-238-recovery-relation.md) (FR-057). | As above: an authored relation the evaluator decides recovery through, not an assessed subject. |
| definition | Typed by [FR-288](FR-288-qualified-protocol-definition-identity.md). FR-050 requires role instances and workflow instances to be kept "distinct from their static definitions". | A static definition qualifies authored protocol objects; it is not a concrete runtime subject kind beside `role-instance` and `workflow-instance`. |
| participant | A distinct concrete runtime/configuration binding object keyed by FR-287 kind `participant`; [FR-217](FR-217-role-internality.md) defines its boundary from role and role-instance. | A participant fills a role instance through a separately validated relationship but is not itself selected by the assessable event-subject vocabulary in this table. It is not a collective synonym for role. |
| component | No object artifact. FR-050-AC-2 refuses "a component identity for a role instance". | Named only as an identity a role instance must not be substituted by. |
| endpoint | No object artifact. FR-050-AC-2 refuses "an endpoint or channel identity for a role". | Named only as an identity a role must not be substituted by. |

The typed objects in the table retain their own identity rules. FR-287 types the
runtime subject references and FR-288 types the static definition qualifier;
neither adds a member to this subject-kind vocabulary. The table above is the
complete-V1 membership contract; producer-local ordering remains governed by
FR-287 rather than by table position.

A consumer that needs to hold a channel, registration, commit boundary or
recovery relation apart from a protocol subject does so by referencing the
object artifact, not by minting a subject-kind label.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-210-AC-1 | Every member of the table is named as a distinct subject by the requirement cited in its description. | Inspection |
| FR-210-AC-2 | Two references of different subject kinds compare unequal, and no transport record, trace identifier or equal label makes them equal. | Analysis |
| FR-210-AC-3 | A reference with an absent subject kind compares only to absence and receives no default member. | Analysis |
| FR-210-AC-4 | No member of this table is assigned an ordinal, precedence or universal wire spelling; its exact producer-local discriminator remains authority-qualified by FR-287. | Inspection |
| FR-210-AC-5 | A channel, compensation registration, commit boundary, recovery relation, static definition, participant, component or endpoint reference is not represented as a member of this assessable event-subject vocabulary; each remains a distinct typed object or binding rather than a synonym for a listed member. | Inspection |
| FR-210-AC-6 | A receipt is not a delivery, `operation-attempt` is not `compensation-attempt`, `business-effect` is not `compensation-effect`, and a commit observation is not its static boundary. | Analysis |

## Dependencies

- [FR-049](../../functional/foundation/FR-049-bind-native-choreography-syntax.md) names send,
  receive, delivery, operation attempt, effect and compensation.
- [FR-050](../../functional/protocol/FR-050-bind-protocol-instances.md) binds roles, role
  instances, workflow instances, messages and channels to exact declarations.
- [FR-051](../../functional/protocol/FR-051-preserve-communication-identities.md) names
  message, send, receive, delivery, attempt and effect.
- [FR-035](../../functional/foundation/FR-035-bind-ecosystem-subjects.md) names workflow
  instances, role instances, message deliveries, attempts and effects.
- [FR-111](../../functional/observation/FR-111-correlate-related-instances-explicitly.md)
  names workflow instances, deliveries, attempts, effects and receipts.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted complete-V1 subject-vocabulary contract.
