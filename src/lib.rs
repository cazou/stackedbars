use colored::Colorize;
use colored::CustomColor;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
struct BarItem<'a> {
    label: &'a str,
    width: usize,
    color: CustomColor,
    count: f64,
    parent_sum: f64,
    label_format: String,
    force_label: bool,
}

impl<'a> BarItem<'a> {
    fn new(label: &'a str, count: f64, width: usize, sum: f64) -> BarItem<'a> {
        let this_width = ((width as f64 * count) / sum).floor() as usize;
        let hash = Self::compute_hash(label, count);
        let color = CustomColor::new(
            (hash & 0xff) as u8,
            ((hash & 0xff00) >> 8) as u8,
            ((hash & 0xff0000) >> 16) as u8,
        );

        // TODO: Avoid to_string(), store &str with lifetime
        BarItem {
            label,
            width: this_width,
            color,
            count,
            parent_sum: sum,
            label_format: "".to_string(),
            force_label: false,
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

    fn render_label(&self, format: &str) -> String {
        let mut builder = format.to_string();
        builder = builder.replace("%L", self.label.as_ref());
        builder = builder.replace("%C", format!("{}", self.count).as_str());
        builder = builder.replace(
            "%P",
            format!("{:.2}", 100f64 * self.count / self.parent_sum).as_str(),
        );

        builder
    }

    fn render(&self) -> String {
        let mut builder = self.render_label(&self.label_format);

        if builder.len() > self.width {
            if self.force_label {
                builder.truncate(self.width);
            } else {
                builder = "".to_string();
            }
        }

        builder = (0..(self.width - builder.len()) / 2)
            .map(|_| ' ')
            .collect::<String>()
            + builder.as_str()
            + (0..(self.width - builder.len()) / 2)
                .map(|_| ' ')
                .collect::<String>()
                .as_str();

        if builder.len() < self.width {
            builder += (builder.len()..self.width)
                .map(|_| ' ')
                .collect::<String>()
                .as_str();
        }

        let label_color =
            CustomColor::new(255 - self.color.r, 255 - self.color.g, 255 - self.color.b);

        // FIXME: Is to string avoidable ?
        builder
            .on_custom_color(self.color)
            .custom_color(label_color)
            .to_string()
    }

    fn render_outline_label(&self, format: &str) -> String {
        let label = self.render_label(format);
        " ".on_custom_color(self.color).to_string() + ": " + label.as_str()
    }
}

impl PartialEq for BarItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && other.label == self.label
    }
}

impl Eq for BarItem<'_> {}

impl PartialOrd for BarItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BarItem<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = other.width.cmp(&self.width);
        match cmp {
            std::cmp::Ordering::Equal => self.label.cmp(&other.label),
            _ => cmp,
        }
    }
}

impl Display for BarItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

//TODO: Replace String by anything that can be displayed
//TODO: Should allow any kind of number as value of the hashmap
#[derive(Clone)]
pub struct StackedBar<'a> {
    items: Vec<BarItem<'a>>,
    width: usize,
    outline_labels_format: Option<String>,
}

impl<'a> StackedBar<'a> {
    pub fn new(map: HashMap<&'a str, f64>) -> StackedBar<'a> {
        let sum = map.values().fold(0.0, |sum, val| sum + *val);
        let width = 32;
        let mut items: Vec<BarItem> = map
            .iter()
            .map(|(l, c)| BarItem::new(l, *c, width, sum))
            .collect();

        items.sort();

        StackedBar {
            items,
            width,
            outline_labels_format: None,
        }
    }

    pub fn with_palette(&mut self, palette: &[CustomColor]) -> StackedBar<'a> {
        let mut i = 0;

        for bar in self.items.iter_mut() {
            bar.color = palette[i];
            i = (i + 1) % palette.len();
        }

        self.clone()
    }

    pub fn with_color_map(&mut self, color_map: &HashMap<&str, CustomColor>) -> StackedBar<'a> {
        for bar in self.items.iter_mut() {
            bar.color = color_map[bar.label];
        }

        self.clone()
    }

    pub fn with_width(&mut self, width: usize) -> StackedBar<'a> {
        self.width = width;
        let sum = self.items.iter().fold(0.0, |sum, val| sum + val.count);

        for bar in self.items.iter_mut() {
            let count = bar.count;
            bar.width = ((width as f64 * count) / sum).floor() as usize;
        }

        self.clone()
    }

    pub fn with_labels(&mut self, format: &str, always: bool) -> StackedBar<'a> {
        for bar in self.items.iter_mut() {
            bar.label_format = format.to_string();
            bar.force_label = always;
        }

        self.clone()
    }

    pub fn with_outline_label(&mut self, format: &str) -> StackedBar<'a> {
        self.outline_labels_format = Some(format.to_string());

        self.clone()
    }
}

