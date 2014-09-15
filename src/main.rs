#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;
extern crate core;
extern crate collections;
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use docopt::FlagParser;
use std::io::BufferedReader;
use std::io::BufferedWriter;
use std::io::File;
use core::result::Result;
use collections::string::String;
use collections::vec::Vec;

// Interleaver takes N files (e.g., i_1.txt ... i_N.txt) and transforms them into N output files
// named o_1.txt ... o_N.txt.  The lines of the input files must be prefixed with time stamps.  The
// output files contain the same content as the input files but only one file is allowed to make
// progress on a given line.  The other files are empty.

docopt!(Args, "
Usage:
  interleaver <file>...

Options:
  -h, --help    Show this screen.
", )

type BrType = BufferedReader<core::result::Result<std::io::fs::File,std::io::IoError>>;
type BwType = BufferedWriter<core::result::Result<std::io::fs::File,std::io::IoError>>;

#[deriving(Show,PartialEq,PartialOrd,Eq,Ord)]
enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }

fn string_to_month (s : &str) -> Option<Month> {
  match s {
    "Jan" => Some (Jan),
    "Feb" => Some (Feb),
    "Mar" => Some (Mar),
    "Apr" => Some (Apr),
    "May" => Some (May),
    "Jun" => Some (Jun),
    "Jul" => Some (Jul),
    "Aug" => Some (Aug),
    "Sep" => Some (Sep),
    "Oct" => Some (Oct),
    "Nov" => Some (Nov),
    "Dec" => Some (Dec),
    _     => None
  }
}

#[deriving(Show,PartialEq,Eq)]
struct TimedLine {
  l       : String,
  month   : Month,
  day     : u8,
  hour    : u8,
  minute  : u8,
  second  : u8,
  usecond : u32,
  target  : uint
}

impl TimedLine {
  fn new (l : &String, target : uint) -> TimedLine {
    let re = regex! (r"^(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) +(\d+) (\d+):(\d+):(\d+) (\d+):.*$");
    let caps = match re.captures (l.as_slice()) {
      None => fail! ("Could not analyze line: \"{}\"", l),
      Some (caps) => caps
    };
    let (month, day, hour, minute, second, usecond) =
      match (string_to_month (caps.at(1)), from_str::<u8>(caps.at(2)), from_str::<u8> (caps.at(3)), from_str::<u8> (caps.at(4)), from_str::<u8> (caps.at(5)), from_str::<u32> (caps.at(6))
      ) {
        (Some (month), Some (day), Some (hour), Some (minute), Some (second), Some (usecond)) => (month, day, hour, minute, second, usecond*100),
        _ => fail! ("Could not extract fields from line: \"{}\"", l)
      };

    TimedLine {
      l       : l.clone (),
      month   : month,
      day     : day,
      hour    : hour,
      minute  : minute,
      second  : second,
      usecond : usecond,
      target  : target
    }
  }
}

impl Ord for TimedLine {
  fn cmp(&self, other: &TimedLine) -> Ordering {
    let month_cmp = self.month.cmp (&other.month);
    if month_cmp == Equal {
      let day_cmp = self.day.cmp (&other.day);
      if day_cmp == Equal {
        let hour_cmp = self.hour.cmp (&other.hour);
        if hour_cmp == Equal {
          let minute_cmp = self.minute.cmp (&other.minute);
          if minute_cmp == Equal {
            let second_cmp = self.second.cmp (&other.second);
            if second_cmp == Equal {
              self.usecond.cmp (&other.usecond)
            }
            else { second_cmp }
          }
          else { minute_cmp }
        }
        else { hour_cmp }
      }
      else { day_cmp }
    }
    else { month_cmp }
  }
}

impl PartialOrd for TimedLine {
  fn partial_cmp(&self, other: &TimedLine) -> Option<Ordering> {
    Some (self.cmp (other))
  }
}

fn main() {
  let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());

  let s = String::from_str ("Sep  2 14:25:02 8993: (main|info): --- NODE STARTED ---");
  let tl = TimedLine::new (&s, 5u);
  println!("TimedLine: {}", tl);

  println!("Starting {}", args);

  let file_names = args.arg_file;
  let file_count = file_names.len ();
  let paths_it = file_names.iter ().map (|n| Path::new(n.clone ()));
  let files_it = paths_it.map (|path| File::open(&path));
  let mut buffered_readers_it = files_it.map (|file| BufferedReader::new (file));
  let mut buffered_readers_vec : Vec<BrType> = buffered_readers_it.collect ();

  let paths_out_it = range (0u, file_names.len ()).map (|i| Path::new (i.to_string()));
  let files_out_it = paths_out_it.map (|path_out| {
    let res = File::create(&path_out);
    match res {
      Ok(_)  => println!("Successfully opened"),
      Err(e) => fail!("Could not open output file: {}", e)
    };
    res
  }
  );
  let mut buffered_writers_it = files_out_it.map (|file_out| BufferedWriter::new (file_out) );

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
          let target = buffered_writers_vec.get_mut (i);
          let res = target.write_str (line.unwrap ().as_slice ());
          match res {
            Ok(_) => {
              i = (i + 1) % file_count;
              done = false;
            }
            Err(e) => {
              let ref file_name = file_names[i];
              fail! ("Could not write to {}: {}", file_name, e);
            }
          }
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
