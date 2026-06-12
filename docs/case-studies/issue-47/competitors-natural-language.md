# Competitor study: natural-language grammar systems and shared concept spaces

Research date: **2026-06-10**. Scope: prior art relevant to
[meta-language issue #47](https://github.com/link-foundation/meta-language/issues/47) — (1) actual
parsing of natural-language *grammar* (grammatical/syntactic correctness only, no semantic checks)
and (2) an advanced **shared concept space** between languages, where a concept is reused across
languages **only on exact match**, to support automated translation/transformation between any
languages.

Two families of prior art matter:

- **Grammar parsing systems** — formalisms and engines that decide whether a sentence is
  grammatical (precision grammars: GF, HPSG/DELPH-IN, LFG/XLE, Apertium, LanguageTool,
  CG-3) versus systems that merely *annotate* whatever input they get (UD parsers: UDPipe,
  Stanza, spaCy, Trankit).
- **Concept spaces / interlinguas** — resources that assign language-independent identifiers to
  meanings (WordNet/CILI, BabelNet, ConceptNet, Wikidata lexemes, AMR/UMR, UNL, Abstract
  Wikipedia, FrameNet, PropBank).

Facts below were verified online on 2026-06-10 unless explicitly marked *(unverified)*.

---

## Grammar parsing and grammatical correctness

### Grammatical Framework (GF) and the Resource Grammar Library (RGL)

- **What it does:** A programming language for multilingual grammars
  ([grammaticalframework.org](https://www.grammaticalframework.org/)). A grammar is split into an
  **abstract syntax** (language-independent trees) and per-language **concrete syntaxes**
  (linearization rules). The same abstract tree parses from and generates into every concrete
  language — GF is itself a working "shared grammar space" with translation by tree transfer.
- **Formalism:** Type-theoretical grammars compiled to **PMCFG** (parallel multiple context-free
  grammars) in the **PGF** portable runtime format
  ([PGF paper](https://www.semanticscholar.org/paper/ce4c72d239564e2969c7c3228d9480cbfaba3a02)).
  Dependent types allow encoding agreement and selection constraints in the abstract syntax.
- **Coverage:** The [RGL](https://github.com/GrammaticalFramework/gf-rgl) implements the core
  morphology + syntax of **40+ languages** under one shared abstract API
  ([grammaticalframework.org](https://www.grammaticalframework.org/)); covers all of the top-10
  natural languages (English, Chinese, Hindi, Spanish, Arabic, French, Russian, Portuguese,
  German, Japanese).
- **Correctness vs annotation:** **Checks grammaticality by construction** — the parser only
  accepts strings derivable from the grammar; agreement/inflection errors simply fail to parse.
  This is exactly the "grammatical correctness, no semantics" behavior issue #47 asks for.
- **Data & licenses:** GF compiler GPL; runtime libraries and RGL under **LGPL/BSD**
  ([GF site](https://www.grammaticalframework.org/)). Grammars compile to binary `.pgf` or JSON.
- **Rust:** [`gf-core`](https://crates.io/crates/gf-core)
  ([repo](https://github.com/cryptopatrick/gf-core)) is a pure-Rust GF **runtime** (parse +
  linearize from grammars compiled to JSON); it is not a compiler. Community-maintained, small.
- **Test corpora:** regression test suites and example treebanks live in the
  [gf-rgl repo](https://github.com/GrammaticalFramework/gf-rgl) (`src/*/test`-style suites) and in
  [gf-wordnet](https://github.com/GrammaticalFramework/gf-wordnet) example sentences. *(layout
  detail unverified)*
- **Standout ideas:** abstract/concrete split = links between a language-neutral tree node and
  per-language linearizations; multilingual generation for free; grammars as libraries with a
  shared API; PGF as a portable compiled-grammar interchange format.

### Universal Dependencies (UD) treebanks

- **What it does:** Cross-linguistically consistent dependency-grammar annotation scheme and
  treebank collection ([universaldependencies.org](https://universaldependencies.org/)).
- **Formalism:** dependency trees with universal POS tags (UPOS), universal morphological
  features (UFeats), and universal dependency relations; serialized as
  [CoNLL-U](https://universaldependencies.org/format.html).
- **Coverage:** release 2.16 (May 2025): **319 treebanks, 179 languages**
  ([ELRA announcement](https://list.elra.info/mailman3/hyperkitty/list/corpora@list.elra.info/thread/53SZTXJ5TYER3PHLXI3JO2TUY6T7ZNX4/),
  [download page](https://universaldependencies.org/download.html)).
- **Correctness vs annotation:** **Annotation only.** UD describes whatever text exists; the
  scheme has no notion of ungrammaticality. Statistical UD parsers will happily produce trees for
  ungrammatical input. UD is a *vocabulary* for syntax, not a grammar checker.
- **Data & licenses:** per-treebank licensing — many CC BY-SA 4.0, a significant share
  **CC BY-NC-SA** ([licence overview](https://lindat.mff.cuni.cz/repository/xmlui/page/licence-UD-2.1));
  must be checked treebank-by-treebank.
- **Rust:** CoNLL-U is trivial to consume; crates
  [`rs-conllu`](https://crates.io/crates/rs-conllu) and
  [`conllu`](https://crates.io/crates/conllu) (MIT/Apache-2.0) exist.
- **Test corpora:** the treebanks *are* the test corpora (train/dev/test splits per treebank, via
  [LINDAT](https://universaldependencies.org/download.html)); CoNLL shared-task evaluation script
  (`conll18_ud_eval.py`) is standard.
- **Standout ideas:** a single universal tag/relation/feature inventory across 179 languages is
  the closest thing to a de-facto standard "shared syntax vocabulary" — ideal as the *naming
  scheme* for syntactic link types in a links network, even if UD itself does not check grammar.

### UD parsers: UDPipe, Stanza, spaCy, Trankit

- **What they do:** trainable pipelines (tokenize → tag → lemmatize → parse) producing CoNLL-U.
- **Formalism:** statistical/neural dependency parsing over UD.
- **Coverage & licenses:**
  - [UDPipe](https://github.com/ufal/udpipe): C++ with bindings; code **MPL 2.0**, pre-trained
    models **CC BY-NC-SA** ([ÚFAL](https://ufal.mff.cuni.cz/udpipe/1)); models for 50+ languages.
  - [Stanza](https://arxiv.org/pdf/2003.07082) (Stanford): Python/PyTorch, **Apache 2.0**, ~70
    languages; model licenses follow source treebanks.
  - [spaCy](https://explosion.ai/blog/ud-benchmarks-v3-2): Python/Cython, **MIT** code; trained
    pipelines for ~25 languages, mostly permissive licensed.
  - [Trankit](https://github.com/nlp-uoregon/trankit): transformer-based (XLM-R), **Apache 2.0**,
    ~56 languages, ~90 pretrained pipelines; best UD accuracy of the four
    ([performance docs](https://trankit.readthedocs.io/en/latest/performance.html)).
- **Correctness vs annotation:** all four **annotate only** — they never reject input.
  Grammaticality could at best be approximated by parse-score thresholds (unreliable).
- **Rust:** none are Rust-native; UDPipe 1 is embeddable C++ (FFI feasible); ONNX export of
  neural models is possible *(unverified for these specific tools)*. Output (CoNLL-U) is
  Rust-friendly.
- **Test corpora:** UD treebank test splits; CoNLL 2017/2018 shared-task data.
- **Standout ideas:** language-agnostic single-model pipelines (Trankit's shared multilingual
  encoder with per-language adapters) show that one engine can serve ~100 languages.

### Apertium (rule-based MT) and lttoolbox

- **What it does:** free/open-source rule-based shallow-transfer machine translation platform
  ([apertium.org](https://apertium.org/), [wiki](https://wiki.apertium.org/wiki/Lttoolbox)).
- **Formalism:** finite-state morphology ([lttoolbox](https://github.com/apertium/lttoolbox):
  `lt-comp`/`lt-proc` over XML `.dix` dictionaries), CG-3 or HMM disambiguation, chunk-based
  structural transfer rules.
- **Coverage:** ~50 released translation pairs, dozens more in incubation; monolingual
  morphological analyzers for far more languages *(pair count unverified; varies by release)*.
- **Correctness vs annotation:** morphological analyzers **reject unknown/invalid word forms**
  (a word either has an analysis or not) — i.e., word-level grammatical correctness; sentence
  syntax is only shallowly checked (no full parse, so no full-sentence grammaticality).
- **Data & licenses:** everything **GPL** ([Wikipedia](https://en.wikipedia.org/wiki/Apertium)).
  Dictionaries are clean XML, easy to mine (lemma + paradigm + tags).
- **Rust:** no official Rust port; lttoolbox is C++ (FFI possible). The `.dix` XML format is
  trivially parseable from Rust.
- **Test corpora:** per-pair regression tests and quality evaluations in each `apertium-XX-YY`
  repo on [github.com/apertium](https://github.com/apertium).
- **Standout ideas:** morphology as a finite-state transducer compiled from declarative paradigm
  data — exactly the kind of data that can be re-expressed as links (lemma —paradigm→ form —tags→
  features); pipeline-of-streams architecture.

### LanguageTool

- **What it does:** style and grammar checker for **31 languages**
  ([languagetool.org/dev](https://languagetool.org/dev),
  [GitHub](https://github.com/languagetool-org/languagetool)).
- **Formalism:** ~thousands of declarative **XML error-pattern rules** (token/POS regex over a
  tagged, chunked sentence) plus Java rules; no full parse — it detects *known error patterns*
  rather than proving grammaticality.
- **Correctness vs annotation:** **checks correctness negatively**: absence of triggered rules ≠
  grammatical; presence of a triggered rule = specific, explainable error with suggested fix.
  Complementary to precision grammars (which prove grammaticality positively but can't explain).
- **Data & licenses:** **LGPL 2.1+** ([dev page](https://languagetool.org/dev)); rule XML files
  ship with the source.
- **Rust:** see nlprule below; also [`languagetool-rust`](https://crates.io/crates/languagetool-rust)
  (HTTP client for the LT server).
- **Test corpora:** each XML rule embeds `<example correction=...>` positive/negative example
  sentences — thousands of built-in grammaticality/ungrammaticality test pairs per language.
- **Standout ideas:** error rules with embedded test examples; per-language community-maintained
  declarative rule files that a links engine could import directly.

### nlprule (Rust port of LanguageTool rules)

- **What it does:** [nlprule](https://github.com/bminixhofer/nlprule) re-implements
  LanguageTool's rule engine and tagging/chunking/disambiguation pipeline in pure Rust.
- **Coverage:** English, German, Spanish rule binaries (compiled from LanguageTool v5.2).
- **Licenses:** code **MIT OR Apache-2.0**; the compiled `.bin` rule sets are derived from
  LanguageTool and remain **LGPL 2.1** ([docs.rs](https://docs.rs/nlprule/latest/nlprule/)).
- **Status:** low maintenance activity since ~2021–2023; rules frozen at LT 5.2 *(activity level
  unverified beyond crate metadata)*.
- **Standout ideas:** proves LT's full rule semantics are portable to Rust with small binaries
  and no JVM; its rule-compilation pipeline (`nlprule-build`) is a model for "compile external
  grammar data into a fast native format".

### HPSG / DELPH-IN (ERG, ACE, Grammar Matrix)

- **What it does:** consortium building hand-engineered **precision grammars**
  ([delph-in.github.io](https://delph-in.github.io/docs/home/Home/)). The
  [English Resource Grammar](https://github.com/delph-in/erg) maps running English to normalized
  logical forms; [ACE](https://delph-in.github.io/docs/tools/ToolsTop/) parses *and generates*
  from the same grammar.
- **Formalism:** **HPSG** (typed feature structures, unification) with **Minimal Recursion
  Semantics** (MRS) output. The
  [Grammar Matrix](https://delph-in.github.io/docs/howto/DelphinTutorial_Grammars/) is a
  questionnaire-driven starter kit that emits a working HPSG grammar for a new language.
- **Coverage:** ERG (English) is broad-coverage; other mature grammars: Jacy (Japanese), GG
  (German), Zhong (Chinese), SRG (Spanish), etc.; Matrix-derived grammars for many small
  languages. Far from top-10 coverage at ERG quality.
- **Correctness vs annotation:** **strict grammaticality checking** — a precision grammar rejects
  ungrammatical input (ERG additionally has optional "mal-rules" for robust parsing of errors,
  used in grammar-coaching research, e.g.
  [GAUSS](https://arxiv.org/pdf/2406.18340)).
- **Data & licenses:** open-source; ERG under an MIT-style license *(exact license text
  unverified)*; tools open source per
  [DELPH-IN overview](https://delph-in.github.io/docs/home/Home/).
- **Rust:** none; ACE is C (FFI feasible); `pydelphin` is the Python tooling.
- **Test corpora:** `[incr tsdb()]` test-suite profiles and the Redwoods treebank distributed
  with ERG releases ([ERG docs](https://delph-in.github.io/docs/erg/ErgTop/)).
- **Standout ideas:** "mal-rules" (explicitly modeled error rules) give *explainable*
  ungrammaticality; bidirectional parse/generate from one declarative grammar; Grammar Matrix
  shows grammars can be generated from typological feature choices.

### LFG / XLE (ParGram)

- **What it does:** [XLE](https://ling.sprachwiss.uni-konstanz.de/pages/xle/) parses/generates
  Lexical-Functional Grammars; basis of the **ParGram** parallel-grammar consortium (grammars for
  English, French, German, Norwegian, Japanese, Urdu, Turkish, Polish, Welsh, Wolof, …).
- **Formalism:** **LFG** — parallel c-structure (phrase structure) and f-structure (functional,
  attribute-value) representations; ParGram standardizes f-structure features across languages.
- **Correctness vs annotation:** precision grammars — **reject ungrammatical input**; XLE has
  robustness/fragment modes and OT-marks for graded grammaticality.
- **Licenses:** XLE is **proprietary (PARC copyright), free only by research license** — a
  practical dead end for open-source reuse. Open alternatives exist but are immature
  (e.g. [Free Linguistic Environment](https://github.com/dcavar/fle)).
- **Rust:** none.
- **Test corpora:** ParGram test suites; INESS treebanks (Norwegian infrastructure)
  *(availability details unverified)*.
- **Standout ideas:** ParGram's *parallel f-structures* — same functional attribute space across
  languages with language-specific phrase structure — is a strong precedent for "shared abstract
  layer, per-language surface layer".

### Constraint Grammar (VISL CG-3)

- **What it does:** [CG-3](https://edu.visl.dk/cg3/single/) applies hand-written contextual
  REMOVE/SELECT/ADD rules to disambiguate morphological readings and add syntactic tags.
- **Formalism:** Constraint Grammar (reductionist, rule-based tagging over ambiguous FST output).
- **Coverage:** grammars for **30+ languages**
  ([Wikipedia](https://en.wikipedia.org/wiki/Constraint_grammar)); core of
  [GiellaLT](https://giellalt.uit.no/tools/cg3-usage.html) infrastructure for Sámi and other
  minority languages; also used inside Apertium pipelines.
- **Correctness vs annotation:** primarily disambiguation/annotation, but GiellaLT's **Divvun
  grammar checkers** are built from CG rules that detect real grammatical errors — CG is a proven
  substrate for production grammar checking of morphologically rich languages.
- **Licenses:** CG-3 is **GPL** ([UiO](https://www.uio.no/english/services/it/research/sensitive-data/help/hpc/software/VISL%20CG-3.html));
  GiellaLT grammars are open source.
- **Rust:** none; CG-3 is C++ (`cg3` CLI / library, FFI feasible).
- **Test corpora:** GiellaLT per-language regression test suites (yaml tests in `lang-*` repos).
- **Standout ideas:** contextual constraint rules are naturally expressible as link patterns
  ("remove reading X if a link to a finite verb exists to the left"); proven on low-resource,
  morphology-heavy languages.

### UniMorph

- **What it does:** universal schema + datasets for inflectional morphology
  ([unimorph.github.io](https://unimorph.github.io/)): triples *(lemma, inflected form, feature
  bundle)*.
- **Formalism:** the UniMorph feature schema (universal inventory of morphological features).
- **Coverage:** **169+ languages** annotated ([UniMorph 4.0](https://arxiv.org/pdf/2205.03608)).
- **Correctness vs annotation:** data, not a checker — but the tables directly support
  **word-form validation** (is this form a valid inflection of this lemma with these features?).
- **Licenses:** open source; per-language repos mostly **CC BY-SA 3.0** (much data derives from
  Wiktionary) *(per-repo license must be checked; unverified globally)*.
- **Rust:** plain TSV — trivially loadable; no dedicated crate needed.
- **Test corpora:** SIGMORPHON shared-task splits (e.g.
  [2022InflectionST](https://github.com/sigmorphon/2022InflectionST)).
- **Standout ideas:** a universal morphological feature vocabulary aligned with UD features —
  the natural feature alphabet for morphology links in a links network.

---

## Shared concept spaces and interlinguas

### WordNet, Open English WordNet, Open Multilingual Wordnet (OMW) + CILI

- **What it does:** Princeton WordNet groups words into **synsets** (concepts) linked by
  hypernymy etc.; [OMW](https://omwn.org/) federates 40+ open wordnets;
  [CILI](https://github.com/globalwordnet/cili) (Collaborative Interlingual Index) assigns a
  **stable language-independent ILI identifier to every concept**, so wordnets in different
  languages link to the same ILI only when the concept matches
  ([CILI paper](https://aclanthology.org/2016.gwc-1.9/)).
- **Coverage:** OMW: 40+ languages with open licenses; Open English WordNet
  ([globalwordnet/english-wordnet](https://github.com/globalwordnet/english-wordnet), CC BY 4.0)
  is the maintained English hub; CILI itself is CC BY.
- **Exact-match semantics:** CILI's policy is essentially issue #47's rule: a new concept gets a
  **new ILI id** unless it exactly matches an existing one; ILI definitions are frozen to prevent
  drift.
- **Data & licenses:** open (CC BY / wordnet licenses); machine-readable **WN-LMF XML** and RDF
  ([Global WordNet formats](https://globalwordnet.github.io/schemas/)); Python
  [`wn`](https://github.com/goodmami/wn) library implements interlingual queries via ILI.
- **Rust:** WN-LMF is plain XML; small wordnet crates exist but none is canonical *(unverified)*.
- **Test corpora:** SemCor and wordnet gloss corpora (English); per-wordnet validation in OMW.
- **Standout ideas:** the **ILI = concept-as-link-id** model; "reuse only on exact match,
  otherwise mint a new id" is institutionalized here. The single most directly relevant design.

### BabelNet

- **What it does:** merges WordNet, Wikipedia, Wikidata, Wiktionary etc. into multilingual
  synsets ([babelnet.org/about](https://babelnet.org/about)).
- **Coverage:** v5.3: **600 languages, ~23M synsets, ~1.7B senses**
  ([statistics](https://babelnet.org/statistics)).
- **Exact match:** synsets merge senses across resources — merging is automatic and noisy, the
  *opposite* of strict exact-match minting.
- **Licenses:** **CC BY-NC-SA 3.0 + custom non-commercial research license**, API-key gated
  ([license](https://babelnet.org/license)) — **not redistributable / not usable** in an
  open-source MIT-style project. Treat as inspiration only.
- **Rust:** HTTP API only.
- **Standout ideas:** scale of automatic sense linking; Babel synset ids as cross-resource glue.

### ConceptNet

- **What it does:** commonsense knowledge graph of words/phrases
  ([conceptnet.io](https://conceptnet.io/)); edges like `/c/en/dog —IsA→ /c/en/animal`.
- **Coverage:** **304 languages** (deep coverage in ~10); ~34M assertions in 5.7
  ([release post](http://blog.conceptnet.io/posts/2019/conceptnet-57-released/)).
- **Exact match:** nodes are **surface terms, not disambiguated senses** — `/c/en/bank` is one
  node for all meanings. Fails issue #47's exact-match requirement at the concept level.
- **Licenses:** **CC BY-SA 4.0**, bulk TSV download
  ([downloads](https://github.com/commonsense/conceptnet5/wiki/Downloads)) — open and usable
  (share-alike applies to derived data).
- **Rust:** plain TSV; no canonical crate.
- **Standout ideas:** URI scheme for multilingual terms; relation vocabulary usable as link
  types; cross-lingual `SynonymOf`/`TranslationOf` edges as *candidate* (not exact) alignments.

### Wikidata lexemes

- **What it does:** Wikidata's lexicographical layer
  ([Lexicographical data](https://www.wikidata.org/wiki/Wikidata:Lexicographical_data)): Lexeme
  (L-id) → Forms (inflections with features) and Senses; a Sense can link via "item for this
  sense" (P5137) to a Wikidata **Q-item**, which is the language-independent concept.
- **Coverage:** **1.3M+ lexemes** across hundreds of languages
  ([stats](https://www.wikidata.org/wiki/Wikidata:Lexicographical_data/Statistics/Count_of_lexemes,_forms,_and_senses_by_language));
  uneven per language.
- **Exact match:** Q-items are minted per distinct concept with notability rules; sense→Q-item
  links are explicit editorial assertions — a workable exact-match discipline, community-governed.
- **Licenses:** **CC0 (public domain)** for all lexeme/item data
  ([licensing](https://www.wikidata.org/wiki/Wikidata:Licensing)) — the most permissive concept
  space available, safe for any downstream license.
- **Rust:** JSON dumps + SPARQL; community crates for Wikidata JSON exist (e.g. `wikidata` on
  crates.io) *(crate quality unverified)*.
- **Test corpora:** none as such; data is the resource.
- **Standout ideas:** separation **Lexeme (language-bound) vs Q-item (language-free concept)**
  with an explicit link between them = precisely the two-layer architecture issue #47 needs;
  Forms with feature bundles double as morphological validation data.

### Abstract Meaning Representation (AMR)

- **What it does:** sentence-level semantic graphs (PENMAN notation) abstracting away syntax
  ([amr.isi.edu](https://amr.isi.edu/)); concepts mostly PropBank framesets.
- **Coverage:** English-centric; AMR 3.0 sembank = **59,255 sentences**, distributed by **LDC
  under fee/membership license** ([LDC2020T02](https://catalog.ldc.upenn.edu/LDC2020T02)) —
  annotation *guidelines* are public, the corpus is not freely redistributable.
- **Exact match:** AMR explicitly does **not** aim at interlingual identity (English-biased
  concept inventory).
- **Rust:** PENMAN format is easy to parse; no canonical crate.
- **Standout ideas:** graphs (re-entrancy for coreference) rather than trees; concept+role
  normalization shows what a "semantic layer above grammar" looks like — out of scope for #47's
  no-semantics rule but relevant later.

### Uniform Meaning Representation (UMR)

- **What it does:** AMR's multilingual, document-level successor
  ([umr4nlp.github.io](https://umr4nlp.github.io/web/)); adds aspect, scope, modality,
  inter-sentential coreference; designed for low-resource languages.
- **Coverage:** UMR v2.0: **8 languages** (English, Chinese, Czech, Latin, Arapaho, Kukama,
  Navajo, Sanapaná), **210k+ sentence-level UMRs**
  ([data page](https://umr4nlp.github.io/web/data.html),
  [infrastructure paper](https://openreview.net/forum?id=Vb4547RFqJ)).
- **Licenses:** released openly (LINDAT/GitHub; CC-style) *(exact license per release
  unverified)* — unlike AMR's LDC gating.
- **Standout ideas:** lattice-based feature values (languages pick granularity along shared
  lattices) — a principled way to share a concept/feature space *without* forcing false exact
  matches; directly relevant to "reuse only on exact match".

### UNL (Universal Networking Language)

- **What it does:** 1996 UNU initiative: sentences as hypergraphs of **Universal Words** (UWs)
  linked by ~40 semantic relations and attributes
  ([Wikipedia](https://en.wikipedia.org/wiki/Universal_Networking_Language),
  [UNL archive](https://www.unlarchive.org/wiki/Introduction_to_UNL)).
- **Status:** effectively **defunct** — UNDL Foundation dissolved (project wind-down reported
  2015–2024; [archive](http://www.unlweb.net/)); resources scattered, licenses unclear. Historical
  prior art only.
- **Exact match:** UWs are English headwords + constraint lists (`drink(icl>liquor)`) — concepts
  are *named*, human-readable, and disambiguated by typed constraints rather than opaque ids.
- **Standout ideas:** self-describing concept identifiers (headword + constraints) and the
  cautionary tale: a centrally-managed closed interlingua dies with its institution. Favor
  open, community-minted ids (CILI/Wikidata) instead.

### Abstract Wikipedia / Wikifunctions

- **What it does:** Wikimedia project to store encyclopedic content as language-independent
  **abstract content** (constructors) rendered into natural languages by **Wikifunctions**
  renderer functions using **Wikidata lexemes**
  ([Abstract Wikipedia](https://meta.wikimedia.org/wiki/Abstract_Wikipedia),
  [NLG architecture proposal](https://meta.wikimedia.org/wiki/Abstract_Wikipedia/Natural_language_generation_system_architecture_proposal)).
- **Status (2026-06):** active; 2025 "fragment experiments" generate sentence fragments in
  multiple languages from lexemes
  ([2025 fragment experiments](https://www.wikifunctions.org/wiki/Wikifunctions:Abstract_Wikipedia/2025_fragment_experiments));
  soft launch targeted 2026 ([status updates](https://www.wikifunctions.org/wiki/Wikifunctions:Status_updates)).
- **Licenses:** Wikifunctions code/content under free licenses (Apache-2/CC); data CC0 via
  Wikidata.
- **Standout ideas:** the **closest living competitor** to issue #47's end goal: abstract
  content + per-language renderer functions + CC0 lexeme layer. Its NLG architecture debate
  (template renderers vs GF-style grammars, Ninai/Udiron tree toolkit) is required reading.
  Currently generation-only — it does not *parse* natural language, which leaves #47's parsing
  niche open.

### FrameNet

- **What it does:** lexicon of **semantic frames** (events/situations with roles) and
  frame-annotated sentences ([framenet.icsi.berkeley.edu](https://framenet.icsi.berkeley.edu/framenet_data)).
- **Coverage:** English release 1.7: **~1,221 frames, ~13.6k lexical units**
  ([FrameNet at 25](http://sites.la.utexas.edu/hcb/files/2025/05/Boas-et-al-2025-FrameNet.pdf));
  independent framenets exist for Swedish, Japanese, Brazilian Portuguese, German, etc. (not one
  shared id space).
- **Licenses:** data freely downloadable; release 1.7 under CC BY 4.0 *(license version
  unverified — confirm on download form)*.
- **Standout ideas:** frames as interlingual-ish *event concepts* with typed roles; Multilingual
  FrameNet alignment shows frames only partially match across languages — evidence for strict
  exact-match policies.

### PropBank

- **What it does:** verb sense inventory ("rolesets", e.g. `run.01`) with numbered arguments;
  the concept vocabulary underlying AMR/UMR
  ([propbank.github.io](https://propbank.github.io/)).
- **Coverage:** English frames v3.4 (~11k rolesets *(count unverified)*); Universal Proposition
  Banks project transfer to ~20 languages
  ([UP-1.0](https://github.com/UniversalPropositions/UP-1.0)).
- **Licenses:** frame files **CC BY-SA 4.0**
  ([propbank-frames](https://github.com/propbank/propbank-frames)); annotations are pointers —
  full text requires LDC corpora (OntoNotes).
- **Standout ideas:** stable, versioned, machine-readable XML sense ids that other projects
  (AMR, UMR) reuse — a working example of a shared concept id registry with governance.

---

## Recommended approach signals

For a Rust links-network engine needing (1) grammatical-correctness parsing of the top-10
languages and (2) an exact-match shared concept space:

### Grammatical-correctness parsing

1. **GF + RGL is the only off-the-shelf system that does exactly what issue #47 asks** —
   parse-or-reject grammaticality with *no* semantic checks, for 40+ languages including the
   top 10, under LGPL/BSD, with a compiled grammar format (PGF/JSON) already consumable from Rust
   via [`gf-core`](https://crates.io/crates/gf-core). The abstract/concrete split maps naturally
   onto a links network (abstract tree nodes = shared links; linearizations = per-language
   links). Main risks: `gf-core` is young (consider implementing a native PMCFG/PGF reader on
   links), and RGL coverage is "core grammar", not robust open-text parsing.
2. **Layer the morphology from open FST/table data**: Apertium `.dix` (GPL — keep as optional
   data), UniMorph TSV (CC BY-SA), and **Wikidata lexeme Forms (CC0)** give word-level
   correctness (valid form ⇔ exists with features) without any C++ dependency. CC0 lexemes are
   the safest default.
3. **Use UD as the naming standard, not the checker**: adopt UPOS/UFeats/UD relations as the
   link-type vocabulary so every UD treebank (179 languages) becomes import/test data, and UD
   test splits become regression corpora — while grammaticality itself is decided by the
   GF-style generative layer. Rust crates `rs-conllu`/`conllu` already read the format.
4. **Borrow negative-evidence rules**: LanguageTool XML rules (LGPL) with their embedded
   example sentences provide thousands of ready-made pass/fail test cases per language; nlprule
   proves the engine fits in Rust. DELPH-IN's "mal-rules" pattern (explicit error rules alongside
   the grammar) is the right mechanism for *explaining* why a parse fails.
5. **Avoid as cores**: XLE (proprietary), neural UD parsers (annotate-only, Python/C++ stacks,
   some NC-licensed models) — usable as offline preprocessing/bootstrapping tools only.

### Exact-match shared concept space

1. **Adopt the two-layer Wikidata model**: language-bound *lexeme* (lemma + forms + features)
   linked to a language-free *concept id*. Wikidata lexemes + Q-items are **CC0**, multilingual,
   and continuously growing — the only major concept space with zero licensing friction.
2. **Use CILI as the minting discipline and secondary id space**: Open English WordNet (CC BY)
   + OMW wordnets keyed by **ILI ids** implement "reuse only on exact match, else mint a new id"
   today; WN-LMF XML is easy to load in Rust. Store ILI and Q-ids as aliases of one links-node.
3. **PropBank frames (CC BY-SA) and FrameNet frames** are good optional registries for
   *event/predicate* concepts when verbs need argument-structure identity, but keep them out of
   the grammaticality layer (#47 forbids semantic checks).
4. **Reject BabelNet** (non-commercial, API-gated) and **ConceptNet as a concept layer**
   (surface words, not senses; CC BY-SA contamination risk) — ConceptNet is acceptable only as
   candidate-alignment evidence. **UNL** is a cautionary tale: closed governance killed it.
5. **Watch Abstract Wikipedia/Wikifunctions** (soft launch ~2026): same architecture
   (abstract content + lexemes + renderers), CC0-adjacent, but generation-only — meta-language's
   differentiator is *bidirectional* parse/generate over a lossless links substrate, with
   GF-style abstract syntax for grammar and Wikidata/CILI ids for the exact-match concept space.

### Licensing summary table

| Resource | License | Safe for permissive Rust crate? |
|---|---|---|
| GF RGL / runtime | LGPL + BSD | Yes (link as data/runtime; check per-module) |
| `gf-core` crate | MIT/Apache *(verify)* | Yes |
| UD treebanks | mixed CC BY-SA / CC BY-NC-SA | Per-treebank; tests only is safest |
| LanguageTool rules / nlprule bins | LGPL 2.1 | As external data files, yes |
| Apertium data | GPL | Only as optional external data |
| UniMorph | mostly CC BY-SA 3.0 | Yes, with attribution/share-alike on data |
| ERG / DELPH-IN | MIT-style *(verify)* | Yes |
| XLE | proprietary | No |
| Wikidata (lexemes + items) | **CC0** | Yes — preferred |
| OEWN / OMW / CILI | CC BY (mostly) | Yes |
| PropBank frames | CC BY-SA 4.0 | Yes (share-alike on data) |
| FrameNet 1.7 | CC BY *(verify version)* | Likely yes |
| ConceptNet | CC BY-SA 4.0 | Data layer only, share-alike |
| BabelNet | CC BY-NC-SA + custom | **No** |
| AMR 3.0 corpus | LDC | No (guidelines only) |
| UMR corpora | open *(verify per release)* | Likely yes |
