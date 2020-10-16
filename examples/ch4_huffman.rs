use my_algo::ch4::coding_tree::HuffmanCodingTree;
use my_algo::ch4::complete_heap::CompleteMaxHeap;
use my_algo::ch4::linked_binary_tree::LinkedBinaryTree;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let path = opt.input;
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let tree =
        HuffmanCodingTree::<LinkedBinaryTree<_>>::new::<CompleteMaxHeap<_>>(&contents).unwrap();
    let (_, len) = tree.encoded();
    println!("len = {}B", len / 8);
    #[cfg(debug)]
    assert_eq!(tree.decode(), contents);
    Ok(())
}