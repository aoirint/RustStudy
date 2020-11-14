// Thank you: https://qiita.com/fujitayy/items/12a80560a356607da637

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<std::error::Error>> {
    for result in BufReader::new(File::open("content.txt")?).lines() {
        let l = result?;
        println!("{}", l);
    }

    Ok(())
}

