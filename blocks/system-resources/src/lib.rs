use std::error::Error;

use procstat::ProcStat;

pub struct CpuBlock;

static mut PREVIOUS_TOTAL: u64 = 0;
static mut PREVIOUS_IDLE: u64 = 0;
static mut PREVIOUS_NON_IDLE: u64 = 0;

static mut PREVIOUS_CPU: procstat::CPU = procstat::CPU {
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
};

impl block::Block for CpuBlock {
    fn perform(&self) -> Result<String, Box<dyn Error>> {
        let proc_stat = ProcStat::read();
        let cpu = proc_stat.cpu;

        // This algorithm is from: https://stackoverflow.com/a/23376195/14470574
        // I would prefer not having to use unsafe though...
        // Maybe some mechanism of feeding previous results back into blocks?
        unsafe {
            PREVIOUS_IDLE = PREVIOUS_CPU.idle + PREVIOUS_CPU.iowait;
            PREVIOUS_NON_IDLE = PREVIOUS_CPU.user + PREVIOUS_CPU.nice + PREVIOUS_CPU.system + PREVIOUS_CPU.irq + PREVIOUS_CPU.softirq + PREVIOUS_CPU.steal;
            PREVIOUS_TOTAL = PREVIOUS_IDLE + PREVIOUS_NON_IDLE;
        }

        let idle = cpu.idle + cpu.iowait;
        let non_idle = cpu.user + cpu.nice + cpu.system + cpu.irq + cpu.softirq + cpu.steal;
        let total = idle + non_idle;

        unsafe {
            PREVIOUS_CPU = cpu;
            let totald = total - PREVIOUS_TOTAL;
            let idled = idle - PREVIOUS_IDLE;
            Ok(format!("{:.2}", 100.0 * (totald - idled) as f64 / totald as f64))
        }
    }
}

// TODO: Other system-resource type measures like memory,
