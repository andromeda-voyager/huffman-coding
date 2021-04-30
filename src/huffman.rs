use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn evaluate_frequencies(message: &str) -> Vec<Box<TreeNode>> {
    let mut freq_map = HashMap::new();
    for c in message.chars() {
        let frequency = freq_map.entry(c).or_insert(0);
        *frequency += 1;
    }
    let nodes: Vec<Box<TreeNode>> = freq_map
        .iter()
        .map(|node| Box::new(TreeNode::new(*(node.1), Some(*(node.0) as u8))))
        .collect();
    nodes
}

fn get_huffman_codes(message: &str) -> Vec<Code> {
    let mut codes: Vec<Code> = Vec::new();
    let huffman_tree = create_huffman_tree(message);
    if let Some(huffman_tree) = huffman_tree {
        huffman_tree.get_code_lengths(&mut codes, 0);
    }
    codes.sort();
    codes
}

pub fn create_code_book(codes: Vec<Code>) -> HashMap<char, String> {
    let mut code_book: HashMap<char, String> = HashMap::new();
    for code in codes {
        code_book.insert(code.char(), code.word);
    }
    code_book
}

pub fn rebuild_code_book(code_lengths: &[u8], symbols: &[u8]) -> HashMap<char, String> {
    let mut code_book: HashMap<char, String> = HashMap::new();
    let mut lengths: Vec<usize> = Vec::new();
    let mut length = 1;
    for num in code_lengths.iter() {
        for _ in 0..*num  {
            lengths.push(length);
        }
        length += 1;
    }

    let mut code_word = "0".to_string();
    for (i, &symbol) in symbols.iter().enumerate() {
        code_word = next_canonical_code(&code_word, lengths[i]);
        println!("{} '{}'", code_word, symbol as char);
        code_book.insert(symbol as char, code_word.clone());
    }
    code_book
}

pub fn get_canonical_codes(message: &str) -> Vec<Code> {
    let mut codes = get_huffman_codes(message);
    let mut code_word = "0".to_string();
    for code in codes.iter_mut() {
        code_word = next_canonical_code(&code_word, code.len() as usize);
        code.word = code_word.clone();
    }
    codes
}

fn next_canonical_code(prev_code: &str, len_next: usize) -> String {
    if prev_code == "0" {
        return "0".repeat(len_next);
    }

    let mut next_as_int = isize::from_str_radix(prev_code, 2).unwrap();
    next_as_int += 1;
    let mut next = format!("{:b}", next_as_int);

    if prev_code.len() > next.len() {
        next = "0".repeat(prev_code.len() - next.len()) + &next;
    }

    if len_next > next.len() {
        next += &"0".repeat(len_next - next.len());
    }
    next
}

fn create_huffman_tree(message: &str) -> Option<Box<TreeNode>> {
    let mut heap = BinaryHeap::from(evaluate_frequencies(message));
    while heap.len() > 1 {
        let left_child = heap.pop().unwrap();
        let right_child = heap.pop().unwrap();
        let mut parent = Box::new(TreeNode::new(left_child.weight + right_child.weight, None));
        parent.add_children(left_child, right_child);
        heap.push(parent);
    }
    heap.pop()
}

#[derive(Clone, Eq)]
pub struct Code {
    length: u8,
    symbol: u8,
    word: String,
}

impl Code {
    pub fn len(&self) -> u8 {
        self.length
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn char(&self) -> char {
        self.symbol as char
    }

    pub fn symbol(&self) -> u8 {
        self.symbol
    }
}

impl Ord for Code {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.length.cmp(&other.length) {
            Ordering::Equal => return self.symbol.cmp(&other.symbol),
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
        }
    }
}

impl PartialOrd for Code {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

#[derive(Clone, Eq)]
pub struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    weight: u32,
    symbol: Option<u8>,
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight.cmp(&other.weight) {
            Ordering::Equal => match self.symbol.cmp(&other.symbol) {
                Ordering::Equal => Ordering::Equal,
                Ordering::Less => return Ordering::Greater,
                Ordering::Greater => return Ordering::Less,
            },
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
        }
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }
}

impl TreeNode {
    pub fn new(weight: u32, symbol: Option<u8>) -> Self {
        TreeNode {
            right: None,
            left: None,
            weight,
            symbol,
        }
    }

    fn add_children(&mut self, left_child: Box<TreeNode>, right_child: Box<TreeNode>) {
        self.left = Some(left_child);
        self.right = Some(right_child);
    }

    fn get_code_lengths(&self, code_book: &mut Vec<Code>, code_length: u8) {
        if let Some(symbol) = self.symbol {
            code_book.push(Code {
                length: code_length,
                symbol,
                word: "".to_string(),
            });
        } else {
            if let Some(right) = &self.right {
                right.get_code_lengths(code_book, code_length + 1);
            }
            if let Some(left) = &self.left {
                left.get_code_lengths(code_book, code_length + 1);
            }
        }
    }
}
