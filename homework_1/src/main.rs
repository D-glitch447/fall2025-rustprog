fn assignment1() {

    //Temp converter
    const FREEZE_POINT : f64 = 32.0;
    let mut num: f64 = 80.0;

    println! ("\n---Converting F to C ---");
    for _x in 0..6 {
        let c = fahrenheit_to_celcius(num);
        println!("{}°F = {}°C", num, c);
        num += 1.0;
    }

    fn fahrenheit_to_celcius(f: f64) -> f64 {
       let result = (f - 32.0) * 5.0 / 9.0;
       result
    }

    fn celsius_to_fahrenheit(c: f64) -> f64 {
        let result = (c *9.0/5.0) + 32.0;
         result
    }

} 

fn assignment2() {

    let array : [i32; 10] = [10, 43, 22, 90, 3, 51, 33, 14, 55, 80];

    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    println! ("\n---Analyzing numbers in the array ---");

    for num in array.iter() {
        match (num % 3, num % 5) {
            (0,0) => println!("{} -> FizzBuzz", num),
            (0,_) => println!("{} -> Fizz", num),
            (_,0) => println!("{} -> Buzz", num),
            _ => {
                if is_even(*num) {
                    println!("{} -> Even", num);
                } else {
                    println!("{} -> Odd", num);
                }
            }

        }
    }

    println!("\n---Calculating the sum ---");
    let mut sum = 0;
    let mut i = 0;
    while i < array.len() {
        sum += array[i];
        i += 1;
    }
    println!("The sum of all numbers is {}", sum);

    println!("\n--- Finding the largest number ---");
    let mut largest_num = array[0];
    for &num in array.iter().skip(1) {
        if num > largest_num {
            largest_num = num;
        }
    }
    println!("The largest number is: {}", largest_num);
}

fn assignment3() {
    let secret_number: i32 = 42;
    let mut guess_count = 0;
    let mut current_guess = 50;

    println!("\n---Guessing Game Begin---");

    loop {

        guess_count += 1;

        let result = check_guess(current_guess, secret_number);

        if result == 0 {
            println!("Guess: {} -> Correct!", current_guess);
            println!("It took you {} guesses to win", guess_count);
            println!("---Guessing Game Ends---");
            break;
        } else if result == 1 {
            println!("Guess: {} -> Too high!", current_guess);
            current_guess -= 1; 
        } else {
            println!("Guess: {} -> Too low!", current_guess);
            current_guess += 1;
        }
    }
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    assignment1();
    assignment2();
    assignment3();
}

