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
while consuming exact model/configuration and observation inputs. Detect identity
substitution at the boundary before a dependent evaluator runs.

## Target Integration

D's model/configuration producer feeds A's native linker; its typed linked output
feeds the request binder consuming F's observations and B's scoped requests.
Use the real selected producer/consumer interfaces, without text reparsing or a
fixture-only substitute model. This is a planned boundary under compiler #35,
FS02 and OB01, not an implemented integration.

## Preconditions

The consumed model, configuration, reference, profile and binding contracts have
reviewed exact revisions. The Rust producer and consumers implement those
revisions. All input source units and dependency bytes are supplied locally;
the fixture declares finite bounds rather than deriving them from sample data.

## Inputs

A two-source-unit order package includes a state predicate, temporal obligation
and compensated protocol. O1 and O2 share an external provider but have distinct
workflow, effect, relationship and population identities. Two clock roles have
equal local names. Bind them explicitly, with identified scope/progress inputs.
Supply two otherwise equal configurations with different sufficient resource
ceilings and separate mutants listed in TC-030 and TC-035.

## Test Procedure

1. Link the inventory through the actual model producer and consume the result.
   - IT-010-SC-01: every expected source, model/profile selection, declaration and typed role agrees with the independently authored fixture; both equal-named roles remain distinct.
2. Remove one inventoried source and substitute each wrong model digest domain independently.
   - IT-010-SC-02: namespace or import admission refuses as applicable; the consumer receives no falsely checked dependent declaration.
3. Bind the original O1/O2 requests; then omit, duplicate and substitute a required role mapping and cross-bind O2 progress into O1's population.
   - IT-010-SC-03: explicit compatible role sharing succeeds; each mutant refuses its dependent request with a typed role/scope cause and no evaluator invocation.
4. Substitute only the sufficient resource ceiling in compile and runtime configuration, then substitute a model/profile/binding-contract meaning at runtime.
   - IT-010-SC-04: resource changes preserve the static subject and retain distinct full configuration references; semantic overrides refuse the original subject.
5. Admit the compensation mapping without activation, then activate it and withhold a recovery observation required for the selected assessment.
   - IT-010-SC-05: the first request invents no recovery evidence prerequisite; the second reports incomplete required evidence, never optional-none, false or successful recovery.

## Expected Results

Real producers and consumers agree on the [package contract](../../proposals/quire-v1/package-contract.md).
Static subjects, artifact bytes, build provenance and runtime inputs retain their
separate identity domains. Every rejected mutation has a structured cause;
incomplete evidence and unsupported methods remain distinct from malformed
bindings and logical truth. No executable coverage is claimed by this plan.

## Dependencies

[TC-030](../test-cases/TC-030-bind-composed-definitions.md) and
[TC-035](../test-cases/TC-035-bind-ecosystem-subjects.md) provide independent controls.
D's standard-facing model/configuration contract and F's exact observation
binding interface remain required inputs before implementation of this test.
