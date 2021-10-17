use std::io;
use std::io::BufRead;
use std::process;
use std::sync::Arc;

use std::str::FromStr;

use clap::{App, Arg, ArgMatches};
use futures::{stream, StreamExt};

fn parse_args() -> ArgMatches<'static> {
    App::new("Web grep")
        .version("0.1.0")
        .about("Tool for fetching and counting word")
        .arg(
            Arg::with_name("word")
                .long("word")
                .takes_value(true)
                .required(true)
                .help("Word to find"),
        )
        .arg(
            Arg::with_name("limit")
                .long("limit")
                .default_value("16")
                .help("Concurrency limit"),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .default_value("60")
                .help("Timeout in sec"),
        )
        .get_matches()
}

fn parse_num_arg<T: FromStr>(args: &ArgMatches, name: &str, default: T) -> T {
    match args.value_of(name) {
        None => default,
        Some(s) => match s.parse::<T>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid {} argument: {}", name, s);
                process::exit(1);
            }
        },
    }
}

#[tokio::main]
async fn main() {
    let args = parse_args();
    let word = args.value_of("word").expect("Specify --word to find");
    let limit: u32 = parse_num_arg(&args, "limit", 16);
    let timeout: u32 = parse_num_arg(&args, "timeout", 60);

    if word.len() == 0 {
        println!("Specify --word argument to find");
        process::exit(1);
    }

    if limit < 1 {
        println!("Limit should be positive");
        process::exit(1);
    }

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let word = Arc::new(word);
    let future = web_grep(word, limit, timeout);
    runtime.block_on(future);
}

async fn web_grep(word: Arc<&str>, limit: u32, timeout: u32) {
    let client = reqwest::Client::builder().build().unwrap();

    let mut urls = Vec::new();
    for line in io::stdin().lock().lines() {
        urls.push(line.unwrap());
    }

    let bodies = stream::iter(urls)
        .map(|url| {
            // we need to clone client object inside future
            let client = client.clone();
            tokio::spawn(async move {
                let response = client.get(url).send().await;
                if let Err(e) = response {
                    eprintln!("Got a reqwest::Error: {}", e);
                    return 0;
                }

                let bytes = response.unwrap().bytes().await;
                let cnt: usize = match bytes {
                    Ok(b) => {
                        let s = format!("{:?}", b);
                        let __wrd = word.clone().to_string().as_str();
                        let cnt = s.matches(__wrd).count();
                        cnt
                    }
                    Err(e) => {
                        eprintln!("Invalid response bytes: {}", e);
                        0
                    }
                };

                cnt
            })
        })
        .buffer_unordered(limit as usize);

    bodies
        .for_each(|body| async {
            if let Err(e) = body {
                eprintln!("Got a tokio::JoinError: {}", e)
            }
        })
        .await;
}
