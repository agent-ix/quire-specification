# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""The quire-specification documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. Every count in the figures
is read from this repository's tree: spec artifacts, vector files and test
functions are counted here, never typed.
"""

import re
from pathlib import Path

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

ROOT = Path(__file__).resolve().parents[2]

# One role per kind of thing, fixed across the Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "compiler stage", "box"),
    ("model", "checked package / IR", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]


# ---- repository data ---------------------------------------------------------


def spec_dirs():
    """Artifact counts per spec/ directory, largest kinds in directory order."""
    out = []
    for d in sorted((ROOT / "spec").iterdir()):
        if d.is_dir() and d.name not in ("reviews", "subsystems"):
            out.append((d.name, len(list(d.rglob("*.md")))))
    return out


def vector_files():
    """JSON schemas, vectors and fixtures under proposals/."""
    return len(list((ROOT / "proposals").rglob("*.json")))


def test_functions():
    """#[test] functions in the qualification crate."""
    return sum(len(re.findall(r"#\[test\]", p.read_text(encoding="utf-8"))) for p in (ROOT / "tests").glob("*.rs"))


# ---- figures -----------------------------------------------------------------


def hero():
    """README hero: what the standard authors, what checks it, what it must still prove."""
    dirs = dict(spec_dirs())
    reqs = dirs["functional"] + dirs["non-functional"]
    f = Figure(
        "hero",
        980,
        340,
        "The standard is authored as requirements, test cases, schemas with vectors, and "
        "definition bytes. Today the qualification tests check the schemas, vectors and "
        "definition bytes and report each vector admitted or refused. Compiler acceptance of "
        "the test cases by quire-spec-language and the backends is planned.",
    )
    f.text(0, 12, "AUTHORED HERE", "muted", "mono", SMALL)
    f.text(310, 12, "CHECKED BY", "muted", "mono", SMALL)
    f.text(640, 12, "OUTCOME", "muted", "mono", SMALL)
    f.box(0, 24, 220, 56, "code", "requirements", f"{reqs} functional and quality")
    f.box(0, 100, 220, 56, "code", "test cases", f"{dirs['test-cases']} written procedures")
    f.box(0, 176, 220, 56, "model", "definition bytes", "3 documents in src/lib.rs")
    f.box(0, 252, 220, 56, "model", "schemas + vectors", f"{vector_files()} JSON files in proposals/")
    f.line([(110, 80), (110, 97)], "code")
    f.text(310, 94, "PLANNED", "muted", "mono", SMALL)
    f.box(310, 100, 240, 56, "ink", "implementations", "quire-spec-language · backends")
    f.line([(220, 128), (307, 128)], "code", dashed=True)
    f.box(640, 100, 340, 56, "evidence", "compiler acceptance", "each case passes or is refused", dashed=True)
    f.line([(550, 128), (637, 128)], "ink", dashed=True)
    f.text(310, 214, "QUALIFIED TODAY", "muted", "mono", SMALL)
    f.box(310, 220, 240, 56, "logic", "qualification tests", f"cargo test · {test_functions()} tests")
    f.line([(220, 204), (260, 204), (260, 240), (307, 240)], "model")
    f.line([(220, 280), (260, 280), (260, 256), (307, 256)], "model")
    f.box(640, 220, 340, 56, "evidence", "admitted or refused", "positive vectors pass, adverse ones fail")
    f.line([(550, 248), (637, 248)], "logic", weight=2.4)
    f.legend(0, 334, [ROLES[0], ROLES[1], ROLES[2], ("evidence", "outcome (dashed: planned)", "box"), ROLES[4]])
    return f


def corpus():
    """Spec artifacts per kind, counted from the spec/ tree."""
    dirs = spec_dirs()
    total = sum(n for _, n in dirs)
    f = Figure(
        "corpus",
        980,
        260,
        f"The spec/ tree holds {total} authored artifacts in {len(dirs)} kinds: "
        + ", ".join(f"{name} {n}" for name, n in dirs)
        + ".",
    )
    f.text(0, 16, f"{total} SPEC ARTIFACTS IN {len(dirs)} KINDS", "muted", "mono", SMALL)
    top = max(n for _, n in dirs)
    for i, (name, n) in enumerate(dirs):
        y = 36 + i * 24
        f.text(0, y + 11, name, "ink", "mono", SMALL)
        f.rect(150, y, 700, 14, "muted", stroke=False, opacity=0.18)
        f.bar(150, y, 700 * n / top, 14, "code")
        f.text(150 + 700 * n / top + 10, y + 11, str(n), "muted", "mono", SMALL)
    return f


