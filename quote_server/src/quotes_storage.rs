use async_trait::async_trait;
use rand::{distributions::Uniform, rngs::SmallRng, Rng, SeedableRng};
use std::{
    io::{BufRead, BufReader},
    mem::swap,
    path::PathBuf,
    {fs, vec},
};

#[async_trait]
pub trait QuotesStorage: Sync + Send {
    async fn get_quote(&mut self) -> String;
}

pub struct QuotesStorageImpl {
    quotes: Vec<String>,
    rand_range: SmallRng,
}

impl QuotesStorageImpl {
    pub fn new(quotes_file: &PathBuf, skip_lines: usize) -> QuotesStorageImpl {
        let quotes = Self::read_quotes_from_file(quotes_file, skip_lines);
        QuotesStorageImpl {
            quotes,
            rand_range: SmallRng::from_entropy(),
        }
    }

    fn read_quotes_from_file(quotes_file: &PathBuf, skip_lines: usize) -> Vec<String> {
        let file = fs::File::open(quotes_file).expect(&format!("Failed open: {:?}", quotes_file));
        let reader = BufReader::new(file).lines();
        let mut quotes: Vec<String> = vec![];
        let mut buf = String::new();
        let _ = reader
            .filter_map(|line| line.ok())
            .skip(skip_lines)
            .map(|mut line| {
                if line.is_empty() && buf.is_empty() == false {
                    swap(&mut line, &mut buf);
                    quotes.push(line);
                } else if !line.is_empty() {
                    if !buf.is_empty() {
                        buf.push('\n')
                    }
                    buf.push_str(&line);
                }
                ()
            })
            .filter(|_| false)
            .collect::<()>();

        if !buf.is_empty() {
            quotes.push(buf);
        }
        assert!(!quotes.is_empty(), "Failed to read quotes - empty");
        quotes
    }
}

#[async_trait]
impl QuotesStorage for QuotesStorageImpl {
    async fn get_quote(&mut self) -> String {
        let x = self.rand_range.sample(Uniform::new(0, self.quotes.len()));
        self.quotes[x].to_string()
    }
}
