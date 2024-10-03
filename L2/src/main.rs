use std::{collections::VecDeque, fs, io::{BufReader, Read, Write}};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
pub struct Cli {
    #[clap(short, long, default_value = "1")]
    pub key: usize,

    #[clap(short, long)]
    #[arg(group = "type")]
    pub numeric_sort: bool,

    #[clap(short, long)]
    pub reverse: bool,

    #[clap(short, long)]
    pub unique: bool,

    // #[clap(short='M', long)]
    // #[arg(group = "type")]
    // pub month_sort: bool,

    // #[clap(short, long)]
    // pub check: bool,

    // #[clap(short='b', long)]
    // pub ignore_leading_blanks: bool,

    // #[clap(short, long)]
    // #[arg(group = "type")]
    // pub human_numeric_sort: bool,

    #[clap(required = true)]
    pub input_file_path: String,

    #[clap(required = true)]
    pub output_file_path: String,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let file = fs::File::open(cli.input_file_path)?;

    let mut br = BufReader::new(file);
    let mut contents = String::new();

    let _ = br.read_to_string(&mut contents)?;

    let mut lines: Vec<String> = contents.split('\n')
        .map(|x| String::from(x))
        .collect();

    if cli.unique { 
        lines.sort();
        lines.dedup(); 
    }

    println!("{:#?}", lines);

    lines.sort_by(|x, y| {
        let mut a: VecDeque<String> = x.clone()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();
        let mut b: VecDeque<String> = y.clone()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();

        if cli.key - 1 < a.len() {
            if let Some(x) = a.remove(cli.key - 1) {
                a.push_front(x);
            }
        }

        reord_or_ignore(&mut a, cli.key - 1);
        reord_or_ignore(&mut b, cli.key - 1);

        println!("{:#?}",a);
        println!("{:#?}", b);

        if cli.numeric_sort {
            let mut first_numeric: Option<i64> = None;
            let mut second_numeric: Option<i64> = None;
            if let Some(first) = a.front() {
                first_numeric = match first.parse::<i64>() {
                    Ok(x) => Some(x), 
                    Err(_) => None
                }
            }
            if let Some(second) = b.front() {
                second_numeric = match second.parse::<i64>() {
                    Ok(x) => Some(x),
                    Err(_) => None
                }
            }

            if first_numeric == second_numeric {
                a.pop_front();
                b.pop_front();

                return a.cmp(&b);
            }

            return first_numeric.cmp(&second_numeric);
        }

        a.cmp(&b)
    });

    println!("{:#?}", lines);

    let mut output_file = fs::File::create(cli.output_file_path)?;

    lines.iter()
        .for_each(|x| 
            match output_file.write_fmt(format_args!("{:}\n",  x)) {
                Ok(_) => (),
                Err(_) => panic!("can't write to file")
            }
        );

    Ok(())
}

fn reord_or_ignore(a: &mut VecDeque<String>, index: usize) {
    if index < a.len() {
        if let Some(x) = a.remove(index) {
            a.push_front(x);
        }
    }
}
