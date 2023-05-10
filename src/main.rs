extern crate csv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr: csv::Reader<std::fs::File> = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("input.csv")?;

    let mut out = String::new();

    // テーブルヘッダーの取得と出力
    let headers = rdr.headers()?;
    let metadata = headers
        .iter()
        .map(|h| ":---".to_string())
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
