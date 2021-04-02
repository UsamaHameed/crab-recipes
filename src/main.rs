use std::{collections::HashMap, env, fmt, io, process};

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
        println!("Remove a recipe: r");

        io::stdin()
            .read_line(&mut option)
            .expect("Unable to parse entered text");

        match option.trim() {
            "l" => { print_recipes(&recipes); },
            "a" => { add_recipe_info(&mut recipes); },
            "e" => {()},
            "r" => {()},
            _ => {}
        }

        let mut option2: String = String::new();
        println!("What now?");
        println!("Press any button to see options again!");
        println!("To exit: Press x");

        io::stdin()
            .read_line(&mut option2)
            .expect("Unable to parse entered text");

        if option2.trim() == "x" {
            break;
        }
    }
}

fn add_recipe_info(recipes: &mut Recipes)  {
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    let id = recipes.len() + 1;

    println!("Enter the name:");
    
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    
    println!("Enter the description:");

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    recipes.insert(id, Recipe {
        name: input1.trim().to_string(),
        description: input2.trim().to_string(),
    });
}

// fn remove_recipe(recipes: &Vec<Recipe>, name: String) -> Vec<Recipe> {
//     let mut name = String::new();

//     recipes.iter_mut()
//         .filter(|recipe| recipe.name == name).collect()
    
//     // recipes

// }

fn print_recipes(recipes: &Recipes) {
    println!("\n");
    for (id, recipe) in recipes {
        print!("id: {} ", id);
        println!("{}\n", recipe);
    }
}