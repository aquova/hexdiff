use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};

use ansi_term::Color::Red;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() != 2 {
        eprintln!("hexdiff file1 file2");
        return;
    }

    let mut file1 = BufReader::new(File::open(&args[0]).expect(&format!("Error opening {}", args[0])));
    let mut file2 = BufReader::new(File::open(&args[1]).expect(&format!("Error opening {}", args[1])));

    let mut idx = 0;
    let mut printed_last = false;
    let mut bytes1 = Vec::new();
    let mut bytes2 = Vec::new();

    let file1 = file1.by_ref();
    let file2 = file2.by_ref();
    print_ends();
    loop {
        let read1 = file1.take(BYTES_PER_LINE as u64).read_to_end(&mut bytes1).unwrap();
        let read2 = file2.take(BYTES_PER_LINE as u64).read_to_end(&mut bytes2).unwrap();

        if read1 == 0 && read2 == 0 {
            break;
        }

        if bytes1 != bytes2 {
            if bytes1.len() != BYTES_PER_LINE || bytes2.len() != BYTES_PER_LINE {
                bytes1.resize(BYTES_PER_LINE, 0);
                bytes2.resize(BYTES_PER_LINE, 0);
            }
            print_line(idx, &bytes1, &bytes2);
            printed_last = true;
        } else if printed_last {
            print_empty(idx);
            printed_last = false;
        }

        idx += BYTES_PER_LINE;
        bytes1.clear();
        bytes2.clear();
    }
    print_ends();
}

fn print_line(idx: usize, f1: &[u8], f2: &[u8]) {
    let mut line = format!("| {:08x}", idx);
    let bytes = f1.iter().zip(f2.iter());

    let mut subline1 = String::new();
    let mut subline2 = String::new();
    for (b1, b2) in bytes {
        let hex1 = format!("{:02x}", b1);
        let hex2 = format!("{:02x}", b2);
        if b1 != b2 {
            subline1 = format!("{} {}", subline1, Red.paint(hex1));
            subline2 = format!("{} {}", subline2, Red.paint(hex2));
        } else {
            subline1 = format!("{} {}", subline1, hex1);
            subline2 = format!("{} {}", subline2, hex2);
        }
    }
    line = format!("{} |{} |{} |", line, subline1, subline2);
    println!("{}", line);
}

fn print_empty(idx: usize) {
    let mut line = format!("| {:08x}", idx);
    let space = format!("... {: <1$} |", "", 3 * BYTES_PER_LINE - 5);
    line = format!("{} | {} {}", line, space, space);
    println!("{}", line);
}

fn print_ends() {
    println!("+{:-<1$}+", "", 6 * BYTES_PER_LINE + 14);
}
