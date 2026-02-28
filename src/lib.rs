use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct BarItem {
    label: String,
    width: usize,
    count: f64,
}

impl BarItem {
    fn new(label: &str, count: f64, width: usize, sum: f64) -> BarItem {
        let this_width = ((width as f64 * count) / sum).floor() as usize;

        // TODO: Avoid to_string(), store &str with lifetime
        BarItem {
            label: label.to_string(),
            width: this_width,
            count,
        }
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
        for _ in 0..self.width {
            write!(f, "{}", self.label)?;
        }
        Ok(())
    }
}

//TODO: Replace String by anything that can be displayed
//TODO: Should allow any kind of number as value of the hashmap
pub struct StackedBar {
    items: Vec<BarItem>,
}

impl StackedBar {
    pub fn new(map: HashMap<String, f64>, width: usize) -> StackedBar {
        let sum = map.values().fold(0.0, |sum, val| sum + *val);
        let mut items: Vec<BarItem> = map
            .iter()
            .map(|(l, c)| BarItem::new(l, *c, width, sum))
            .collect();
        items.sort();
        StackedBar { items }
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
