use std::process::{Output, Command};
use colored::Colorize;
use sysinfo::{
    Components, Disks, Networks, System, RefreshKind, CpuRefreshKind,
};

fn main() {
  let sisi = FETCH::new();
  let block = String::from("\u{2588}");
  let left = String::from("\u{250F}");
  let right = String::from("\u{2513}");
  let hor = String::from("\u{2501}");
  let left_bot = String::from("\u{2517}");
  let right_bot = String::from("\u{251B}");

  let height = sisi.os.iter().count();
  let width = sisi.cpu.clone().chars().count();

  // The Top part
  println!("{}{}{}", left, hor.repeat(width+10), right);
  // modules and os name
  FETCH::prints(block);
  // The Bottom part 
  println!("{}{}{}", left_bot, hor.repeat(width+10), right_bot);

}

pub fn colors(text: &str, field: &Option<String>, color: &str) {
  if let Some(texts) = field {
    #[cfg(feature = "field-titles")]
    print!("{} ", text.bright_white());
    println!(
      "{}",
      format!("{}{}", text, texts).color(color)
    );

  }
}

struct FETCH {
  os: Option<String>,
  kernel: String,
  gpu: Option<Vec<String>>,
  cpu: String,
  memory: String,
  user: String,
}

impl FETCH {
  pub fn new() -> Self {
    let mut sys = System::new_all();
    sys.refresh_all();

    Self {
      os: System::name(),
      kernel: System::kernel_long_version().clone(),
      memory: 
        format!("{} gb / {} gb", 
                (sys.total_memory() / 1_000 / 1_000 / 1_000),
                (sys.used_memory() / 1_000 / 1_000 / 1_000))
            .to_string(), 
      gpu: {
        let sys_output = Command::new("sh")
          .arg("-c")
          .arg("lspci | grep VGA | awk -F'VGA compatible controller: ' '{print $2}'")
          .output()
          .expect("failed to execute process");

        let s_sys_output: String = String::from_utf8_lossy(&sys_output.stdout).to_string();
        let mut gpu_vec = vec![];
        for s_trimmed in s_sys_output.trim().split('\n') {
            gpu_vec.push(s_trimmed.to_string());
        }
        
        Some(gpu_vec)
      },

      user: whoami::username(), 
      cpu: String::from(sys.cpus()[0].brand()).clone(),
    }
  }
 
  // to draw a box count height by 
  // number of chars in os name
  // and width by c (char) + cpu 
  fn modules() {
    let sisi = Self::new();
    // name of the user
    colors("User: ", &Some(sisi.user.clone()), "purple");     
    colors("OS: ", &Some(sisi.os.expect("reason").clone()), "green");
    colors("CPU: ", &Some(sisi.cpu.clone()), "red"); 
    if let Some(gpuvec) = sisi.gpu {
        for gpu in gpuvec {
          colors("gpu: ", &Some(gpu), "yellow");
        }
    }
    colors("MEMORY: ", &Some(sisi.memory.clone()), "white"); 
    colors("KERNEL: ", &Some(sisi.kernel.clone()), "blue");
  }

  fn prints(block: String) {
    Self::modules();
    println!();
    
    println!("  {}{}{}{}{}{}{}{}{}{}{}{}", 
      block.blue(), block.blue(), 
      block.black(), block.black(), 
      block.truecolor(0, 247, 255), block.truecolor(0, 247, 255),
      block.truecolor(255, 0, 50), block.truecolor(255, 0, 50),
      block.truecolor(168, 0, 33), block.truecolor(168, 0, 33),
      block.purple(), block.purple()
      ); 
  }

}
