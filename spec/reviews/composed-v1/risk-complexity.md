---
id: SR-405
title: "Composed native v1 risk and complexity review"
type: SpecReview
analysis: risk-complexity
scope: "proposals/quire-v1; AD-001; StR-002; FR-020, FR-030–062, FR-090–095, FR-110–116, FR-130; NFR-010/020/021/040/050/051/052"
review_set: all
evaluated_revision: "788b161bd1b7fd5d632cc42c949f497914df3ce6"
review_date: "2026-09-10"
---

## Summary

The architecture separates native source, typed family semantics, runtime inputs
and evidence ownership coherently. The highest delivery risks are the shared
result/progress boundary, immutable dependency selection, exact arithmetic,
causal compensation and bounded incremental state. These call for staged work
inside the existing roadmap, not a language rewrite or an assurance campaign
before implementation. The register scores the 48 local FRs, seven NFRs and one StR
in this review. Historical obligations remain
dependencies, not newly scored implementation scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Resolve the concrete truth/progress and window-selection disagreements before accepting the affected shared contract. Implementing different interpretations in three consumers would multiply rework. | [SR-401](failure-domain.md), [SR-406](scope-boundary.md); FR-030, FR-048/049, FR-061, FR-094, FR-114 |
| FND-002 | medium | Stage implementation by independently checkable contracts within existing tickets. Bounded syntax alone does not establish bounded aggregate work, sound definedness, causal recovery or replay equivalence. | FR-036/043/044, FR-052/056–059, FR-091–094, NFR-010/020/040/050 |
| FND-003 | medium | Freeze exact dependency closures only after the affected semantic corrections; preserve historical bytes. Selection identifiers without immutable reachable rules leave integrations exposed to drift. | [SR-403](dependency.md); FR-032, FR-110/113, FR-115/116 |

## Risk register

Technical risk measures implementation difficulty or consequences, not an
assertion that code is defective. Volatility measures likely contract change.
Grouped rows assign both scores and the named mitigation to every listed ID.
High scores are planning inputs; they do not impose a new blanket acceptance
gate.

