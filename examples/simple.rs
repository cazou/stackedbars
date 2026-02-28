use std::collections::HashMap;

use stackedbars::StackedBar;

fn main() {
    let mut map = HashMap::new();

    map.insert("Earth".to_string(), 5972.0 * 10e21);
    map.insert("Venus".to_string(), 4867.0 * 10e21);
    map.insert("Mars".to_string(),  641.0 * 10e21);

    let bars = StackedBar::new(map, 100);

    println!("Planets mass: {bars}");
}
