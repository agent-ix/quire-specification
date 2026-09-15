# Complete V1 program routing audit

Status: Task-005 live-graph reconciliation at 2026-09-15.

This audit routes every row in the 175-row atomic inventory through the current
GitHub program graph. It separates a primary delivery epic from downstream
consumer and qualification children. The primary epic is accountable for
closing the row end to end; it does not acquire another repository's semantic
authority.

The audit read live issue state and bodies rather than trusting the maintained
research snapshots. Exact current code and specification evidence remains in
`docs/v1-capability-inventory.md` and
`docs/v1-existing-spec-coverage.md`.

## Baseline live graph findings before repair

1. Research #36 is open and has the correct cross-repository program skeleton,
   but its exit text still permits reviewed defer/reject dispositions.
2. QSpec #1 is open and accurately credits the current standard, Kani,
   observation and bounded Protocol work, but still permits closure when work is
   explicitly deferred. QSpec #9 still treats `Deferred` as a valid ledger
   state and points readers first to the older roll-up.
3. QSL #1 closed on 2026-09-13 after its bounded L1-L6 campaign. The current
   compiler revision still has all fifteen hard-refused spellings enumerated by
   the inventory, and complete V1 has missing type, expression, tooling and
   runtime work. Its closed child evidence remains valid; the parent must reopen
   for the broader accepted scope.
4. Research #65, #66 and #67 are open but expressly classify pure
   functions/collections, model/numeric domains and dynamic choreography as
   future non-V1 research. Those are now Required inventory rows.
5. Protocol #14 is open and its completed leaves accurately establish the
   bounded consumer architecture. Its current outcome does not own dynamic
   participants, symbolic queues, general loop obligations, projection,
   realizability, assume-guarantee composition, refinement, synthesis or the
   authorized workflow-adapter path.
6. QObs #1 is correctly closed for its bounded authority/topology layer.
   Research #37 stays open and is the parent for new uncertainty, partial
   valuation, repair and runtime-monitoring children; the completed QObs epic is
   not rewritten.
7. Contract IR #69 and Codegen #44 are correctly closed for the bounded Kani
   profile. Contract IR #2 remains the lowering parent and #52 remains the
   bridge/output-mapping parent. Their completed children remain evidence; new
   language families extend the parents.
8. Verification #13 is open and has no children. It is the existing owner to
   expand for relational/hyperproperty analysis, model testing, verification
   plans, replayable counterexamples and cross-method qualification.
9. TL Syntax #1 and its active child epics remain the temporal ecosystem owner.
   Their current active agent lane is not modified by this planning branch.
10. Research #39 and #40 remain the integration and qualification parents.
    Setup #28 still has open integration/conformance repository children #34
    and #35, so no duplicate repository or top-level epic is justified.
11. Filament remains an external authoritative producer. No FCD ticket or code
    is amended by this campaign; model-consumer correspondence is owned by
    QSpec/QSL and exact producer revisions are dependencies.
12. Quoin #519 correctly remains behind Rust-port epic #373. The architecture
    may define its typed consumer contract, but Task-009 cannot activate Quoin
    work before that dependency lands.

## Live repairs applied

The audit immediately repaired the parent-level tracking that otherwise made
the plan ambiguous:

- Research #36, #37-#43, QSpec #1/#9, Protocol #14, Verification #13 and
  Contract IR #2/#52 now carry complete-V1 reconciliation blocks that supersede
  their narrower or deferral-based closure language.
- QSL #1 is reopened and retitled as the complete compiler, reference-runtime
  and tooling epic. Its completed L1-L6 children remain closed evidence.
- Research #65/#66/#67 are retitled and rewritten as required delivery epics
  for complete pure/collection, numeric/model and dynamic-protocol scope.
- Every repaired issue links its completion to the atomic inventory and forbids
  closing a required capability through deferral or a failed implementation
  approach.

The detailed child issues remain intentionally uncreated until the comprehensive
architecture and specification are accepted. Task-009 creates them from exact
requirements rather than guessing interfaces during this tracking repair.

## Parent amendment manifest

The following are repairs to existing owners, not replacement roadmaps:

