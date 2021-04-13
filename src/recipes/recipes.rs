use std::{collections::HashMap, fmt, usize};

type Id = usize;
pub type  Recipes = HashMap<Id, Recipe>;
pub struct Recipe {
    pub name: String,
    pub description: String,
    // ingredients
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Description: {}", self.name, self.description)
    }
}

pub trait RecipeFns {
    fn add_recipe(&mut self, new_recipe: Recipe);
    fn remove_recipe(&mut self, id: &str);
    fn edit_recipe(&mut self, id: &str, new_recipe: Recipe);
    fn print_all(&mut self);
}

impl RecipeFns for Recipes {
    fn add_recipe(&mut self, new_recipe: Recipe)  {
        let id = self.len() + 1;
        self.insert(id, new_recipe);
    }

    fn remove_recipe(&mut self, id: &str) {
		match id.trim().parse::<u32>() {
			Ok(x) => { 
				self.remove(&(x as usize));
			},
			Err(_) => { println!("Please enter a valid id..."); }
		}
    }

    fn edit_recipe(&mut self, id: &str, new_recipe: Recipe) {
        match id.trim().parse::<usize>() {
            Ok(x) => {
                if self.contains_key(&x) {
                    self.remove(&x);
                    self.add_recipe(new_recipe);
                } else {
                    println!("Recipe with id: {} not found", id);
                }
            },
            Err(_) => { println!("Error parsing the text entered"); },
        }
    }

    fn print_all(&mut self) {
        println!("\n");
        for (id, recipe) in self {
            print!("id: {} ", id);
            println!("{}\n", recipe);
        }
    }
}