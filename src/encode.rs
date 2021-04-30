use crate::huffman;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;


pub fn encode(file_name: &str) {
    println!("\nENCODE");

    let data = fs::read_to_string(file_name).expect("file error");
    let mut canonical_codes = huffman::get_canonical_codes(&data);
    let mut file_data: Vec<u8> = Vec::new();

    if let Some(last_code) = canonical_codes.last() {
        println!("max code length {}", last_code.len());
        println!("Number of codes {}", canonical_codes.len());
        file_data.push(last_code.len());
        //  println!("Unable to encode file. File contains non-ASCII letters or symbols.");
    }

    for i in canonical_codes.iter() {
        println!("{} '{}'", i.word(), i.char())
    }
    add_tree_data(&mut file_data, &mut canonical_codes);

    add_encoded_data(
        &mut file_data,
        &data,
        huffman::create_code_book(canonical_codes),
    );
    write_to_file(file_data);

}

fn add_tree_data(file_data: &mut Vec<u8>, canonical_codes: &mut Vec<huffman::Code>) {
    let mut count = 0;
    let mut len_prev = 1;
    if canonical_codes.len() > 0 {
        for code in canonical_codes.iter() {
            while len_prev < code.len() - 1 {
                len_prev += 1;
                file_data.push(0);
            }
            if len_prev != code.len() {
                file_data.push(count);
                count = 1;
            } else {
                count += 1;
            }
            len_prev = code.len();
        }
        if count > 0 {
            file_data.push(count);
        }
    }

    for code in canonical_codes {
        file_data.push(code.symbol());
    }
}

fn add_encoded_data(file_data: &mut Vec<u8>, data: &str, code_book: HashMap<char, String>) {
    let mut bits_in_buffer = 0;
    let mut buffer = 0;
    for ch in data.chars() {
        if let Some(code_word) = code_book.get(&ch) {
            for bit in code_word.chars() {
                buffer = buffer << 1;
                if bit == '1' {
                    buffer = buffer | 1;
                }
                bits_in_buffer += 1;
                if bits_in_buffer == 8 {
                    file_data.push(buffer);
                    buffer = 0;
                    bits_in_buffer = 0;
                }
            }
        }
    }
    if bits_in_buffer > 0 {
        buffer = buffer << (8 - bits_in_buffer);
        file_data.push(buffer);
    }
}

fn write_to_file(encoded_data: Vec<u8>) {
    let mut file = fs::File::create("encoded.txt").expect("file create failed");
    file.write_all(&encoded_data).expect("file write failed");
}
