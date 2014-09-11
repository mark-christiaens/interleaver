#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate core;
extern crate collections;

use docopt::FlagParser;
use std::io::BufferedReader;
use std::io::File;
use std::io::Lines;
use std::io::IoResult;
use core::result::Result;
use collections::string::String;
use collections::vec::Vec;

docopt!(Args, "
interleaver

Usage: interleaver FILENAMES...
")

fn main() {
  let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

  // The Args struct satisfies `Show`:
  println!("{}", args);

  let paths_it = args.arg_FILENAMES.iter ().map (|n| Path::new(n.clone ()));
  let files_it = paths_it.map (|path| File::open(&path));
  let mut readers_it = files_it.map (|file| BufferedReader::new (file));
  let readers_vec : Vec<BufferedReader<core::result::Result<std::io::fs::File,std::io::IoError>>> = readers_it.collect ();
  // let line_iterators = readers.map (|mut reader| reader.lines ());

  let c = args.arg_FILENAMES.len();

  // let mut candidates : Vec<Option<IoResult<String>>> = Vec::new ();

  // for line_iterator in line_iterators.iter () {
  //   // let line = line_iterator.next ();
  //   // candidates.push (line);
  // }

  // let path = Path::new("p054_poker.txt");

  // let mut file = BufferedReader::new(File::open(&path));
  // for line in file.lines() {
  //   let l = line.unwrap ();
  // }
}
