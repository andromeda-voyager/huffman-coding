use crate::huffman;


pub fn decode(filename: &str) {
    let mut data: &[u8];
    if let Ok(file_data) = std::fs::read(filename) {
        data = &file_data;
        if let Some(max_code_len) = data.first() {
            if data[1..].len() < *max_code_len as usize {
                return;
            }
            let code_lengths = &data[1..*max_code_len as usize + 1];
            data = &data[*max_code_len as usize + 1..];
            let num_codes: u8 = code_lengths.iter().sum();
            let symbols = &data[0..num_codes as usize];
            data = &data[num_codes as usize..];
            let code_book = huffman::rebuild_code_book(code_lengths, symbols);

            let mut code_word = "".to_string();
            let mut decoded_msg = "".to_string();
            for block in data.iter() {
                let mut b = *block;
                for _ in 0..8 {
                    if b & 0x80 != 0 { // most significant bit is 1
                        code_word += "1";
                    } else {
                        code_word += "0";
                    }
                    if let Some(ch) = code_book.get(&code_word) {
                        decoded_msg.push(*ch);
                        code_word = "".to_string();
                    }
                    b =  b << 1
                }
            }
            println!("{}", decoded_msg);
        }
    }
}
