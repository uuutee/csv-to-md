extern crate csv;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    match env::args().skip(1).next() {
        None => {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle
                .read_to_string(&mut buffer)
                .expect("failed to read from standard input");
        }
        Some(file) => {
            let file = File::open(&file).expect("failed to open file");
            let mut handle = BufReader::new(file);
            handle
                .read_to_string(&mut buffer)
                .expect("failed to read from file");
        }
    }

    let mut rdr: csv::Reader<&[u8]> = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(buffer.as_bytes());

    let mut out = String::new();

    // テーブルヘッダーの取得と出力
    let headers = rdr.headers()?;
    let metadata = headers
        .iter()
        .map(|_| ":---".to_string())
        .collect::<Vec<String>>();
    let header_row = headers
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>();
    out += &format!(
        "| {} |\n|{}|\n",
        header_row.join(" | "),
        metadata.join(" | ")
    );

    // 各行の処理と出力
    for result in rdr.records() {
        let row = result?;
        let row_str = row.iter().map(|r| r.to_string()).collect::<Vec<String>>();
        out += &format!("| {} |\n", row_str.join(" | "));
    }

    println!("{}", out);
    Ok(())
}
