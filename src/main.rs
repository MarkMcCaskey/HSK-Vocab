use opencc_rust::*;
use std::io::prelude::*;
use std::path::PathBuf;

const SIMPLIFIED_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/simplified");
const TRADITIONAL_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/traditional");

fn main() -> anyhow::Result<()> {
    let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();

    for hsk_ent in std::fs::read_dir(SIMPLIFIED_PATH)? {
        let hsk_ent = hsk_ent?;
        println!("Converting entry {}", hsk_ent.path().display());

        let mut f = std::fs::File::open(hsk_ent.path())?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let mut out = "n,character,pinyin\n".to_string();
        for (num, word, pinyin) in s.lines().skip(1).map(|s| {
            let mut line_iter = s.split(',');
            let num = line_iter.next().unwrap();
            let simplified_word = line_iter.next().unwrap();
            let word = opencc.convert(simplified_word);
            if simplified_word != word {
                println!("\tConverted {} -> {}", &simplified_word, &word);
            }
            let pinyin = line_iter.next().unwrap();
            (num, word, pinyin)
        }) {
            out.push_str(&format!("{},{},{}\n", num, word, pinyin));
        }
        let mut path = PathBuf::from(TRADITIONAL_PATH);
        path.push(hsk_ent.path().file_name().unwrap());
        println!("Writing output to {}", path.display());

        let mut out_file = std::fs::File::create(path)?;
        out_file.write_all(out.as_bytes())?;
    }

    println!("All done");
    Ok(())
}
