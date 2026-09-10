# Native finite global protocol definition

Profile identity: `quire.protocol.finite-global/v1`; revision: `1-draft.2`.
Language: `ix:native` / `1-draft`. **Proposed, not adopted or implemented.**

## Required definitions and interfaces

| Identity | Revision | Definition artifact | Exact byte digest | Consumed interface |
| --- | --- | --- | --- | --- |
| `quire.state.graph/v1` | `1-draft.1` | [State/query/graph](state-graph.md) | `sha256:1d5ae5e65b60d51fd2b0de653fb232f2ef23bcf977d0fb5ec81da49b75331c65` | Checked shared values, guards, captures and predicate references |
| `quire.temporal.bounded-facet/v1` | `1-draft.1` | [Temporal meaning](temporal-common.md) | `sha256:0f03afe04bb60062d7ec5bb6051ce5e7baf3d507ad14441b2352867c5fd06d61` | Explicit activation, bounded clock/history/progress and temporal-reference contract |

The source root admits protocol declarations. State and temporal declarations
select their own profiles; an imported declaration retains that exact selection.
Each authored timed obligation additionally selects its concrete temporal
clock/boundary profile through its `using` alias or referenced temporal clause.
The common temporal facet alone supplies no default clock interpretation.

## Admitted forms and obligations

Admit finite typed roles/relationships/channels; distinct send, receive,
operation-attempt, effect and domain-event nodes; checks; explicit causal
sequence; role-owned labeled choice with total visible exclusive/exhaustive
guards; bounded parallel branches with explicit all-branch join; and bounded
repetition with progress and an explicit exhausted path. No implicit race,
cancellation, message order, reliable delivery or exactly-once business effect
is assumed.

Admit exact related-instance annotations, finite snapshot/window inputs, awaited
events with inclusive deadlines, compensation templates with separate forward
registration and eligible activation captures, causal retries, achieved recovery
relations, commit boundaries and explicit workflow closure. Definite value
availability follows the native causal graph; a sibling or optional branch does
not supply a value before its required join/path. Repeated delivery, invocation
attempt and successful effect remain distinct identities.

All value conditions use the shared expression rules and their supplied anchors;
they cannot inspect pending monitor status, retag captures, infer population
membership or perform business actions. Forward success creates a modeled
compensation obligation, not an external compensator. A successful refund call
alone cannot establish achieved recovery or complete global conformance.

Global conformance checks the exact declared causal, channel, visibility,
correlation, activation, recovery and closure premises. All permitted branch
interleavings remain admissible. Open/incomplete histories cannot establish
complete global success. Finite replay and settled incremental results must
agree for equal admitted inputs/premises.

Projection, realizability, refinement and composition remain separately requested
claims with their own premises and backend dispositions; a finite passing trace
does not prove them. No result grants testing adequacy or evidence sufficiency
from observed conformance. C's assurance completion is not a semantic dependency.

## Selected protocol rules

All files below are selected at repository revision `15beef776f632310e77a18c838bd7f7a4a1088a1`.
The native surface fixes concrete syntax/binding details; B's requirements own
protocol/conformance/result meaning. A disagreement is an integration defect,
not permission to choose a favorable interpretation or overwrite either owner.

