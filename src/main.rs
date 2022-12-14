extern crate clipboard;

use crate::clipboard::ClipboardProvider;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, f32, str::FromStr};

/// From a Vec<f32> returns the average,min,max and mean (in that order)
/// ```
/// stat(vec![vec![4.0, 2.0, 1.0, 3.0]])
/// >>> (2.5, 1.0, 4.0, 3.0)
/// ```
#[derive(Debug, PartialEq)]
struct Stats {
    average: f32,
    min: f32,
    max: f32,
    mean: f32,
}

fn stats(mut x: Vec<f32>) -> Stats {
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len: i32 = x.len() as i32;
    // Ensures that if there are no numbers that a zero is returned
    if len == 0 {
        return Stats {
            average: 0.0,
            min: 0.0,
            max: 0.0,
            mean: 0.0,
        };
    }
    let sum: f32 = x.iter().sum();
    Stats {
        average: sum as f32 / len as f32,
        min: x[0],
        max: x[(len - 1) as usize],
        mean: x[{ ((len as f32) / 2.0) as i32 } as usize],
    }
}

fn get_from_clipboard() -> Vec<f32> {
    let mut nums: Vec<f32> = Vec::new();
    // Get the clipboard if it is not possible return a empty string
    for x in match clipboard::ClipboardContext::new() {
        Ok(mut t) => match t.get_contents() {
            Ok(clipboard_contents) => clipboard_contents,
            Err(_) => "".to_string(),
        },
        Err(_) => "".to_string(),
    }
    .split(' ')
    {
        if let Ok(t) = f32::from_str(x) {
            nums.push(t)
        }
    }
    nums
}

fn get_from_file(args: Vec<String>) -> Vec<f32> {
    let mut nums: Vec<f32> = Vec::new();
    match File::open(&args[1]) {
        Ok(t) => {
            for (_, line) in BufReader::new(t).lines().enumerate() {
                if let Ok(t) = line {
                    if let Ok(t) = f32::from_str(&t) {
                        nums.push(t);
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}")
        }
    };
    nums
}

fn main() {
    // todo: add a usage section allow it to be reached by using -h or --h

    let args: Vec<String> = env::args().collect();

    println!("Welcome to Averager");

    if args.len() >= 2
        && !{ args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) }
    {
        let fstats = stats({
            //Runs scenario for a clipboard content
            if args.contains(&"-c".to_string()) {
                get_from_clipboard()
            } else if args.len() == 2 {
                get_from_file(args)
            } else {
                vec![0.0]
            }
        });
        println!("Average = {}", fstats.average);
        println!("Min     = {}", fstats.min);
        println!("Max     = {}", fstats.max);
        println!("Mean    = {}", fstats.mean);

    } else {
        let helptext = "
            Welcome to 'Average' a quick way to get a the average number: \n
            Usage:
                <path>     : gets the average of a file
                -h, --help : Displays this help message
                -c         : Gets the average of the numbers on the system clipboard 
            ";
        println!("{}", helptext);
    }
}

#[cfg(test)]
mod tests {
    use crate::stats;
    use crate::Stats;
    #[test]
    fn status_test() {
        assert_eq!(
            stats(vec![1.0, 2.0, 3.0, 4.0]),
            Stats {
                average: 2.5,
                min: 1.0,
                max: 4.0,
                mean: 3.0
            }
        );
    }
}
