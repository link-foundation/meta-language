# The Links Theory 0.0.2

**Author:** [Константин Дьяченко (Konard)](https://habr.com/en/users/Konard/)

**Type:** Translation

**Original article:** [Vasily Solopov, Roman Vertushkin, Ivan Glazunov, Konstantin Diachenko](https://habr.com/ru/companies/deepfoundation/articles/804617/)

**Published:** April 1, 2025 (updated August 10, 2025)

**Reading time: 27 min | Difficulty: Medium | Views: 5259**

**Hubs:** Data Engineering, Open source, Mathematics, Abnormal programming, Programming

**Tags:** metatheory, links theory, relational theory, associative theory, math, set theory, type theory, graph theory, relational algebra, associative model of data

---

Last April 1st, as you might have guessed, we were joking. It’s time to fix that, and now everything is $100\\%$ serious.

#### TL;DR (too long; didn't read)

This article contains many letters, but it can be represented using just 4 symbols from [set theory](https://en.wikipedia.org/wiki/Set_theory):

> $\displaystyle L \to L^2$

Everything else follows from them.

## Overview

This article is primarily aimed at programmers and mathematicians, yet we’ve designed it to be accessible to anyone interested in the ideas it presents. We believe that the concepts discussed here can serve as inspiration across a wide range of scientific disciplines.

Our goal was to create a self-contained text that guides you through each topic in a clear, logical order. Throughout the article, you’ll find links to [Wikipedia](https://www.wikipedia.org) for those who wish to explore specific terms or concepts in more depth — but this is entirely optional. The text is intended to be easily understood when read from start to finish.

Every symbol and formula is explained individually, with concise definitions provided where needed. We’ve also included images to help illustrate key ideas. If you come across anything that isn’t clear, please let us know so we can improve it.

## Comparison of theories

To quickly dive in, we begin by comparing the mathematical foundations of the two most popular [data models](https://en.wikipedia.org/wiki/Data_model) with that of the [associative model of data](https://web.archive.org/web/20210814063207/https://en.wikipedia.org/wiki/Associative_model_of_data).

In the course of our research, we discovered that traditional theories were sometimes overly complex or redundant, while at other times they imposed too many artificial constraints.

This overall lack of flexibility, adaptability, and universality motivated us to search for a simpler yet all-encompassing informational theory and a data storage model that future artificial intelligence could easily understand and effectively utilize. Along the way, we drew inspiration from the workings of our own associative memory and associative thought processes.

### Relational Algebra

[Relational algebra](https://en.wikipedia.org/wiki/Relational_algebra) and the [relational model](https://en.wikipedia.org/wiki/Relational_model) are based on the concepts of [relations](https://en.wikipedia.org/wiki/Relation_(database)) and [n-tuples](https://en.wikipedia.org/wiki/Tuple).

A [relation](https://en.wikipedia.org/wiki/Relation_(database)) is defined as a [set](https://en.wikipedia.org/wiki/Set_(mathematics)) of [n-tuples](https://en.wikipedia.org/wiki/Tuple):

> $\mathbf{R \subseteq S_1 \times S_2 \times \dots \times S_n.}$ [[1]](https://dl.acm.org/doi/abs/10.1145/362384.362685)

![Figure 1](images/figure-1.png)

**Figure 1.** A table is described by a [relation](https://en.wikipedia.org/wiki/Relation_(database)) $\mathbf{R}$, which is represented as a [set](https://en.wikipedia.org/wiki/Set_(mathematics)) of [rows](https://en.wikipedia.org/wiki/Row_(database)) $\mathbf{r}$, belonging to a [Cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) $\mathbf{S_1 \times S_2 \times \dots \times S_n}$.

**Where:**

- The [symbol](https://en.wikipedia.org/wiki/Glossary_of_mathematical_symbols) $\mathbf{R}$ denotes a [relation](https://en.wikipedia.org/wiki/Relation_(mathematics)) ([table](https://en.wikipedia.org/wiki/Table_(database)));
- The [symbol](https://en.wikipedia.org/wiki/Glossary_of_mathematical_symbols) $\subseteq$ indicates that the left side of the [expression](https://en.wikipedia.org/wiki/Expression_(mathematics)) is a [subset](https://en.wikipedia.org/wiki/Subset) of the right side;
- The [symbol](https://en.wikipedia.org/wiki/Glossary_of_mathematical_symbols) $\times$ denotes the [Cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) of two [sets](https://en.wikipedia.org/wiki/Set_(mathematics));
- The [expression](https://en.wikipedia.org/wiki/Expression_(mathematics)) $\mathbf{S_n}$ represents the [domain](https://en.wikipedia.org/wiki/Data_domain), i.e., the [set](https://en.wikipedia.org/wiki/Set_(mathematics)) of all possible [values](https://en.wikipedia.org/wiki/Value_(computer_science)) that each [cell](https://en.wikipedia.org/wiki/Table_(database)) in a [column](https://en.wikipedia.org/wiki/Table_(database)) can contain.

[Rows](https://en.wikipedia.org/wiki/Row_(database)), or [elements](https://en.wikipedia.org/wiki/Element_(mathematics)) of the [relation](https://en.wikipedia.org/wiki/Relation_(database)) $\mathbf{R}$, are represented as [n-tuples](https://en.wikipedia.org/wiki/Tuple).

[Data](https://en.wikipedia.org/wiki/Data_(computer_science)) in the [relational model](https://en.wikipedia.org/wiki/Relational_model) is grouped into [relations](https://en.wikipedia.org/wiki/Relation_(database)). By using [n-tuples](https://en.wikipedia.org/wiki/Tuple) in this [model](https://en.wikipedia.org/wiki/Database_model), one can precisely represent any conceivable [data structure](https://en.wikipedia.org/wiki/Data_structure), if only we actually ever used [n-tuples](https://en.wikipedia.org/wiki/Tuple) for that. And are [n-tuples](https://en.wikipedia.org/wiki/Tuple) even necessary? For example, every [n-tuple](https://en.wikipedia.org/wiki/Tuple) can be represented as [nested ordered pairs](https://en.wikipedia.org/wiki/Tuple#Tuples_as_nested_ordered_pairs), which suggests that [ordered pairs](https://en.wikipedia.org/wiki/Ordered_pair) alone might be sufficient to represent any [data](https://en.wikipedia.org/wiki/Data_(computer_science)). Moreover, it’s uncommon for [column](https://en.wikipedia.org/wiki/Table_(database)) values in [tables](https://en.wikipedia.org/wiki/Table_(database)) to be represented as [n-tuples](https://en.wikipedia.org/wiki/Tuple) (although, for instance, a [number](https://en.wikipedia.org/wiki/Number) can be [decomposed](https://en.wikipedia.org/wiki/Decomposition_(computer_science)) into an [n-tuple](https://en.wikipedia.org/wiki/Tuple) of [bits](https://en.wikipedia.org/wiki/Bit)). In some [SQL](https://en.wikipedia.org/wiki/SQL) [databases](https://en.wikipedia.org/wiki/Database), it is even forbidden to use more than $\mathbf{32}$ [columns](https://en.wikipedia.org/wiki/Table_(database)) in a [table](https://en.wikipedia.org/wiki/Table_(database)) (and, by extension, in its corresponding [n-tuple](https://en.wikipedia.org/wiki/Tuple)). Thus, the actual value of $\mathbf{n}$ is usually lower than $\mathbf{32}$. Therefore, in these cases, there are no [true](https://en.wikipedia.org/wiki/Truth) [n-tuples](https://en.wikipedia.org/wiki/Tuple) — even in modern [relational](https://en.wikipedia.org/wiki/Relational_model) [databases](https://en.wikipedia.org/wiki/Database).

![Figure 2](images/figure-2.png)

**Figure 2.** Comparison of the [relational model](https://en.wikipedia.org/wiki/Relational_model) and the [associative model of data](http://iacis.org/iis/2009/P2009_1301.pdf) (the original [model](https://en.wikipedia.org/wiki/Data_model) proposed by [Simon Williams](https://www.linkedin.com/in/s1m0n) was simplified by us twice) [[3]](https://web.archive.org/web/20181219134621/http://sentences.com/docs/amd.pdf). In other words, representing all [data](https://en.wikipedia.org/wiki/Data_(computer_science)) in the [relational model](https://en.wikipedia.org/wiki/Relational_model) requires a multitude of [tables](https://en.wikipedia.org/wiki/Table_(database)) — one for each [data type](https://en.wikipedia.org/wiki/Data_type) — whereas in the[ associative model](https://web.archive.org/web/20210814063207/https://en.wikipedia.org/wiki/Associative_model_of_data), it turned out that initially just two [tables](https://en.wikipedia.org/wiki/Table_(database)) were sufficient (`items` and `links`), and eventually just a single [table](https://en.wikipedia.org/wiki/Table_(database)) (`links`) of [triplet](https://en.wikipedia.org/wiki/Tuple)-links or [doublet](https://en.wikipedia.org/wiki/Ordered_pair)-links was enough.

### Directed Graph

Directed graphs — and [graphs](https://en.wikipedia.org/wiki/Graph_theory) in general — are based on the concepts of [vertices](https://en.wikipedia.org/wiki/Vertex_(graph_theory)) and [edges](https://en.wikipedia.org/wiki/Glossary_of_graph_theory#edge) ([2-tuples](https://en.wikipedia.org/wiki/Ordered_pair)).

[A directed graph](https://en.wikipedia.org/wiki/Directed_graph) $\mathbf{G}$ is defined as follows:

> $\mathbf{G = (V, E), \quad E \subseteq V \times V.}$ [[2]](https://books.google.com/books?id=vaXv_yhefG8C)

Where:

- $\mathbf{V}$ is a [set](https://en.wikipedia.org/wiki/Set_(mathematics)) whose elements are called [vertices](https://en.wikipedia.org/wiki/Vertex_(graph_theory)), nodes, or [points](https://en.wikipedia.org/wiki/Point_(geometry));
- $\mathbf{E}$ is a set of [ordered pairs](https://en.wikipedia.org/wiki/Ordered_pair) (2-[tuples](https://en.wikipedia.org/wiki/Tuple)) of [vertices](https://en.wikipedia.org/wiki/Vertex_(graph_theory)), referred to as arcs, directed [edges](https://en.wikipedia.org/wiki/Glossary_of_graph_theory#edge) (sometimes simply [edges](https://en.wikipedia.org/wiki/Glossary_of_graph_theory#edge)), arrows, or directed [lines segments](https://en.wikipedia.org/wiki/Line_segment).

In the directed graph model, data is represented by two separate [sets](https://en.wikipedia.org/wiki/Set_(mathematics)): [nodes](https://en.wikipedia.org/wiki/Vertex_(graph_theory)) and [edges](https://en.wikipedia.org/wiki/Glossary_of_graph_theory#edge). This [model](https://en.wikipedia.org/wiki/Data_model) can be used to represent almost all [data structures](https://en.wikipedia.org/wiki/Data_structure), except perhaps [sequences](https://en.wikipedia.org/wiki/Sequence) ([n-tuples](https://en.wikipedia.org/wiki/Tuple)). Sometimes, chains of vertices are used to represent sequences. Although this method works, it invariably leads to data duplication, and deduplication in such cases is either complicated or unfeasible. Furthermore, sequences in graphs might be represented by decomposing the [sequence into nested sets](https://en.wikipedia.org/wiki/Tuple#Tuples_as_nested_sets), but in our view, this is not a practical approach. It appears that we are not alone in this belief, which may explain why we have not encountered examples of others employing such method.

![Figure 3](images/figure-3.png)

**Figure 3.** Comparison of the graph theory and the links theory. A vertex is equivalent to [a self-referential link](https://linksplatform.github.io/itself.html) — a link that begins and ends in itself. A directed edge is represented as a directed doublet-links, while an undirected edge is represented as a pair of directed doublet-links in both opposite directions. In other words, while graph theory requires two types of entities — vertices and edges — in the links theory only links (which most closely resemble edges) are necessary.

### The links theory

The links theory is based on the concept of a link.

In the projection of the links theory into set theory, [a link](https://habr.com/ru/companies/deepfoundation/articles/576398) is defined as an [n-tuple](https://en.wikipedia.org/wiki/Tuple) of references to links, which has its own reference that other links can use to refer to it.

> It is worth noting that the separate notion of a reference is required here solely because [circular definitions](https://en.wikipedia.org/wiki/Circular_definition) are not available in set theory. In fact, the links theory can describe itself without needing a distinct term for a reference — in other words, a reference is simply a special case of a link.

#### Duplets

A doublet-link is represented by a duplet (2-tuple or [ordered pair](https://en.wikipedia.org/wiki/Ordered_pair)) of references to links. A doublet-link also has its own reference.

```python
L = { 1 , 2 }

L × L = {
  (1, 1),
  (1, 2),
  (2, 1),
  (2, 2),
}
```

**Where**:

- $L$ is the set of references (from the English word “Links” as in “References”).

In this example, the set $L$ contains only $2$ references to links, namely $1$ and $2$. In other words, in a network of links built on such a set of references, there can be only $2$ links.

To obtain all possible values of a link, [the Cartesian product](https://en.wikipedia.org/wiki/Cartesian_product) of $L$ with itself is used, i.e., $L \times L$.

![Figure 4](images/figure-4.png)

**Figure 4.** A matrix representing the Cartesian product of the set {1, 2} with itself. Here we see that links with two references to links can have only 4 possible values.

![Figure 5](images/figure-5.png)

**Figure 5.** A table of rows containing all possible variants of link values for a network with two links; these variants are obtained using the Cartesian product of {1, 2} with itself.

The **doublet-links network** is defined as:

> $\displaystyle \mathbf{\lambda: L \to L \times L}$

Where:

- $\to$ denotes [a mapping (function)](https://en.wikipedia.org/wiki/Function_(mathematics));
- $\mathbf{\lambda}$ represents the function that defines the duplet-links network;
- $\mathbf{L}$ denotes the set of references to links.

**Example**:

> $\displaystyle 1 \to (1, 1)$
>
> $\displaystyle 2 \to (2, 2)$
>
> $\displaystyle \mathbf{3 \to (1, 2)}$

![Figure 6](images/figure-6.png)

**Figure 6. A network of three links.** The representation of the duplet‑links network resembles a graph, but we refer to this visualization as a network of links. The first and second links have a similar structure — that is, both begin from themselves and end in themselves. As a result, instead of the traditional depiction of a vertex as a point in graph theory, we get a graphical representation of a closed self‑referential arrow that resembles an infinity symbol.

![Figure 7](images/figure-7.png)

**Figure 7.** This is a graphical representation of the Cartesian product in the form of a matrix, which displays all possible link values. The links that define a specific network are highlighted in orange. In other words, out of 9 possible link value variants, only 3 links are selected, corresponding to the size of the set **L**.

A network of doublet-links can represent any data structure.

For example, doublet-links can:

- Link an object with its properties;
- Connect two links together, which is something graph theory definition does not allow;
- Represent any sequence (n-tuple) as a tree built from nested ordered pairs;
- Describe a natural language sentence, for instance, using a [subject-predicate](https://en.wikipedia.org/wiki/Predicate_(grammar)) linguistics model.

Thanks to this and other facts, we believe that doublet-links can represent any conceivable data structure.

#### Triplets

A triplet-link is represented by a triplet (3-tuple) of references to links.

```python
L = { 1 , 2 }

L × L = {
  (1, 1),
  (1, 2),
  (2, 1),
  (2, 2),
}

L × L × L = {
  (1, 1, 1),
  (1, 1, 2),
  (1, 2, 1),
  (1, 2, 2),
  (2, 1, 1),
  (2, 1, 2),
  (2, 2, 1),
  (2, 2, 2),
}
```

![Figure 8](images/figure-8.png)

**Figure 8.** A three-dimensional cube-matrix that represents all possible values of a triplet-link. Such a cube is obtained by recursively taking the Cartesian product of the set {1, 2} with itself, i.e., { 1, 2 } × { 1, 2 } × { 1, 2 }.

![Figure 9](images/figure-9.png)

**Figure 9.** A table of all possible variants of triplet-link values that can be obtained by taking the Cartesian product of the set { 1, 2 } with itself recursively, i.e., { 1, 2 } × { 1, 2 } × { 1, 2 }. **Note:** The first reference can be interpreted as the beginning, the second as the type, and the third as the end; the user determines how to interpret the components of the reference vector in accordance with the task at hand.

A **triplet links network** is defined as:

> $\displaystyle \mathbf{\lambda : L \to L \times L \times L}$

Where:

- $\mathbf{\lambda}$ denotes the function that defines the triplet links network;
- $\mathbf{L}$ denotes the set of references to links.

Example of a function specifying a particular triplet links network:

> $\displaystyle 1 \to (1, 1, 1)$
>
> $\displaystyle 2 \to (2, 2, 2)$
>
> $\displaystyle 3 \to (3, 3, 3)$
>
> $\displaystyle \mathbf{4 \to (1, 2, 3)}$

![Figure 10](images/figure-10.jpg)

**Figure 10.** An associative triplet network represented as a colored directed graph. In this associative network, there are 4 triplet-links corresponding to the function defined above. The nodes correspond to links, and the edge colors correspond to references to links as shown in Figure 9 (red - from, blue - type, green - to).

Triplet-links can perform the same functions as doublet-links. Since triplet-links include an additional reference, that extra element can, for example, be used to indicate the type of link.

For instance, triplet-links can:

- Link an object, its property, and its value;
- Link two links together using a defined relation;
- Describe a natural language sentence, for example, using a [subject-verb-object](https://en.wikipedia.org/wiki/Subject%E2%80%93verb%E2%80%93object_word_order) model.

#### Sequences

A sequence of link references — also known as an [n-tuple](https://en.wikipedia.org/wiki/Tuple) — are the general case.

In general, a links network is defined as:

> $\displaystyle \mathbf{\lambda : L \rightarrow \underbrace{ L \times L \times \ldots \times L}_{n}}$

Where:

- The $\mathbf{\lambda}$ symbol denotes the function that defines the links network;
- The $\mathbf{L}$ symbol denotes the set of link references.

Example:

> $\displaystyle 1 \to (1)$
>
> $\displaystyle 2 \to (2, 2)$
>
> $\displaystyle 3 \to (3, 3, 3)$
>
> $\displaystyle \mathbf{4 \to (1, 2, 3, 2, 1)}$

In this example, n-tuples of variable lengths are used as link's values. 

Sequences (vectors) are, in essence, equivalent in expressive power to the relational model — a fact that remains to be proven within the developing theory. However, once we observed that doublet-links and triplet-links are sufficient for representing sequences of any size, we hypothesized that there is no need to use sequences directly as they representable by double-links.

### Comparison summary

The [relational data model](https://en.wikipedia.org/wiki/Relational_model) can represent everything — even [the associative model](https://web.archive.org/web/20210814063207/https://en.wikipedia.org/wiki/Associative_model_of_data), but to do so [well-ordering](https://en.wikipedia.org/wiki/Well-order) must be introduced, which usually comes in a form of separate ID column. As the relational data model is based on notion of [set](https://en.wikipedia.org/wiki/Set_(mathematics)) instead of a [sequence](https://en.wikipedia.org/wiki/Sequence). In contrast, the graph model excels at representing relationships, but is less effective at representing unique deduplicated sequences.

Although the relational model itself doesn’t require data to be split across several tables, traditional implementations typically adopt that approach with fixed schemas, which leads to fragmentation of related data and complicates the reconstruction of inherent relationships. In contrast, the associative model employs a unified links storage that achieves the highest possible degree of normalization. This design simplifies the one-to-one mapping of the business domain, thereby easing rapid requirement changes [[4]](https://www.researchgate.net/publication/255670856_A_COMPARISON_OF_THE_RELATIONAL_DATABASE_MODEL_AND_THE_ASSOCIATIVE_DATABASE_MODEL).

The associative model can easily represent n-tuples of unlimited length using tuples with $\mathbf{n \geq 2}$. It is as capable as graph theory in representing associations and as powerful as the relational model, being able to fully represent any SQL table. Additionally, the associative model can represent strict sequences, allowing any sequence to be encapsulated in a unique single link, which is beneficial for deduplication.

In the relational model, only one relation is needed to mimic the behavior of the associative model, and typically no more than 2-3 columns are required aside from an explicit ID or built-in row ID. The ID itself is required in relational model because it is built up on concept of set, for the contrast the links theory builds up on concept of sequence thus explicit ID is not required (and can optionally added if user really needs it).

By definition, the graph model cannot directly create an edge between edges. Thus, it would need either a redefinition or an extension with an unambiguous method for storing unique deduplicated sequences. While sequences might be stored as [nested sets](https://en.wikipedia.org/wiki/Tuple#Tuples_as_nested_sets) within the graph model, this approach is not popular. Although the graph model is closest to doublet-links, it still differs by definition.

Using the associative model means there is no longer a need to choose between SQL and NoSQL databases; instead, an associative data store can represent everything in the simplest possible way, with data always kept in its form closest to the original (usually this the normalized form, but denormalization also possible if needed).

## Mathematical introduction to the links theory

### Introduction

Now that we have briefly introduced the origins of our work, it is time to delve deeper into the theory.

> The links theory is being developed as a more fundamental framework compared to set theory or type theory, and as a replacement for relational algebra and graph theory as unifying theory. While type theory is built upon the basic notions of “type” and “term”, and set theory on “set” and “element”, the links theory reduces everything to the single concept of a “link”.

In this section, we will explain the core concepts and terminology used in the links theory. 

We will then present the definitions of the links theory within the framework of set theory, and subsequently project these definitions into type theory using the interactive theorem prover Coq.

Finally, we will summarize our findings and outline directions for further research and development of the links theory.

#### The links theory

At the heart of the links theory lies the unified concept of a link. The additional notion of a reference to a link is introduced only for theories that do not support circular definitions — such as set theory and type theory.

#### Link

A link possesses an asymmetrical recursive (fractal) structure, which can be simply expressed as: a link **links** links (as in “a link **connects** links”). The term “asymmetrical” refers to the fact that every link has a direction — from its source (beginning) to its destination (end).

### The links theory definitions within set theory framework

A **reference to a vector** is a unique identifier or ordinal number, which is associated with a specific vector representing a sequence of references to other vectors.

Set of references to vectors:

> $\displaystyle \mathbf{L ⊆ ℕ_0}$

A **vector of references** is a vector consisting of zero or more references to vectors, where the number of references corresponds to the number of elements in the vector.

Set of all vectors of references of length $n ∈ ℕ_0$:

> $\displaystyle \mathbf{V_n = L^n}$

The Cartesian power $L^n$ always produces a vector of length $n$ , since all its components are of the same type $L$.
In other words, $L^n$ represents the set of all possible n-element vectors (essentially n‑tuples), in which every element belongs to the set $L$.

An **association** is an ordered pair consisting of a reference to a vector and a vector of references. This structure serves as a mapping between references and vectors.

> $\displaystyle \mathbf{A = L \times V_n}$

An **associative network** of vectors of length **n** (or an n-dimensional associative network) is defined by a family of functions $\{anetv^n\}$, where each function $anetv^n: L → V_n$ maps a reference $l ∈ L$ to a vector of references of length $n$, belonging to $V_n$, thereby identifying points in an n-dimensional space.
$n$ in $anetv^n$ indicates that the function returns vectors containing $n$ references. Each n-dimensional associative network thus represents a sequence of points in n-dimensional space.

**Family of functions:**

> $\displaystyle \mathbf{∪_f \{anetv^n | n ∈ ℕ_0\} ⊆ A}$

Here, the union symbol $∪_f$ denotes the aggregation of all functions in the family $\{anetv^n\}$, and the symbol $⊆$ indicates that these ordered pairs — viewed as functional binary relations — are a subset of the set $A$ of all associations.

**Set of duplets (ordered pairs or 2-dimensional vectors) of references:**

> $\displaystyle \mathbf{D = L^2}$

This is the set of all duplets $(L, L)$, i.e., the second Cartesian power of $L$ .

**Associative network of duplets (or a 2-dimensional associative network):**

> $\displaystyle \mathbf{anetd: L → L^2}$

Each associative network of duplets thus represents a sequence of points in a two‑dimensional space.

An empty vector (vector of length zero) is represented by the empty tuple, denoted as $()$ or $∅$.

**Associative network of nested ordered pairs:**

> $\displaystyle \mathbf{anetl: L → NP}\textbf{, where }\mathbf{NP = \{(∅, ∅) | (l, np), l ∈ L, np ∈ NP\} }$

$NP$ is the set of nested ordered pairs consisting of empty pairs and pairs containing one or more elements. In this way, a vector of length $n \in \mathbb{N}_0$ can be represented as nested ordered pairs.

### Projection of the links theory into type theory (coq) via set theory

#### About Coq

[Coq](https://ru.wikipedia.org/wiki/Coq) is an interactive theorem prover based on higher-order type theory, also known as the Calculus of Inductive Constructions (CIC). It is a powerful environment for formalizing complex mathematical theorems, checking proofs for correctness, and extracting executable code from formally verified specifications. Coq is widely used both in academia for the formalization of mathematics and in the IT industry for the verification of software and hardware.

The decision to use Coq to describe the links theory within type theory was driven by the need for rigorous formalization of proofs and assurance of logical correctness during the development of the links theory. Coq enables the precise expression of properties and operations on links through its robust type system and advanced proof mechanisms.

In anticipation of extensive work aimed at proving the equivalence of the relational model and the associative network of doublets, this section presents the initial steps undertaken using the Coq proof system. In the first phase, our goal is to formalize the structures of associative networks by defining the basic types, functions, and structures within Coq.

#### Definitions of associative networks

[[Link to source code]](https://github.com/deep-foundation/deep-theory/blob/main/associative_proofs/coq/ANetDefs.v)

```coq
Require Import PeanoNat.
Require Import Coq.Init.Nat.
Require Import Vector.
Require Import List.
Require Import Coq.Init.Datatypes.
Import ListNotations.
Import VectorNotations.

(* Set of vector references: L ⊆ ℕ₀ *)
Definition L := nat.

(* Default value for L: zero *)
Definition LDefault : L := 0.

(* Set of vectors of references of length n ∈ ℕ₀: Vn ⊆ Lⁿ *)
Definition Vn (n : nat) := t L n.

(* Default value for Vn *)
Definition VnDefault (n : nat) : Vn n := Vector.const LDefault n.

(* Set of all associations: A = L × Vn *)
Definition A (n : nat) := prod L (Vn n).

(* Associative network of vectors of length n (or n-dimensional associative network) from the family of functions {anetvⁿ : L → Vn} *)
Definition ANetVf (n : nat) := L -> Vn n.

(* Associative network of vectors of length n (or n-dimensional associative network) as a sequence *)
Definition ANetVl (n : nat) := list (Vn n).

(* Nested ordered pairs *)
Definition NP := list L.

(* Associative network of nested ordered pairs: anetl : L → NP *)
Definition ANetLf := L -> NP.

(* Associative network of nested ordered pairs as a sequence of nested ordered pairs *)
Definition ANetLl := list NP.

(* Duplet of references *)
Definition D := prod L L.

(* Default value for D: a pair of two LDefault values, used to denote an empty duplet *)
Definition DDefault : D := (LDefault, LDefault).

(* Associative network of duplets (or two-dimensional associative network): anetd : L → L² *)
Definition ANetDf := L -> D.

(* Associative network of duplets (or two-dimensional associative network) as a sequence of duplets *)
Definition ANetDl := list D.
```

#### Functions for converting associative network

```coq
(* Function to convert Vn to NP *)
Fixpoint VnToNP {n : nat} (v : Vn n) : NP :=
  match v with
  | Vector.nil _ => List.nil
  | Vector.cons _ h _ t => List.cons h (VnToNP t)
  end.

(* Function to convert ANetVf to ANetLf *)
Definition ANetVfToANetLf {n : nat} (a: ANetVf n) : ANetLf :=
  fun id => VnToNP (a id).

(* Function to convert ANetVl to ANetLl *)
Definition ANetVlToANetLl {n: nat} (net: ANetVl n) : ANetLl :=
  map VnToNP net.

(* Function to convert NP to Vn, returning an option *)
Fixpoint NPToVnOption (n: nat) (p: NP) : option (Vn n) :=
  match n, p with
  | 0, List.nil => Some (Vector.nil nat)
  | S n', List.cons f p' => 
  match NPToVnOption n' p' with
  | None => None
  | Some t => Some (Vector.cons nat f n' t)
  end
  | _, _ => None
  end.

(* Function to convert NP to Vn using VnDefault *)
Definition NPToVn (n: nat) (p: NP) : Vn n :=
  match NPToVnOption n p with
  | None => VnDefault n
  | Some t => t
  end.

(* Function to convert ANetLf to ANetVf *)
Definition ANetLfToANetVf { n: nat } (net: ANetLf) : ANetVf n :=
  fun id => match NPToVnOption n (net id) with
  | Some t => t
  | None => VnDefault n
  end.

(* Function to convert ANetLl to ANetVl *)
Definition ANetLlToANetVl {n: nat} (net : ANetLl) : ANetVl n :=
  map (NPToVn n) net.

(* Function to convert NP to ANetDl with an index offset *)
Fixpoint NPToANetDl_ (offset: nat) (np: NP) : ANetDl :=
  match np with
  | nil => nil
  | cons h nil => cons (h, offset) nil
  | cons h t => cons (h, S offset) (NPToANetDl_ (S offset) t)
  end.

(* Function to convert NP to ANetDl *)
Definition NPToANetDl (np: NP) : ANetDl := NPToANetDl_ 0 np.

(* Function to append NP to the tail of ANetDl *)
Definition AddNPToANetDl (anet: ANetDl) (np: NP) : ANetDl :=
  app anet (NPToANetDl_ (length anet) np).

(* Function that removes the head of anetd and returns the tail starting at offset *)
Fixpoint ANetDl_behead (anet: ANetDl) (offset : nat) : ANetDl :=
  match offset with
  | 0 => anet
  | S n' =>
  match anet with
  | nil => nil
  | cons h t => ANetDl_behead t n'
  end
  end.

(* Function to convert ANetDl to NP with indexing starting at the beginning of ANetDl from offset *)
Fixpoint ANetDlToNP_ (anet: ANetDl) (offset: nat) (index: nat): NP :=
  match anet with
  | nil => nil
  | cons (x, next_index) tail_anet =>
  if offset =? index then
  cons x (ANetDlToNP_ tail_anet (S offset) next_index)
  else
  ANetDlToNP_ tail_anet (S offset) index
  end.

(* Function to read NP from ANetDl by the duplet index *)
Definition ANetDl_readNP (anet: ANetDl) (index: nat) : NP :=
  ANetDlToNP_ anet 0 index.

(* Function to convert ANetDl to NP starting from the head of the anet list *)
Definition ANetDlToNP (anet: ANetDl) : NP := ANetDl_readNP anet 0.

(*
  Now everything is ready for converting the associative network of nested ordered pairs anetl : L → NP
  into the associative network of duplets anetd : L → L².
  
  This conversion can be done in different ways: either preserving the original references to vectors
  or with reindexing. Reindexing can be omitted if one writes an additional function for the duplet associative network
  that returns the nested ordered pair by its reference.
*)

(* Function to add ANetLl to ANetDl *)
Fixpoint AddANetLlToANetDl (anetd: ANetDl) (anetl: ANetLl) : ANetDl :=
  match anetl with
  | nil => anetd
  | cons h t => AddANetLlToANetDl (AddNPToANetDl anetd h) t
  end.

(* Function to convert ANetLl to ANetDl *)
Definition ANetLlToANetDl (anetl: ANetLl) : ANetDl :=
  match anetl with
  | nil => nil
  | cons h t => AddANetLlToANetDl (NPToANetDl h) t
  end.

(* Function to find NP in the tail of ANetDl starting at offset by its ordinal number.
  Returns the NP offset. *)
Fixpoint ANetDl_offsetNP_ (anet: ANetDl) (offset: nat) (index: nat) : nat :=
  match anet with
  | nil => offset + (length anet)
  | cons (_, next_index) tail_anet =>
  match index with
  | O => offset
  | S index' => 
  if offset =? next_index then
  ANetDl_offsetNP_ tail_anet (S offset) index'
  else
  ANetDl_offsetNP_ tail_anet (S offset) index
  end
  end.

(* Function to find NP in ANetDl by its ordinal number.
  Returns the NP offset. *)
Definition ANetDl_offsetNP (anet: ANetDl) (index: nat) : nat :=
  ANetDl_offsetNP_ anet 0 index.

(* Function to convert ANetVl to ANetDl *)
Definition ANetVlToANetDl {n : nat} (anetv: ANetVl n) : ANetDl :=
  ANetLlToANetDl (ANetVlToANetLl anetv).

(*
  Now everything is ready for converting the duplet associative network anetd : L → L²
  into the associative network of nested ordered pairs anetl : L → NP.
  
  We will perform this conversion while preserving the original references to vectors.
  Reindexing can be omitted because there is the function ANetDl_offsetNP for the duplet associative network
  that returns the offset of the nested ordered pair by its reference.
*)

(* Function that removes the first NP from ANetDl and returns the tail *)
Fixpoint ANetDl_beheadNP (anet: ANetDl) (offset: nat) : ANetDl :=
  match anet with
  | nil => nil
  | cons (_, next_index) tail_anet =>
  if offset =? next_index then (* end of NP *)
  tail_anet
  else (* NP not ended yet *)
  ANetDl_beheadNP tail_anet (S offset)
  end.

(* Function to convert NP and ANetDl with an offset into ANetLl *)
Fixpoint ANetDlToANetLl_ (anetd: ANetDl) (np: NP) (offset: nat) : ANetLl :=
  match anetd with
  | nil => nil (* discard NP even if incomplete *)
  | cons (x, next_index) tail_anet =>
  if offset =? next_index then (* end of NP, move to the next NP *)
  cons (app np (cons x nil)) (ANetDlToANetLl_ tail_anet nil (S offset))
  else (* NP not finished yet, continue parsing the duplet network *)
  ANetDlToANetLl_ tail_anet (app np (cons x nil)) (S offset)
  end.

(* Function to convert ANetDl to ANetLl *)
Definition ANetDlToANetLl (anetd: ANetDl) : ANetLl :=
  ANetDlToANetLl_ anetd nil LDefault.
```

#### Predicates of equivalence for associative networks

```coq
(* The definition ANetVf_equiv introduces a predicate for the equivalence of two associative networks of vectors of length n,
  anet1 and anet2 of type ANetVf.

  This predicate describes the property of "equivalence" for such networks.
  It asserts that anet1 and anet2 are considered "equivalent" if, for every reference id, the vector associated with id in anet1
  exactly matches the vector associated with the same id in anet2.
*)
Definition ANetVf_equiv {n: nat} (anet1: ANetVf n) (anet2: ANetVf n) : Prop :=
  forall id, anet1 id = anet2 id.

(* The definition ANetVl_equiv_Vl introduces a predicate for the equivalence of two associative networks of vectors of length n,
  anet1 and anet2 of type ANetVl.
*)
Definition ANetVl_equiv_Vl {n: nat} (anet1: ANetVl n) (anet2: ANetVl n) : Prop :=
  anet1 = anet2.

(* Equivalence predicate for associative networks of duplets ANetDf *)
Definition ANetDf_equiv (anet1: ANetDf) (anet2: ANetDf) : Prop := forall id, anet1 id = anet2 id.

(* Equivalence predicate for associative networks of duplets ANetDl *)
Definition ANetDl_equiv (anet1: ANetDl) (anet2: ANetDl) : Prop := anet1 = anet2.
```

#### Lemmas of equivalence of associative networks

```coq
(* Lemma on preservation of vector length in the associative network *)
Lemma Vn_dim_preserved : forall {l: nat} (t: Vn l), List.length (VnToNP t) = l.
Proof.
  intros l t.
  induction t.
  - simpl. reflexivity.
  - simpl. rewrite IHt. reflexivity.
Qed.


(* Lemma on the mutual inversion of the functions NPToVnOption and VnToNP

  H_inverse proves that every Vn vector can be converted losslessly to an NP
  using VnToNP and then back to Vn using NPToVnOption.

  Formally, forall n: nat, forall t: Vn n, NPToVnOption n (VnToNP t) = Some t states that
  for every natural number n and each Vn vector of length n,
  we can convert Vn to NP using VnToNP,
  then convert the result back to Vn using NPToVnOption n,
  and ultimately obtain the same Vn vector we started with.

  This property is very important because it guarantees that these two functions
  form an inverse pair on the set of convertible vectors Vn and NP.
  When you apply both functions to values in this set, you end up with the original value.
  This means that no information is lost during the transformations,
  so you can freely convert between Vn and NP as required in implementations or proofs.
*)
Lemma H_inverse: forall n: nat, forall t: Vn n, NPToVnOption n (VnToNP t) = Some t.
Proof.
  intros n.
  induction t as [| h n' t' IH].
  - simpl. reflexivity.
  - simpl. rewrite IH. reflexivity.
Qed.


(*
  The Wrapping and Recovery Theorem for the Associative Network of Vectors:

  Let an associative network of vectors of length n be given, denoted as anetvⁿ : L → Vⁿ.
  Define an operation that maps this network to the associative network of nested ordered pairs anetl : L → NP,
  where NP = {(∅,∅) | (l, np), l ∈ L, np ∈ NP}.
  Then define the inverse mapping from the associative network of nested ordered pairs back to the associative network of vectors of length n.

  The theorem states:

  For any associative network of vectors of length n, anetvⁿ, applying the transformation to the associative network
  of nested ordered pairs and then the inverse transformation back to the associative network of vectors of length n
  recovers the original network anetvⁿ.
  In other words:

  ∀ anetvⁿ : L → Vⁿ, inverse(forward(anetvⁿ)) = anetvⁿ.
*)
Theorem anetf_equiv_after_transforms : forall {n: nat} (anet: ANetVf n),
  ANetVf_equiv anet (fun id => match NPToVnOption n ((ANetVfToANetLf anet) id) with
  | Some t => t
  | None => anet id
  end).
Proof.
  intros n net id.
  unfold ANetVfToANetLf.
  simpl.
  rewrite H_inverse.
  reflexivity.
Qed.


(* Lemma on preservation of the length of NP lists in the duplet associative network *)
Lemma NP_dim_preserved : forall (offset: nat) (np: NP), 
  length np = length (NPToANetDl_ offset np).
Proof.
  intros offset np.
  generalize dependent offset. 
  induction np as [| n np' IHnp']; intros offset.
  - simpl. reflexivity.
  - destruct np' as [| m np'']; simpl; simpl in IHnp'.
  + reflexivity.
  + rewrite IHnp' with (offset := S offset). reflexivity.
Qed.
```

#### Examples of conversions between associative networks

```coq
(* Notation for list notation *)
Notation "{ }" := (nil) (at level 0).
Notation "{ x , .. , y }" := (cons x .. (cons y nil) ..) (at level 0).

(* Three-dimensional associative network *)
Definition complexExampleNet : ANetVf 3 :=
  fun id => match id with
  | 0 => [0; 0; 0]
  | 1 => [1; 1; 2]
  | 2 => [2; 4; 0]
  | 3 => [3; 0; 5]
  | 4 => [4; 1; 1]
  | S _ => [0; 0; 0]
  end.

(* Vectors of references *)
Definition exampleTuple0 : Vn 0 := [].
Definition exampleTuple1 : Vn 1 := [0].
Definition exampleTuple4 : Vn 4 := [3; 2; 1; 0].

(* Conversion of vectors of references into nested ordered pairs (lists) *)
Definition nestedPair0 := VnToNP exampleTuple0.
Definition nestedPair1 := VnToNP exampleTuple1.
Definition nestedPair4 := VnToNP exampleTuple4.

Compute nestedPair0. (* Expected result: { } *)
Compute nestedPair1. (* Expected result: {0} *)
Compute nestedPair4. (* Expected result: {3, 2, 1, 0} *)

(* Computing the values of the converted function of the three-dimensional associative network *)
Compute (ANetVfToANetLf complexExampleNet) 0. (* Expected result: {0, 0, 0} *)
Compute (ANetVfToANetLf complexExampleNet) 1. (* Expected result: {1, 1, 2} *)
Compute (ANetVfToANetLf complexExampleNet) 2. (* Expected result: {2, 4, 0} *)
Compute (ANetVfToANetLf complexExampleNet) 3. (* Expected result: {3, 0, 5} *)
Compute (ANetVfToANetLf complexExampleNet) 4. (* Expected result: {4, 1, 1} *)
Compute (ANetVfToANetLf complexExampleNet) 5. (* Expected result: {0, 0, 0} *)

(* Associative network of nested ordered pairs *)
Definition testPairsNet : ANetLf :=
  fun id => match id with
  | 0 => {5, 0, 8}
  | 1 => {7, 1, 2}
  | 2 => {2, 4, 5}
  | 3 => {3, 1, 5}
  | 4 => {4, 2, 1}
  | S _ => {0, 0, 0}
  end.

(* Converted associative network of nested ordered pairs into a three-dimensional associative network (dimensions must match) *)
Definition testTuplesNet : ANetVf 3 :=
  ANetLfToANetVf testPairsNet.

(* Computing the values of the converted function of the associative network of nested ordered pairs *)
Compute testTuplesNet 0. (* Expected result: [5; 0; 8] *)
Compute testTuplesNet 1. (* Expected result: [7; 1; 2] *)
Compute testTuplesNet 2. (* Expected result: [2; 4; 5] *)
Compute testTuplesNet 3. (* Expected result: [3; 1; 5] *)
Compute testTuplesNet 4. (* Expected result: [4; 2; 1] *)
Compute testTuplesNet 5. (* Expected result: [0; 0; 0] *)

(* Conversion of nested ordered pairs into the associative network of duplets *)
Compute NPToANetDl { 121, 21, 1343 }.
(* Should return: {(121, 1), (21, 2), (1343, 2)} *)

(* Adding nested ordered pairs to the associative network of duplets *)
Compute AddNPToANetDl {(121, 1), (21, 2), (1343, 2)} {12, 23, 34}. 
(* Expected result: {(121, 1), (21, 2), (1343, 2), (12, 4), (23, 5), (34, 5)} *)

(* Conversion of the associative network of duplets into nested ordered pairs *)
Compute ANetDlToNP {(121, 1), (21, 2), (1343, 2)}. 
(* Expected result: {121, 21, 1343} *)
  
Compute ANetDlToNP {(121, 1), (21, 2), (1343, 2), (12, 4), (23, 5), (34, 5)}. 
(* Expected result: {121, 21, 1343} *)

(* Reading nested ordered pairs from the associative network of duplets by the duplet index (start of the nested ordered pair) *)
Compute ANetDl_readNP {(121, 1), (21, 2), (1343, 2), (12, 4), (23, 5), (34, 5)} 0.
(* Expected result: {121, 21, 1343} *)

Compute ANetDl_readNP {(121, 1), (21, 2), (1343, 2), (12, 4), (23, 5), (34, 5)} 3.
(* Expected result: {12, 23, 34} *)

(* Defining an associative network of nested ordered pairs *)
Definition test_anetl := { {121, 21, 1343}, {12, 23}, {34}, {121, 21, 1343}, {12, 23}, {34} }.

(* Converted associative network of nested ordered pairs into the associative network of duplets *)
Definition test_anetd := ANetLlToANetDl test_anetl.

(* Computing the converted associative network of nested ordered pairs into the associative network of duplets *)
Compute test_anetd.
(* Expected result:
 {(121, 1), (21, 2), (1343, 2),
  (12, 4), (23, 4),
  (34, 5),
  (121, 7), (21, 8), (1343, 8),
  (12, 10), (23, 10),
  (34, 11)} *)

(* Converting the associative network of nested ordered pairs into the associative network of duplets and back into test_anetl *) 
Compute ANetDlToANetLl test_anetd.
(* Expected result:
  {{121, 21, 1343}, {12, 23}, {34}, {121, 21, 1343}, {12, 23}, {34}} *)

(* Computing the offset of nested ordered pairs in the associative network of duplets by their ordinal number *)
Compute ANetDl_offsetNP test_anetd 0. (* Expected result: 0 *)
Compute ANetDl_offsetNP test_anetd 1. (* Expected result: 3 *)
Compute ANetDl_offsetNP test_anetd 2. (* Expected result: 5 *)
Compute ANetDl_offsetNP test_anetd 3. (* Expected result: 6 *)
Compute ANetDl_offsetNP test_anetd 4. (* Expected result: 9 *)
Compute ANetDl_offsetNP test_anetd 5. (* Expected result: 11 *)
Compute ANetDl_offsetNP test_anetd 6. (* Expected result: 12 *)
Compute ANetDl_offsetNP test_anetd 7. (* Expected result: 12 *)

(* Defining a three-dimensional associative network as a sequence of vectors of length 3 *)
Definition test_anetv : ANetVl 3 :=
  { [0; 0; 0], [1; 1; 2], [2; 4; 0], [3; 0; 5], [4; 1; 1], [0; 0; 0] }.

(* Converted three-dimensional associative network into the associative network of duplets via the associative network of nested ordered pairs *)
Definition test_anetdl : ANetDl := ANetVlToANetDl test_anetv.

(* Computing the three-dimensional associative network converted into the associative network of duplets via the associative network of nested ordered pairs *)
Compute test_anetdl.
(* Expected result:
{ (0, 1), (0, 2), (0, 2),
  (1, 4), (1, 5), (2, 5),
  (2, 7), (4, 8), (0, 8),
  (3, 10), (0, 11), (5, 11),
  (4, 13), (1, 14), (1, 14),
  (0, 16), (0, 17), (0, 17)} *)

(* Converted three-dimensional associative network into the associative network of duplets via the associative network of nested ordered pairs and then back into a three-dimensional associative network *)
Definition result_TuplesNet : ANetVl 3 :=
  ANetLlToANetVl (ANetDlToANetLl test_anetdl).

(* Final check of the equivalence of associative networks *)
Compute result_TuplesNet.
(* Expected result:
  { [0; 0; 0], [1; 1; 2], [2; 4; 0], [3; 0; 5], [4; 1; 1], [0; 0; 0] } *)
```

## Practical implementation

There are several practical implementations: [Deep](http://github.com/deep-foundation/), [LinksPlatform](https://github.com/linksplatform) and [the model of relations](https://github.com/netkeep80/jsonRVM).

### Deep

[Deep](http://github.com/deep-foundation/) is a system based on the links theory. In links theory, links can be used to represent any data or knowledge, as well as to perform programming. Deep is built around this philosophy: in Deep, everything is a link. However, if we divide these links into two categories, we have data itself and behavior. Behavior — represented by code in Deep — is stored in the associative store as links, and for execution it is passed to a Docker container of the corresponding programming language, where it runs in isolation and safely. All communication between different parts of the code is carried out through links in the store (database), making the database a universal data-based API (in contrast to the traditional practice of calling functions and methods). At present, PostgreSQL is used as the associative store in Deep, which will later be replaced by a data engine based on doublets and triplets from LinksPlatform.

Deep makes all software on the planet interoperable by representing all its parts as links. It is also possible to store any data and code together, linking events or actions on various types of associations with the corresponding code that is executed to handle those events. Each handler can retrieve the necessary links from the associative store and insert/update/delete links in it, which may trigger further cascade execution of handlers.

The `links` table in Deep’s PostgreSQL database contains records that can be interpreted as links. They have columns such as `id`, `type_id`, `from_id`, and `to_id`. The links types help the developer of associative packages predefine the semantics of the relationships between various elements, ensuring an unambiguous understanding of links by both people and code in the associative packages. In addition to the `links` table, the system also includes tables named `numbers`, `strings`, and `objects` for storing numeric, string, and JSON values, respectively. Each link can be associated with only one value. This is temporary solution, that is used until Deep will not migrate to use of LinksPlatform as a database engine. Once migration is complete all these seemingly basic types will be built from the ground up using only links. It will allow to use deduplication (that arises as a consequence of the links theory) and deep understanding of values inner structure. Also it is planned to add indexing of such complex values represented by links, to improve performance to make it as fast or faster than current PostgreSQL implementation.

### LinksPlatfrom

[LinksPlatfrom](https://github.com/linksplatform) is a cross-platform, multi-language framework aimed at providing a low-level implementation of associativity in the form of a database engine constructor. For example, at present we have a [benchmark](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets) that compares the implementation of doublets in PostgreSQL with a similar implementation in pure Rust/C++; the leading implementation in Rust outperforms PostgreSQL by $1746 \dots 15745$ times in write operations and $100 \dots 9694$ times in read operations.

### The model of relations

[The model of relations](https://github.com/netkeep80/jsonRVM) is a meta-programming language based on representing a program as a three-dimensional associative network. The model of relations adheres to entity-oriented programming, where the entity is used as the single fundamental concept — that is, it assumes that everything is an entity and there is nothing besides entities.

In the model of relations, an entity, depending on its internal constitutive principle, can be either a structure (object) or a function (method). Unlike the well-known ER model, which uses two basic concepts — entity and relation — to represent the database schema, in the model of relations the entity and the relation are essentially the same. This representation allows one to describe not only the external relationships of an entity, but also its internal model — the model of relationships.

An entity, in its internal principle, is triune (threefold, consisting of three elements) because it is a synthesis of three aspects (qualities) of other entities (or of itself).

[jsonRVM](https://github.com/netkeep80/jsonRVM) is a multithreaded virtual machine for executing the JSON projection of the model of relations. The model of relations, when represented as JSON, allows programs to be written directly in JSON. This representation is a hybrid of data and code segments and makes it easy to deserialize/execute/serialize the projection of the model of relations, as well as to use JSON editors for programming. In the execution process of the model of relations, the meta-program can not only process data but also generate multithreaded programs and meta-programs, and either execute them immediately or export them as JSON.

## Conclusion

In this article, we examined the mathematical foundations of relational algebra and graph theory, and presented the definitions of the links theory in terms of set theory and its projection into type theory. We also defined a set of functions and lemmas necessary for proving the possibility of an equivalent conversion from any vector/sequence into nested doublet-links and back. This means that only one formula is sufficient to represent any possible type of information:

> $\displaystyle L \to L^2$

Thus, this forms the basis for testing the hypothesis that any other data structure can be represented by doublet-links. In other words, doublet-links are sufficient to represent any tables, graphs, strings, arrays, lists, numbers, sound, images, videos, and much more.

Another consequence of this proof is that we can represent tape of Turing Machine using only doublet‑links. That means links can be as powerful as a Turing Machine in its storage capacity. Meaning we can use links for all use cases there Turing Machine is used. But there is no need to try to fit data into zeroes and ones, no need to puzzle about that. Because in links the “alphabet” is essentially is unlimited and you can add any number of links, assign to them any meaning you need and link or connect them together in any way you need. Usually it means that your concept, object or thing from the real or abstract world will be mapped 1 to 1 into links or as close to original as possible, which is sometimes not possible with traditional methods.

We continue to make progress in synchronizing meaning between our three projects and among the people in our associative community. These projects are designed to bring associativity into the world and make it useful for humanity. This article is another iteration of our discussion, allowing us to agree on a unified meaning of words and terms within the general associative theory. We believe this theory can become a meta-language on which is already used by people and machines to communicate.

With each further refinement, we will be one step closer to speaking a common language and making this idea more understandable to everyone. This theory will also be useful for various optimizations in the associative implementations under development, and in the future, for the design of associative chips (or coprocessors) to accelerate operations on data represented as links.

### Plans for the future

This article has demonstrated only a small part of all the developments in the links theory that have accumulated over several years of work and research. In subsequent articles, other projections of the links theory will gradually be revealed, in terms of other theories such as relational algebra, graph theory, and also in terms of type theory without using set theory directly, as well as an analysis of the differences from [Simon Williams’ associative model of data](https://web.archive.org/web/20181219134621/http://sentences.com/docs/amd.pdf) [[3]](https://web.archive.org/web/20181219134621/http://sentences.com/docs/amd.pdf).

We also plan to project the links theory to into itself, showing that it can be used as meta-theory. That will also open a door for projecting set theory and type theory into the links theory, meaning we now complete the cycle of definition (the links theory is defined in the set theory which itself can be defined in the links theory). We also be able to compare set theory, type theory, graph theory, relational algebra and links theory, that will help us to test the equivalence of these theory or at least get the precise bijective function to convert between them.

There are also plans to present a clear and unified terminology of the links theory, its basic postulates, aspects and so on. The current progress in developing the theory can be observed in the [deep-theory repository](https://github.com/deep-foundation/deep-theory).

To get updates, we recommend subscribing to the [Deep.Foundation blog](https://habr.com/ru/companies/deepfoundation) here or checking out our [work on GitHub](https://github.com/deep-foundation) now, or directly contacting us at [our Telegram public chat](https://t.me/unilinkness) (especially if you’re afraid of getting downvoted in the comments).

We welcome any feedback you may have, whether on Habr, GitHub, or Telegram. You can also participate in the development of the theory or help accelerate its progress by engaging with us in any way.

### CLI demo

Now you can get the sense of how associative theory works using our [CLI demo tool](https://github.com/link-foundation/link-cli), that is build up on [links notation](https://github.com/linksplatform/Protocols.Lino) and [Doublet-links storage](https://github.com/linksplatform/Data.Doublets) from [LinksPlatform](https://github.com/linksplatform) project. [Links notation](https://github.com/linksplatform/Protocols.Lino) is a language for data expressed in the links and references only. [Doublets](https://github.com/linksplatform/Data.Doublets) are a database engine written in C# from the ground up to support only associative storage and transformations.

In this demo we build up on links notation to create a dialect that is able to describe single universal operation - substitution. As with unification of data types, it is also possible to unify creation, read, update and deletion into single substitution operation. That is similar to the only operation from [Markov algorithm](https://en.wikipedia.org/wiki/Markov_algorithm), which is proven to be [Turing-complete](https://en.wikipedia.org/wiki/Turing_completeness#:~:text=13%20External%20links-,Non%2Dmathematical%20usage,purpose%20computer%20or%20computer%20language.).

![Figure 11](images/figure-11.png)

**Figure 11.** In this image you can see creation of two links `(1: 1 1)` and `(2: 2 2);` update of first link to `(1: 1 2)`; update/substitution using variables to swap sources and targets of each link; and a deletion of all links using `(* *)` pattern.

### Visual demos

![Figure 12](images/figure-12.png)

**Figure 12.** Link blueprint designer build on top of a configurable spline: [konard.github.io/links-visuals/blueprint.html](http://konard.github.io/links-visuals/blueprint.html) (move control points of spline that represents the link)

![Figure 13](images/figure-13.jpg)

**Figure 13.** [H-tree](https://en.wikipedia.org/wiki/H_tree) like fractal build using links represented by straight arrows: [konard.github.io/links-visuals/H-fractal.html](https://konard.github.io/links-visuals/H-fractal.html) (click at any place to iterate the fractal)

### P.S.

This and previous articles will be updated as the links theory develops and expands over the next 6 months.

### P.S.S.

If you have become a fan of the links theory, we invite you to spread this formula as a meme-virus on social media.

Using Unicode symbols:

> L ↦ L²

Using LaTeX:

> L \to L^2

Which is rendered as SVG (clickable):

> $\displaystyle L \to L^2$

### References

1. Edgar F. Codd, IBM Research Laboratory, San Jose, California, June 1970, [“Relational Model of Data for Large Shared Data Banks.”, paragraph 1.3., page 379](https://dl.acm.org/doi/abs/10.1145/362384.362685)
2. Bender, Edward A.; Williamson, S. Gill (2010). [“Lists, Decisions and Graphs. With an Introduction to Probability.”, section 2, definition 6, page 161](https://books.google.com/books?id=vaXv_yhefG8C)
3. Simon Williams, Great Britain (1988), [The Associative Model Of Data](https://web.archive.org/web/20181219134621/http://sentences.com/docs/amd.pdf)
4. Homan, J. V., & Kovacs, P. J. (2009). [A Comparison of the Relational Database Model and the Associative Database Model](https://www.researchgate.net/publication/255670856_A_COMPARISON_OF_THE_RELATIONAL_DATABASE_MODEL_AND_THE_ASSOCIATIVE_DATABASE_MODEL). Issues in Information Systems, X(1), 208.

---

**Tags:** metatheory, links theory, relational theory, associative theory, math, set theory, type theory, graph theory, relational algebra, associative model of data

**Hubs:** [Data Engineering](https://habr.com/en/hubs/data_engineering/), [Open source](https://habr.com/en/hubs/open_source/), [Mathematics](https://habr.com/en/hubs/maths/), [Abnormal programming](https://habr.com/en/hubs/crazydev/), [Programming](https://habr.com/en/hubs/programming/)

**Votes: +10 | Views: 5259**

**Author:** [Константин Дьяченко (Konard)](https://habr.com/en/users/Konard/)
