# Native Quire composed edition definition

Definition identity: `ix:native`; language edition: `1-draft`;
definition revision: `1-draft.2`. **Proposed, not adopted or implemented.**

## Selection and scope

The compiler inventory binds this exact definition artifact; source headers
select `language "ix:native" edition "1-draft";`. The artifact byte digest,
definition revision and language/edition remain distinct. This successor selects
the corrected static/runtime prerequisites and producer/native correspondence
from SR-402 FND-001 and SR-406 FND-002. Earlier `1-draft.1` bytes remain intact;
historical `0-draft` sources retain their separate identity.

This definition owns the complete lexical and syntactic grammar, closed source
inventory, common name resolution, source correspondence, static package
components, profile dependency resolution and typed diagnostic stages. Parsing
a temporal/protocol form does not admit its semantics without that family's
exact selected definition.

A profile definition's admitted-form catalog determines which recognized
constructs it permits. Its selected rule files supply their semantics and all
applicable type, definedness, source and binding constraints. Rules describing
another facet do not grant that facet's forms. A child profile may add explicitly
listed forms; it cannot weaken inherited numeric, presence, identity, purity,
anchor or resource rules. Imported predicates retain their own defining profile.

Required definitions use exact identity, revision and byte digest. Unknown or
conflicting definitions, cycles and incompatible inherited rules refuse dependent
admission. No definition is a runtime script, plugin or implicit network fetch.
A compiler reports support for the selected contract separately from knowing its
bytes. Markdown is the normative document encoding, not another clause parser.

Only the rule files explicitly selected below and transitive required definitions
are normative dependencies. Their external/background links do not silently add
unversioned rules. Source-model and observation contracts remain explicit
parameters of the linked package; neither this definition nor a profile supplies
missing producer authority.

## Selected rule files

All rule files below are selected at repository revision `93e82651a361e955d95545bcbf212e293cfaabf9`.
The digest is SHA-256 of the entire file's exact bytes. A link is a locator;
changed working-tree bytes do not replace the selected snapshot.

| Rule file | Exact byte digest |
| --- | --- |
| [shared-grammar.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/proposals/quire-v1/shared-grammar.md) | `sha256:d6f74466c1afaf1f4f235668edaaf445c3b5042e3a75d690083d5713cdc4d38f` |
| [package-contract.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/proposals/quire-v1/package-contract.md) | `sha256:2e7c2e00c85756e7d22e15c4dab25e6c91b8ca49a83fc00e478e7c54f3adfc71` |
| [FR-030-bind-composed-definitions.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-030-bind-composed-definitions.md) | `sha256:6c10e301352c876dd87a9b625d1596f9ef0af2951619db11ed4cd82f2a4241e2` |
| [FR-031-report-requested-capabilities.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-031-report-requested-capabilities.md) | `sha256:b10661586f0396ac9f204f17f11042ed662f57de2f8f682cda3f30cfe9f666dd` |
| [FR-032-preserve-language-evolution.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-032-preserve-language-evolution.md) | `sha256:49b5de82e2ee9c0e97995dce6fba7ca22013a1343382c69c4a9be01f761474e4` |
| [FR-035-bind-ecosystem-subjects.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-035-bind-ecosystem-subjects.md) | `sha256:d6997e2f42dacd4cdab81810ed42a43553cb673db4815b808bb00e5ea44aefe2` |
| [FR-036-retain-lexical-source-locations.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-036-retain-lexical-source-locations.md) | `sha256:227cbe0215a391fcb91535a9cdbeadb979ee00038c37d544b711cb4b0ad853ef` |
| [FR-037-parse-shared-native-expressions.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-037-parse-shared-native-expressions.md) | `sha256:0dcf6e73097d8fe0ee45d864e70731b2fcda6379eb058af331b975302b2547b2` |
| [FR-038-resolve-predicate-scopes.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-038-resolve-predicate-scopes.md) | `sha256:fec72cfaa7fc5ea767dfc7ecb966dc6ecf6011a0794f455b6d6767a624d1695a` |
| [FR-040-admit-explicit-state-extensions.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-040-admit-explicit-state-extensions.md) | `sha256:857f11a85f3c210b2dfc71ead4005e5215ce38c4dd2be4dc081a51849ad7d15f` |
| [FR-047-emit-typed-located-causes.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/functional/FR-047-emit-typed-located-causes.md) | `sha256:794d0a76cec53bd113dddfb750b1a8db4c8bc5677be73999582d6861dadb9805` |
| [NFR-010-bound-composed-processing.md](https://github.com/agent-ix/quire-specification/blob/93e82651a361e955d95545bcbf212e293cfaabf9/spec/non-functional/NFR-010-bound-composed-processing.md) | `sha256:d9419cc39ac28a1562dea3ef74bdf7b092b5b4b32cf172f80d95976ae8baaf16` |
