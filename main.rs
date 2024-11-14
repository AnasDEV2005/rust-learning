

//////////////////
/// M  A  I  N ///
//////////////////



fn main() {
  let numbers = [3, 5, 2, 8, 1];
  println!("Max value: {:?}", find_max(&numbers).unwrap()); // Output: Some(8)
  
  let empty: [i32; 0] = [];
  println!("Max value: {:?}", find_max(&empty)); // Output: None
}


/* 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 1 - hello world

fn helloworkd() {
    println!("Hello, World!");
  }
 

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 2 - fizzbuzz

fn fizzbuzz() {
  for mut i in 0..100 {
    i+=1;
    if i % 3 == 0 {
      println!("Fizz");
    } else if i % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}",i);
    }
  }
}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 3 - fibo

// looks like it important to indicate this
//           input  ->  output (types)
fn fibonacci(n: u32) -> u32 {
  if n == 1 || n == 0 {
    return n;
  }
  return fibonacci(n-1)+fibonacci(n-2);
}

fn fibonaci(n: u32) -> u32 {
  if n == 1 || n==0 {
    return n;
  }
  let mut a = 0;
  let mut b = 1;
  for i in 2..=n {
    let temp = b;
    b = b+a;
    a = temp;
  }
  return b;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 4 - word counter

fn count_words(text: &str) -> usize {
  let mut n = 1;
  for i in text.chars() { // string.chars()
    if i == ' ' {
      n += 1
    }
  }
  return n;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 5 - guessing game

use std::io::stdin;
use rand::Rng;

fn guessgame() {

  let random_int: u32 = rand::thread_rng().gen_range(1..=100);
  let mut guess = 101;
  let mut read = String::new();


  while guess != random_int {

    let mut read = String::new();
    
    println!("Guess a number: ");
    stdin()
      .read_line(&mut read)
      .expect("Expected an int");


    guess = match read.trim().parse() {
      Ok(num) => {
        if num < 1 || num > 100 {
          println!("Please enter a number between 1 and 100.");
          continue;
      };
      num
      }
      Err(_) => {
        println!("Expected an int between 1 and 100");
        continue;
      }
    };

    if guess > random_int {
      println!("Wrong. Try smaller..")
    } else {
      println!("Wrong. Try bigger..")
    }
  };
  println!("You have won, the number was: {random_int}");
}
*/
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// 6 - find a max in an array

fn find_max(array: &[i32]) -> Option<&i32> {
  let mut array_max: &i32 = &0;
  for i in array.iter() {
    if i > array_max {
      array_max = i;
    }
  };
  if array_max == &0 {
    return None;
  };
  return Some(array_max);
}

