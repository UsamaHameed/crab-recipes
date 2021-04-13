use std::{collections::HashMap, process, io};
use super::super::recipes::Recipes;

pub struct App {
	pub recipes: Recipes
}

pub trait AppFns {
	fn new() -> App;
	fn take_input(
		buffer: &mut String, 
		prompt_message: Option<&str>, 
		error_message: Option<&str>
	);
	fn prompt_exit(buffer: &mut String);
	fn exit();
}

impl AppFns for App {
	fn new() -> App {
		App {
			recipes: HashMap::new()
		}
	}

	fn take_input(buffer: &mut String, 
		prompt_message: Option<&str>, 
		error_message: Option<&str>
	) {
		while buffer.trim().is_empty() {
			let prompt_message = match prompt_message {
				Some(x) => x,
				None => "Please enter some text",
			};
			
			let error_message = match error_message {
				Some(x) => x,
				None => "Failed to parse text...\nPlease try again!",
			};
			
			println!("{}", prompt_message);
		
			io::stdin()
				.read_line(buffer)
				.expect(error_message);
		}
	}

	fn prompt_exit(buffer: &mut String) {
		let prompt = Some(
			"Press any button to see options again!\n
			To exit: Press x\n"
		);
		
		App::take_input(buffer, prompt, None);
	}

	fn exit() {
		process::exit(1);
	}
}