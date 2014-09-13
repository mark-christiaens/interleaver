#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate core;
extern crate collections;

use docopt::FlagParser;
use std::io::BufferedReader;
use std::io::BufferedWriter;
use std::io::File;
use core::result::Result;
use collections::string::String;
use collections::vec::Vec;

docopt!(Args, "
interleaver

Usage: interleaver FILENAMES...
", )

type BrType = BufferedReader<core::result::Result<std::io::fs::File,std::io::IoError>>;
type BwType = BufferedWriter<core::result::Result<std::io::fs::File,std::io::IoError>>;

fn main() {
  let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

  // The Args struct satisfies `Show`:
  println!("{}", args);

  let file_names = args.arg_FILENAMES;
  let paths_it = file_names.iter ().map (|n| Path::new(n.clone ()));
  let files_it = paths_it.map (|path| File::open(&path));
  let mut buffered_readers_it = files_it.map (|file| BufferedReader::new (file));
  let mut buffered_readers_vec : Vec<BrType> = buffered_readers_it.collect ();

  let paths_out_it = range (0u, file_names.len ()).map (|i| Path::new (i.to_string()));
  let files_out_it = paths_out_it.map (|path_out| File::open(&path_out));
  let mut buffered_writers_it = files_out_it.map (|file_out| BufferedWriter::new (file_out));
  let mut buffered_writers_vec : Vec<BwType> = buffered_writers_it.collect ();

  let mut done;
  let mut i = 0;

  loop {
    done = true;
    for buffered_reader in buffered_readers_vec.mut_iter () {
      let mut lines = buffered_reader.lines ();
      match lines.next () {
        None => (),
        Some (line) => {
          buffered_writers_vec.get_mut (i).write_str (line.unwrap ().as_slice ());
          i = i + 1;
          done = false;
        }
      }
    }
    if done { break; }
  }





  // let line_readers_it = buffered_readers_vec.iter().map (|mut buffered_reader| { buffered_reader.read_to_string(); });

  // let line_iterators = readers.map (|mut reader| reader.lines ());

  // let c = args.arg_FILENAMES.len();

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
