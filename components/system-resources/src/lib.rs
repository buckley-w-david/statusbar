use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;

use procstat::ProcStat;

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

// NOTE: Presumably as more resource related blocks are implemented, they will also all use results
// from reading the /proc/stat file. It would be nice if I could somehow cache the result of
// reading and parsing it each iteration and allow each block to use the cached version.
// This would introduce complexity when update intervals are implemented.

#[async_trait]
impl resource::Resource for CpuResource {
    async fn fetch(&self) -> Result<String, Box<dyn Error>> {
        // Do I really need a dependency just to read and parse the /proc/stat file?
        // TODO: Think about doing that myself
        let proc_stat = ProcStat::read();
        let cpu = proc_stat.cpu;

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

// TODO: Other system-resource type measures like memory
