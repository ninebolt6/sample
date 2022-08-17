use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use sample::{get_top10_avg, get_top10_scorer, Log};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("no filepath specified.")
    }
    let input_file = File::open(&args[1]).expect("file not found.");
    let mut lines = BufReader::new(input_file).lines();
    let _header = lines.next().unwrap().unwrap();
    let logs = lines
        .map(|line| line.expect("bad csv"))
        .map(|line| {
            let base: Vec<&str> = line.split(',').collect();
            Log {
                date: base[0].to_string(),
                player_id: base[1].to_string(),
                score: base[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let top10_avg = get_top10_avg(logs);
    let ranking = get_top10_scorer(top10_avg);

    // output
    let mut output_file = File::create("output.csv").expect("error creating output file.");
    writeln!(output_file, "rank,player_id,mean_score").expect("error writing data to output file.");
    for line in ranking {
        writeln!(
            output_file,
            "{},{},{}",
            line.rank, line.player_id, line.mean_score
        )
        .expect("error writing data to output file.");
    }
    output_file
        .flush()
        .expect("error saving data to output file.");
}
