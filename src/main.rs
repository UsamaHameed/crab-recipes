mod app;
mod recipes;
use app::{AppFns, App};
use recipes::RecipeFns;

fn main() {
    let app = App::new();
    // app.recipes.add_recipe();

    // app.recipes.insert(1, Recipe { 
    //     name: String::from("crabs"), 
    //     description: String::from("haha") 
    // });

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
        
        // take_input(&mut option, prompt, None);

        // match option.trim() {
        //     "l" => { recipes.print_all(); },
        //     "a" => { recipes.add_recipe_info(); },
        //     "e" => { recipes.edit_recipe() },
        //     "r" => { recipes.remove_recipe(); },
        //     other => { 
        //         println!("invalid option: {}", other);
        //         continue; 
        //     }
        // }

        // prompt_exit(&mut option2);
    }
}
