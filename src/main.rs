use std::{collections::HashMap};
use crab_recipes::{Recipes, Recipe, take_input, prompt_exit, RecipeFns };

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
