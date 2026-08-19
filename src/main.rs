use sysinfo::{
    Components, Disks, Networks, System, RefreshKind, CpuRefreshKind,
};
use whoami;
use gfxinfo::active_gpu;

struct App {
    username: String,
    os: String,
    cpu: String,
    gpu: String,
    memory: String,
    de: String,
}

impl App {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::everything().with_cpu(CpuRefreshKind::everything()),
        );
        
        Self {
            username: {
                format!("{}@{}", whoami::username().unwrap().to_string(), whoami::hostname().unwrap().to_string())
            },
            os: {
                let mut os: String = String::new(); 
                if System::name() == Some("Darwin".to_string()) {
                    os = r###"
  ________________________.___.__      __  _____ _____________________
 /   _____/\______   \__  |   /  \    /  \/  _  \\______   \_   _____/
 \_____  \  |     ___//   |   \   \/\/   /  /_\  \|       _/|    __)_ 
 /        \ |    |    \____   |\        /    |    \    |   \|        \
/_______  / |____|    / ______| \__/\  /\____|__  /____|_  /_______  /
        \/            \/             \/         \/       \/        \/"###.to_string();
                }
                else {
                    os = System::name().expect("Failed to retreive OS name").to_string();
                }
                os
            },
            cpu: {
                // Clean up later. Check if CPU result are equal and if they are then return just one.
                let mut cpus: Vec<String> = vec![];
                for cpu in sys.cpus() {
                    cpus.push(cpu.brand().to_string());
                }
                cpus
                    .iter()
                    .map(|x| x.trim().split_whitespace())
                    .collect::<Vec<_>>();
                <String as Clone>::clone(&cpus[0])
            },
            gpu: {
                let gpu = active_gpu();
                gpu.expect("Failed to obtain GPU's model").model().to_string()
            },
            memory: format!("{} Mib / {} Mib",
                            (sys.total_memory() / 1_000 / 1_000),
                            (sys.used_memory() / 1_000 / 1_000))
                .to_string(),
            de: {
                let mut e: String = String::new();
                match whoami::desktop_env() {
                    Some(environment) => {
                        e = environment.to_string();
                    }
                    None => println!("No DE was found"),
                }
                e
            }
        }
    }

    /*fn calculate_box(self) -> u32 {
        // Based on the longest length do width
        let width: u32 = {
            
        };
        let height: u32;
    }*/

    pub fn print(self) {
        println!("{}\n--------\nOS: {}\nCPU: {}\nGPU: {}\nRAM: {}\nDE: {}",
        self.username, self.os, self.cpu, self.gpu, self.memory, self.de);
    }
}

fn main() {
    let fetch = App::new();
    fetch.print();
}
