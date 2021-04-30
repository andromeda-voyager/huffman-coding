mod huffman;
mod encode;
mod decode;


fn main() {
  encode::encode("message.txt");
  decode::decode_file("encoded.txt");

}
