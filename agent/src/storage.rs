use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    // Create file if not exists.
    std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(filepath)?;
    // And read it.
    let buf = std::fs::read(filepath)?;

    if buf.is_empty() {
        let data = Data {
            first_measurement: SystemTime::now(),
            last_measurement: SystemTime::now(),
            measurements: vec![value],
        };

        write_to_file(filepath, data)?;

        return Ok(vec![]);
    }

    let mut data: Data = serde_json::from_slice(&buf)?;
    if data.first_measurement.eq(&UNIX_EPOCH) {
        data.first_measurement = SystemTime::now();
    }
    data.measurements.push(value);

    if data.last_measurement.duration_since(data.first_measurement)? >= H24 {
        let saved_measurements = data.measurements.clone();

        data.first_measurement = UNIX_EPOCH;
        data.last_measurement = UNIX_EPOCH;
        data.measurements = vec![];
        write_to_file(filepath, data)?;

        return Ok(saved_measurements);
    }

    data.last_measurement = SystemTime::now();
    write_to_file(filepath, data)?;

    Ok(vec![])
}

fn write_to_file(filepath: &str, data: Data) -> Res<()> {
    let buf = serde_json::to_vec(&data)?;
    Ok(std::fs::write(filepath, buf)?)
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, time::SystemTime};

    use crate::storage;

    #[test]
    fn test() {
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
