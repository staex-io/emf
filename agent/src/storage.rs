use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::Res;

const H24: Duration = Duration::from_secs(60 * 60 * 24);

pub(crate) struct Storage {
    file: File,
}

#[derive(Serialize, Deserialize)]
struct Data {
    first_measurement: SystemTime,
    last_measurement: SystemTime,
    measurements: Vec<u128>,
}

impl Storage {
    pub(crate) fn new(file_path: &str) -> Res<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(file_path)?;
        Ok(Self { file })
    }

    pub(crate) fn write(&mut self, value: u128) -> Res<Vec<u128>> {
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf)?;

        if buf.is_empty() {
            let data = Data {
                first_measurement: SystemTime::now(),
                last_measurement: SystemTime::now(),
                measurements: vec![value],
            };

            let buf = serde_json::to_vec(&data)?;
            self.file.seek(SeekFrom::Start(0))?;
            self.file.write_all(&buf)?;
            self.file.flush()?;
            self.file.set_len(buf.len() as u64)?;

            return Ok(vec![]);
        }

        let mut data: Data = serde_json::from_slice(&buf)?;
        if data.first_measurement.eq(&UNIX_EPOCH) {
            data.first_measurement = SystemTime::now();
        }
        data.last_measurement = SystemTime::now();
        data.measurements.push(value);
        if data.last_measurement.duration_since(data.first_measurement)? >= H24 {
            data.first_measurement = UNIX_EPOCH;
            data.last_measurement = UNIX_EPOCH;
            data.measurements = vec![];

            let buf = serde_json::to_vec(&data)?;
            self.file.seek(SeekFrom::Start(0))?;
            self.file.write_all(&buf)?;
            self.file.flush()?;
            self.file.set_len(buf.len() as u64)?;

            return Ok(data.measurements);
        }

        let buf = serde_json::to_vec(&data)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&buf)?;
        self.file.flush()?;
        self.file.set_len(buf.len() as u64)?;

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env::temp_dir,
        io::{Read, Write},
        time::SystemTime,
    };

    use crate::storage::{Data, H24};

    use super::Storage;

    #[test]
    fn test() {
        let path = format!("{}{}", temp_dir().to_string_lossy(), rand::random::<u128>());
        eprintln!("\nTemporary storage file path: {path}");
        let mut storage = Storage::new(&path).unwrap();

        for _ in 0..89 {
            let res = storage.write(rand::random()).unwrap();
            assert!(res.is_empty());
        }

        // drop(storage);

        let mut buf = Vec::new();
        storage.file.read_to_end(&mut buf).unwrap();
        let mut data: Data = serde_json::from_slice(&buf).unwrap();
        data.last_measurement = SystemTime::now() + H24;
        let buf = serde_json::to_vec(&data).unwrap();
        storage.file.write_all(&buf).unwrap();
        storage.file.flush().unwrap();

        let random_number = rand::random();
        let res = storage.write(random_number).unwrap();
        assert!(!res.is_empty());
        assert_eq!(90, res.len());
        assert_eq!(random_number, *res.last().unwrap());
    }
}
