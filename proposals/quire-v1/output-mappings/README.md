# Output mapping profiles

FS06 accepts one non-authoritative output-mapping subsystem and three bounded
target profiles. Native Quire remains the only editable clause source. Generated
OCL, SysML/KerML and FRETish are derived data accompanied by a complete
per-obligation mapping account.

| Contract | Selected identity | Implementation owner |
| --- | --- | --- |
| [Common request, record and package](common.md) | `quire.output.mapping/v1` | qcir #52 shared boundary |
| [OCL 2.4](ocl-2.4.md) | `quire.output.ocl24/v1` | qcir #55 |
| [SysML 2.0 / KerML 1.0](sysml-2.0-kerml-1.0.md) | `quire.output.sysml2-kerml1/v1` | qcir #56 |
| [FRETish 3.1](fretish-3.1.md) | `quire.output.fretish31/v1` | qcir #57 |
| [Sources and rights](sources-and-rights.md) | FS06 inspection record | each implementation owner |

All four profile identities are exact selections, not public registrations.
Changing a target standard, selected fragment, mapping rule or canonical field
requires a new profile identity. No historical generated package is rewritten.

## Common outcome rule

Every requested native obligation receives exactly one
[FR-298](../../../spec/objects/output-mapping/FR-298-per-obligation-output-mapping-record.md)
record classified with [FR-269](../../../spec/objects/observation/FR-269-consumer-mapping-loss.md):
`preserved`, `conditional`, `unrepresented` or `refused`. Missing records,
missing classifications and best-effort omissions are invalid. A target parser
accepting bytes is a structural observation and cannot change the classification.

## Non-goals

- No authored target-language frontend or round-trip authority.
- No claim of full OCL, SysML, KerML or FRET conformance.
- No foreign runtime required for production or qualification.
- No target-generated truth, evidence sufficiency or public release decision.
