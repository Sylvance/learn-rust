fn main() {
  println!("Hello, world! Let's run Rust.");

  // conditions #1

  let x = 4;

  if x < 5  {
    println!("X is less than 5");
  }

  if x % 4 == 0 {
    println!("number is divisible by 4");
  } else if x % 3 == 0 {
    println!("number is divisible by 3");
  } else if x % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  // conditions #2
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {}", number);

  // loops #1
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);

  // loops #2
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);

    number -= 1;
  }

  println!("LIFTOFF!!!");

  // loops #3
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("the value is: {}", a[index]);

    index += 1;
  }

  // loops #4
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is: {}", element);
  }

  // loops #5
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
