use std::env;
mod huffman;
mod encoder;
mod decode;


fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 3 {
    println!("Invalid arguments. Please provide an option and a filename.");
    return
  }

  let option = &args[1];
  let filename = &args[2];

  match option.as_str() {
    "-d" =>  decode::decode(filename),
    "-e" =>  encoder::encode(filename),
    _ => println!("{} option not reconginzed.", option)
  }
}