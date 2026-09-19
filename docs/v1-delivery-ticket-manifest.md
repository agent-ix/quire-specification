# Complete V1 implementation ticket manifest

Status: accepted by the Task-010 coverage gate on 2026-09-15.

This manifest translates the accepted complete-V1 specification at commit
`8d0fbad` into five autonomous agent lanes under the existing program epics.
The [atomic inventory](v1-capability-inventory.md) remains the scope authority;
this file owns implementation-ticket allocation. Product implementation starts
only after Plan-008 Task-010 proves this map complete.

The passing mechanical evidence is recorded in the
[campaign coverage audit](v1-campaign-coverage-audit.md).

## Autonomous ownership lanes

| Lane | Agent | Repository/subsystem ownership | Autonomous campaign |
| --- | --- | --- | --- |
| A | Agent-A | `quire-spec-language`, `quire-wasm`; source/package, semantic kernel, model binding, reference runtime and native tooling | QSL #116–123 and WASM #6 |
| B | Agent-B | `tl-syntax`, `tl-parse`, `tl-mltl`, `quire-protocol`; temporal language/evaluation and Protocol | existing Protocol #8/#12/#9/#10/#63, TL #72–74, tl-parse #40, tl-mltl #68, Protocol #78/#83/#79–82 |
| C | Agent-C | `quire-observation`, then new `quire-integration`; uncertain/revisioned evidence, composed runtime, workflow adapter and cross-repo integration | QObs #22–26, Research #34/#74–78 |
| D | Agent-D | `quire-verification`, `quire-analyze`; model testing, relational/hyperproperty, method plans, hybrid/synthesis and evidence qualification | Verification #34–38, Analyze #33 |
| E | Agent-E | `quire-contract-ir`, `quire-contract-runtime`, `quire-contract-codegen`; IR, oracles, Kani, replay and mappings | Contract IR #99–101/#55–58/#40, Runtime #16, Codegen #48–50 |

An agent owns its whole listed chain and continues ticket-by-ticket without
routine reassignment. One ticket/PR and fresh default-branch worktree at a time
remain mandatory. If blocked, the agent records `blocked on X because Y` and
continues only with another unblocked ticket inside its own lane.

## Dependency waves

1. **Gate:** merge QSpec #63 and pass Task-010.
2. **Lane adoption:** A00, B's existing Protocol #8 entry, C00/C10, D00 and E00
   reconcile their repo-local plans once against the already reviewed common
   architecture. A local major `/specify` + self `/spec-review` is run once
   where that repository must install owning requirements; it cannot redesign a
   shared interface unilaterally.
3. **Independent foundations:** A01/A02, B bounded Protocol completion plus TL
   profile/parser, C observation uncertainty plus integration-repo creation,
   D generator/relational foundations and E IR/runtime can proceed concurrently.
4. **Semantic consumers:** A03–A06, B dynamic/analysis Protocol, C repair/runtime,
   D method providers and E codegen/Kani/replay consume only merged pinned
   artifacts.
5. **Qualification:** lane qualification tickets run independently, then C14
   executes IT-070–078 under one ecosystem lock and D05 records the attributed
   qualification ledger.

## Ticket shorthand

QSL = agent-ix/quire-spec-language; TL Syntax = agent-ix/tl-syntax; TL MLTL =
agent-ix/tl-mltl; QObs = agent-ix/quire-observation; Protocol =
agent-ix/quire-protocol; Verification = agent-ix/quire-verification; Analyze =
agent-ix/quire-analyze; Contract IR = agent-ix/quire-contract-ir; Codegen =
agent-ix/quire-contract-codegen; WASM = agent-ix/quire-wasm; Research =
agent-ix/quire-research. Issue numbers are live identifiers as of 2026-09-15.

## Atomic primary and qualification allocation

Every inventory row appears exactly once below. `Primary ticket` owns production
implementation or retains the exact closed leaf that already delivered it.
`Qualification ticket` owns the remaining verification/integration/claim
closure and cannot promote missing execution.

