mod app;
mod recipes;
use app::App;

fn main() {
    let mut app = App::new();
    app.listen();
}