| Req | Tech Risk | Volatility | Drivers | Mitigation |
| --- | --- | --- | --- | --- |
| StR-002 | High | Medium | First composed enterprise language; multiple semantic owners | A uses the common order/payment/shipment/refund case as the integration slice under standard #12; preserve each family's separate outcome. |
| FR-020 | Low | Low | Capability reporting rather than new evaluator semantics | A keeps release capability inventory separate from profile selection and backend support. |
| FR-030, FR-130 | High | Medium | Exact dependency linking and outer/inner identity boundaries | A resolves stage prerequisites and closes package selections in compiler #35; test independent child refusal and exact-dependency substitution. |
| FR-031 | High | Medium | Partial execution and cross-family result prerequisites | A/B exercise dependency-closed independent requests and failed prerequisites before composing execution consumers. |
| FR-032 | High | Medium | Evolution must preserve previously selected meaning | A adds successor selections and historical controls in compiler #35; never rewrite shared definition bytes. |
| FR-033, FR-034 | High | Medium | Shared total predicates across different environments | A establishes one checked predicate interface in compiler #36; E/B consume explicit argument/anchor/capture bindings. |
| FR-035 | High | Medium | Related instances, finite membership and multiple identity authorities | A/F bind one explicit enterprise subject at a time and discriminate equal labels with different identities. |
| FR-036 | Medium | Low | Lexer, parser, spans and whole-source admission | A retains the existing lexer and structured parser; use grammar/precedence/span controls in compiler #35 instead of a parser rewrite. |
| FR-037 | Medium | Low | Shared expression precedence, complete parsing and family dispatch | A uses one token cursor and expression parser in compiler #35; grammar recognition stays separate from profile admission. |
| FR-038 | Medium | Low | Lexical scope, acyclic calls and exact declaration resolution | A uses explicit symbol scopes and shadowing/cycle controls in compiler #36. |
| FR-039 | High | Low | Exact rational normalization before authored bounds | A implements exact normalization with an independent arithmetic oracle and reducible-intermediate controls in compiler #36. |
| FR-040 | Medium | Medium | Several explicit state extensions and historic restrictions | A keeps core/query/graph admission as separate selected facets. |
| FR-041 | High | Low | Occurrence-sensitive aggregates and whole-prefix definedness | A bounds query work and compares small finite examples to an independent oracle in compiler #36. |
| FR-042 | High | Low | Pre-state selection without changing immutable captures | A tests cross-snapshot alias/capture provenance and eligible composite reads in compiler #36. |
| FR-043 | High | Low | Cycles, positive-length reachability and incomplete universes | A separates identity from traversal state in compiler #37 and uses finite reference-graph controls. |
| FR-044 | High | Low | Conservative but sound numeric proof, exact units and range checks | A separates proved error, unproved definedness and resource exhaustion; start with the specified exact domain in compiler #36. |
| FR-045 | High | Low | Branch-sensitive facts and qualified optional identities | A tests skipped execution separately from full static admission; guard facts carry their precise value/observation identity. |
| FR-046 | High | Medium | Complete pre/post populations and frame correspondence | A validates invocation inputs independently of predicate evaluation; retain the original ConfigVersion admission controls. |
| FR-047 | Medium | Low | Stable typed diagnostic catalog and located causes | A uses existing diagnostic/response boundaries; mutation of human wording must not change classification. |
| FR-048 | High | Medium | Shared value grammar inside temporal formulas, typed captures | A/E implement source linking before clock-bound execution in compiler #38, with false-extension constant/valuation controls. |
| FR-049 | High | Medium | Causal value availability, joins, iteration and compensation scopes | A/B implement typed native protocol linking in compiler #39 against B's exact representation; no string-based protocol parsing. |
| FR-050, FR-051 | High | Low | Cross-system subjects; delivery is not business effect | B/F use separate typed identities and wrong-order/duplicate-receipt controls under research #38/#47. |
| FR-052, FR-053, FR-054 | High | Medium | Concurrent causal structure, visible choice and channel assumptions | B starts with bounded global conformance and explicit premises in research #47; defer general realizability without dropping visibility information. |
| FR-055, FR-056 | High | Low | Trigger identity and two distinct compensation capture instants | B/F independently test registration, first activation and duplicate triggers under research #48. |
| FR-057, FR-058 | High | Medium | Commit, retry, partial recovery and actual effect correspondence | B specifies finite outcome tables before implementing research #48; use partial-refund and already-committed controls. |
| FR-059 | High | Medium | Global conformance over concurrent observations and incomplete inputs | B composes validated identity/control/result boundaries, preserving decisive support and separate global closure premises. |
| FR-060 | Medium | Low | Claim classification can exceed actual backend capability | B preserves distinct conformance, projection and realizability requests with explicit unsupported outcomes. |
| FR-061 | High | Medium | Orthogonal result axes and legal combinations shared by all consumers | B's canonical result contract owns the combination table; A/E/F add decisive-open and unsettled controls before consumer implementation. |
| FR-062 | High | Medium | Outgoing verification projection may lose semantic distinctions | B reuses verification #13/#22/#23 boundaries and preserves canonical results, exact mapping selection and explicit loss findings. |
| FR-090 | High | Medium | Three clock families and independent profile selection | E/F resolve the observation-range mapping for each selected clock, then bind exact definitions; no implicit UTC conversion. |
| FR-091, FR-092 | High | Low | Bounded future/past semantics and boundary extension | E uses independent finite truth tables under standard #5 and IR #63/#64; current TL limitations remain explicit. |
| FR-093 | High | Low | Immutable per-trigger captures and later valuations | E/F distinguish equal values from equal trigger identities and preserve activation anchors in compiler #38 integration. |
| FR-094 | High | Medium | Two closure axes, silent progress and decisive settlement | E/F reconcile authority and exact support with B results before incremental implementation; include quiet deadlines and late contradiction controls. |
| FR-095 | High | Medium | Backend correspondence across richer native meaning | E scopes each bridge to admitted meanings and proves specified vectors under IR #64; unsupported timestamped/past mappings retain native identity. |
| FR-110, FR-111 | High | Medium | Typed observations and explicit related-instance correlation | F closes immutable binding selections in research #44 and reject equal-label or trace-ID substitutions. |
| FR-112, FR-113 | High | Medium | Membership, clock authority, incomplete data and late records | F/E resolve the window/progress boundary and use declared finite populations under research #45; absence of data cannot invent authority. |
| FR-114 | High | Medium | Offline/incremental equivalence with bounded retained state | F corrects the open-prefix veto, then compares exact-support result fields over identical admitted histories in research #46. |
| FR-115, FR-116 | High | Medium | Consumer preservation, immutable history and output losses | F consumes B's exact result combinations; test one-axis and one-dependency mutations in existing observation/consumer tickets. |
| NFR-010 | High | Low | Aggregate source, proof, query and graph work | A charges before expansion and exercises zero/exact/insufficient limits per boundary; compiler #35–37. |
| NFR-020 | High | Low | Concurrent instances, protocol structures and retained outputs | B uses independently counted limits and forbids business side effects in research #47/#48. |
| NFR-021 | High | Low | Equivalent causal presentations must reproduce semantic results | B distinguishes sets from sequences and permutes independent events with exact provenance retained. |
| NFR-040 | High | Low | Checked horizon/history arithmetic and live captures | E/F test nested bounds, retained obligations and exhaustion independently; compiler #38 and research #46. |
| NFR-050 | High | Low | Event, active-key, history and population limits are independent | F refines existing TC-142/144 boundary procedures and measures retained state; see SR-404. |
| NFR-051 | High | Low | Exact replay provenance across modes and revisions | F binds provenance-omission controls to the metric, rather than comparing only dispositions; see SR-404. |
| NFR-052 | Medium | Low | Executable dependency containment under the owner policy | F inventories owned runtime/qualification paths using the existing Rust policy; retained Quoin does not authorize expansion. |