| Capability | Lane | Primary ticket | Qualification ticket |
| --- | --- | --- | --- |
| V1-SRC-001 | A | QSL #117 | QSL #123 |
| V1-SRC-002 | A | QSL #117 | QSL #123 |
| V1-SRC-003 | A | QSL #117 | QSL #123 |
| V1-SRC-004 | A | QSL #117 | QSL #123 |
| V1-SRC-005 | A | QSL #117 | QSL #123 |
| V1-SRC-006 | A | QSL #117 | QSL #123 |
| V1-SRC-007 | A | QSL #117 | QSL #123 |
| V1-SRC-008 | A | QSL #117 | QSL #123 |
| V1-SRC-009 | A | QSL #117 | QSL #123 |
| V1-SRC-010 | A | QSL #123 | QSL #123 |
| V1-SRC-011 | A | QSL #123 | QSL #123 |
| V1-SRC-012 | A | QSL #117 | QSL #123 |
| V1-SRC-013 | A | QSL #123 | QSL #123 |
| V1-SRC-014 | A | QSL #123 | QSL #123 |
| V1-SRC-015 | A | QSL #117 | QSL #123 |
| V1-TYPE-001 | A | QSL #118 | QSL #123 |
| V1-TYPE-002 | A | QSL #118 | QSL #123 |
| V1-TYPE-003 | A | QSL #118 | QSL #123 |
| V1-TYPE-004 | A | QSL #118 | QSL #123 |
| V1-TYPE-005 | A | QSL #118 | QSL #123 |
| V1-TYPE-006 | A | QSL #118 | QSL #123 |
| V1-TYPE-007 | A | QSL #118 | QSL #123 |
| V1-TYPE-008 | A | QSL #118 | QSL #123 |
| V1-TYPE-009 | A | QSL #118 | QSL #123 |
| V1-TYPE-010 | A | QSL #119 | QSL #123 |
| V1-TYPE-011 | A | QSL #119 | QSL #123 |
| V1-TYPE-012 | A | QSL #119 | QSL #123 |
| V1-TYPE-013 | A | QSL #119 | QSL #123 |
| V1-TYPE-014 | A | QSL #119 | QSL #123 |
| V1-TYPE-015 | A | QSL #119 | QSL #123 |
| V1-TYPE-016 | A | QSL #119 | QSL #123 |
| V1-TYPE-017 | A | QSL #119 | QSL #123 |
| V1-TYPE-018 | A | QSL #119 | QSL #123 |
| V1-TYPE-019 | A | QSL #119 | QSL #123 |
| V1-TYPE-020 | A | QSL #119 | QSL #123 |
| V1-TYPE-021 | A | QSL #120 | QSL #123 |
| V1-TYPE-022 | A | QSL #120 | QSL #123 |
| V1-TYPE-023 | A | QSL #120 | QSL #123 |
| V1-TYPE-024 | A | QSL #120 | QSL #123 |
| V1-TYPE-025 | A | QSL #120 | QSL #123 |
| V1-TYPE-026 | A | QSL #120 | QSL #123 |
| V1-TYPE-027 | A | QSL #120 | QSL #123 |
| V1-TYPE-028 | A | QSL #120 | QSL #123 |
| V1-TYPE-029 | A | QSL #120 | QSL #123 |
| V1-TYPE-030 | A | QSL #118 | QSL #123 |
| V1-TYPE-031 | A | QSL #118 | QSL #123 |
| V1-EXPR-001 | A | QSL #119 | QSL #123 |
| V1-EXPR-002 | A | QSL #119 | QSL #123 |
| V1-EXPR-003 | A | QSL #119 | QSL #123 |
| V1-EXPR-004 | A | QSL #119 | QSL #123 |
| V1-EXPR-005 | A | QSL #118 | QSL #123 |
| V1-EXPR-006 | A | QSL #118 | QSL #123 |
| V1-EXPR-007 | A | QSL #118 | QSL #123 |
| V1-EXPR-008 | A | QSL #118 | QSL #123 |
| V1-EXPR-009 | A | QSL #118 | QSL #123 |
| V1-EXPR-010 | A | QSL #119 | QSL #123 |
| V1-EXPR-011 | A | QSL #119 | QSL #123 |
| V1-EXPR-012 | A | QSL #119 | QSL #123 |
| V1-EXPR-013 | A | QSL #119 | QSL #123 |
| V1-EXPR-014 | A | QSL #119 | QSL #123 |
| V1-EXPR-015 | A | QSL #119 | QSL #123 |
| V1-EXPR-016 | A | QSL #119 | QSL #123 |
| V1-EXPR-017 | A | QSL #120 | QSL #123 |
| V1-EXPR-018 | A | QSL #120 | QSL #123 |
| V1-EXPR-019 | A | QSL #120 | QSL #123 |
| V1-EXPR-020 | A | QSL #120 | QSL #123 |
| V1-EXPR-021 | A | QSL #120 | QSL #123 |
| V1-EXPR-022 | A | QSL #120 | QSL #123 |
| V1-EXPR-023 | A | QSL #123 | QSL #123 |
| V1-EXPR-024 | A | QSL #119 | QSL #123 |
| V1-EXPR-025 | A | QSL #119 | QSL #123 |
| V1-EXPR-026 | A | QSL #119 | QSL #123 |
| V1-TEMP-001 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-002 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-003 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-004 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-005 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-006 | C | QObs #23 | QObs #26 |
| V1-TEMP-007 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-008 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-009 | C | QObs #23 | QObs #26 |
| V1-TEMP-010 | C | QObs #23 | QObs #26 |
| V1-TEMP-011 | C | QObs #23 | QObs #26 |
| V1-TEMP-012 | C | QObs #23 | QObs #26 |
| V1-TEMP-013 | C | QObs #23 | QObs #26 |
| V1-TEMP-014 | C | QObs #23 | QObs #26 |
| V1-TEMP-015 | C | QObs #23 | QObs #26 |
| V1-TEMP-016 | C | QObs #23 | QObs #26 |
| V1-TEMP-017 | C | QObs #24 | QObs #26 |
| V1-TEMP-018 | B | TL MLTL #68 | TL Syntax #74 |
| V1-TEMP-019 | E | Contract IR #71 (closed) | Contract IR #101 |
| V1-TEMP-020 | E | Contract IR #70 (closed) | Contract IR #101 |
| V1-TEMP-021 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-022 | B | TL Syntax #73 | TL Syntax #74 |
| V1-TEMP-023 | B | TL Syntax #73 | TL Syntax #74 |
| V1-PROT-001 | B | Protocol #83 | Protocol #82 |
| V1-PROT-002 | B | Protocol #83 | Protocol #82 |
| V1-PROT-003 | B | Protocol #83 | Protocol #82 |
| V1-PROT-004 | B | Protocol #83 | Protocol #82 |
| V1-PROT-005 | B | Protocol #83 | Protocol #82 |
| V1-PROT-006 | B | Protocol #83 | Protocol #82 |
| V1-PROT-007 | B | Protocol #83 | Protocol #82 |
| V1-PROT-008 | B | Protocol #83 | Protocol #82 |
| V1-PROT-009 | B | Protocol #83 | Protocol #82 |
| V1-PROT-010 | B | Protocol #83 | Protocol #82 |
| V1-PROT-011 | B | Protocol #83 | Protocol #82 |
| V1-PROT-012 | B | Protocol #79 | Protocol #82 |
| V1-PROT-013 | B | Protocol #79 | Protocol #82 |
| V1-PROT-014 | B | Protocol #79 | Protocol #82 |
| V1-PROT-015 | B | Protocol #83 | Protocol #82 |
| V1-PROT-016 | B | Protocol #83 | Protocol #82 |
| V1-PROT-017 | B | Protocol #83 | Protocol #82 |
| V1-PROT-018 | B | Protocol #83 | Protocol #82 |
| V1-PROT-019 | B | Protocol #83 | Protocol #82 |
| V1-PROT-020 | B | Protocol #83 | Protocol #82 |
| V1-PROT-021 | B | Protocol #83 | Protocol #82 |
| V1-PROT-022 | B | Protocol #83 | Protocol #82 |
| V1-PROT-023 | B | Protocol #83 | Protocol #82 |
| V1-PROT-024 | B | Protocol #83 | Protocol #82 |
| V1-PROT-025 | B | Protocol #83 | Protocol #82 |
| V1-PROT-026 | B | Protocol #83 | Protocol #82 |
| V1-PROT-027 | B | Protocol #80 | Protocol #82 |
| V1-PROT-028 | B | Protocol #80 | Protocol #82 |
| V1-PROT-029 | B | Protocol #81 | Protocol #82 |
| V1-PROT-030 | B | Protocol #81 | Protocol #82 |
| V1-PROT-031 | C | Research #76 | Research #78 |
| V1-RUN-001 | A | QSL #121 | QSL #123 |
| V1-RUN-002 | C | QObs #24 | QObs #26 |
| V1-RUN-003 | B | Protocol #83 | Protocol #82 |
| V1-RUN-004 | C | QObs #24 | QObs #26 |
| V1-RUN-005 | B | Protocol #8 | Protocol #82 |
| V1-RUN-006 | B | Protocol #8 | Protocol #82 |
| V1-RUN-007 | C | QObs #24 | QObs #26 |
| V1-RUN-008 | C | QObs #25 | QObs #26 |
| V1-RUN-009 | B | Protocol #83 | Protocol #82 |
| V1-RUN-010 | A | QSL #121 | QSL #123 |
| V1-RUN-011 | B | Protocol #82 | Research #78 |
| V1-RUN-012 | B | Protocol #82 | Research #78 |
| V1-RUN-013 | A | QSL #121 | QSL #123 |
| V1-RUN-014 | D | Verification #35 | Verification #38 |
| V1-RUN-015 | D | Verification #36 | Verification #38 |
| V1-RUN-016 | D | Verification #36 | Verification #38 |
| V1-RUN-017 | D | Verification #37 | Verification #38 |
| V1-RUN-018 | D | Analyze #33 | Verification #38 |
| V1-RUN-019 | D | Analyze #33 | Verification #38 |
| V1-RUN-020 | C | Research #76 | Research #78 |
| V1-BACK-001 | E | Contract IR #100 | Contract IR #101 |
| V1-BACK-002 | E | Contract IR #100 | Contract IR #101 |
| V1-BACK-003 | E | Contract IR #100 | Contract IR #101 |
| V1-BACK-004 | E | Contract IR #100 | Contract IR #101 |
| V1-BACK-005 | E | Codegen #49 | Contract IR #101 |
| V1-BACK-006 | E | Codegen #50 | Contract IR #101 |
| V1-BACK-007 | E | Contract IR #55 | Contract IR #101 |
| V1-BACK-008 | E | Contract IR #56 | Contract IR #101 |
| V1-BACK-009 | E | Contract IR #57 | Contract IR #101 |
| V1-BACK-010 | E | Contract IR #101 | Contract IR #101 |
| V1-BACK-011 | E | Contract IR #101 | Contract IR #101 |
| V1-BACK-012 | E | Contract IR #101 | Contract IR #101 |
| V1-BACK-013 | E | Contract IR #101 | Contract IR #101 |
| V1-BACK-014 | E | Contract IR #100 | Contract IR #101 |
| V1-BACK-017 | D | Analyze #40 | Verification #38 |
| V1-BACK-018 | A | QSL #185 | QSL #123 |
| V1-EVID-001 | D | Verification #38 | Research #78 |
| V1-EVID-002 | D | Verification #38 | Research #78 |
| V1-EVID-003 | D | Verification #38 | Research #78 |
| V1-EVID-004 | D | Verification #38 | Research #78 |
| V1-EVID-005 | D | Verification #38 | Research #78 |
| V1-EVID-006 | D | Verification #38 | Research #78 |
| V1-TOOL-001 | A | QSL #122 | QSL #123 |
| V1-TOOL-002 | A | QSL #122 | QSL #123 |
| V1-TOOL-003 | A | QSL #117 | QSL #123 |
| V1-TOOL-004 | A | QSL #117 | QSL #123 |
| V1-TOOL-005 | C | Research #77 | Research #78 |
| V1-TOOL-006 | A | QSL #122 | QSL #123 |
| V1-TOOL-007 | A | QSL #122 | QSL #123 |
| V1-TOOL-008 | A | QSL #119 | QSL #123 |
| V1-TOOL-009 | A | WASM #6 | QSL #123 |
| V1-TOOL-010 | C | Research #77 | Research #78 |
| V1-TOOL-011 | A | QSL #191 | QSL #123 |
| V1-TOOL-012 | A | QSL #192 | QSL #123 |

## Planning invariants

- All 176 required/integrity rows have one primary implementation ticket and one
  qualification owner; no row is deferred, research-only or unowned.
- Closed leaves remain immutable evidence but their qualification tickets still
  gate complete-V1 acceptance.
- Shared QSpec interfaces I01–I21 and payloads FR-320–339 are frozen inputs.
  Any genuine contradiction returns to QSpec before implementation changes.
- Agents do not share mutable repository worktrees. Cross-lane consumers pin
  merged revisions only.
- Rust only, `publish = false`, local tests, literal
  `--target-dir target-codex-backends`, no hosted CI dispatch, no edits to
  `resources/native-v1/`, and no tag deletion/movement.
- At PR time every Rust ticket runs self `/rust-review` and `/gap-analysis`,
  fixes all findings, admin-merges, then updates its ticket, parent epic, this
  manifest, QSpec #9 and affected matrices.
