use std::process::Output;
use colored::Colorize;
// use wmctrl::*;
// was needed for wm.
// maybe will use in the future
// fuck it
use sysinfo::{
    Components, Disks, Networks, System, RefreshKind, CpuRefreshKind,
};
use std::process::Command;

// to me tmrw
// finish this shit as u planned the first time
// make in the damn box
fn main() {

  let sisi = FETCH::new();

  // this sucks and i know that it does
  // probably will change in the future
  let block = String::from("\u{2588}");
  let left = String::from("\u{250F}");
  let right = String::from("\u{2513}");
  let hor = String::from("\u{2501}");
  let left_bot = String::from("\u{2517}");
  let right_bot = String::from("\u{251B}");

  let height = sisi.os.iter().count();
  let width = sisi.cpu.clone().chars().count();

  // top
  println!("{}{}{}", left, hor.repeat(width+10), right);
  // modules and os name
  FETCH::make_logo(block);
  // bottom 
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
  //wm: String,
  user: String,
}

impl FETCH {
  pub fn new() -> Self {
    // sys specs
    let mut sys = System::new_all();
    sys.refresh_all();

    Self {
      os: System::name(),
      kernel: System::kernel_long_version().clone(),
      memory: 
        format!("{} gb / {} gb", 
          (sys.total_memory() / 1_000 / 1_000 / 1_000), (sys.used_memory() / 1_000 / 1_000 / 1_000))
        .to_string(), 
     // wm: {
       // let output = Command::new("sh")
        //  .args(["-c", ""
      //},

      //to find the gpu:
      // lspci | grep VGA | awk -F'VGA compatible controller: ' '{print $2}' 
      gpu: {
        let output = Command::new("sh")
          .arg("-c")
          .arg("lspci | grep VGA | awk -F'VGA compatible controller: ' '{print $2}'")
          .output()
          .expect("failed to execute process");
        
        let returning: String = String::from_utf8_lossy(&output.stdout).to_string();
        let mut gpu_vec = vec![];
        for s in returning.trim().split('\n') {
            gpu_vec.push(s.to_string());
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
  fn just_modules() {
    let sisi = Self::new();

    // name of the user
    colors("  user: ", &Some(sisi.user.clone()), "purple");     
    colors("  os: ", &Some(sisi.os.expect("reason").clone()), "green");
    colors("  cpu: ", &Some(sisi.cpu.clone()), "red"); 

    if let Some(gpuvec) = sisi.gpu {
        for gpu in gpuvec {
          colors("  gpu: ", &Some(gpu), "yellow");
        }
    }
    colors("  memory: ", &Some(sisi.memory.clone()), "white"); 
    colors("  kernel: ", &Some(sisi.kernel.clone()), "blue");
    //colors_non_std("  wm: ", &Some(sisi.wm.clone()), "magenta");
    //
  }

  fn make_logo(block: String) {

    Self::just_modules();
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
