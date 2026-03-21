use std::collections::HashMap;

use colored::CustomColor;
use stackedbars::StackedBar;

fn main() {
    let mut map = HashMap::new();

    map.insert("Earth", 5972.0 * 10e21);
    map.insert("Venus", 4867.0 * 10e21);
    map.insert("Mars", 641.0 * 10e21);

    let mut bars = StackedBar::new(map).with_width(150);
    println!("Planets mass: {bars}");

    bars = bars.with_palette(&[
        CustomColor::new(0, 0, 255),
        CustomColor::new(0, 255, 0),
        CustomColor::new(255, 0, 0),
    ]);

    bars.with_labels("%L - %P %", false);

    println!("Planets mass: {bars}");

    let mut color_map = HashMap::new();

    color_map.insert("Earth", CustomColor::new(128, 128, 255));
    color_map.insert("Venus", CustomColor::new(128, 255, 128));
    color_map.insert("Mars", CustomColor::new(255, 128, 128));

    bars = bars.with_color_map(&color_map);
    bars.with_outline_label("%L - %C Kg");

    println!("Planets mass: {bars}");
}
