use std::collections::HashMap;

use colored::CustomColor;
use stackedbars::StackedBar;

fn main() {
    let mut map = HashMap::new();

    map.insert("Earth".to_string(), 5972.0 * 10e21);
    map.insert("Venus".to_string(), 4867.0 * 10e21);
    map.insert("Mars".to_string(),  641.0 * 10e21);

    let mut bars = StackedBar::new(map).with_width(150);
    println!("Planets mass: {bars}");

    bars = bars.with_palette(&[CustomColor::new(0, 0, 255), CustomColor::new(0, 255, 0), CustomColor::new(255, 0, 0), ]);
    println!("Planets mass: {bars}");

    let mut color_map = HashMap::new();

    color_map.insert("Earth".to_string(), CustomColor::new(128, 128, 255));
    color_map.insert("Venus".to_string(), CustomColor::new(128, 255, 128));
    color_map.insert("Mars".to_string(),  CustomColor::new(255, 128, 128));

    bars = bars.with_color_map(&color_map);
    println!("Planets mass: {bars}");
}
