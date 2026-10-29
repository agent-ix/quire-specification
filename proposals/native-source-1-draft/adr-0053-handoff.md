# ADR-0053 producer amendment handoff

Completed FS01 producer handoff. Contract-IR PR #72 applied this disposition and
accepted ADR-0053.
This file preserves the reviewed handoff and decision correspondence; the owning
ADR is now authoritative.

Decision record: [owner ruling](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5611950216),
with the [emitting/dependency clarification](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5611958246)
and [closed-#54 correction](https://github.com/agent-ix/quire-contract-ir/issues/53#issuecomment-5612031500).

## Applied replacement decision text

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

## Applied disposition in the owning ADR

| Existing section | Applied disposition |
| --- | --- |
| Title, Status and decision table | State the native-source ruling: clauses are authored in Quire. Cite its record. Acceptance records the owner's actual ruling, not completed compiler qualification. |
| Context/model boundary | Reconcile with already accepted ADR-0054. Existing formal APIs remain usable; do not conflate archetypes with formal-model semantics. |
| Decision proposed for approval | Replaced with the native-source decision. The superseded `state-finite/1-draft` candidate was not adopted as a compiler profile. |
| Source authority, eligibility, shared admission and anchors | Preserve the binding rules. Change source-specific references only where necessary to name native Quire; do not weaken the refusal or provenance obligations. |
| Separate #54 reference/bounds decision | Preserve the gate but state #54 is closed. Any later reference environment or IR extension needs its own ticket; neither is admitted by this first profile. |
| OCL, SysML and FRETish profile sections | Output mapping contracts under FS06, with explicit loss/refusal and semantic-correspondence obligations. They are not source frontends and require no foreign-runtime qualification. |
| ConfigVersion example | Reference native compiler FR-032, retain the minimum-only/parent gate, and separate the bounded native example from the original declaration. |
| Tool table | Keep inspected tool versions and license facts as historical research. Foreign CLI/build commands are not runnable qualification prerequisites. Retain the pilot's LGPL-3.0-or-later correction. |

Public promotion of this private packet, its links or rationale is not implied.
Contract-IR owns future ADR amendments; FS01 retains only the language/profile
decision correspondence recorded here.
