use dialoguer::Input;

fn main() {
    let input = Input::<String>::new()
        .with_prompt("Enter your weight (kg)")
        .interact()
        .unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    let weight_on_mars = calculate_weight_on_mars(weight);

    println!("Your weight on Mars is {} kg.", weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight / 9.81 * 3.711
}
