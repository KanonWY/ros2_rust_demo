use std::os::unix::process;

use r2r::jszr_robots_dog_msg::msg::Monitor;
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System,
};

/// get special pids info
pub fn get_current_system_info() -> Monitor {
    let msg = r2r::jszr_robots_dog_msg::msg::Monitor::default();
    return msg;
}

/// get special pids Monitor info
pub fn get_monitor_info(pids: Vec<usize>) -> Option<Monitor> {
    let mut sys = System::new_all();
    sys.refresh_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything()),
    );
    let mut msg = r2r::jszr_robots_dog_msg::msg::Monitor::default();

    for pid in &pids {
        if let Some(process) = sys.process(sysinfo::Pid::from(*pid)) {
            msg.process_infos.push(assign_single_process_info(process));
        } else {
            println!("{} process node exist!", *pid);
        }
    }
    if msg.process_infos.len() > 0 {
        return Some(msg);
    }
    None
}

fn assign_single_process_info(
    process: &sysinfo::Process,
) -> r2r::jszr_robots_dog_msg::msg::ProcessInfo {
    let mut msg = r2r::jszr_robots_dog_msg::msg::ProcessInfo::default();
    msg.process_name = r2r::std_msgs::msg::String {
        data: std::format!("{}", process.name().to_string_lossy()),
    };
    msg.memory_usage = (process.memory() as f64) / 1024.0 / 1024.0;
    msg.cpu_usage = process.cpu_usage() as f64;
    msg
}

fn get_pid_by_name(name: &str) -> Option<sysinfo::Pid> {
    let sys = System::new_all();
    for process in sys.processes_by_name("component_container_isolated".as_ref()) {
        return Some(process.pid());
    }
    None
}

#[test]
fn test_get_pids_info() {
    let msg = get_monitor_info(vec![234]);
    println!("{:?}", msg);
}
