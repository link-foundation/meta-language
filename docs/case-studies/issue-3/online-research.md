# Online Research: Language Rankings & Verified Facts

> Compiled 2026-06-05. Independent verification of the language lists the crate
> commits to, plus the external facts the case study relies on. Every figure is
> sourced. Where the repository's current data drifts from the sources, the drift
> is flagged so a follow-up can reconcile it.

## 1. Programming languages — TIOBE May 2026 top 10

The repository pins the programming-language set to the **TIOBE May 2026** top-ten
(`docs/parity-roadmap.md`, `PROGRAMMING_LANGUAGE_TARGETS`). Verified against
<https://www.tiobe.com/tiobe-index/>:

| # | Language | In repo? |
|---|---|---|
| 1 | Python | ✅ |
| 2 | C | ✅ |
| 3 | Java | ✅ |
| 4 | C++ | ✅ |
| 5 | C# | ✅ |
| 6 | JavaScript | ✅ |
| 7 | Visual Basic | ✅ |
| 8 | R | ✅ |
| 9 | SQL | ✅ |
| 10 | Delphi/Object Pascal | ✅ |

**Result: the repository's programming-language list matches the source exactly.**
TIOBE is a monthly index, so the *set* is stable but the *order* drifts month to
month; the crate's fixtures are keyed by language, not rank, so rank drift does not
break them. Parser-availability reality (from the Rust-libraries survey) is the
real constraint: 7 of 10 have official tree-sitter grammars; **Visual Basic, SQL,
and Delphi are the gaps**, with Visual Basic the most exposed.

## 2. Natural languages — total speakers (Ethnologue 2025 / Britannica)

The repository pins the natural-language set to the Britannica/Ethnologue
"languages by total number of speakers" list (`NATURAL_LANGUAGE_TARGETS`).
Verified against Ethnologue 2025 (via Britannica
<https://www.britannica.com/topic/languages-by-total-number-of-speakers-2228881>):

| # (Ethnologue 2025) | Language | Total speakers | In repo? | Repo rank |
|---|---|---|---|---|
| 1 | English | ~1.53 B | ✅ | 1 |
| 2 | Mandarin Chinese | ~1.18 B | ✅ | 2 |
| 3 | Hindi | ~609 M | ✅ | 3 |
| 4 | Spanish | ~560 M | ✅ | 4 |
| 5 | Modern Standard Arabic | ~332 M | ✅ | **6** |
| 6 | French | ~312 M | ✅ | **5** |
| 7 | Bengali | ~278 M | ✅ | 7 |
| 8 | Portuguese | ~264 M | ✅ | **9** |
| 9 | Russian | ~255 M | ✅ | **8** |
| 10 | Urdu | ~238 M | ✅ | 10 |

**Result: the repository's natural-language *set* matches the source exactly (all
10 present).** Two ordering nuances exist, and both are harmless to the
language-keyed fixtures:

- **Arabic (#5) / French (#6):** the repo lists French #5, Arabic #6. Total-speaker
  counts for these two are close and swap between sources/years (Britannica's prose
  has historically ranked French ahead of Arabic by total speakers; Ethnologue 2025
  ranks Arabic ahead). Either ordering is defensible.
- **Portuguese (#8) / Russian (#9):** Ethnologue 2025 ranks Portuguese ahead of
  Russian; the repo lists Russian #8, Portuguese #9.

**Recommendation:** because fixtures are keyed by language name (not rank), no code
change is required. If exact rank fidelity is desired, update the doc comment in
`NATURAL_LANGUAGE_TARGETS` / `docs/parity-roadmap.md` to either (a) match Ethnologue
2025 order (Arabic #5, French #6, Bengali #7, Portuguese #8, Russian #9) or (b) add
a note that ranks are approximate and the *set* is what's pinned. This is captured
as a proposed low-priority follow-up, not a blocker.

### Script/processing diversity of the natural-language set

The 10 languages deliberately span the hard cases for lossless text processing —
this is why they're a good fixture set:

- **Latin script, space-delimited:** English, Spanish, French, Portuguese.
- **Cyrillic:** Russian.
- **Devanagari (complex shaping, conjuncts):** Hindi.
- **Bengali script (Brahmic, conjuncts):** Bengali.
- **Han/CJK, no word spaces:** Mandarin Chinese — needs dictionary segmentation
  (`lindera`), not UAX #29.
- **Arabic script, RTL + contextual shaping:** Modern Standard Arabic, Urdu — need
  `unicode-bidi` + normalization.

Byte-exact reconstruction across all six script families is the real test; the
existing `LANGUAGE_FIXTURES` already include non-ASCII samples (`你好。`,
`नमस्ते।`, `সমস্যা`/Bengali, `Гавайи это штат.`, `سلام۔`) that exercise these
byte ranges through `reconstruct_text()`.

## 3. Verified facts about competitor projects (test-adoption safety)

From the competitor test-suite survey (full detail in
`competitor-test-suites.md`), the licensing facts that make "copy all the tests
from competitors" safe:

| Project | License | Test-data adoption |
|---|---|---|
| tree-sitter (+ grammars) | MIT | ✅ safe |
| LibCST | MIT (prefer `_nodes/tests/`; `_parser/parso/` is dual MIT/PSF) | ✅ safe with caveat |
| Recast | MIT | ✅ safe |
| jscodeshift | MIT | ✅ safe |
| rowan | Apache-2.0 / MIT | ✅ safe |
| cstree | Apache-2.0 / MIT | ✅ safe |
| Roslyn | MIT | ✅ safe |

All are permissive and compatible with this repository's **Unlicense**. When
copying verbatim test data, retain a provenance comment (upstream path + license).

## 4. Verified corrections to founding-issue (#1) figures

From the ecosystem survey (full detail in `ecosystem-foundations.md`), three
figures cited in issue #1 drift from the verified source and should be cited
correctly in downstream issues:

- formal-ai "**706-case corpus**" → unverifiable; corpus is
  `data/seed/*.lino` + `data/benchmarks/*.lino`.
- meta-expression "**328 concepts**" → verified **351 concepts** in
  `semantic-lexicon.json`.
- links-notation "**90+ tests/language**" → verified **~138 tests per language
  binding**.
- storage crate "**doublets-rs**" → crate name is **`doublets`**; `doublets-rs`
  is the repository.

## 5. Search methodology

- Programming ranking: TIOBE index page (May 2026 snapshot), cross-checked against
  the repo's own cited source.
- Natural-language ranking: Britannica "languages by total number of speakers"
  (Ethnologue 2025 data), cross-checked speaker counts.
- Competitor/ecosystem facts: live GitHub Contents API + raw file fetches against
  each upstream repo (recorded in `competitor-test-suites.md`,
  `rust-libraries-survey.md`, `ecosystem-foundations.md`), not secondary summaries.
- All retrieval performed 2026-06-05.