| Rule file | Exact byte digest |
| --- | --- |
| [FR-049-bind-native-choreography-syntax.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-049-bind-native-choreography-syntax.md) | `sha256:c36a50b85d1f347d7666c1b3abcb1a3dd5a00a9a237737a50517014338809b09` |
| [FR-050-bind-protocol-instances.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-050-bind-protocol-instances.md) | `sha256:ab7b116092f81a6210661a1a6b5d51dc0bbcebebf21fc6c317aaf855b8fd419d` |
| [FR-051-preserve-communication-identities.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-051-preserve-communication-identities.md) | `sha256:ba73423b9430997acd4b616a8e578d8f1cc657949858ec8089025d3d4c41429c` |
| [FR-052-represent-bounded-control.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-052-represent-bounded-control.md) | `sha256:07247cd0606733e729baec76898baba01de137da72062a65d7fa2ffa0edc467d` |
| [FR-053-enforce-choice-visibility.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-053-enforce-choice-visibility.md) | `sha256:d64e52b656d9baaced463478172b339498ccad685247c9c74ae8f7dc113f0b91` |
| [FR-054-bind-channel-premises.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-054-bind-channel-premises.md) | `sha256:5fb5fb6b33e9b081355720885e4ffb8e864054e24e7ccab2f10cd1d980f7ee26` |
| [FR-055-activate-protocol-obligations.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-055-activate-protocol-obligations.md) | `sha256:263ef764bd1abc25a4c4e8da5cf6d7381b9c799963fd9a73c5cb0c5e95d54518` |
| [FR-056-register-compensation.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-056-register-compensation.md) | `sha256:72266a7fbf97a5b0f85695000882544ad5fc1e10a26c6c9bf60e164ec94706a6` |
| [FR-057-enforce-commit-recovery.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-057-enforce-commit-recovery.md) | `sha256:9562108f46609bf3338cbc2661e90cfbf09f7f2a872c6c8af656ec4a7427f91e` |
| [FR-058-preserve-retry-and-partial-recovery.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-058-preserve-retry-and-partial-recovery.md) | `sha256:98f2ebb1d9ca389d8ae32ff67c02bc89480ef0a2da7ee09115f1dd5824cf27a3` |
| [FR-059-assess-finite-global-conformance.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-059-assess-finite-global-conformance.md) | `sha256:d3ebc63fd4833306bbaa4cf91dd5feaf6980b965bcb0910888d0ca55723a1a1c` |
| [FR-060-separate-protocol-claims.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-060-separate-protocol-claims.md) | `sha256:ec3894bbf79f985e8308a70ce3e751887af56dbe68d8dd167a884623ead275c5` |
| [FR-061-report-orthogonal-results.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/functional/FR-061-report-orthogonal-results.md) | `sha256:717ee2a2c3615026af3c1789dd1c28fc62d90189ce0343d2d561776b9543a6e3` |
| [NFR-020-bound-protocol-processing.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/non-functional/NFR-020-bound-protocol-processing.md) | `sha256:d28c5c3ef7cb19fff3da8313305f7e3ebcb4e01d61ec8a0001f7ef2145f4ae29` |
| [NFR-021-reproduce-protocol-results.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/spec/non-functional/NFR-021-reproduce-protocol-results.md) | `sha256:27d1ba16dbe5fcddbd6478b4cb732b58f0f2ed0e6fefc6212dcb13cbc11cece4` |
| [choreography-surface.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/proposals/quire-v1/choreography-surface.md) | `sha256:04d7f34a082428a31016c592c6e22f359b0a9c7965e6d384e296b2767840877e` |
| [protocol-contract.md](https://github.com/agent-ix/quire-specification/blob/15beef776f632310e77a18c838bd7f7a4a1088a1/proposals/quire-v1/protocol-contract.md) | `sha256:d28f8ab992bbe313384a9183797a91b7c58f39fce7937c662494cd575204d01d` |

## Result settlement and compatibility

This revision selects B's reviewed separation of assessment execution, subject
progress/closure, truth, settlement basis and exact decision support. A completed
assessment may settle an obligation from a decisive witness or counterexample
while the decision scope and surrounding execution remain open, only when the
selected temporal meaning preserves that truth under every admitted continuation.
Missing support makes that decision unavailable; missing unrelated facts remain
visible without erasing independently settled truth. A later contradiction
creates a linked superseding or invalidating result with a corrected input
identity and preserves the prior result bytes.

This does not weaken FR-059's complete global conformance rule. A satisfied
payment obligation cannot certify the still-open workflow, an unknown population
or an unsupported independently requested claim.

The prior [1-draft.1 candidate](protocol-finite.md) and its exact pinned rule
bytes remain unchanged. The same identity with revision 1-draft.1 cannot alias
these revised result semantics. A reader selecting that revision retains its
original contract; changing to this revision changes the linked subject.

B's contribution was reviewed at `cb2748d` in SR-200 through SR-207. That family
review does not review this new definition or the composed baseline. F's consumer
must preserve the added result dimensions, D's model/configuration contribution
remains outstanding, and the selected composed review still precedes adoption.

