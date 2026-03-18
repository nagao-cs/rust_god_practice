mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

/// ファイルをオープンし, 行ごとにマッチングを行う
/// 
/// マッチングはそれぞれの行頭から1文字ずつずらして行い, 
/// いずれかにマッチした場合に, その行がマッチしたものとみなす
/// 
/// 例えば, abcdという文字列があった場合, 以下の順にマッチが行われ, 
/// このいずれかにマッチした場合, 与えられた正規表現にマッチする行と判定する
/// 
fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    println!("regex: {}", expr);
    println!();

    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], true)? {
                println!("{line}");
                break;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), DynError>{
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }
    Ok(())
}
