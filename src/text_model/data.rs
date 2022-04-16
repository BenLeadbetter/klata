use quick_xml::{
    Reader,
    events::Event,
};
use std::{
    boxed::Box,
    error::Error,
};

#[derive(Default, Debug, PartialEq)]
pub struct Data {
    pub text: String,
    pub author: Option<String>,
    pub date: Option<[u16;3]>,
}

#[derive(Debug)]
enum DataError {
    UnsupportedCharacter(char),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataError::UnsupportedCharacter(c) => {
                write!(f, "Unsupported character '{}'", c)
            }
        }
    }
    
}

impl Error for DataError {}

impl Data {
    pub fn from_string(s: &str) -> Result<Data, Box<dyn Error>> {
        let mut reader = Reader::from_str(s);
        let mut data = Data::default();
        loop {
            match reader.read_event(&mut Vec::new())? {
                Event::Start(e) => {
                    if e.name() == "klata_text".as_bytes() {
                        data = read_text(&mut reader)?;
                    }
                },
                Event::Eof => break,
                _ => {},
            }
        }

        data.text = data.text.trim_start().to_string();

        if let Some(c) = data.text
            .chars()
            .filter(|c| !c.is_ascii_alphanumeric())
            .filter(|c| !c.is_ascii_whitespace())
            .filter(|c| !c.is_ascii_punctuation())
            .filter(|c| *c != '\n')
            .next() {
            return Err(Box::new(DataError::UnsupportedCharacter(c)));
        }
            
        Ok(data)
    }
}

fn read_text<B: std::io::BufRead>(reader: &mut Reader<B>) -> Result<Data, Box<dyn Error>> {
    let mut data = Data::default();
    loop {
        match reader.read_event(&mut Vec::new())? {
            Event::End(end_bytes) => {
                if end_bytes.name() == "klata_text".as_bytes() {
                    break;
                }
            },
            Event::Start(start_bytes) => {
                if start_bytes.name() == "text".as_bytes() {
                    data.text = reader.read_text(start_bytes.name(), &mut Vec::new())?;
                } else if start_bytes.name() == "author".as_bytes() {
                    data.author = Some(reader.read_text(start_bytes.name(), &mut Vec::new())?);
                } else if start_bytes.name() == "date".as_bytes() {
                    data.date = Some(read_date(reader)?);
                }
            }
            _ => {},
        }
    }
    Ok(data)
}

fn read_date<B: std::io::BufRead>(reader: &mut Reader<B>) -> Result<[u16;3], Box<dyn Error>> {
    let mut date: [u16;3] = [0, 0, 0];
    loop {
        match reader.read_event(&mut Vec::new())? {
            Event::End(end_bytes) => {
                if end_bytes.name() == "date".as_bytes() {
                    break;
                }
            },
            Event::Start(start_bytes) => {
                if start_bytes.name() == "d".as_bytes() {
                    let s = reader.read_text(start_bytes.name(), &mut Vec::new())?;
                    date[0] = s.parse::<u16>()?;
                } else if start_bytes.name() == "m".as_bytes() {
                    let s = reader.read_text(start_bytes.name(), &mut Vec::new())?;
                    date[1] = s.parse::<u16>()?;
                } else if start_bytes.name() == "y".as_bytes() {
                    let s = reader.read_text(start_bytes.name(), &mut Vec::new())?;
                    date[2] = s.parse::<u16>()?;
                } 
            }
            _ => {},
        }
    }
    Ok(date)
}