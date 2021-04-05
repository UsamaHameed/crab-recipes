use std::{collections::HashMap, fmt, io, usize};

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
    fn add_recipe_info(&mut self);
    fn remove_recipe(&mut self);
    fn print_all(&mut self);
    fn edit_recipe(&mut self);
}

impl RecipeFns for Recipes {
    fn add_recipe_info(&mut self)  {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let id = self.len() + 1;
        
        take_input(&mut input1, Some("Enter the name: "), None);
        take_input(&mut input2, Some("Enter the description: "), None);
    
        self.insert(id, Recipe {
            name: input1.trim().to_string(),
            description: input2.trim().to_string(),
        });
    }
    fn remove_recipe(&mut self) {
        let mut id = String::new();
    
        loop {
            take_input(&mut id, Some("Enter id to remove: "), None);
            
            match id.trim().parse::<u32>() {
                Ok(x) => { 
                    self.remove(&(x as usize));
                    break;
                },
                Err(_) => { println!("Please enter a valid id..."); }
            }
        }
    }

    fn edit_recipe(&mut self) {
        let mut id = String::new();
        take_input(&mut id, Some("Enter id to edit: "), None);

        match id.trim().parse::<usize>() {
            Ok(x) => {
                if self.contains_key(&x) {
                    self.remove(&x);
                    self.add_recipe_info();
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

pub fn take_input(
    buffer: &mut String, prompt_message: Option<&str>, error_message: Option<&str>
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

pub fn prompt_exit(buffer: &mut String) {
    let prompt = Some(
"Press any button to see options again!\n
To exit: Press x\n"
    );

    take_input(buffer, prompt, None);
}