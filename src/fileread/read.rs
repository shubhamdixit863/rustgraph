// File reading
use std::fs::File;
use std::io::{self, BufRead,BufReader};


pub fn file_read() ->std::io::Result<()>{
    // We will open the file
    let file=File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
      let line=line;
       // println!("{}",line)
    }

    Ok(())
}