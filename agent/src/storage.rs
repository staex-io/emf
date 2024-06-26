use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::MetadataExt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::Res;

const H24: Duration = Duration::from_secs(60 * 60 * 24);

#[derive(Serialize, Deserialize)]
struct Data {
    first_measurement: SystemTime,
    last_measurement: SystemTime,
    measurements: Vec<u128>,
}

pub(crate) fn save(filepath: &str, value: u128) -> Res<Vec<u128>> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(filepath)?;
    let file_size = file.metadata()?.size() as usize;

    let mut buf: Vec<u8> = vec![0; file_size];
    let n = file.read(&mut buf)?;
    buf.truncate(n);

    if buf.is_empty() {
        let data = Data {
            first_measurement: SystemTime::now(),
            last_measurement: SystemTime::now(),
            measurements: vec![value],
        };

        write_to_file(file, data)?;

        return Ok(vec![]);
    }

    let mut data: Data = serde_json::from_slice(&buf)?;
    data.measurements.push(value);

    let time_to_accumulate: u64 = std::env::var("TIME_TO_ACCUMULATE")
        .unwrap_or(format!("{}", H24.as_secs()))
        .parse()
        .unwrap();
    trace!("time to accumulate {time_to_accumulate}");
    if data.last_measurement.duration_since(data.first_measurement)?
        >= Duration::from_secs(time_to_accumulate)
    {
        let saved_measurements = data.measurements.clone();

        data.first_measurement = UNIX_EPOCH;
        data.last_measurement = UNIX_EPOCH;
        data.measurements = vec![];
        write_to_file(file, data)?;

        return Ok(saved_measurements);
    }

    data.last_measurement = SystemTime::now();
    write_to_file(file, data)?;

    Ok(vec![])
}

fn write_to_file(mut file: File, data: Data) -> Res<()> {
    let buf = serde_json::to_vec(&data)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&buf)?;
    file.set_len(buf.len() as u64)?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, time::SystemTime};

    use crate::storage;

    #[test]
    fn test_storage() {
        let filepath = format!("{}{}", temp_dir().to_string_lossy(), rand::random::<u128>());
        eprintln!("\nTemporary storage file path: {filepath}");

        for _ in 0..89 {
            let random_number = rand::random();
            let res = storage::save(&filepath, random_number).unwrap();
            assert!(res.is_empty());
        }

        let buf = std::fs::read(&filepath).unwrap();
        let mut data: storage::Data = serde_json::from_slice(&buf).unwrap();
        data.last_measurement = SystemTime::now() + storage::H24;
        let buf = serde_json::to_vec(&data).unwrap();
        std::fs::write(&filepath, buf).unwrap();

        let random_number = rand::random();
        let res = storage::save(&filepath, random_number).unwrap();
        assert!(!res.is_empty());
        assert_eq!(90, res.len());
        assert_eq!(random_number, *res.last().unwrap());
    }
}
