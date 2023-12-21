
# Sequences containing unique kmers

Not that interesting, but we recursively mutate a kmer to generate unique kmer sequences till a specfified maximum number of sequences is reached. It generates `max_kmers` unique kmers of size `k` and save them to a fasta, `fasta_output`. Alternativly a last argument can be passed `tail_length` to append a random sequence of `<= tail_length` to the end of the kmer.


## Usage

`UniqueMers <fasta_output> <kmer_length> <max_kmers> <tail_length>`


## Installation

```
git clone https://github.com/rickbeeloo/UniqueMers
cd UniqueMers/
cargo build --release
```

The executable should now be in `/target/release/`. For example on Windows this would be `./target/release/UniqueMers.exe` on windows.

## Examples
if we run `UniqueMers.exe kmers.fasta 2 16 0` the `kmers.fasta` looks like:

```
>seq1
AAT
>seq2
ACG
>seq3
AG
>seq4
ATG
>seq5
CAA
>seq6
CC
>seq7
CGCT
>seq8
CTA
>seq9
GAGA
>seq10
GCCG
>seq11
GG
>seq12
GTA
>seq13
TA
>seq14
TC
>seq15
TGTT
>seq16
TTG
```
Note that we can have a max of 16 unique kmers (`4^2`), so passing 17 as `max_kmers` will panic with:

>Cannot generate sufficient unique kmers, increase k

If we want to add random "tails" to these unique kmers we can set the tails to `5` for example, then we get something like this:

```
>seq1
AACGGCT
>seq2
ACTGGTA
>seq3
AGA
>seq4
ATG
>seq5
CAAAAA
>seq6
CC
```

While now we still have some sequences equal to `k`, like `>seq6`, we also have longer sequences, up to `k+tail=2+5=7`, like `>seq1`.

