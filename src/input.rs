use crate::logger;
use std::io::stdin;
use std::str::FromStr;

pub struct Input {
    tokens: Vec<String>,
    current_index: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            current_index: 0,
        }
    }

    pub fn read_line(&self) -> String {
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => logger::info(&format!("read line: {}", &buffer)),
            Err(_) => logger::abort("server connection closed, exiting"),
        };

        buffer
    }

    pub fn parse_line(&mut self) {
        let line = self.read_line();

        self.tokens = line
            .split_whitespace()
            .filter(|c| !c.is_empty())
            .map(|c| c.to_string())
            .collect();

        self.current_index = 0;
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        let token = &self.tokens[self.current_index];
        self.current_index += 1;

        let result = token
            .parse()
            .unwrap_or_else(|_| logger::abort("server connection closed, exiting"));

        result
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next()
    }

    pub fn next_usize(&mut self) -> usize {
        self.next()
    }
}