REPOS = [
    (1, 0, "quire-driver", "the quire command"),
    (2, 0, "quire-integration", "tests the whole chain"),
    (3, 0, "quire-verification", "plans verification"),
    (0, 1, "quire-specification", "the standard"),
    (1, 1, "quire-spec-language", "compiler · evaluator"),
    (2, 1, "quire-contract-ir", "contract model"),
    (3, 1, "quire-contract-codegen", "code · tests · proofs"),
    (4, 1, "quire-contract-runtime", "runs generated checks"),
    (0, 2, "quire-observation", "qualified observations"),
    (1, 2, "quire-protocol", "protocol conformance"),
    (2, 2, "quire-analyze", "SMT consistency"),
    (0, 3, "quire-mltl", "MLTL over observations"),
    (1, 3, "quire-eval", "evaluation core"),
]
FEEDS = [
    ("quire-specification", "quire-spec-language"),
    ("quire-spec-language", "quire-contract-ir"),
    ("quire-contract-ir", "quire-contract-codegen"),
    ("quire-contract-codegen", "quire-contract-runtime"),
    ("quire-spec-language", "quire-protocol"),
    ("quire-contract-ir", "quire-analyze"),
    ("quire-observation", "quire-protocol"),
    ("quire-observation", "quire-mltl"),
]
SHARED = ["quire-eval", "quire-exact", "quire-semantic-value", "quire-walk", "quire-canonical",
          "quire-verification-contracts"]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories: the main chain, its branches, and what runs and shares it."""
    f = Figure(
        "ecosystem",
        980,
        476,
        "The Quire repositories. The main chain runs from the standard, quire-specification, to "
        "the compiler, quire-spec-language, then quire-contract-ir, quire-contract-codegen and "
        "quire-contract-runtime. quire-spec-language also feeds quire-protocol, and "
        "quire-contract-ir feeds quire-analyze. quire-observation feeds quire-protocol and "
        "quire-mltl. Above the chain, quire-driver runs it, quire-integration tests it and "
        "quire-verification plans verification. quire-eval evaluates checked terms. Below it, shared libraries used throughout: "
        "quire-eval, quire-exact, quire-semantic-value, quire-walk, quire-canonical and "
        "quire-verification-contracts.",
    )
    w, h, step = 176, 50, 201
    rows = {0: 20, 1: 120, 2: 210, 3: 290}
    where = {}
    for col, row, name, sub in REPOS:
        x, y = col * step, rows[row]
        where[name] = (x, y)
        role = "logic" if name == here else "ink"
        f.box(x, y, w, h, role, name, sub)
    for a, b in FEEDS:
        (ax, ay), (bx, by) = where[a], where[b]
        if ay == by:
            f.line([(ax + w, ay + h / 2), (bx - 3, by + h / 2)], "model")
        else:
            f.line([(ax + w / 2, ay + h), (bx + w / 2, by - 3)], "model")
    f.text(0, 12, "ORCHESTRATION", "muted", "mono", SMALL)
    f.text(0, 112, "THE CHAIN", "muted", "mono", SMALL)
    f.text(0, 382, "SHARED LIBRARIES · USED THROUGHOUT", "muted", "mono", SMALL)
    gap = 10
    widths = [FONTS["title"].width(name, TITLE) + 24 for name in SHARED]
    spare = (980 - sum(widths) - gap * (len(SHARED) - 1)) / len(SHARED)
    x = 0
    for name, w0 in zip(SHARED, widths):
        w0 = round(w0 + spare)
        f.box(x, 390, w0, h, "logic" if name == here else "ink", name)
        x += w0 + gap
    f.legend(0, 472, [("logic", "this repository", "box"), ("model", "feeds", "fill")])
    return f


def ecosystem_here():
    return ecosystem(here="quire-specification")


FIGURES = [hero, corpus, ecosystem_here]
