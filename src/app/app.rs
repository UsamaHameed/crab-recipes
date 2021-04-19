use std::{collections::HashMap, process, io};
use crate::recipes::{Recipe, Recipes, RecipeFns};

pub struct App {
	pub recipes: Recipes,
}

impl App {
	pub fn new() -> Self {
		App {
			recipes: HashMap::new()
		}
	}

	pub fn listen(&mut self) {
		let mut exit_flag = String::new();

		while exit_flag.trim() != "x" {
			let mut option = String::new();
			let prompt = Some(
"Options: \n
List all recipes: l\n
Add a new recipe: a\n
Edit a recipe: e\n
Remove a recipe: r"
			);
						
			self.take_input(&mut option, prompt, None);
		
			match option.trim() {
				"l" => { self.recipes.print_all(); },
				"a" => { self.prompt_add_recipe(); },
				"e" => { self.prompt_edit_recipe() },
				"r" => { self.prompt_remove_recipe(); },
				other => { 
					println!("invalid option: {}", other);
					continue; 
				}
			}
	
			self.prompt_exit(&mut exit_flag);
		}
	}

	fn prompt_add_recipe(&mut self) {
		let mut name = String::new();
		let mut description = String::new();

		self.take_input(
			&mut name, 
			Some("Please enter the name: "), 
			None
		);

		self.take_input(
			&mut description, 
			Some("Please enter the description: "), 
			None
		);

		self.recipes.add_recipe(Recipe { 
			name, description
		});
	}

	fn prompt_edit_recipe(&mut self) {
		let mut id = String::new();
		let mut name = String::new();
		let mut description = String::new();

		self.take_input(
			&mut id, 
			Some("Please enter the id to of the recipe to edit: "), 
			None
		);

		self.take_input(
			&mut name, 
			Some("Please enter name: "), 
			None
		);

		self.take_input(
			&mut description, 
			Some("Please enter the description: "), 
			None
		);

		self.recipes.remove_recipe(id.as_str());
		self.recipes.add_recipe(Recipe { 
			name, description
		});
	}

	fn prompt_remove_recipe(&mut self) {
		let mut id = String::new();

		self.take_input(
			&mut id, 
			Some("Please enter the id to remove: "), 
			None
		);

		self.recipes.remove_recipe(id.as_str());
	}

	fn prompt_exit(&mut self, buffer: &mut String) {
		let prompt = Some(
			"Press any button to see options again!\n
To exit: Press x\n"
		);
		
		self.take_input(buffer, prompt, None);

		match buffer.trim() {
			"x" => { 
				self.exit(); 
			},
			_ => { 
				self.listen(); 
			}
		}
	}

	fn take_input(
		&self,
		buffer: &mut String, 
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

	fn exit(&self) {
		process::exit(1);
	}
}
