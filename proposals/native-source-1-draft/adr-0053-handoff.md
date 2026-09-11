# ADR-0053 producer amendment handoff

FS01 proposal under specification #2, for the current Contract IR/ADR owner.
This file supplies reviewable replacement text and disposition instructions;
it does not edit the producer repository or claim its ADR has been accepted.

Decision record: [owner ruling](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5611950216),
with the [emitting/dependency clarification](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5611958246)
and [closed-#54 correction](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5612031500).

## Proposed replacement decision text

Adopt native Quire as the single editable formal-clause source language. Its
definition and profile identity are owned by FS01; admission and qualification
remain explicit per-stage obligations. The first-party compiler receives no
exemption from the bounded semantic admission table. Native correspondence
remains unqualified until the scoped FS01 and consumer evidence discharge it.

OCL 2.4, SysML v2 and FRETish are export-only mapping profiles. Their target
contracts remain wanted; no external syntax is an alternate editable authority.
Rust emitters retain source identities and report each preserved, conditional,
unrepresented or refused obligation. Upstream parser acceptance alone does not
establish equivalence. Every first-party production and possible qualification
path, including integration code, is Rust. No Java, JVM, Maven, Eclipse or
Electron build/test/qualification dependency is authorized by this decision.
Emitting text for such consumers does not create that dependency.

Keep the source-authority rules, shared bounded semantic admission table,
evaluation-instant/operation-anchor table, reference/bounds design gate, and
SysML LGPL-3.0-or-later correction. The ConfigVersion worked example is assessed
through native compiler FR-032. Its deliberately bounded historical model does
not supply a finite bound or reference correspondence for the original model.
Temporal remains behind the separately specified tl-* bridge. Criterion
eligibility and coverage states remain language-neutral and do not require
every criterion to be formalized.

## Edits to apply in the owning ADR

| Existing section | Required disposition |
| --- | --- |
| Title, Status and decision table | Replace OCL-first recommendation and absent-decision wording with native-source ruling; cite its record. Acceptance records the owner's actual ruling, not completed compiler qualification. |
| Context/model boundary | Reconcile with already accepted ADR-0054. Existing formal APIs remain usable; do not recreate a universal Filament reader prerequisite or conflate archetypes with formal-model semantics. |
| Decision proposed for approval | Use the replacement text above. Refer to the reviewed FS01 identity; the candidate state-finite/1-draft is not yet an adopted compiler profile. |
| Source authority, eligibility, shared admission and anchors | Preserve the binding rules. Change source-specific references only where necessary to name native Quire; do not weaken the refusal or provenance obligations. |
| Separate #54 reference/bounds decision | Preserve the gate but state #54 is closed. Any later reference environment or IR extension needs its own ticket; neither is admitted by this first profile. |
| OCL, SysML and FRETish profile sections | Retain as output mapping contracts under FS06, with explicit loss/refusal and semantic-correspondence obligations. Remove wording that enables them as source frontends or requires foreign-runtime qualification. |
| ConfigVersion example | Reference native compiler FR-032, retain the minimum-only/parent gate, and separate the bounded native example from the original declaration. |
| Tool table | Keep inspected pins and license facts as historical research. Foreign CLI/build commands are not runnable qualification prerequisites. Retain the pilot's LGPL-3.0-or-later correction. |

Public promotion of this private packet, its links or rationale is not implied.
The ADR owner must apply the producer change within its own disclosure and review
scope. C/D coordination owns the producer routing; A retains FS01 and compiler #30.
