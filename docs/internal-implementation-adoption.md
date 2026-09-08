# Internal implementation adoption — 2026-09-08

The owner adopted the proposal in private PR8 at
e897f810a7356d4ce8fd19026221ebda7b65596f as the contract for internal LC02
implementation, replying **“ok i agree to adopt it too.”**

The immediately preceding explanation identified the decision as exact
model/import identity (FS02), finite-state typing/evaluation and pre/post rules
(FS03), and strict versioned shared references (FS05). It explicitly kept
public standard approval and publication separate. The corresponding earlier
question also kept B/C adoption and shared-adapter qualification as independent
integration acceptance gates.

This record changes the proposal's adoption status for Agent A's internal
implementation target. It does not modify the pinned definitions, profile
digests, authored expected judgments or historical fixture bytes. References
inside that immutable packet retain their original status/provenance fields;
they are historical inputs, not newly observed execution outcomes.

The eight specification reviews at 36293ba remain the recorded review set.
Their evidence-planning, method-catalog, dependency and consumer-qualification
findings still require scoped disposition before corresponding implementation
or integration acceptance. Owner adoption is not a claim that those tasks ran.
Compiler Plan-002 already resolved its native readiness findings and landed in
language PR7 at 2be73504edff1224d6396da15edca3775905d37b.

At the initial adoption checkpoint, IR was decc99a and a new #54 typed-model
adapter was believed to be required. **That prerequisite is superseded** by
accepted IR ADR-0054, merged in PR61 at
690bde7f2dc58662cf9ff0595c2c0e3b17107c6f with #54 closed on 2026-09-08.
Filament owns archetype schemas and datatype generation, not formal semantics.
A proceeds with native resolution/typechecking/lowering over the existing
FR-013 DeclarationEnvironment, FR-019 Rust API and FR-023 executable binder.
A concrete archetype projection is specified by its modeling-language owner
only for a proof case that needs it. Object/reference and finite-bound gaps
remain explicit case-specific work, not a universal reader prerequisite.

The original FS02/FS03/FS05 bytes and judgments remain pinned historical inputs.
Read ADR-0054 as the subsequent ownership correction; a structural artifact or
generated Rust layout does not establish the required formal correspondence.
Compiler FR-005 and IT-005 carry the corrected native boundary. No duplicate
archetype authority or executable binder is authorized.

Standard document/reusable-artifact terms and public promotion remain deferred.
New compiler implementation retains AGPL-3.0-only, preserving existing grants.
Fresh TypeSpec/Node producer execution and hosted CI dispatch were not approved.
The broader FS and LC02 issues stay open until their actual acceptance evidence
exists.

The decision is also recorded on PR8, language#3 and private CO01 research#54.
