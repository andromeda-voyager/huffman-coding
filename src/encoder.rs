use crate::huffman;
use std::io::prelude::*;

struct Encoder {
    codes: Vec<huffman::Code>,
    buffer: Vec<u8>,
}

pub fn encode(filename: &str) {
    let file_data = std::fs::read_to_string(filename).expect("failed to read file");
    let canonical_codes = huffman::get_canonical_codes(&file_data);
    let mut encoder = Encoder::new(canonical_codes);
    encoder.add_tree();
    encoder.encode_file(&file_data);
    write_to_file(encoder.buffer, filename);
}

impl Encoder {
    fn new(codes: Vec<huffman::Code>) -> Self {
        Encoder {
            codes,
            buffer: Vec::new(),
        }
    }

    fn add_tree(&mut self) {
        if let Some(last_code) = self.codes.last() {
            self.buffer.push(last_code.len());
        }
        let mut count = 0;
        let mut len_prev = 1;
        for code in self.codes.iter() {
            if len_prev != code.len() {
                self.buffer.push(count);
                count = 0;
                if len_prev < code.len() - 1 {
                    for _ in 1..code.len() - len_prev {
                        self.buffer.push(0);
                    }
                }
            }
            count += 1;
            len_prev = code.len();
        }
        if count > 0 {
            self.buffer.push(count);
        }

        for code in self.codes.iter() {
            self.buffer.push(code.symbol());
        }
    }

    fn encode_file(&mut self, file_data: &str) {
        let code_book = huffman::create_code_book(&self.codes);
        let mut bits_in_buffer = 0;
        let mut buffer = 0;
        for ch in file_data.chars() {
            if let Some(code_word) = code_book.get(&ch) {
                for bit in code_word.chars() {
                    buffer = buffer << 1;
                    if bit == '1' {
                        buffer = buffer | 1;
                    }
                    bits_in_buffer += 1;
                    if bits_in_buffer == 8 {
                        self.buffer.push(buffer);
                        buffer = 0;
                        bits_in_buffer = 0;
                    }
                }
            }
        }
        if bits_in_buffer > 0 {
            buffer = buffer << (8 - bits_in_buffer);
            self.buffer.push(buffer);
        }
    }


}

fn write_to_file(encoded_data: Vec<u8>, filename: &str) {
    let mut file =
        std::fs::File::create(filename.to_string() + ".hzmp").expect("file create failed");
    file.write_all(&encoded_data).expect("file write failed");
}
