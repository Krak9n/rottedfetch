use std::process::Output;
use colored::Colorize;
use wmctrl::*;
use sysinfo::{
    Components, Disks, Networks, System, RefreshKind, CpuRefreshKind,
};
use terminal_size::{
  Width, Height, terminal_size
};

// stopped yesteday on the moment of figuring out how to display 
// edges of a box on the right side
// also dont forget to add names of modules in the box
fn main() {

  let sisi = FETCH::new();

  let left = String::from("\u{250F}");
  let right = String::from("\u{2513}");
  let hor = String::from("\u{2501}");
  let ver = String::from("\u{2503}");
  let left_bot = String::from("\u{2517}");
  let right_bot = String::from("\u{251B}");

  let size = terminal_size();
  let Some((Width(w), Height(h))) = size else { todo!() };

    // top
    println!("\t{}{}{}", left, hor.repeat(w.into()), right);

    // modules and os name
    FETCH::make_logo(ver.clone(), sisi.os.clone(), w.into());
 
    // bottom 
    println!("\t{}{}{}", left_bot, hor.repeat(w.into()), right_bot);
 
}

/*
// made specifically for memory
// have to find another solution 
pub fn colors_non_std(text: &str, field: &Output, color: &str) {
  if let texts = field {
    #[cfg(feature = "field-titles")]
    print!("{} ", text.bright_white());
    println!(
      "{}",
      format!("{}{}", text, texts).color(color)
    );
  }
}
*/

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
  //gpu: &'a str,
  cpu: String,
  memory: String,
  wm: Output,
}

impl FETCH {
  pub fn new() -> Self {
    // sys specs
    let mut sys = System::new_all();
    sys.refresh_all();

    Self {
      os: System::name(),
      kernel: System::kernel_long_version().clone(),
      memory: (sys.total_memory() / 1_000).to_string(), // later make it in function for printing
                                  // in mbits
      wm: wmctrl::show_wm_information(), // later make some loop
                                         // for checking if x11 or
                                         // wayland
      // later gpu: ,
      cpu: String::from(sys.cpus()[0].brand()).clone(),
    }
  }
 
  // to draw a box count height by 
  // number of chars in os name
  // and width by c (char) + cpu 
  fn just_modules() {
    let sisi = Self::new();
    //colors_non_std("memory: ", &sisi.wm, "magenta"); 
    //for mut modules in 0..3 {
      colors("cpu: ", &Some(sisi.cpu.clone()), "magenta"); 
      //colors("memory: ", &Some(sisi.memory.clone()), "magenta"); 
      //colors("kernel: ", &Some(sisi.kernel.clone()), "magenta");
      //modules +=1;
    //}
  }

  fn make_logo(ver: String, s: Option<String>, count: usize) {
    // colors("", &sys.os, "green");
    let cp: String = s.expect("REASON").to_string();

    for c in cp.chars() {
      print!("\t{}", ver);
      print!(" {} ", c);
      for k in 0..2 {
        let b: u8 = cp.as_bytes()[k];
        let d: char = b as char; 

        if c == d {
          break;
        }
        Self::just_modules();
      }
      print!("{: <1$}", "", count - 3);
      println!("{}", ver);
    }
  }

}
