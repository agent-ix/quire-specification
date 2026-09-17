---
id: SR-403
title: "Dependency review of the composed native Quire v1 foundation"
type: SpecReview
analysis: dependency
scope: "Composed native requirements and exact definition dependencies"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-030, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-110, type: reviews }
---
## Summary

The native state and current temporal/protocol definition dependencies form a
directed acyclic graph. The composed selection is nevertheless incomplete:
F's binding/progress candidates do not close their immutable rule dependencies.
One stale temporal prerequisite also needs correction.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | F's 1-draft.1 definitions name A/B/E meanings through relative or unnamed selected contracts without exact immutable rule/dependency closure. Result consumers retaining whatever selections they receive does not select these missing dependencies. Owner: F for successor contracts, A for the composed catalog, with B/E agreement. Publish new exact selections after seam reconciliation; retain the old bytes. | observation-binding.md:15–26; observation-progress.md:6–10,19–24; definitions/README.md, final selection boundary; FR-030/032/110 | missing-requirement |
| FND-002 | medium | FR-090 still calls FR-020 the common profile-composition prerequisite, but FR-020 now reports release capability scope and explicitly transfers definition admission to FR-030. Owner: E with A. Reference the actual definition/binding prerequisite so temporal admission is not ordered behind the downstream release claim or mistaken for its completion. | FR-090:70–75; FR-020, Description and decomposition note; FR-030 | wrong-requirement |

## Classification

Enablement supplies shared contracts, representations or processing foundations;
Feature supplies an author/assessor/consumer-visible result. Every in-scope
local FR/StR/NFR appears once. Cross-cutting constraints apply during dependent work even
when their final conformance observation necessarily follows that work.

| Requirement | Class | Rationale |
| --- | --- | --- |
| StR-002 | Feature | Enterprise composition is the stakeholder outcome. |
| FR-020 | Feature | Reports actual full-v1 or partial capability scope. |
| FR-030 | Enablement | Selects exact definitions, model closure and linked roles. |
| FR-031 | Feature | Dispatches and retains every requested capability result. |
| FR-032 | Enablement | Defines immutable version and evolution boundaries. |
| FR-033 | Enablement | Supplies checked reusable predicate interfaces. |
| FR-034 | Enablement | Supplies cross-family predicate environments. |
| FR-035 | Enablement | Supplies shared request subject and role binding. |
| FR-036 | Enablement | Supplies located tokens from original bytes. |
| FR-037 | Enablement | Supplies the one shared expression syntax tree. |
| FR-038 | Enablement | Supplies exact scoped declaration resolution. |
| FR-039 | Enablement | Supplies exact admitted rational literals. |
| FR-040 | Enablement | Defines explicit state-extension admission. |
| FR-041 | Feature | Evaluates ordered queries and exact aggregates. |
| FR-042 | Enablement | Supplies observation-qualified pre-state expressions. |
| FR-043 | Feature | Evaluates finite graph identity and reachability. |
| FR-044 | Enablement | Supplies exact numeric definedness admission. |
| FR-045 | Enablement | Supplies common control and presence interpretation. |
| FR-046 | Enablement | Validates state invocation and frame inputs. |
| FR-047 | Enablement | Supplies versioned typed diagnostics. |
| FR-048 | Enablement | Connects native temporal syntax to typed environments. |
| FR-049 | Enablement | Connects native choreography syntax to protocol subjects. |
| FR-050 | Enablement | Supplies typed finite protocol subject bindings. |
| FR-051 | Enablement | Supplies distinct message, delivery, attempt and effect correspondence. |
| FR-052 | Enablement | Supplies bounded causal control structure. |
| FR-053 | Enablement | Establishes choice ownership and visibility admission. |
| FR-054 | Enablement | Supplies explicit channel and delivery premises. |
| FR-055 | Feature | Classifies protocol activation with immutable captures. |
| FR-056 | Feature | Assesses effect-dependent compensation registration. |
| FR-057 | Feature | Assesses commit and recovery relations. |
| FR-058 | Feature | Assesses retries, duplicate effects and aggregate recovery. |
| FR-059 | Feature | Assesses finite global conformance. |
| FR-060 | Feature | Reports separate protocol analysis claims. |
| FR-061 | Feature | Produces the orthogonal canonical result. |
| FR-062 | Feature | Projects canonical results to portable verification. |
| FR-090 | Enablement | Selects exact temporal clock/profile interpretation. |
| FR-091 | Feature | Evaluates bounded future obligations. |
| FR-092 | Feature | Evaluates bounded past obligations. |
| FR-093 | Enablement | Supplies temporal activation and capture environments. |
| FR-094 | Feature | Interprets scoped progress, history and closure. |
| FR-095 | Feature | Preserves or refuses native-to-TL correspondence. |
| FR-110 | Enablement | Supplies exact typed observation bindings. |
| FR-111 | Enablement | Supplies explicit relationship correlation. |
| FR-112 | Enablement | Supplies population/window membership and completeness inputs. |
| FR-113 | Enablement | Supplies scoped progress, lateness and restoration authority. |
| FR-114 | Feature | Requires replay/incremental settled-result agreement. |
| FR-115 | Feature | Preserves observation adequacy and result facts. |
| FR-116 | Feature | Preserves consumer outcomes and mapping loss. |
| FR-130 | Enablement | Supplies outer package-reference interpretation. |
| NFR-010 | Enablement | Supplies composed accounting and exhaustion constraints. |
| NFR-020 | Enablement | Supplies protocol processing limits. |
| NFR-021 | Enablement | Supplies protocol reproducibility constraints. |
| NFR-040 | Enablement | Supplies temporal work/history constraints. |
| NFR-050 | Enablement | Supplies bounded observation retention. |
| NFR-051 | Enablement | Supplies immutable replay identity constraints. |
| NFR-052 | Enablement | Supplies executable-language containment. |

