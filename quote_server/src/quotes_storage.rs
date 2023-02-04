use async_trait::async_trait;
use log::error;
use rand;
use rand::distributions::Uniform;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::io::{BufRead, BufReader, Read};
use std::mem::swap;
use std::path::PathBuf;
use std::{fs, vec};

#[async_trait]
pub trait QuotesStorage: Sync + Send {
    async fn get_quote(&mut self) -> String;
}

pub struct QuotesStorageImpl {
    quotes: Vec<String>,
    rand_range: SmallRng,
}

impl QuotesStorageImpl {
    pub fn new(quotes_file: &PathBuf) -> QuotesStorageImpl {
        let mut file = fs::File::open(quotes_file).unwrap();
        let mut reader = BufReader::new(file).lines();
        let mut quotes: Vec<String> = vec![];
        let mut buf = String::new();
        for mut line in reader.filter_map(|line| line.ok()).skip(4) {
            if line.is_empty() && buf.is_empty() == false {
                swap(&mut line, &mut buf);
                quotes.push(line);
            } else if !line.is_empty() {
                if !buf.is_empty() {
                    buf.push('\n')
                }
                buf.push_str(&line);
            }
        }
        if !buf.is_empty() {
            quotes.push(buf);
        }

        QuotesStorageImpl {
            quotes,
            rand_range: SmallRng::from_entropy(),
        }
    }
}

#[async_trait]
impl QuotesStorage for QuotesStorageImpl {
    async fn get_quote(&mut self) -> String {
        let x = self.rand_range.sample(Uniform::new(0, self.quotes.len()));
        self.quotes[x].to_string()
    }
}
