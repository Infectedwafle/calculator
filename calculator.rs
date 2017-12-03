use std::io;
use std::io::Write;
use std::io::stdout;

fn main() {
	let mut input_text = String::new();
	let mut operator = String::new();
	let mut number_1 = 0.0;
	let mut number_2 = 0.0;
	let mut total = 0.0;

	loop {	    
		println!("Enter an operator(+-*/^): ");
		stdout().flush();
		io::stdin().read_line(&mut operator).expect("failed to read from stdin");

		println!("Enter the first number: ");
		stdout().flush();
		io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

		match input_text.trim().parse::<f32>() {
	        Ok(n) => number_1 = n,
	        Err(..) => println!("this was not a number: {}", input_text.trim())
	    };
	    input_text = String::new();

		println!("Enter the second number: ");
		stdout().flush();
		io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

		match input_text.trim().parse::<f32>() {
	        Ok(n) => number_2 = n,
	        Err(..) => println!("this was not a number: {}", input_text.trim())
	    };
		input_text = String::new();

		match operator.trim().as_ref() {
		    "+" => total += number_1 + number_2,
		    "-" => total += number_1 - number_2,
		    "*" => total += number_1 * number_2,
		    "/" => total += number_1 / number_2,
		    "^" => total += number_1.powf(number_2),
		    _ => println!("Not a valid operation, {}", operator),
		};

		println!("Total: {}", total);
		operator = String::new();
		number_1 = 0.0;
		number_2 = 0.0;
		total = 0.0;
	}
}
	