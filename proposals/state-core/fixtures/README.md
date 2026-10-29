# Review fixtures and expected cases

These files are **proposed fixtures, not executed language results**.
`model-expectations.json` illustrates field declarations and the proposed
identity-reference mapping; it is explicitly not a domain
package. `parent-order.native` is a parse/source fixture. Its `model M =
"example/config";` names the domain package by identity alone and no domain
package is supplied, so linking it is an intentional `missing_import`/`missing-selection`
refusal, not a claim of a working import. Qualifying linking supplies a real
reviewed domain package under that identity; the resolver derives its digest into the
lock, and the source is unchanged.

`subject.json` supplies concrete authored/profile/source identities to the
verification owner. Its model binding is explicitly unavailable. The expression span
selects the original body bytes after a non-ASCII comment. It carries no
method/plan/result envelope and no fabricated linked/projection identity.

`operation-subjects.json` supplies pre/post subjects for `version-update.native`,
including exact clause and expression spans and a stable operation ID.
`invocation-cases.json` pairs those subjects with immutable snapshots, invocation
identities, creation/deletion inventories and proposed frame bindings. A changed
version number is permitted by the frame so its adverse case actually violates
the postcondition; it cannot be accidentally refused by an earlier frame check.
The wrong-invocation, missing-pre, forbidden-frame and deleted-object controls
have distinct expected failures. Operation/model linking remains unavailable.
The invocation fixture declares separate named frame expectations; cases select
one by exact artifact reference. A runtime caller is not authorized to redefine an accepted
operation frame. These alternate synthetic bindings still require their own
model/operation review and differ in evidence identity.

`state-cases.json` supplies original synthetic input/expected-value cases for
the eventual evaluator. The healthy and violating expectations are manually
specified, independent of any evaluator. These inputs do not bypass the unresolved
model linking requirement. `boundary-cases.json` describes precise malformed,
unsupported, stale and incomplete mutations and expected diagnostics for later
independent readers. Expected diagnostics are assertions to implement, not
observed tool output.

Reproduce the fixture source-span checks from this directory:

```sh
python3 - <<'PY'
import json
from pathlib import Path
s = json.loads(Path('subject.json').read_text())
raw = Path(s['source']['path']).read_bytes()
span = s['source']['expressionSpan']
actual = raw[span['startByte']:span['endByte']].decode('utf-8')
assert actual == s['source']['expectedExpression']
for prefix in ('start', 'end'):
    before = raw[:span[prefix + 'Byte']].decode('utf-8')
    assert span[prefix + 'Line'] == before.count('\n') + 1
    assert span[prefix + 'Column'] == len(before.rsplit('\n', 1)[-1]) + 1
assert s['modelBinding']['state'] == 'unavailable'
assert 'linkedPackageIdentity' not in s and 'projectionIdentity' not in s
print('fixture source span verified; no compiler executed')
PY
```

The same source checks apply to each `subjects` entry in
`operation-subjects.json`. Each source also supplies `clauseSpan`
and `expressionSpan`; both cover their exact original
byte ranges. The clause region starts at its introducer and ends after its
closing brace, excluding the trailing line ending. The expression region
excludes surrounding indentation and line endings.

The expanded source subject uses fixture version `agent-a-source-subject/1-draft`.
It adds clause-region and expression spans to the earlier `0-draft`
fixture. A consumer must explicitly support this version; neither version is
frozen or registered. Source-language edition and semantic profile remain
unchanged. Old fixture bytes and their identities stay associated with their
original fixture version rather than being rewritten as current evidence.

Full compiler integration acceptance still needs real model intake,
parser, linker, evaluator, generated backend and clean-consumer execution.

Operation fixtures now use `agent-a-invocation-cases/1-draft`. Each case selects
one exact invocation file. That file binds case-scoped immutable pre/post
observation files and one frame file using `artifactRef` with authority, identity
and explicit fixture profile.
Expected outcomes remain outside invocation bytes.
No fixture contains a claimed evaluator result/evidence identity.

Snapshots, frames and invocation inputs each have one authoritative payload file;
there is no second inline copy to synchronize. Paths locate local bytes and are
not identity. These are review
profiles, not registered production wire contracts. Model/operation binding
remains unavailable. The source-subject fixture version stays unchanged.

`reference-cases.json` supplies the separate ArtifactRef and
SemanticRef around the existing exact source fixture. Its semantic profile
names the private candidate `../profile.md`; it claims no
canonical semantic identity or bound model. Positive/adverse mutations are
review expectations for B/C's independent consumers. The older operation/source
wrapper versions are preserved; no implicit version conversion is performed.

`typing-cases.json` contains 51 independently authored FS03 judgment examples
under explicit abstract type/input hypotheses. They cover guarded definedness,
snapshot capture, numeric bounds and units, negative division/remainder, sequence
order/duplicates, identity/cycles, validation failures and work exhaustion. They
name the proposed rule document by path. They are not a domain
package, a complete runtime population, typechecker/evaluator output, or accepted
extensions of the earlier profile. The native parser can check their expression
syntax separately; it cannot establish their expected typing or logical results.

Reproduce the syntax check from the language implementation checkout after
`cargo build --locked --target-dir target`:

```sh
python3 tools/check_rule_syntax.py /path/to/specification/proposals/state-core
```

Observed on this packet: 50 expressions parsed, and the collect case refused as
unsupported. These observations concern grammar only; all 51 typing/evaluation
judgments remain authored expectations under the stated hypotheses.
