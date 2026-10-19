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
    f.columns(0, 30, 980, 226, [n for _, n in dirs], labels=[name for name, _ in dirs], role="code")
    return f


REPOS = [
    ("ORCHESTRATION", [("quire-verification", True), ("quire-driver", True), ("quire-integration", True)]),
    ("BACKENDS", [("quire-contract-codegen", False), ("quire-contract-runtime", False),
                  ("quire-analyze", False), ("quire-protocol", True), ("quire-mltl", False)]),
    ("LANGUAGE", [("quire-specification", True), ("quire-spec-language", False), ("quire-contract-ir", False)]),
    ("FOUNDATIONS", [("quire-exact", False), ("quire-semantic-value", False), ("quire-walk", False),
                     ("quire-canonical", False), ("quire-verification-contracts", False),
                     ("quire-observation", False)]),
]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories in bands; each band builds on the ones below."""
    f = Figure(
        "ecosystem",
        980,
        364,
        "The Quire repositories in four bands. Foundations: quire-exact, quire-semantic-value, "
        "quire-walk, quire-canonical, quire-verification-contracts, quire-observation. Language: "
        "quire-specification, quire-spec-language, quire-contract-ir. Backends: "
        "quire-contract-codegen, quire-contract-runtime, quire-analyze, quire-protocol, "
        "quire-mltl. Orchestration: quire-verification, quire-driver, quire-integration.",
    )
    y = 0
    gap = 10
    for band, repos in REPOS:
        f.text(0, y + 12, band, "muted", "mono", SMALL)
        widths = [FONTS["title"].width(name, TITLE) + 24 for name, _ in repos]
        spare = (980 - sum(widths) - gap * (len(repos) - 1)) / len(repos)
        x = 0
        for (name, private), w in zip(repos, widths):
            w = round(w + spare)
            role = "logic" if name == here else "ink"
            f.box(x, y + 20, w, 50, role, name, "private" if private else "public", dashed=private)
            x += w + gap
        y += 88
    f.legend(0, 360, [("logic", "this repository", "box"), ("ink", "private", "dash")])
    return f


def ecosystem_here():
    return ecosystem(here="quire-specification")


FIGURES = [hero, corpus, ecosystem_here]
