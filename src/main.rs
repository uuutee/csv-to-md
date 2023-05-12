extern crate csv;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

struct Args {
    file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args {
        file: env::args().skip(1).next(),
    };
    let content = get_content(args)?;

    let rdr: csv::Reader<&[u8]> = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    print_as_markdown(rdr)?;
    Ok(())
}

fn get_content(args: Args) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    match args.file {
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
    Ok(buffer)
}

fn print_as_markdown(mut rdr: csv::Reader<&[u8]>) -> Result<(), Box<dyn std::error::Error>> {
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
