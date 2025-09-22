use std::fmt::{self, Display};
// This file is now lib.rs
const MAX_QUALITY: i32 = 50;
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

fn change_item_quality_by(item: &mut Item, amount: i32) {
        item.quality += amount;
        if item.quality < 0 {
            item.quality = 0;
        } else if item.quality > MAX_QUALITY {
            item.quality = MAX_QUALITY;
        }
    }

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if self.items[i].name == "Sulfuras, Hand of Ragnaros" {
                continue;
            }
            self.items[i].sell_in = self.items[i].sell_in - 1;
            match self.items[i].name.as_str() {
                "Aged Brie" => {
                    if self.items[i].sell_in < 0 {
                        change_item_quality_by(&mut self.items[i], 2);
                    } else {
                        change_item_quality_by(&mut self.items[i], 1);
                    }
                }
                "Backstage passes to a TAFKAL80ETC concert" => {
                    if self.items[i].sell_in >= 0 {
                        if self.items[i].sell_in < 6 {
                            change_item_quality_by(&mut self.items[i], 3);
                        } else if self.items[i].sell_in < 11 {
                            change_item_quality_by(&mut self.items[i], 2);
                        } else {
                            change_item_quality_by(&mut self.items[i], 1);
                        }
                    } else {
                        self.items[i].quality = 0;
                    }
                }
                _ => {
                    if self.items[i].sell_in < 0 {
                        change_item_quality_by(&mut self.items[i], -2);
                    } else {
                        change_item_quality_by(&mut self.items[i], -1);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }
}
