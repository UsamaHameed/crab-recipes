use std::fmt;

// #[derive(Debug)]
struct Recipe {
    name: String,
    id: u32,
    description: String,
    // ingredients
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nId: {}\nDescription: {}", self.name, self.id, self.description)
    }
}

fn main() {
    // store recipes
    let mut recipes: Vec<Recipe> = Vec::new();

    recipes.push(
        Recipe { 
            id: 1, 
            name: String::from("crabs"), 
            description: String::from("haha") 
        }
    );
    
    // display recipes
    for recipe in recipes {
        println!("{}", recipe);
    }
}
