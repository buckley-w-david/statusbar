use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::num::ParseIntError;
use futures_lite::io::{BufReader, AsyncBufReadExt};

use systemstat::{System, Platform};

use async_fs::File;
use async_trait::async_trait;

pub struct CpuResource;

#[derive(Debug)]
struct CpuError(String);

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for CpuError {}

static PREVIOUS_CPU: [AtomicU64; 7] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

struct ProcCpuMeasure {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
}

impl FromStr for ProcCpuMeasure {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measures: Vec<&str> = s.trim_start_matches("cpu").trim_start()
                                 .split(' ')
                                 .collect();

        Ok(
            ProcCpuMeasure {
                user: measures[0].parse::<u64>()?,
                nice: measures[1].parse::<u64>()?,
                system: measures[2].parse::<u64>()?,
                idle: measures[3].parse::<u64>()?,
                iowait: measures[4].parse::<u64>()?,
                irq: measures[5].parse::<u64>()?,
                softirq: measures[6].parse::<u64>()?,
            }
        )
    }
}

// NOTE: Presumably as more resource related blocks are implemented, they will also all use results
// from reading the /proc/stat file. It would be nice if I could somehow cache the result of
// reading and parsing it each iteration and allow each block to use the cached version.
// This would introduce considerable complexity though

#[async_trait]
impl resource::Resource for CpuResource {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let mut file = BufReader::new(File::open("/proc/stat").await?);
        let mut buf = String::new();
        file.read_line(&mut buf).await?;
        let cpu = buf.parse::<ProcCpuMeasure>()?;

        // Algorithm for determining percentage from slstatus
        // https://git.suckless.org/slstatus/file/components/cpu.c.html
        let b = PREVIOUS_CPU[0..7]
            .iter()
            .map(|i| i.load(Ordering::Relaxed))
            .sum::<u64>() as i64;

        let a = (cpu.user + cpu.nice + cpu.system + cpu.idle + cpu.iowait + cpu.irq + cpu.softirq)
            as i64;

        let sum = b - a;
        if sum == 0 {
            return Err(Box::new(CpuError("Oops".into())));
        }

        let b = (PREVIOUS_CPU[0].load(Ordering::Relaxed)
            + PREVIOUS_CPU[1].load(Ordering::Relaxed)
            + PREVIOUS_CPU[2].load(Ordering::Relaxed)
            + PREVIOUS_CPU[5].load(Ordering::Relaxed)
            + PREVIOUS_CPU[6].load(Ordering::Relaxed)) as f64;

        let a = (cpu.user + cpu.nice + cpu.system + cpu.irq + cpu.softirq) as f64;

        PREVIOUS_CPU[0].store(cpu.user, Ordering::Relaxed);
        PREVIOUS_CPU[1].store(cpu.nice, Ordering::Relaxed);
        PREVIOUS_CPU[2].store(cpu.system, Ordering::Relaxed);
        PREVIOUS_CPU[3].store(cpu.idle, Ordering::Relaxed);
        PREVIOUS_CPU[4].store(cpu.iowait, Ordering::Relaxed);
        PREVIOUS_CPU[5].store(cpu.irq, Ordering::Relaxed);
        PREVIOUS_CPU[6].store(cpu.softirq, Ordering::Relaxed);

        Ok(format!("{:.2}", 100.0 * (b - a) / (sum as f64)))
    }
}

pub struct LoadAverageResource;

#[async_trait]
impl resource::Resource for LoadAverageResource {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let sys = System::new();
        let avg = sys.load_average()?;
        Ok(format!("{} {} {}", avg.one, avg.five, avg.fifteen))
    }
}
