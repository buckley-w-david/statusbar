use std::sync::atomic::{AtomicU64, Ordering};

use std::error::Error;

use procstat::ProcStat;

pub struct CpuBlock;

// TODO replace with atomic primitives
// https://doc.rust-lang.org/std/sync/atomic/index.html
static PREVIOUS_TOTAL: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_IDLE: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_NON_IDLE: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_USER: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_NICE: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_SYSTEM: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_IRQ: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_SOFTIRQ: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_STEAL: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_IOWAIT: AtomicU64 = AtomicU64::new(0);
static PREVIOUS_CPU_IDLE: AtomicU64 = AtomicU64::new(0);

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
        // Would be be nice if I had a better method than the store/load AtomicU64 stuff.
        // I like it better than the mutex version I guess, it's just very verbose.
        let previous_cpu_idle = PREVIOUS_CPU_IDLE.load(Ordering::Relaxed);
        let previous_cpu_iowait = PREVIOUS_CPU_IOWAIT.load(Ordering::Relaxed);

        let previous_idle = previous_cpu_idle + previous_cpu_iowait;
        PREVIOUS_IDLE.store(previous_idle, Ordering::Relaxed);

        let previous_cpu_user = PREVIOUS_CPU_USER.load(Ordering::Relaxed);
        let previous_cpu_nice = PREVIOUS_CPU_NICE.load(Ordering::Relaxed);
        let previous_cpu_system = PREVIOUS_CPU_SYSTEM.load(Ordering::Relaxed);
        let previous_cpu_irq = PREVIOUS_CPU_IRQ.load(Ordering::Relaxed);
        let previous_cpu_softirq = PREVIOUS_CPU_SOFTIRQ.load(Ordering::Relaxed);
        let previous_cpu_steal = PREVIOUS_CPU_STEAL.load(Ordering::Relaxed);

        let previous_non_idle = previous_cpu_user
            + previous_cpu_nice
            + previous_cpu_system
            + previous_cpu_irq
            + previous_cpu_softirq
            + previous_cpu_steal;
        PREVIOUS_NON_IDLE.store(previous_non_idle, Ordering::Relaxed);

        let previous_total = previous_idle + previous_non_idle;
        PREVIOUS_TOTAL.store(previous_total, Ordering::Relaxed);

        let idle = cpu.idle + cpu.iowait;
        let non_idle = cpu.user + cpu.nice + cpu.system + cpu.irq + cpu.softirq + cpu.steal;
        let total = idle + non_idle;

        PREVIOUS_CPU_IDLE.store(cpu.idle, Ordering::Relaxed);
        PREVIOUS_CPU_IOWAIT.store(cpu.iowait, Ordering::Relaxed);
        PREVIOUS_CPU_USER.store(cpu.user, Ordering::Relaxed);
        PREVIOUS_CPU_NICE.store(cpu.nice, Ordering::Relaxed);
        PREVIOUS_CPU_SYSTEM.store(cpu.system, Ordering::Relaxed);
        PREVIOUS_CPU_IRQ.store(cpu.irq, Ordering::Relaxed);
        PREVIOUS_CPU_SOFTIRQ.store(cpu.softirq, Ordering::Relaxed);
        PREVIOUS_CPU_STEAL.store(cpu.steal, Ordering::Relaxed);

        let totald = total - previous_total;
        let idled = idle - previous_idle;
        Ok(format!(
            "{:.2}",
            100.0 * (totald - idled) as f64 / totald as f64
        ))
    }
}

// TODO: Other system-resource type measures like memory
