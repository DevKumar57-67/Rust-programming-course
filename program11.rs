
// A basic rust program for transferring ownership from x to y


 
fn main () {
  let x = 66;
  let y = x;
  println!("y = {}", x);
}

// Here the value 66 is owned by x but then transferred to y variable 