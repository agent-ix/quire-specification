# Native Quire v1 design

Proposed under [standard #10](https://github.com/agent-ix/quire-specification/issues/10),
following the owner's 2026-09-09 full-language handoff. **State, temporal, and
protocol/choreography are required v1 families.** The bounded state development
profile is an implementation foundation, not the language's product boundary.

This packet selects a finite product capability set. It does not attempt to
implement OCL, FRET, MLTL, KerML or SysML wholesale. Native Quire is the single
editable clause language; external textual forms are output mappings. The
language standard owns their selected meanings rather than inheriting unstated
semantics from those external languages or from an evaluator.

The [research and recommendation report](report.md) tracks the requested report
and staged plan; it remains a draft until the coherent baseline is complete and
reviewed. No implementation is part of this assignment.

The [capability matrix](capabilities.md) distinguishes product scope from observed
implementation and routes each family. The common architecture, profile
requirements and conformance cases are being specified together before
implementation tasks are finalized. This proposal does not enable any new
grammar or claim that its selected capabilities are implemented.

Existing `state-finite/0-draft` definitions and fixtures remain immutable. The
[state admission amendment](../native-source-1-draft/README.md) remains a scoped
candidate, not the complete v1 language. Its unresolved numeric, reference and
source-anchor obligations remain visible during this redesign.

The baseline is reviewed as a whole before profile-specific delivery resumes.
Implementation is then divided by owned interface and testable behavior, with
state, temporal and choreography all required before a Quire v1 capability claim.
Earlier engineering milestones may identify their delivered subset explicitly.
