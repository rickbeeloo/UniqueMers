use std::env;
use std::fs::File;
use std::io::{self, Write};

fn generate_kmers_recursive(
    current_kmer: &str,
    length: usize,
    nucleotides: &[char],
    file: &mut File,
    max_kmers: usize,
    i: &mut usize,
) {
    if *i >= max_kmers {
        return;
    }

    if length == 0 {
        writeln!(file, ">kmer{}", *i).expect("Write failed");
        writeln!(file, "{}", current_kmer).expect("Write failed");
        *i += 1;
    } else {
        for &nucleotide in nucleotides {
            let new_kmer = format!("{}{}", current_kmer, nucleotide);
            generate_kmers_recursive(&new_kmer, length - 1, nucleotides, file, max_kmers, i);
        }
    }
}

fn generate_kmers(
    kmer_length: usize,
    max_kmers: usize,
    file: &mut File,
) -> Vec<String> {
    let nucleotides = ['A', 'C', 'G', 'T'];
    let mut kmers = Vec::new();
    let mut i = 1;

    generate_kmers_recursive("", kmer_length, &nucleotides, file, max_kmers, &mut i);

    kmers
}

fn write_fasta(filename: &str, kmers: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;

    for (i, kmer) in kmers.iter().enumerate() {
        writeln!(file, ">kmer{}", i + 1)?;
        writeln!(file, "{}", kmer)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Access command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is correct
    if args.len() != 3 {
        println!("Usage: {} <kmer_length> <max_kmers>", args[0]);
        return Ok(());
    }

    // Output
    let fasta_filename = "unique_kmers.fasta";
    let mut file = File::create(&fasta_filename).expect("Could not open output file");

    // Parse command-line arguments
    let kmer_length: usize = args[1].parse().expect("Invalid kmer_length");

    let max_kmers: usize = args[2].parse().expect("Not an int");

    generate_kmers(kmer_length, max_kmers, &mut file);

    // Print the total number of unique k-mers
    println!("FASTA file saved as: {}", fasta_filename);

    Ok(())
}
