extern crate regex;

use regex::Regex;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
pub struct Subtitle {
    name: String,
    entries: Vec<SubEntry>
}

#[derive(Debug)]
struct SubEntry {
    num: usize,
    start: Timestamp,
    stop: Timestamp,
    text: String
}

#[derive(PartialEq, Debug)]
struct Timestamp {
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16
}

impl Subtitle {
    pub fn new(name: &str) -> Subtitle {
        Subtitle {
            name: String::from(name),
            entries: Vec::new()
        }
    }

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

    pub fn translate(self) {
        for entry in self.entries.into_iter() {
            println!("\nTranslate: {}", entry.text);
        }
    }
}

impl SubEntry {
    fn new(num: usize, start: Timestamp, stop: Timestamp, text: &str) -> SubEntry {
        SubEntry {
            num: num,
            start: start,
            stop: stop,
            text: String::from(text)
        }
    }

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
        println!("text: {}", text);

        Ok(SubEntry{
            num: num,
            start: start,
            stop: stop,
            text: text
        })
    }
}

impl Timestamp {
    fn new(h: u8, m: u8, s: u8, ms: u16) -> Timestamp {
        Timestamp {
            hour: h,
            minute: m,
            second: s,
            millisecond: ms
        }
    }

    fn from(timestamp_text: &str) -> Result<Timestamp, &'static str> {
        let re = Regex::new(r"(\d{2}):(\d{2}):(\d{2}),(\d{3})");
        match re {
            Ok(re) => {
                if let Some(matches) = re.captures(timestamp_text) {
                    Ok(Timestamp{
                        hour: matches.at(1).unwrap().parse::<u8>().unwrap(),
                        minute: matches.at(2).unwrap().parse::<u8>().unwrap(),
                        second: matches.at(3).unwrap().parse::<u8>().unwrap(),
                        millisecond: matches.at(4).unwrap().parse::<u16>().unwrap()
                    })
                } else {
                    Err("Text could not be parsed as Timestamp")
                }
            }
            Err(e) => Err("An error with the regex compiliation")
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