| Issue | Applied amendment | Closure rule |
| --- | --- | --- |
| Research #36 | Replace defer/reject closure and the old finite V1 list with all 175 atomic inventory rows; name #63 as the planning gate and #65/#66/#67 as required delivery epics. | Close only after Task-010 proves every row specified, implemented where executable, verified, integrated and qualified, or delivered as an exact tested Integrity refusal. |
| QSpec #1 | Make the atomic inventory the scope authority; remove explicit-deferral closure; retain all completed standard/reference evidence; require comprehensive Task-006/007/008 architecture and specification acceptance. | Close with QSpec #9 only after the three coverage audits are 100%. |
| QSpec #9 | Replace Deferred/Missing roll-up states with the inventory's independent Spec/Impl/Ver/Int/Qual states; link each incomplete row to its live primary epic and children. | Remain open and update after every merge; close with QSpec #1. |
| QSL #1 | Reopen the parent without reopening its completed children; expand it from the bounded L1-L6 slice to the complete compiler, reference-runtime and native tooling surface; link the Task-004 structural migration. | Close only after every QSL-owned inventory row and downstream parity matrix is complete. |
| Research #65 | Promote from future study to **required V1 delivery epic** for pure functions, checked recursion, set/bag/ordered-set semantics, collection conversions/queries, recursive bounded values and reusable libraries. | Children must cover QSpec semantics, QSL syntax/check/eval, IR/backend representation, negative bounds, integration and qualification. |
| Research #66 | Promote from future study to **required V1 delivery epic** for rational/decimal/float/integer semantics, units, tuples/records/equality, inheritance, conformance, redefinition, dispatch and systems-model references. | Children must cover the same complete producer-consumer-backend chain without editing FCD authority. |
| Research #67 | Promote from future study to **required V1 delivery epic** for dynamic roles, bounded/symbolic queues, general loop obligations and complete population/history traversal. | Children must cover QSpec, QSL, Protocol, Observation, analysis and qualification; no unbounded claim may be approximated. |
| Research #37 | Expand observation/replay scope to clock uncertainty, partial valuation, revisioned repair and incremental monitoring while retaining closed OB01-03/QObs #1 evidence. | New children close the additional rows; historical bounded work remains closed. |
| Protocol #14 | Preserve the accepted bounded architecture, supersede its flat-layout ADR as Task-004 requires, and extend it with the complete protocol, analysis and authorized workflow-adapter interfaces allocated by Task-006. | Current bounded children remain credited; the parent cannot close until all Protocol-owned rows and their QSL/QObs/verification seams qualify. |
| Verification #13 | Expand the existing production-qualification epic to model testing, relational/hyperproperty methods, verification-plan DSL, counterexample replay/shrink, synthesis result classification and complete cross-method results. | Every method has explicit domain, precondition, bounds, non-conclusive outcomes and independent replay/qualification evidence. |
| Contract IR #2/#52 | Extend the existing typed IR/lowering and mapping parents to every Task-006-selected language family; retain the closed bounded-Kani and mapping-foundation children as exact evidence. | Every source construct is supported exactly or refused per requested backend without approximation or sibling-result loss. |
| Research #39/#40 | Expand existing integration and qualification parents to the complete package, API, CLI, WASM, analyzer, generator and conformance corpus; use setup children #34/#35 rather than creating duplicate repositories. | Exact revision/configuration compatibility and independent release claims cover all 175 rows. |
| Research #41/#42/#43 | Convert model testing, relational analysis, simulation/hybrid analysis and bounded synthesis from optional research disposition into required V1 delivery tracks with scoped soundness constraints. | Experiments may reject an approach, but the capability remains open and must receive another sound implementation route; rejection cannot close the capability. |

No new top-level epic is justified. The existing program already has coherent
parents for all eight inventory families. Task-009 creates bounded child tickets
under these repaired parents after the comprehensive specification is accepted.

At this checkpoint, zero rows are satisfied end to end because all 175 still
have missing qualification evidence. Twenty-one rows retain completed direct
implementation and verification leaves but need integration/qualification
children. The other 154 have an existing primary parent and need one or more
semantic delivery children. No row lacks a primary epic, and no new top-level
epic is required.

## Atomic inventory-to-epic routing

`Primary epic` is unique per row. `Parent action` repairs the current graph.
`Child manifest` states what Task-009 must materialize; it never treats a
completed bounded leaf as proof of the broader row.

