use sysinfo::{Pid, System};
fn main()  {
     let mut sys = System::new();
// First we update all information of our `System` struct.
     sys.refresh_all();
     println!("=> system:");
// RAM and swap information:
     println!("total memory: {} bytes", sys.total_memory());
     println!("used memory : {} bytes", sys.used_memory());
     println!("total swap  : {} bytes", sys.total_swap());
     println!("used swap   : {} bytes", sys.used_swap());
// Display system information:
     println!("System name:             {:?}", System::name());
     println!("System kernel version:   {:?}", System::kernel_version());
     println!("System OS version:       {:?}", System::os_version());
     println!("System host name:        {:?}", System::host_name());
// Number of CPUs:
     println!("Number CPUs: {}", sys.cpus().len());
// Display processes ID, name na disk usage:
     for (pid, process) in sys.processes() {
         println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
     }
}