## Top hazards

1. **Different meanings at the same interface.** Resolve SR-401's open-prefix
   conflict and SR-406's clock/range mapping before accepting affected
   selections. F is a delivered input; these are bounded corrections to its
   interface.
2. **Unsound arithmetic or provenance.** FR-039/042/044/045 need independent
   oracle and mutation controls in the later implementation slices. Do not
   replace conservative refusal with a successful proof or retag a capture.
3. **False recovery in concurrent choreography.** FR-051/056–059 require distinct
   message, attempt, effect, registration and activation identities; use the
   common split-shipment/partial-refund cases before broad integration.
4. **Live state silently lost.** NFR-010/020/040/050 need separately accounted
   limits. Finite intervals and bounded loops do not bound active instances or
   all aggregate work.
5. **Definition drift after review.** Close F's reachable dependency rules and
   retain all old candidate bytes, then derive successor digests once per
   coherent correction. See SR-403.

## Failure-domain gaps and sequencing

[SR-401](failure-domain.md) records the concrete open-prefix defect; [SR-406](scope-boundary.md) covers observation-range authority.
The score register does not replace those findings or add new feature scope.

Resolve these spec corrections and the evidence-plan gaps in the existing
baseline PR. Then refine compiler #35 (package/selection), #36 (predicates and
queries), #37 (graphs), #38 (temporal integration) and #39 (choreography), with
B/E/F's already assigned representation and observation tickets consuming the
accepted interfaces. Independent accepted work may proceed. C's engineering
assurance campaign, hosted CI, full qualification and general local realizability
are not blanket gates. This review performs no implementation or heavy testing.