## Dependency graph

The edge table is an adjacency representation of the requirement DAG; all
classification rows are vertices, including constraints with no hard
implementation predecessor. An entry means prerequisite → consumer. Edges
come from explicit `depends_on`/Dependencies or an explicitly consumed output.
Architecture references, verification links and mutual owner agreement are not
automatically whole-implementation prerequisites. FR-090's corrected edge is a
proposed disposition of FND-002, not an edit already made to the requirements.

| Prerequisite | Dependent requirement(s) | Grounding |
| --- | --- | --- |
| FR-032 | FR-036 | Lexer consumes the selected edition; evolution preserves old keyword meaning. |
| FR-036 | FR-037 | Parser consumes located tokens. |
| FR-037 | FR-038, FR-039 | Resolution and literal admission consume located syntax. |
| FR-038, FR-044, FR-045 | FR-033 | Predicate admission consumes resolved typed total expressions. |
| FR-033 | FR-034 | Family reference consumes checked predicate identity. |
| FR-030 | FR-040, FR-090 | State/temporal selection consumes exact definitions; temporal edge corrects FND-002. |
| FR-040, FR-044 | FR-041 | Query/aggregate evaluation requires admitted forms and numeric definedness. |
| FR-040 | FR-043 | Graph evaluation requires graph profile and finite typed universe. |
| FR-034 | FR-093 | Explicit temporal capture/environment dependency. |
| FR-034, FR-090, FR-093 | FR-048 | Native temporal binding consumes shared environments and temporal selections. |
| FR-030, FR-032 | FR-130 | Explicit package-reference dependencies. |
| FR-110, FR-111, FR-112 | FR-035 | Request binder consumes selected observation/correlation/scope inputs. |
| FR-035 | FR-050 | Explicit protocol subject-binding dependency. |
| FR-050 | FR-051, FR-052 | Explicit protocol identity/control dependencies. |
| FR-052, FR-033 | FR-053 | Choice admission consumes control predecessors and typed guards. |
| FR-050, FR-051 | FR-054 | Explicit channel/communication dependencies. |
| FR-034, FR-035, FR-090, FR-110 | FR-049 | Native choreography consumes the stated shared binding contracts. |
| FR-090 | FR-091, FR-092 | Explicit future/past profile prerequisite. |
| FR-091, FR-092, FR-113 | FR-094 | Lifecycle interpretation consumes temporal operators and F assertions. |
| FR-051, FR-093, FR-113 | FR-055 | Protocol activation consumes event, anchor and observation authority. |
| FR-051, FR-055 | FR-056 | Explicit compensation-registration dependencies. |
| FR-056, FR-034, FR-094 | FR-057 | Recovery consumes registration, state predicates and temporal/closure meaning. |
| FR-057, FR-112 | FR-058 | Retry and aggregate recovery consume registration outcome and finite membership. |
| FR-052 through FR-058 | FR-059 | Explicit global-conformance prerequisites. |
| FR-031, FR-059 | FR-060 | Explicit claim separation dependencies. |
| FR-031, FR-059, FR-060, FR-091, FR-094 | FR-061 | Explicit canonical-result dependencies. |
| FR-061, external verification FR-007 | FR-062 | Explicit outgoing result-projection dependencies. |
| FR-091, FR-092, FR-094, FR-059 | FR-114 | Replay/incremental agreement compares E/B assessment meanings. |
| FR-061, external verification FR-007 | FR-115 | Explicit canonical and independent-finding result contracts. |
| FR-115 | FR-116 | Consumer handoff preserves the supplied result contribution. |
| FR-030, FR-031 | FR-020 | Explicit capability report dependencies. |
| FR-030, FR-035, FR-041, FR-020 | StR-002 | Declared stakeholder satisfaction relationships, checked after the constituent requirements. |

