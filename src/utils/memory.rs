// use cfg_if::cfg_if;

// cfg_if! {
//     if #[cfg(target_os = "linux")] {
//         use sysinfo::{System, SystemExt};

//         pub fn get_memory_usage() -> (u64, u64) {
//             let system = System::new_all();
//             system.refresh_all();
//             let total_memory = system.get_total_memory();
//             let used_memory = system.get_used_memory();

//             (total_memory, used_memory)
//         }
//     } else if #[cfg(target_os = "windows")] {
//         use winapi::um::psapi::{GetPerformanceInfo, PERFORMANCE_INFORMATION};

//         pub fn get_memory_usage() -> (u64, u64) {
//             let mut perf_info: PERFORMANCE_INFORMATION = unsafe { std::mem::zeroed() };
//             unsafe {
//                 GetPerformanceInfo(&mut perf_info, std::mem::size_of::<PERFORMANCE_INFORMATION>() as u32);
//             }
//             let total_memory = perf_info.PhysicalTotal << 10;
//             let used_memory = (perf_info.PhysicalTotal - perf_info.PhysicalAvailable) << 10;

//             (total_memory, used_memory)
//         }
//     } else {
//         pub fn get_memory_usage() -> (u64, u64) {
//             // Unsupported platform
//             (0, 0)
//         }
//     }
// }
