fn eat_meal(mut meal: String) -> String {
    println!("{meal}");
    meal.clear();
    println!("{meal}");
    meal
}

fn main() {
    let is_concert: bool = true;
    let is_event: bool = is_concert;
    // the ownership is independent through the copy trait 
    println!("{is_concert}{is_event}");

    let sushi: &str = "Salmon";
    let dinner: &str = sushi;
// I dont think it will transfer ownership because strings do not possess the copy trait.

println!("{sushi} {dinner}");

    let orange: String = String::from("grapes");
    let apple: String = orange;

    println!("{apple}");

    println!("{}", eat_meal(apple))


}