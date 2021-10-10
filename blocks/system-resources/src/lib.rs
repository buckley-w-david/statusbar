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

impl block::Block for CpuBlock {
    fn perform(&self) -> Result<String, Box<dyn Error>> {
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

// TODO: Other system-resource type measures like memory,
