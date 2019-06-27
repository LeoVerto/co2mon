use co2mon::{Result, Sensor};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
    let sensor = Sensor::open_default()?;
    
    match sensor.read() {
        Ok(reading) => printData(reading),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}

fn printData(reading: co2mon::Reading) {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
    .expect("Time went backwards").as_secs();
    
    println!(
        "{:?}000000000,{:.4},{}",
        since_the_epoch,
        reading.temperature(),
        reading.co2(),
    );
}
