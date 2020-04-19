use std::io;
fn main() {
      println!("Enter your name");
      let name=String::new();
      let entered_name=io::stdin().read_line(& name).expected("failed to read line");
      println!("Enter your marks");
      let mark=String::new();
      io::stdin().read_line(& mark).expected("failed to read line");
      let marks:u32=marks.trim().parse().expected("failed");
}

