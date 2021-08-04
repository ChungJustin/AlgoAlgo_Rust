use std::io;

fn main() {
   let mut s = String::new();

   io::stdin()
      .read_line(&mut s)
      .unwrap();

   let mut_str = s.as_mut_str();
   let iter = mut_str.split_whitespace();

   let values:Vec<i32> = iter 
      .map(|s| s.parse().unwrap())
      .collect();

   println!("{}", values[0] + values[1]);
}
