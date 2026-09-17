---
id: IT-010
title: "Bind assessments to an exact linked package"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-030
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-035
    type: verifies
---
## Objective

Verify that the native linker and assessment binder preserve static selections
while consuming the exact domain package, configuration and observation inputs. Detect identity
substitution at the boundary before a dependent evaluator runs.

## Target Integration

The package's spec artifacts pass through quire-rs semantic extraction and the
FCD extraction-frontend lift into a domain package. QSL model intake admits it
and A's native linker links it into a checked package whose `model_selections`
name that domain package. The typed linked output feeds the request binder
consuming F's observations and B's scoped requests. Use the real extraction,
intake and binder interfaces, without text reparsing or a fixture-only
substitute model. This is a planned boundary under compiler #35,
FS02 and OB01, not an implemented integration.

## Preconditions

The domain package, configuration, reference, profile and binding contracts have
reviewed exact revisions. The Rust extraction, intake and binder implement those
revisions. All input source units and dependency bytes are supplied locally;
the fixture declares finite bounds rather than deriving them from sample data.

## Inputs

A two-source-unit order package declares `Order` and its relationships and
populations in `entity` and `population` artifacts and includes a state predicate, temporal obligation
and compensated protocol. O1 and O2 share an external provider but have distinct
workflow, effect, relationship and population identities. Two clock roles have
equal local names. Bind them explicitly, with identified scope/progress inputs.
Supply two otherwise equal configurations with different sufficient resource
ceilings and separate mutants listed in TC-030 and TC-035.

## Test Procedure

1. Extract and lift the inventory into its domain package, admit it through QSL model intake, link it and consume the result.
   - IT-010-SC-01: every expected source, model/profile selection, declaration and typed role agrees with the independently authored fixture; both equal-named roles remain distinct.
2. Remove one inventoried source and substitute each wrong domain package digest domain independently.
   - IT-010-SC-02: namespace or import admission refuses as applicable; the consumer receives no falsely checked dependent declaration.
3. Bind the original O1/O2 requests; then omit, duplicate and substitute a required role mapping and cross-bind O2 progress into O1's population.
   - IT-010-SC-03: explicit compatible role sharing succeeds; each mutant refuses its dependent request with a typed role/scope cause and no evaluator invocation.
4. Substitute only the sufficient resource ceiling in compile and runtime configuration, then substitute a model/profile/binding-contract meaning at runtime.
   - IT-010-SC-04: resource changes preserve the static subject and retain distinct full configuration references; semantic overrides refuse the original subject.
5. Admit the compensation mapping without activation, then activate it and withhold a recovery observation required for the selected assessment.
   - IT-010-SC-05: the first request invents no recovery evidence prerequisite; the second reports incomplete required evidence, never optional-none, false or successful recovery.
6. Through an explicitly selected `quire.observation.range/v1` mapping, bind a window of the declared population with inclusive start 0 and exclusive end 30 to a native timestamped `eventually[0,30] holds(p)` obligation. Put its sole p witness at time 30. Supply truthful coverage for only that window, then falsely relabel that coverage as closing the native inclusive interval. Repeat with a separately selected window [0,31), exact progress covering time 30, and the boundary witness admitted by the mapping; also use a decisive witness at time 29 as a control.
   - IT-010-SC-06: the shorter window cannot establish closed-scope truth for the inclusive obligation; truthful missing boundary coverage stays incomplete, and the false closure mapping refuses. The explicitly covering window admits the time-30 witness and settles satisfied; a valid decisive time-29 witness remains satisfied with its exact support despite the still-uncovered upper boundary. No implicit time increment, unit conversion, endpoint rounding or reinterpretation of membership makes the shorter window complete.

Additional stage and declaration-key controls for SR-402/406:

7. Link the three-family template with its exact model, profile and binding
   contracts before any workflow instance or runtime population/window exists.
   Bind a later assessment first with all required inputs, then with its required
   scope selection removed and separately with an unavailable required record.
   - IT-010-SC-07: static linking succeeds without observations; the valid assessment binds, the missing selection refuses, and unavailable required evidence remains incomplete. No compile-time window or synthesized record fills a later prerequisite.
8. Bind runtime subject references to effective declaration keys (domain
   package identity, IR node identity, `sha256-jcs`). Swap the digest domain,
   name an IR node the domain package does not declare, and supply a runtime
   reference keyed to a foreign domain package independently. Change one
   declared field in a source artifact and relift. Intake the relifted package
   with its bytes supplied under the old ModelSelection's digest, then with its
   bytes supplied under its own digest and a ModelSelection naming that digest. Separately remove one referenced
   artifact, relift and bind with its old declaration key.
   - IT-010-SC-08: the original declaration keys survive linking and binding; each substitution refuses its dependent binding. After the field change, the declaration key is unchanged; the old ModelSelection refuses `stale_dependency`/`byte-digest-mismatch`, and the new ModelSelection binds the changed declaration. After the removal, the old key refuses `missing_declaration`/`missing-name`.

## Expected Results

Real extraction, intake and binder stages agree on the [package contract](../../proposals/quire-v1/package-contract.md).
Static subjects, artifact bytes, build provenance and runtime inputs retain their
separate identity domains. Every rejected mutation has a structured cause;
incomplete evidence and unsupported methods remain distinct from malformed
bindings and logical truth. No executable coverage is claimed by this plan.

## Dependencies

[TC-030](../test-cases/TC-030-bind-composed-definitions.md) and
[TC-035](../test-cases/TC-035-bind-ecosystem-subjects.md) provide independent controls.
[AD-006](../assurance/AD-006-model-graph-state-contracts.md) defines the
domain package and declaration keys. F's exact observation binding interface is
a delivered review input. SC-06 uses the
[observation range definition](../../proposals/quire-v1/definitions/observation-range.md);
it never infers temporal coverage from a population's member IDs alone. E owns
temporal interval meaning, F scoped coverage assertions, and S3 the declared
population and its closed extent.

SC-07/08 consume the stage and declaration-key rules of the package contract.
A test fixture cannot invent domain declarations or stand in for extraction and
intake.
