
fn temperature_converter() {
    const  FREEZE_POINT: f64 = 32.0;

    fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZE_POINT) / 1.8
    }

    fn celsius_to_fahrenheit(c: f64) -> f64 {
        1.8 * c + FREEZE_POINT
    }

    let mut temperature: f64 = 80.0;   // Temperature in Fahrenheit

    println!("Temperature is: {}F°", temperature);

    temperature = fahrenheit_to_celsius(temperature);

    println!("Converting to Celcius is: {:.0}C°", temperature);

    temperature = celsius_to_fahrenheit(temperature);

    println!("Here are the next 5 temperatures after 80F° in Celcius:");

    loop{

        temperature += 1.0;
        temperature = fahrenheit_to_celsius(temperature);
        println!("{:.0}C°", temperature);
        temperature = celsius_to_fahrenheit(temperature);

        if temperature >= 85.0 {
            break;
        }
    }
}

fn number_analyzer(){
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    let numbers: [i32; 10] = [10, 86, 65, 66, 91, 59, 79, 19, 99, 65];

    for num in numbers.iter(){
        
        let divisible_by_two = if is_even(*num) {
            "even"
        } else {
            "odd"
        };

        println!("{}", divisible_by_two);

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Buzz");
        } else if num % 5 == 0 {
            println!("Fizz");
        }
    }

    let mut sum = 0;
    let mut i = 0;  // Index variable for array

    while i < numbers.len(){
        sum += numbers[i];
        i += 1;
    }
    println!("{}", sum);

    i = 0;
    let mut max_number = numbers[i];
    loop {
        
        if numbers[i] > max_number{
            max_number = numbers[i];
        }

        i += 1;

        if i == 10{
            break;
        }
    }
    
    println!("{}", max_number);
}

fn guessing_game() {
    let mut secret_number: i32 = 77;

    fn check_guess(guess: i32, secret: i32) -> i32 {
        if guess == secret {
            0
        }
        else if guess > secret {
            1
        }
        else {
            -1
        }
    }

    let mut attempts: i32 = 0;

    loop{
        attempts += 1;
        let mut guess: i32 = 77;

        println!("Your guess is {}", guess);

        if check_guess(guess, secret_number) == 0 {
            println!("That is the correct number!");
            break;
        }
        else if check_guess(guess, secret_number) > 0 {
            println!("Too high!");
        }
        else {
            println!("Too low!");
        }
    }

    println!("Number of attempts: {}", attempts);
}

fn main() {
    
    temperature_converter();

    number_analyzer();

    guessing_game();
}
