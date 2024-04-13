use std::{
    fs,
    io::BufRead,
    path::{Path, PathBuf},
};

use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to file
    path: Option<PathBuf>,

    /// Show count in file
    #[arg(short)]
    count: bool,

    /// Show count of lines in file
    #[arg(short)]
    line: bool,

    /// Show total of words in file
    #[arg(short)]
    word: bool,

    /// Show total of character (multibyte) in file
    #[arg(short)]
    multibyte: bool,
}

struct Wc {
    path: PathBuf,
    value: String,
    count: usize,
    lines: usize,
    words: usize,
    multibyte: usize,
}

impl Wc {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone(),
            value: fs::read_to_string(path).unwrap(),
            count: 0,
            lines: 0,
            words: 0,
            multibyte: 0,
        }
    }

    fn count(mut self) -> Self {
        self.count = self.value.len();
        self
    }

    fn line(mut self) -> Self {
        // -1 because im on mac
        // https://stackoverflow.com/questions/12616039/wc-command-of-mac-showing-one-less-result#12616274
        self.lines = self.value.as_bytes().lines().count() - 1;
        self
    }

    fn multibyte(mut self) -> Self {
        // multibyte UTF8 safe with chars(), if ASCII can use as_bytes()
        let total_multibyte = self.value.chars().count();
        self.multibyte = total_multibyte;
        self
    }

    fn word(mut self) -> Self {
        let total_words = self.value.split_ascii_whitespace().count();
        self.words = total_words;
        self
    }

    fn display(&self) {
        fn format(val: usize) -> String {
            if val > 0 {
                format!("{} ", val)
            } else {
                String::new()
            }
        }
        println!(
            "{}{}{}{}{}",
            format(self.lines),
            format(self.multibyte),
            format(self.count),
            format(self.words),
            self.path.display()
        );
    }
}

fn main() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(path) => {
            let check_path = Path::new(&path);
            if !Path::exists(check_path) {
                panic!("File not found in path");
            }
            path
        }
        None => panic!("Path not specified!"),
    };

    let wc = Wc::new(path);
    if cli.count {
        wc.count().display();
    } else if cli.line {
        wc.line().display();
    } else if cli.multibyte {
        wc.multibyte().display();
    } else if cli.word {
        wc.word().display();
    } else {
        wc.count().line().word().multibyte().display()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::process::Command;

    fn call_wc(args: &str) -> usize {
        let output = Command::new("wc")
            .arg(args)
            .arg("test.txt")
            .output()
            .expect("Failed to execute command");
        let output_string = String::from_utf8_lossy(&output.stdout);
        let value = output_string
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        println!("{value:?}");
        value
    }
    #[test]
    fn test_count() {
        let path = PathBuf::from("test.txt");
        let wc = Wc::new(path.clone()).count();
        assert_eq!(wc.count, call_wc("-c"));
    }

    #[test]
    fn test_line() {
        let path = PathBuf::from("test.txt");
        let wc = Wc::new(path.clone()).line();
        assert_eq!(wc.lines, call_wc("-l"));
    }

    #[test]
    fn test_multibyte() {
        let path = PathBuf::from("test.txt");
        let wc = Wc::new(path.clone()).multibyte();
        assert_eq!(wc.multibyte, call_wc("-m"));
    }

    #[test]
    fn test_word() {
        let path = PathBuf::from("test.txt");
        let wc = Wc::new(path.clone()).word();
        assert_eq!(wc.words, call_wc("-w"));
    }
}
