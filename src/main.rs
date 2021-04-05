use std::{collections::HashMap, fmt, io, usize};

type Id = usize;
type  Recipes = HashMap<Id, Recipe>;

struct Recipe {
    name: String,
    description: String,
    // ingredients
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} Description: {}", self.name, self.description)
    }
}

trait RecipeFns {
    fn add_recipe_info(&mut self);
    fn remove_recipe(&mut self);
    fn print_all(&mut self);
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

    fn print_all(&mut self) {
        println!("\n");
        for (id, recipe) in self {
            print!("id: {} ", id);
            println!("{}\n", recipe);
        }
    }
}

fn main() {
    // store recipes
    let mut recipes: Recipes = HashMap::new();

    recipes.insert(1, Recipe { 
        name: String::from("crabs"), 
        description: String::from("haha") 
    });


    let mut option2 = String::new();

    while option2.trim() != "x" {
        let mut option = String::new();

        let prompt = Some(
"Options: \n
List all recipes: l\n
Add a new recipe: a\n
Edit a recipe: e\n
Remove a recipe: r"
        );
        
        take_input(&mut option, prompt, None);

        match option.trim() {
            "l" => { recipes.print_all(); },
            "a" => { recipes.add_recipe_info(); },
            "e" => {()},
            "r" => { recipes.remove_recipe(); },
            other => { 
                println!("invalid option: {}", other);
                continue; 
            }
        }

        prompt_exit(&mut option2);
    }
}

fn print_recipes(recipes: &Recipes) {
    println!("\n");
    for (id, recipe) in recipes {
        print!("id: {} ", id);
        println!("{}\n", recipe);
    }
}

fn take_input(
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

fn prompt_exit(buffer: &mut String) {
    let prompt = Some(
"Press any button to see options again!\n
To exit: Press x\n"
    );

    take_input(buffer, prompt, None);
}
