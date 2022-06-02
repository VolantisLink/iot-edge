use std::io::Write;
use std::str::FromStr;
use file_rotate::{
    FileRotate, 
    ContentLimit, 
    suffix::AppendCount,
    compression::Compression,
};
use bytesize;

use crate::message::Chunk;
use crate::config::ConfigLog;

pub struct FileLogger(FileRotate<AppendCount>);

impl From<&ConfigLog> for FileLogger {
    fn from(config: &ConfigLog) -> Self {
        let keep = config.rotate_keep;
        let size = bytesize::ByteSize::from_str(&config.rotate_size).unwrap().as_u64();

        FileLogger(FileRotate::new(
            config.path.clone(), 
            AppendCount::new(keep), 
            ContentLimit::Bytes(size as usize),
            Compression::None
        ))
    }
}

impl FileLogger {
    pub fn write(&mut self, chunk: &Chunk) {
        let line_str = serde_json::to_string(chunk).unwrap();
        writeln!(self.0, "{}", &line_str).unwrap();
    }
}

#[test]
fn test() {
    let mut log = FileRotate::new(
        "logs/test.log", 
        AppendCount::new(2), 
        ContentLimit::Lines(3),
        Compression::None
    );

    // Write a bunch of lines
    writeln!(log, "Line 1: Hello World!").unwrap();
    for idx in 2..=10 {
        writeln!(log, "Line {}", idx).unwrap();
    }
}

#[test]
fn test_size() {
    let expected = bytesize::ByteSize::gb(20);
    
    let size = bytesize::ByteSize::from_str("20 G").unwrap();
    assert_eq!(size, expected);

    let size = bytesize::ByteSize::from_str("20G").unwrap();
    assert_eq!(size, expected);

    let size = bytesize::ByteSize::from_str("20 GB").unwrap();
    assert_eq!(size, expected);

    let size = bytesize::ByteSize::from_str("20GB").unwrap();
    assert_eq!(size, expected);

    let size = bytesize::ByteSize::from_str("20          G").unwrap();
    assert_eq!(size, expected);

    let size = bytesize::ByteSize::from_str("20          GB").unwrap();
    assert_eq!(size, expected);
}

