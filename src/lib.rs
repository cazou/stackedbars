use std::{collections::HashMap, fmt::Display, rc::Rc};
use colored::Colorize;
use colored::CustomColor;

#[derive(Debug, Clone)]
struct BarItem {
    label: Rc<String>,
    width: usize,
    color: CustomColor,
    count: f64,
}

impl BarItem {
    fn new(label: &str, count: f64, width: usize, sum: f64) -> BarItem {
        let this_width = ((width as f64 * count) / sum).floor() as usize;
        let hash = Self::compute_hash(label, count);
        let color = CustomColor::new((hash & 0xff) as u8, ((hash & 0xff00) >> 8) as u8, ((hash & 0xff0000) >> 16) as u8);

        // TODO: Avoid to_string(), store &str with lifetime
        BarItem {
            label: Rc::new(label.to_string()),
            width: this_width,
            color,
            count,
        }
    }

    fn compute_hash(label: &str, count: f64) -> u32 {
        let mut hash = 0x101010;
        let mut i = 0;

        for b in label.as_bytes() {
            hash += (*b as u32) << (i * 8);
            i += 1;
            if i == 3 {
                i = 0;
            }
            if hash & 0xff000000 != 0 {
                let surplus = (hash & 0xff000000) >> 24;
                hash += surplus;
                hash &= 0xffffff;
            }
        }

        for b in count.to_le_bytes() {
            hash += (b as u32) << (i * 8);
            i += 1;
            if i == 3 {
                i = 0;
            }
            if hash & 0xff000000 != 0 {
                let surplus = (hash & 0xff000000) >> 24;
                hash += surplus;
                hash &= 0xffffff;
            }
        }

        hash
    } 
}

impl PartialEq for BarItem {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && other.label == self.label
    }
}

impl Eq for BarItem {}

impl PartialOrd for BarItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BarItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = other.width.cmp(&self.width);
        match cmp {
            std::cmp::Ordering::Equal => self.label.cmp(&other.label),
            _ => cmp,
        }
    }
}

impl Display for BarItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: If the label can fit in the bar, print it there
        for _ in 0..self.width {
            write!(f, "{}", " ".on_custom_color(self.color))?;
        }

        Ok(())
    }
}

//TODO: Replace String by anything that can be displayed
//TODO: Should allow any kind of number as value of the hashmap
#[derive(Clone)]
pub struct StackedBar {
    items: Vec<BarItem>,
    width: usize,
}

impl StackedBar {
    pub fn new(map: HashMap<String, f64>) -> StackedBar {
        let sum = map.values().fold(0.0, |sum, val| sum + *val);
        let width = 32;
        let mut items: Vec<BarItem> = map
            .iter()
            .map(|(l, c)| BarItem::new(l, *c, width, sum))
            .collect();

        items.sort();

        StackedBar { items, width }
    }

    pub fn with_palette(&mut self, palette: &[CustomColor]) -> StackedBar {
        let mut i = 0;

        for bar in self.items.iter_mut() {
            bar.color = palette[i];
            i = (i + 1) % palette.len();
        }

        self.clone()
    }

    pub fn with_color_map(&mut self, color_map: &HashMap<String, CustomColor>) -> StackedBar {
        for bar in self.items.iter_mut() {
            bar.color = color_map[bar.label.as_str()];
        }

        self.clone()
    }

    pub fn with_width(&mut self, width: usize) -> StackedBar {
        self.width = width;
        let sum = self.items.iter().fold(0.0, |sum, val| sum + val.count);

        for bar in self.items.iter_mut() {
            let count = bar.count;
            bar.width = ((width as f64 * count) / sum).floor() as usize;
        }

        self.clone()
    }
}

impl Display for StackedBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            write!(f, "{item}")?;
        }

        Ok(())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort1() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 2.0);
        values.insert("b".to_string(), 1.0);
        values.insert("c".to_string(), 0.5);

        let bar = StackedBar::new(values, 35);

        assert_eq!(
            bar.to_string(),
            "aaaaaaaaaaaaaaaaaaaabbbbbbbbbbccccc".to_string()
        );
    }

    #[test]
    fn sort2() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 2.0);
        values.insert("b".to_string(), 1.0);
        values.insert("c".to_string(), 1.0);

        let bar = StackedBar::new(values, 4);

        assert_eq!(bar.to_string(), "aabc".to_string());
    }

    #[test]
    fn sort3() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 2.0);
        values.insert("b".to_string(), 1.0);
        values.insert("c".to_string(), 2.0);

        let bar = StackedBar::new(values, 5);

        assert_eq!(bar.to_string(), "aaccb".to_string());
    }

    #[test]
    fn all_equal() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 1.0);
        values.insert("b".to_string(), 1.0);
        values.insert("c".to_string(), 1.0);
        values.insert("d".to_string(), 1.0);

        let bar = StackedBar::new(values, 32);

        assert_eq!(
            bar.to_string(),
            "aaaaaaaabbbbbbbbccccccccdddddddd".to_string()
        );
    }

    #[test]
    fn small_width() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 10.0);
        values.insert("b".to_string(), 20.0);
        values.insert("c".to_string(), 1.0);
        values.insert("d".to_string(), 2.0);

        let bar = StackedBar::new(values, 10);

        assert_eq!(bar.to_string(), "bbbbbbaaa".to_string());
    }

    #[test]
    fn small_values() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 1.0);
        values.insert("b".to_string(), 1.0);
        values.insert("c".to_string(), 1.0);
        values.insert("d".to_string(), 1.0);
        values.insert("e".to_string(), 1.0);
        values.insert("f".to_string(), 1.0);
        values.insert("g".to_string(), 1.0);
        values.insert("h".to_string(), 1.0);
        values.insert("i".to_string(), 1.0);
        values.insert("j".to_string(), 1.0);

        let bar = StackedBar::new(values, 5); //FIXME: Maybe this should fail

        assert_eq!(bar.to_string(), "".to_string());
    }
}
*/
