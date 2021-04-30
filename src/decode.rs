use crate::huffman;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn decode_file(file_name: &str) {
    println!("\nDECODE");
   
    let mut data: &[u8];
    if let Ok(file_data) = std::fs::read(file_name) {
        data = &file_data;
        if let Some(max_code_len) = data.first() {
            println!("max code length {}", max_code_len);
            if data[1..].len() < *max_code_len as usize {
                return;
            }
            let code_lengths = &data[1..*max_code_len as usize + 1];
            data = &data[*max_code_len as usize + 1..];
            let num_codes: u8 = code_lengths.iter().sum();
            println!("Number of codes {}", num_codes);
            let symbols = &data[0..num_codes as usize];
            let code_book = huffman::rebuild_code_book(code_lengths, symbols);
        }
    }

}
