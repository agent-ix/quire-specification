# Unicode 17 text-profile definition

Definition identity: `quire.value.text.unicode-17.0.0/v1`.
This definition selects the Unicode 17.0.0 normalization data and
algorithms for the `nfc`, `nfd`, `nfkc` and `nfkd` profiles in FR-141.

The normative external closure is the following exact set of Unicode-hosted
artifacts, all at Unicode version 17.0.0.

| Artifact | Authoritative URL |
| --- | --- | --- |
| UAX #15 revision 57 (Unicode 17.0.0) | `https://www.unicode.org/reports/tr15/tr15-57.html` |
| `UnicodeData.txt` | `https://www.unicode.org/Public/17.0.0/ucd/UnicodeData.txt` |
| `DerivedNormalizationProps.txt` | `https://www.unicode.org/Public/17.0.0/ucd/DerivedNormalizationProps.txt` |
| `CompositionExclusions.txt` | `https://www.unicode.org/Public/17.0.0/ucd/CompositionExclusions.txt` |
| `NormalizationTest.txt` | `https://www.unicode.org/Public/17.0.0/ucd/NormalizationTest.txt` |
| Unicode data license | `https://www.unicode.org/license.txt` |

UAX #15 supplies the algorithm, the four UCD files supply the normalization
properties, exclusions and conformance vectors, and the license supplies the
redistribution terms. A producer compiles or vendors tables from the Unicode 17.0.0 files named above. Implementations may encode the verified tables differently
only when every Unicode scalar sequence produces the same normalization result
and the complete `NormalizationTest.txt` corpus passes. `unicode-scalars` and
`binary-utf8` select FR-141's non-normalizing relations but retain this
definition when a package also uses a normalization profile. Under `nfc`,
`nfd`, `nfkc` and `nfkd`, text length is the scalar count of the sequence
normalized by this definition, not of the input sequence.

This is a leaf semantic-profile definition selected alongside
`quire.value.complete/v1`; it has no reverse dependency on that root.