FR-095 additionally requires the external contract-IR #63 typed-predicate
projection, #64 bridge interface and exact selected TL profiles. These are
bridge prerequisites; current TL's unavailable timestamped/past lowering does
not block native source admission or authorize narrowing v1 meaning. Global
conformance does not require general realizability or local projection.

## Topological order and gates

1. Agree owner contracts, domain/digest/window/result seam dispositions and exact
   successor definitions. Establish edition/evolution,
   lexical/value rules, resource/diagnostic constraints and B's result interface.
   This is enablement specification work, not execution of feature evaluators.
2. Establish shared parsing/resolution, checked predicates, state admission,
   observation/correlation/scope contracts, typed linked-package and reference
   interfaces. Supply every exact dependency before dependent admission.
3. Establish temporal/profile/capture and protocol subject/control/channel
   bindings. Implement selected state, temporal and protocol features only
   after their required enablement is satisfied, with NFRs enforced from entry.
4. Compose lifecycle, recovery, global conformance, canonical results and F's
   replay/consumer preservation. Qualify IT-010/020/021/040/041/050/051 against
   selected real interfaces after implementation exists.
5. Assess capability/conformance evidence for FR-020 and StR-002. A partial
   profile release remains possible without claiming the complete enterprise
   capability set or delaying it on unrelated later research.

No hard-edge cycle is found in this normalized graph. B's result schema and
F/E's binding schema must be agreed before their implementations exchange
results; that interface co-design is not evidence that FR-061's full evaluator
implementation must precede every temporal or observation contract. The only
missing definition closure asserted here is the concrete F selection in
FND-001. SR-401 and SR-406 identify the semantic agreements it must incorporate.

## Verification boundary

All TCs and integration controls are planned. The three-family baseline is not
implemented or qualified by this graph, a definition byte census or structural
Quire validation. FR-070–089 reservations are not a duplicated-file quota. Historical
native-source and earlier temporal/protocol/observation selections remain intact.
