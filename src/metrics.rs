use sysinfo::{System, Disks};

/// A structure representing system metrics, including CPU, memory, and disk usage.
///
/// # Fields
/// - `cpu_usage_percent`:
///   The overall CPU usage as a percentage.
/// - `cpu_usage_per_core`:
///   A vector containing the CPU usage percentage for each individual core.
/// - `memory_usage_percent`:
///   The memory usage as a percentage of total available memory.
/// - `disk_usage_percent`:
///   The disk usage as a percentage of total available disk space.
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub cpu_usage_per_core: Vec<f64>,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
}

/// Retrieves the CPU usage percentage for each core.
/// This function refreshes the CPU information in the provided `System`
/// instance and returns a vector containing the CPU usage for each core.
///
/// # Arguments
/// - `sys`: A mutable reference to a `System` instance that holds the CPU information.
pub fn get_cpu_usage_per_core(sys: &mut System) -> Vec<f64> {
    sys.refresh_cpu();

    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage() as f64)
        .collect()
}

/// Calculates the percentage of memory usage.
///
/// This value is derived by dividing the used memory by the total memory
/// and multiplying the result by 100.0 to convert it into a percentage.
///
/// # Variables
/// - `used_memory`: The amount of memory currently in use.
/// - `total_memory`: The total available memory.
///
/// # Returns
/// A floating-point value representing the percentage of memory usage.
///
/// # Example
/// ```rust
/// let used_memory = 4.0; // in GB
/// let total_memory = 16.0; // in GB
/// let memory_usage_percent = (used_memory / total_memory) * 100.0;
/// assert_eq!(memory_usage_percent, 25.0);
/// ```    
pub fn collect_metrics(sys: &mut System) -> SystemMetrics {
    sys.refresh_cpu(); // Refresh CPU usage
    sys.refresh_memory(); // Refresh memory usage

    let cpu_usage_by_core = get_cpu_usage_per_core(sys);

    let cpu_usage = sys.global_cpu_info().cpu_usage() as f64;
    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let memory_usage_percent = (used_memory / total_memory) * 100.0;

    // For simplicity, let's just use the first disk found
    let mut disk_usage_percent = 0.0;
    
    //Disks::new_with_refreshed_list().first()
    if let Some(disk) = Disks::new_with_refreshed_list().first() {
        let total_space = disk.total_space() as f64;
        let available_space = disk.available_space() as f64;
        disk_usage_percent = ((total_space - available_space) / total_space) * 100.0;
    }

    SystemMetrics {
        cpu_usage_percent: cpu_usage,
        cpu_usage_per_core: cpu_usage_by_core,
        memory_usage_percent: memory_usage_percent,
        disk_usage_percent: disk_usage_percent,
    }
}