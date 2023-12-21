
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

## Example 
if we run `UniqueMers.exe kmers.fasta 3 5 5` the `kmers.fasta` looks like:

```
>seq1
AAA
>seq2
AACGTGAT
>seq3
AAGCTA
>seq4
AATG
>seq5
ACAGT
```

Note that while for example `seq1` is exactly the kmer size (`3`) some sequences are longer cause of the tail being `5`. For example `seq2` is `k + 5`.

## Performance
If we use `k=17` and a tail size of `10`:
```
1 thousand sequences: 0m0.047s
1 million sequences: 0m0.393s
10 million sequences: 0m1.215s
1 billion sequences: 1m56.440s
2^32 sequences: 20m16.077s
```
(real times reported by `time`)
