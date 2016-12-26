extern crate regex;
extern crate itertools;

use regex::Regex;
use itertools::Itertools;

use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
/// A subtitle comprise a string and a vector of entries.
pub struct Subtitle {
    name: String,
    entries: Vec<SubEntry>
}

#[derive(Debug, Clone)]
struct SubEntry {
    num: usize,
    start: Timestamp,
    stop: Timestamp,
    text: String
}

#[derive(PartialEq, Debug, Clone)]
struct Timestamp {
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16
}

impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut entries_text = String::new();
        for entry in self.entries.iter() {
            entries_text.push_str(&entry.to_string());
            entries_text.push_str("\n");
        }
        write!(f, "{}", entries_text)
    }
}

impl fmt::Display for SubEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{} --> {}\n{}", self.num, self.start, self.stop, self.text)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02},{:03}", self.hour, self.minute, self.second, self.millisecond)
    }
}


impl Subtitle {
    /// Create a new, uninitialized subtitle.
    /// If a non-existent subtitle path is provided, no error will be provided until when parsing.
    pub fn new(name: &str) -> Subtitle {
        Subtitle {
            name: String::from(name),
            entries: Vec::new()
        }
    }

    /// Parse the subtitle, filling the subtitle struct with information that can be translated.
    pub fn parse(&mut self) {
        let f = File::open(&self.name).unwrap();
        let buf = BufReader::new(f);


        let mut prev_blank = false;
        let mut entry_text = Vec::new();

        // Iterate over lines in buffer
        for line in buf.lines() {
            let line = line.unwrap();

            println!("Line: {}", line);
            if line == "" {
                prev_blank = true;
            } else if prev_blank == true {
                println!("Line to be kuked: \n{}\n", entry_text.join("\n"));
                self.entries.push(SubEntry::from(&entry_text.join("\n")).unwrap());
                entry_text = vec!(line);
                prev_blank = false;
            } else {
                prev_blank = false;
                entry_text.push(line);
            }
        }
        self.entries.push(SubEntry::from(&entry_text.join("\n")).unwrap());
    }

    /// Translate the subtitle.
    /// Returning a copy of the subtitle, but where the name is changed and the text is translated
    pub fn translate(&self, trans_name: &str) -> Subtitle {
        let mut translated: Subtitle = self.clone();
        translated.name = String::from(trans_name);

        for (orig, trans)  in self.entries.iter().zip(translated.entries.iter_mut()) {
            println!("\nTranslate: {}\n", orig.text);
            println!("\nInto: {}\n", trans.text);
            let mut buffer = String::new();
            if let Ok(_) = io::stdin().read_to_string(&mut buffer) {
                let buffer = buffer;
                trans.text = buffer;
                println!("Yey");
            } else {
                println!("WTF");
            }
        }

        translated
    }

    /// Write the subtitle to disk.
    /// Will either fail and print an error, or write the translated subtitle to disk.
    pub fn write(&self) {
        let file = OpenOptions::new().write(true).create(true).open(&self.name);
        match file {
            Ok(mut file) => { write!(file, "{}", self); },
            Err(e) => println!("Error opening/creating file: {}", e)
        }
    }
}

impl SubEntry {
    /// Manually create a subentry
    #[allow(unused)]
    fn new(num: usize, start: Timestamp, stop: Timestamp, text: &str) -> SubEntry {
        SubEntry {
            num: num,
            start: start,
            stop: stop,
            text: String::from(text)
        }
    }

    /// Parse an entry text into a SubEntry.
    /// This is dangerous and is very error prone because of very lacking error handling.
    /// TODO: Fix error handling
    fn from(entry_text: &str) -> Result<SubEntry, &'static str> {
        // Get the lines
        let mut lines = entry_text.trim().lines();

        // The first line is a number
        let num = lines.next().unwrap().parse::<usize>().unwrap();

        // The next line is the timestamp
        let mut stamps = lines.next().unwrap().split("-->");
        let start = Timestamp::from(stamps.next().unwrap()).unwrap();
        let stop = Timestamp::from(stamps.next().unwrap()).unwrap();

        let text = lines.join("\n");

        Ok(SubEntry{
            num: num,
            start: start,
            stop: stop,
            text: text
        })
    }
}

impl Timestamp {
    /// Manually create a timestamp
    #[allow(unused)]
    fn new(h: u8, m: u8, s: u8, ms: u16) -> Timestamp {
        Timestamp {
            hour: h,
            minute: m,
            second: s,
            millisecond: ms
        }
    }

    /// Parse a string into a timestamp
    fn from(timestamp_text: &str) -> Result<Timestamp, String> {
        let re = Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3})");
        match re {
            Ok(re) => {
                if let Some(matches) = re.captures(timestamp_text) {
                    // These unwraps should be totally safe, because the combination of the Ok and
                    // Some mean that all four integer fields were matched.
                    Ok(Timestamp{
                        hour: matches.at(1).unwrap().parse::<u8>().unwrap(),
                        minute: matches.at(2).unwrap().parse::<u8>().unwrap(),
                        second: matches.at(3).unwrap().parse::<u8>().unwrap(),
                        millisecond: matches.at(4).unwrap().parse::<u16>().unwrap()
                    })
                } else {
                    Err(String::from("Text could not be parsed as Timestamp"))
                }
            }
            Err(e) => Err(format!("Regex error: {}", e.description()))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_work() {
        let stuff = super::Timestamp::from("00:00:01,013 ");
        assert!(
            match stuff {
                Ok(stuff) => {
                    stuff == super::Timestamp{ hour: 0, minute: 0, second: 1, millisecond: 13}
                } Err(e) => { 
                    println!("Error: {}", e);
                    false
                }
            }
        )
    }

    #[test]
    fn testentry() {
        let text = "
        
        372
            00:40:48,721 --> 00:40:52,089
            Hvorfor tror du jeg tillater dette?
            

            ";
        println!("{:?}", super::SubEntry::from(text));
    }
}
