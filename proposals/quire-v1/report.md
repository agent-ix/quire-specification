# Quire full-language redesign — research and recommendation report

**Status: research draft, saved 2026-09-09.** This report and its plan are the
requested output. They do not authorize implementation. The capability matrix,
architecture and detailed profile specifications must be made consistent and
reviewed before the report is presented as the completed recommendation.

## Finding

The custom-language decision was not the scope error. The finite-state
development profile became the apparent official language boundary. The master
requirements explicitly excluded temporal/protocol/message semantics even
though the program architecture describes executable temporal and distributed
obligations. Native temporal source-to-TL correspondence was missing, and native
choreography had no specification ticket. Existing state code remains a useful
foundation; it does not establish the wider language's delivery.

The owner now requires state, temporal and protocol/choreography in Quire v1.
The later clarification also asks for a reasoned evaluation of inherited scope
choices against the broader OCL/FRET/KerML/SysML feature families. The complete
language roadmap must not be bounded by today's evaluator. Nor does interest in
those capabilities require implementing four upstream languages wholesale.

## Recommendation being developed

Specify one native language with common identity, types, source correspondence,
observation and composition rules, and typed state/temporal/protocol families.
Separate the language's meaning from the capabilities of each evaluator,
analyzer and exporter. Choose a coherent bounded v1 set using the planned state,
timed-refund and branching compensated workflows. Retain explicit later feature
families and their future interface needs.

Both executable assessment and incremental monitoring belong in the design.
Qualification-oriented use additionally needs exact subject/tool/binding
provenance and conformance evidence. Neither a passing trace nor parser-accepted
generated text is itself qualification of the system. Global replay, protocol
conformance, local projection and realizability remain different requested claims.

## Saved evidence and draft specifications

- [Capability decisions and source inventory](capabilities.md): program plans,
  owner decisions, observed component revisions, primary external sources,
  v1/later/refuse choices and a feature-family scope audit.
- [Composed architecture](../../spec/assurance/AD-001-composed-native-language.md):
  parsing, typing, representations, evaluation, monitoring, evidence and ownership.
- [Profile composition requirement](../../spec/functional/FR-020-compose-native-profiles.md):
  exact definitions, cross-profile references, backend refusals and migration.
- [Stakeholder need](../../spec/stakeholder/StR-002-composed-language.md) and
  [workflow story](../../spec/usecase/US-005-author-composed-workflows.md).
- Corrected [master scope](../../spec/spec.md), preserving historical state
  definition and fixture bytes.

Research consumed the maintained architecture, executive summary, technical
brief, evaluation plan, implementation plan and source-evidence ledger in
`quire-research` at `3df57c91773d8d64b1431649aeafc422a87d6578`. Original discovery
documents supplied examples and hypotheses, not current implementation claims.
The exact source links and component limits are retained in the capability matrix.

## Plan to finish the recommendation

| Stage | Concrete output | Owning coordination |
| --- | --- | --- |
| Capability and scope audit | Complete feature-family decisions, dependencies, effort drivers and consequences of deferral, including execution/monitoring/qualification use | Standard #10 and #9; A leads |
| Consistent semantic baseline | Common profile/type/source/result rules; temporal clocks/history/observations; choreography roles/messages/order/choices/concurrency/iteration/compensation; explicit state migration | Standard #10, FS01–05 and choreography #12 |
| Concrete conformance scenarios | Healthy, violating, untriggered, incomplete and refused cases for all required families and their composition | A with B/C/TL contracts; existing QA01 and observation/protocol owners |
| Review the baseline | Owner-selected QUOIN base plus seven analyses, with actual findings and dispositions; no intermediate review churn | A / standard #10 |
| Derive implementation scope | Bounded tickets tied to reviewed requirements, real interface dependencies and expected acceptance evidence; no implementation begins | Existing native/compiler, IR/TL, observation and protocol workstreams |
| Final report and plan | Recommended v1 set, later roadmap, architectural decisions/tradeoffs, work reuse, required changes, staged tickets and remaining owner choices | Report under standard #10 |

Created [choreography specification #12](https://github.com/agent-ix/quire-specification/issues/12).
[Temporal specification #5](https://github.com/agent-ix/quire-specification/issues/5)
and [native-to-TL bridge #64](https://github.com/agent-ix/quire-contract-ir/issues/64)
already exist and will be reused. Research PT01/PT02 and OB01–03 retain their
implementation routing. Implementation tickets are finalized after the coherent
specification baseline, as the owner requested; new repositories are not assumed.

## Work still required before the final report

The initial matrix is a proposal, not the final v1 decision. Broad deferrals must
be tested against product needs and future extension cost. Detailed native
temporal/protocol semantics, exact profile identities, event/clock contracts,
bounded collection extensions, state-graph admission, conformance cases,
cross-stream dispositions and the final ticket dependency plan remain unfinished.
In particular, the original minimum-only ConfigVersion and FCD #95 presence
decisions must stay visible. No existing review or passing state test discharges
these new obligations.

No grammar, runtime, backend, monitor, executable test or hosted workflow has
been implemented or run as part of this redesign work.
