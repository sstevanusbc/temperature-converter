use std::io;

fn main() {
	println!("Convert Temperature from Fahrenheit to Celcius and vice versa");
	println!("1: Fahrenheit to Celcius");
	println!("2: Celcius to Fahrenheit");
	println!("q (or other key): exit the program");
	

	loop {
		println!("Press the key [1/2/q]: ");

		let mut option = String::new();

		io::stdin()
			.read_line(&mut option)
			.expect("Failed to read input");

		let option: u32 = match option.trim().parse() {
			Ok(num) => num,
			Err(_) => break,
		};

		// println!("Your option: {option}");

		let mut fahrenheit = String::new();
		let mut celcius = String::new();

		if option == 1 {
			println!("Fahrenheit: ");
			io::stdin()
				.read_line(&mut fahrenheit)
				.expect("Failed to read input");
			let fahrenheit: i32 = match fahrenheit.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Please type a number!");
					continue
				},
			};
			celcius = ((fahrenheit - 32) * 5 / 9).to_string();
			println!("{fahrenheit}F is {celcius}C"); 
		}

		if option == 2 {
			println!("Celcius: ");
			io::stdin()
				.read_line(&mut celcius)
				.expect("Failed to read input");
			let celcius: i32 = celcius.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("Please type a number!");
					continue
				},
			};
			fahrenheit = ((celcius * 9 / 5) + 32).to_string();
			println!("{celcius}C is {fahrenheit}F");
		}
	}
}

