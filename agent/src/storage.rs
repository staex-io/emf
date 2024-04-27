use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

use crate::Res;

const DELIMITER: char = '-';

pub(crate) struct Storage {
    file: File,
    current_size: usize,
    iteration_size: usize,
}

impl Storage {
    pub(crate) fn new(file_path: &str) -> Res<Self> {
        let file = OpenOptions::new().create(true).read(true).append(true).open(file_path)?;
        let current_size = Self::count_current_size(&file)?;
        Ok(Self {
            file,
            current_size,
            iteration_size: 30,
        })
    }

    pub(crate) fn write(&mut self, value: u128) -> Res<Vec<u128>> {
        writeln!(self.file, "{}", value)?;
        self.current_size += 1;
        self.file.sync_all()?;

        if self.current_size % self.iteration_size == 0 {
            let last_iteration = self.read_last_iteration()?;
            writeln!(self.file, "{}", DELIMITER)?;
            self.file.sync_all()?;
            return Ok(last_iteration);
        }

        Ok(vec![])
    }

    fn read_last_iteration(&mut self) -> Res<Vec<u128>> {
        let mut pos = self.file.metadata()?.len();
        let mut reader = BufReader::new(self.file.try_clone()?);
        let mut values = Vec::new();

        while pos > 0 && values.len() < self.iteration_size {
            reader.seek(SeekFrom::Start(pos))?;
            let mut byte = [0; 1];
            if reader.read(&mut byte)? == 0 || byte[0] == b'\n' {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                if line.trim().is_empty() || line.contains(DELIMITER) {
                    pos = pos.saturating_sub(1);
                    continue;
                }
                if let Ok(number) = line.trim().parse::<u128>() {
                    values.push(number);
                }
            } else {
                pos = pos.saturating_sub(1);
            }
        }
        Ok(values)
    }

    fn count_current_size(file: &File) -> Res<usize> {
        let reader = BufReader::new(file);
        Ok(reader
            .lines()
            .filter(|line| {
                if let Ok(line) = line.as_ref() {
                    if let Some(char) = line.chars().last() {
                        return char != DELIMITER;
                    }
                }
                false
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use super::Storage;

    #[test]
    fn test() {
        let path = format!("{}{}", temp_dir().to_string_lossy(), rand::random::<u128>());
        let mut storage = Storage::new(&path).unwrap();
        for _ in 0..89 {
            storage.write(rand::random()).unwrap();
        }
        let res = storage.write(rand::random()).unwrap();
        assert_eq!(30, res.len());
        let res = storage.write(rand::random()).unwrap();
        assert!(res.is_empty());
    }
}