| Capability | Primary epic | Parent action | Graph classification | Required child disposition |
| --- | --- | --- | --- | --- |
| V1-SRC-001 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-002 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-003 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-004 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-005 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-006 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-007 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-008 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-009 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-010 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-011 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-012 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-013 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-014 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-SRC-015 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-001 | QSpec #1 | Amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-002 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TYPE-003 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TYPE-004 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-005 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-006 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-007 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-008 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-009 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-010 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-011 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-012 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-013 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-014 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TYPE-015 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-016 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-017 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-018 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-019 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-020 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-021 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-022 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TYPE-023 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-024 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TYPE-025 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-026 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-027 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-028 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-029 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-030 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TYPE-031 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-001 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-002 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-003 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-004 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-005 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-006 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-007 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-008 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-009 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-010 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-011 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-012 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-013 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-014 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-015 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-016 | Research #66 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-017 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-018 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-019 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-020 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EXPR-021 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-022 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-023 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-024 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-025 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EXPR-026 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TEMP-001 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-002 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-003 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-004 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-005 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-006 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-007 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-008 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-009 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-010 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-011 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-012 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-013 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-014 | Research #37 | Extend existing observation/replay epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TEMP-015 | Research #37 | Extend existing observation/replay epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-TEMP-016 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-017 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-018 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-019 | Contract IR #52 | Amend existing bridge epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-020 | Contract IR #52 | Amend existing bridge epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-021 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TEMP-022 | TL Syntax #1 | Extend existing temporal ecosystem epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-001 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-002 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-003 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-004 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-005 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-006 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-007 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-008 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-009 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-010 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-011 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-012 | Research #67 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-013 | Research #67 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-014 | Research #67 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-015 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-016 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-017 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-018 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-019 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-020 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-021 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-022 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-023 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-024 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-025 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-026 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-027 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-028 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-029 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-030 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-PROT-031 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-001 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-RUN-002 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-003 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-004 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-005 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-006 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-007 | Research #37 | Extend existing observation/replay epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-RUN-008 | Research #37 | Extend existing observation/replay epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-009 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-010 | QSL #1 | Reopen and amend existing epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-RUN-011 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-012 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-013 | Research #41 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-014 | Research #41 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-015 | Research #42 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-016 | Research #42 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-017 | Research #42 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-018 | Research #43 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-019 | Research #43 | Promote existing research epic to required delivery | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-RUN-020 | Protocol #14 | Amend existing bounded epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-001 | Contract IR #2 | Amend existing lowering epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-002 | Contract IR #2 | Amend existing lowering epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-003 | Contract IR #2 | Amend existing lowering epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-BACK-004 | Contract IR #2 | Amend existing lowering epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-005 | Contract IR #2 | Amend existing lowering epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-006 | Contract IR #2 | Amend existing lowering epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-007 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-008 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-009 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-010 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-011 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-012 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-013 | Contract IR #52 | Extend existing mapping epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-BACK-014 | Contract IR #2 | Amend existing lowering epic | Delivered leaf retained; completion children missing | Retain delivered leaf; add missing integration/qualification children. |
| V1-EVID-001 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EVID-002 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EVID-003 | Research #40 | Extend existing qualification epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EVID-004 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EVID-005 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-EVID-006 | Verification #13 | Amend existing verification expansion epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-001 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-002 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-003 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-004 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-005 | Research #39 | Extend existing integration epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-006 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-007 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-008 | Research #65 | Promote existing ticket to required delivery epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-009 | QSL #1 | Reopen and amend existing epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |
| V1-TOOL-010 | Research #39 | Extend existing integration epic | Existing parent; delivery children missing | Add complete semantic, verification, integration and qualification children after Task-008. |

## Reconciliation invariants

- Every atomic inventory ID appears exactly once in the routing table.
- Every row has exactly one primary existing epic.
- No row is classified as future, deferred, research-only, unowned or
  unticketed.
- No completed leaf is reopened merely because its parent gains scope.
- An unsuccessful research or backend experiment returns a typed result and
  triggers an alternative child; it does not remove the capability.
- Task-009 may split child work by repository and dependency, but may not change
  the primary epic without updating this map, the atomic inventory, QSpec #9
  and the maintained research graph in the same reconciliation.
- Product implementation remains blocked through Task-010.