impl Display for StackedBar<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            write!(f, "{item}")?;
        }
        if let Some(format) = self.outline_labels_format.as_ref() {
            writeln!(f)?;
            for item in &self.items {
                writeln!(f, "{}", item.render_outline_label(format))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invert_color(color: &CustomColor) -> CustomColor {
        CustomColor::new(255 - color.r, 255 - color.g, 255 - color.b)
    }

    fn render_out(widths: &[usize], colors: &[CustomColor]) -> String {
        assert_eq!(widths.len(), colors.len());

        let mut out = String::new();
        for (i, width) in widths.iter().enumerate() {
            out += (0..*width)
                .map(|_| ' ')
                .collect::<String>()
                .on_custom_color(colors[i])
                .custom_color(invert_color(&colors[i]))
                .to_string()
                .as_str();
        }

        out
    }

    #[test]
    fn sort1() {
        let mut values = HashMap::new();
        values.insert("a", 2.0);
        values.insert("b", 1.0);
        values.insert("c", 0.5);

        let bar = StackedBar::new(values).with_width(35);

        let widths = vec![20, 10, 5];

        let colors = vec![
            CustomColor {
                r: 113,
                g: 16,
                b: 80,
            },
            CustomColor {
                r: 114,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 115,
                g: 240,
                b: 79,
            },
        ];

        let out = render_out(&widths, &colors);
        assert_eq!(bar.to_string(), out);
    }

    #[test]
    fn sort2() {
        let mut values = HashMap::new();
        values.insert("a", 2.0);
        values.insert("b", 1.0);
        values.insert("c", 1.0);

        let bar = StackedBar::new(values).with_width(4);

        let widths = vec![2, 1, 1];

        let colors = vec![
            CustomColor {
                r: 113,
                g: 16,
                b: 80,
            },
            CustomColor {
                r: 114,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 115,
                g: 0,
                b: 80,
            },
        ];
        let out = render_out(&widths, &colors);
        assert_eq!(bar.to_string(), out);
    }

    #[test]
    fn sort3() {
        let mut values = HashMap::new();
        values.insert("a", 2.0);
        values.insert("b", 1.0);
        values.insert("c", 2.0);

        let bar = StackedBar::new(values).with_width(5);

        let widths = vec![2, 2, 1];

        let colors = vec![
            CustomColor {
                r: 113,
                g: 16,
                b: 80,
            },
            CustomColor {
                r: 115,
                g: 16,
                b: 80,
            },
            CustomColor {
                r: 114,
                g: 0,
                b: 80,
            },
        ];
        let out = render_out(&widths, &colors);
        assert_eq!(bar.to_string(), out);
    }

    #[test]
    fn all_equal() {
        let mut values = HashMap::new();
        values.insert("a", 1.0);
        values.insert("b", 1.0);
        values.insert("c", 1.0);
        values.insert("d", 1.0);

        let bar = StackedBar::new(values).with_width(32);

        let colors = vec![
            CustomColor {
                r: 113,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 114,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 115,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 116,
                g: 0,
                b: 80,
            },
        ];

        let widths = vec![8, 8, 8, 8];

        let out = render_out(&widths, &colors);

        assert_eq!(bar.to_string(), out);
    }

    #[test]
    fn small_width() {
        let mut values = HashMap::new();
        values.insert("a", 10.0);
        values.insert("b", 20.0);
        values.insert("c", 1.0);
        values.insert("d", 2.0);

        let bar = StackedBar::new(values).with_width(10);

        let colors = vec![
            CustomColor {
                r: 114,
                g: 68,
                b: 80,
            },
            CustomColor {
                r: 113,
                g: 52,
                b: 80,
            },
            CustomColor {
                r: 116,
                g: 16,
                b: 80,
            },
            CustomColor {
                r: 115,
                g: 0,
                b: 80,
            },
        ];

        let widths = vec![6, 3, 0, 0];

        let out = render_out(&widths, &colors);

        assert_eq!(bar.to_string(), out);
    }

    #[test]
    fn small_values() {
        let mut values = HashMap::new();
        values.insert("a", 1.0);
        values.insert("b", 1.0);
        values.insert("c", 1.0);
        values.insert("d", 1.0);
        values.insert("e", 1.0);
        values.insert("f", 1.0);
        values.insert("g", 1.0);
        values.insert("h", 1.0);
        values.insert("i", 1.0);
        values.insert("j", 1.0);

        let bar = StackedBar::new(values).with_width(5); //FIXME: Maybe this should fail

        let colors = vec![
            CustomColor {
                r: 113,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 114,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 115,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 116,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 117,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 118,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 119,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 120,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 121,
                g: 0,
                b: 80,
            },
            CustomColor {
                r: 122,
                g: 0,
                b: 80,
            },
        ];

        let widths = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let out = render_out(&widths, &colors);

        assert_eq!(bar.to_string(), out);
    }
}
