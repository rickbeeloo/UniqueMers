use std::env;
use std::fs::File;
use std::io::{self, Write, BufWriter};
use rand::Rng;
use rand::seq::SliceRandom;

fn generate_kmers_recursive(
    current_kmer: &str,
    length: usize,
    nucleotides: &[u8],
    file: &mut BufWriter<File>,
    max_kmers: usize,
    tail_length: usize,
    rng: &mut impl Rng,
    i: &mut usize,
    reusable_tail: &mut Vec<u8>,
    buffer: &mut Vec<u8>,
) {
    // Done if we reached max kmers
    if *i > max_kmers {
        return;
    }

    if length == 0 {
        generate_random_tail(tail_length, nucleotides, rng, reusable_tail);
        buffer.clear();
        buffer.extend_from_slice(current_kmer.as_bytes());
        buffer.extend_from_slice(&reusable_tail);
        writeln!(file, ">seq{}", *i).expect("Write failed");
        file.write_all(&buffer).expect("Write failed");
        write!(file, "\n").expect("Write failed");
        *i += 1;

        // Print progress for every 1 million kmers with percentage rounded to two decimals
        if *i % 1_000_000 == 0 {
            let percentage = (*i as f64 / max_kmers as f64) * 100.0;
            println!("Generated {} kmers ({:.2}%)", *i, percentage);
        }

    } else {
        for &nucleotide in nucleotides {
            let mut next_kmer = current_kmer.to_string();
            next_kmer.push(nucleotide as char);

            generate_kmers_recursive(
                &next_kmer,
                length - 1,
                nucleotides,
                file,
                max_kmers,
                tail_length,
                rng,
                i,
                reusable_tail,
                buffer,
            );
        }
    }
}

// Generate a random tail for the chosen kmer size
fn generate_random_tail(max_tail_size: usize, nucleotides: &[u8], rng: &mut impl Rng, tail: &mut Vec<u8>) {
    tail.clear(); // Clear the tail before reuse
    let tail_size = rng.gen_range(0..=max_tail_size);

    for _ in 0..tail_size {
        tail.push(*nucleotides.choose(rng).unwrap());
    }
}

fn generate_kmers(
    kmer_length: usize,
    max_kmers: usize,
    tail_length: usize,
    file: &mut BufWriter<File>,
) {
    let nucleotides: Vec<u8> = b"ACGT".to_vec(); // Use u8 instead of char
    let mut i = 1;
    let mut rng = rand::thread_rng(); // Initialize the random number generator
    let mut reusable_tail = Vec::with_capacity(tail_length);
    let mut buffer = Vec::with_capacity(kmer_length + tail_length); // Preallocate buffer

    generate_kmers_recursive(
        "",
        kmer_length,
        &nucleotides,
        file,
        max_kmers,
        tail_length,
        &mut rng,
        &mut i,
        &mut reusable_tail,
        &mut buffer,
    );
}

// Check if we can generate sufficient kmers with the given k size
fn can_generate_kmers(k: usize, max_kmers: usize) -> bool {
    let num_nucleotides: u64 = 4;
    let total_kmers: usize = num_nucleotides.pow(k as u32) as usize;
    max_kmers <= total_kmers
}

fn main() -> io::Result<()> {
    // Access command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is correct
    if args.len() != 5 {
        println!("Usage: {} <fasta_output> <k> <max_kmers> <tail_length>", args[0]);
        return Ok(());
    }

    // Parse command-line arguments
    let fasta_filename = &args[1];
    let kmer_length: usize = args[2].parse().expect("Invalid kmer_length");
    let max_kmers: usize = args[3].parse().expect("Not an int");
    let tail_length: usize = args[4].parse().expect("Invalid tail_length");

    // Does this make sense
    if !can_generate_kmers(kmer_length, max_kmers) {
        panic!("Cannot generate sufficient unique kmers, increase k");
    }

    // Replaced with BufferedWriter to make writing a bit faster
    let file = File::create(&fasta_filename).expect("Could not open output file");
    let mut buffered_file = BufWriter::new(file);

    generate_kmers(kmer_length, max_kmers, tail_length, &mut buffered_file);

    // Print the total number of unique k-mers
    println!("FASTA file saved as: {}", fasta_filename);

    Ok(())
}