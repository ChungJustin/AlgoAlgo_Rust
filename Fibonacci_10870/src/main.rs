use std::io;

fn jc_fibonacci(x: u32) -> u32
{
    match x {
       0 => 0,
       1 => 1,
       _ => jc_fibonacci(x-1) +jc_fibonacci(x-2),
    }
}
fn main() {
   let mut s = String::new();

   io::stdin()
      .read_line(&mut s)
      .unwrap();

   let value:u32 = s.trim_end().parse().unwrap(); 

   let fib_result = jc_fibonacci(value);

   println!("{}", fib_result);

}