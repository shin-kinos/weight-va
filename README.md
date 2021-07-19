# weight-va
A small program thet calculates sequence weighting in MSA using Distance-Based Method.

## Description
* It calculates weighting scores in MSA to reduce sequence redundancy using Distance-Based Method by Vingron and Argos[1][2].

## How to compile
You can compile this program with `Cargo`🦀📦.

[e.g.]

```
% cd weight-va-main
% cargo build --release
```

## Input file format
Aligned Multi-FASTA format in amino acid sequences.

See some example files in `demo` directory.

## Usage

Options :
* `-i` : Input file name, REQUIRED.

[e.g.]

```
% ./weight-va -i input.fasta
```

Type `-h` to see a help.

## Result
Number `\t` Weighting score `\t` Name of the sequence (the title line)

[e.g.]

## References
1. Henikoff, Steven, and Jorja G. Henikoff. "Position-based sequence weights." Journal of molecular biology 243.4 (1994): 574-578.
2. Vingron, Martin, and Patrick Argos. "A fast and sensitive multiple sequence alignment algorithm." Bioinformatics 5.2 (1989): 115-121.
