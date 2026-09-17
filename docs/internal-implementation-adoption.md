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

At the initial adoption checkpoint, IR was decc99a. Accepted IR ADR-0054,
merged in PR61 at 690bde7f2dc58662cf9ff0595c2c0e3b17107c6f, closed #54 on
2026-09-08 without a typed-model adapter. The domain model is declared in spec
artifacts ([AD-006](../spec/assurance/AD-006-model-graph-state-contracts.md)):
quire-rs extracts it, the FCD extraction-frontend lifts it into a `sha256-jcs`
domain package, and QSL model intake owns its formal semantics. FCD types
documents and generates datatypes, not formal semantics. A proceeds with
native resolution/typechecking/lowering over the existing FR-013
DeclarationEnvironment, FR-019 Rust API and FR-023 executable binder.
Object/reference and finite-bound gaps remain explicit case-specific work, not
a universal reader prerequisite.

The original FS02/FS03/FS05 bytes and judgments remain pinned historical inputs.
A structural artifact or generated Rust layout does not establish formal model
meaning. Compiler FR-005 and IT-005 carry the native boundary. No duplicate
archetype authority or executable binder is authorized.

Standard document/reusable-artifact terms and public promotion remain deferred.
New compiler implementation is AGPL-3.0-or-later (relicensed from
AGPL-3.0-only by owner decision 2026-09-13), preserving existing grants.
Fresh TypeSpec/Node producer execution and hosted CI dispatch were not approved.
The broader FS and LC02 issues stay open until their actual acceptance evidence
exists.

The decision is also recorded on PR8, language#3 and private CO01 research#54.

## Subsequent verification-consumer amendment

B's conditional PR8 review identified an unrepresentable JCS canonical domain
and missing exact artifact roles. The separate
[2-draft amendment](../proposals/shared-reference-2-draft/README.md), specified
at 9244dcc and reviewed through SR-009–016 at f446a64, addresses those findings.
It selects envelope 2-draft and artifact 3-draft. These versions are separate
from the state-core schema.

The new Rust tests qualify schema structure and actual B canonicalizer parity.
B's explicit consumer adoption, strict intake/role matching and actual report
qualification remain independent work. Neither this amendment nor a PR merge
turns synthetic fixture data into evaluator results or a human disposition.
