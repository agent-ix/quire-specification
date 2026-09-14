# FS06 sources and rights record

Inspected 2026-09-14 for semantic/version selection only. QSpec copies no
upstream grammar, implementation, standard library, example corpus or generated
artifact.

| Target | Selected reference | Current disposition |
| --- | --- | --- |
| OCL | [OMG OCL 2.4](https://www.omg.org/spec/OCL/2.4/), normative document `formal/14-02-03`, adopted February 2014 | Reference by link. OMG specification text is not redistributed. No Eclipse OCL runtime or library is selected. |
| SysML | [OMG SysML 2.0](https://www.omg.org/spec/SysML/2.0/), Language `formal/26-03-02`, adopted September 2025 | Reference by link. OMG specification and machine-readable libraries are not redistributed. |
| KerML | [OMG KerML 1.0](https://www.omg.org/spec/KerML/1.0/), `formal/26-03-01`, adopted September 2025 | Reference by link as the selected SysML foundation. No pilot implementation is selected. |
| FRETish | [NASA FRET v3.1.0](https://github.com/NASA-SW-VnV/fret/releases/tag/v3.1.0), released 2026-03-13 | Reference by link. The repository identifies FRET 3.0 software as Apache-2.0; any later runtime/library adoption must re-inventory the exact v3.1.0 tag and transitive closure before use. No FRET code or corpus is copied here. |

Generated target text is produced solely from user/native inputs and these
original Quire mapping rules. Each implementation ticket must record the exact
dependencies, licenses, invocation and artifact hashes it actually adopts.
Ambiguous rights or unavailable artifacts refuse that optional observer; they
do not block the pure Rust exporter or authorize redistribution.
