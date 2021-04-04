use std::{collections::HashMap, fmt, io, usize};

type Recipes = HashMap<usize, Recipe>;

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

fn main() {
    // store recipes
    let mut recipes: Recipes = HashMap::new();

    recipes.insert(1, Recipe { 
        name: String::from("crabs"), 
        description: String::from("haha") 
    });


    loop {
        let mut option = String::new();

        println!("Options: ");
        println!("List all recipes: l");
        println!("Add a new recipe: a");
        println!("Edit a recipe: e");
        take_input(&mut option, Some("Remove a recipe: r"), None);

        match option.trim() {
            "l" => { print_recipes(&recipes); },
            "a" => { add_recipe_info(&mut recipes); },
            "e" => {()},
            "r" => { remove_recipe(&mut recipes); },
            _ => {}
        }

        let mut option2: String = String::new();
        println!("What now?");
        println!("Press any button to see options again!");
        take_input(&mut option2, Some("To exit: Press x"), None);

        if option2.trim() == "x" {
            break;
        }
    }
}

fn add_recipe_info(recipes: &mut Recipes)  {
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    let id = recipes.len() + 1;
    
    take_input(&mut input1, Some("Enter the name: "), None);
    take_input(&mut input2, Some("Enter the description: "), None);

    recipes.insert(id, Recipe {
        name: input1.trim().to_string(),
        description: input2.trim().to_string(),
    });
}

fn remove_recipe(recipes: &mut Recipes) {
    let mut id: String = String::new();

    loop {
        take_input(&mut id, Some("Enter id to remove: "), None);
        
        match id.trim().parse::<u32>() {
            Ok(x) => { 
                recipes.remove(&(x as usize));
                break;
            },
            Err(_) => { println!("Please enter a valid id..."); }
        }
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
    while buffer.is_empty() {
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