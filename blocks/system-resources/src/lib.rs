#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use std::error::Error;

use procstat::ProcStat;

pub struct CpuBlock;

lazy_static! {
    static ref PREVIOUS_TOTAL: Mutex<u64> = Mutex::new(0);
    static ref PREVIOUS_IDLE: Mutex<u64> = Mutex::new(0);
    static ref PREVIOUS_NON_IDLE: Mutex<u64> = Mutex::new(0);
    static ref PREVIOUS_CPU: Mutex<procstat::CPU> = Mutex::new(procstat::CPU {
        user: 0,
        nice: 0,
        system: 0,
        idle: 0,
        iowait: 0,
        irq: 0,
        softirq: 0,
        steal: 0,
        guest: 0,
        guest_nice: 0,
    });
}

// NOTE: Presumably as more resource related blocks are implemented, they will also all use results
// from reading the /proc/stat file. It would be nice if I could somehow cache the result of
// reading and parsing it each iteration and allow each block to use the cached version.
// This would introduce complexity when update intervals are implemented.

impl block::Block for CpuBlock {
    fn perform(&self) -> Result<String, Box<dyn Error>> {
        // Do I really need a dependency just to read and parse the /proc/stat file?
        // TODO: Think about doing that myself
        let proc_stat = ProcStat::read();
        let cpu = proc_stat.cpu;

        // This algorithm is from: https://stackoverflow.com/a/23376195/14470574
        // TODO: something more elegent than this mutex crap?
        let mut previous_idle = PREVIOUS_IDLE.lock()?;
        let mut previous_total = PREVIOUS_TOTAL.lock()?;
        let mut previous_non_idle = PREVIOUS_NON_IDLE.lock()?;
        let mut previous_cpu = PREVIOUS_CPU.lock()?;

        *previous_idle = previous_cpu.idle + previous_cpu.iowait;
        *previous_non_idle = previous_cpu.user
            + previous_cpu.nice
            + previous_cpu.system
            + previous_cpu.irq
            + previous_cpu.softirq
            + previous_cpu.steal;
        *previous_total = *previous_idle + *previous_non_idle;

        let idle = cpu.idle + cpu.iowait;
        let non_idle = cpu.user + cpu.nice + cpu.system + cpu.irq + cpu.softirq + cpu.steal;
        let total = idle + non_idle;

        *previous_cpu = cpu;
        let totald = total - *previous_total;
        let idled = idle - *previous_idle;
        Ok(format!(
            "{:.2}",
            100.0 * (totald - idled) as f64 / totald as f64
        ))
    }
}

// TODO: Other system-resource type measures like memory
