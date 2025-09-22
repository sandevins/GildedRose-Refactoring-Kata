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
    let multiplier: i32 = if item.name.starts_with("Conjured") { 2 } else { 1 };
    item.quality += amount * multiplier;
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
        for item in &mut self.items {
            if item.name == "Sulfuras, Hand of Ragnaros" {
                continue;
            }
            item.sell_in -= 1;
            match item.name.as_str() {
                "Aged Brie" => Self::update_aged_brie(item),
                "Backstage passes to a TAFKAL80ETC concert" => Self::update_backstage_pass(item),
                _ => Self::update_normal(item),
            }
        }
    }

    fn update_aged_brie(item: &mut Item) {
        let amount = if item.sell_in < 0 { 2 } else { 1 };
        change_item_quality_by(item, amount);
    }

    fn update_backstage_pass(item: &mut Item) {
        if item.sell_in < 0 {
            item.quality = 0;
        } else if item.sell_in < 6 {
            change_item_quality_by(item, 3);
        } else if item.sell_in < 11 {
            change_item_quality_by(item, 2);
        } else {
            change_item_quality_by(item, 1);
        }
    }

    fn update_normal(item: &mut Item) {
        let amount = if item.sell_in < 0 { -2 } else { -1 };
        change_item_quality_by(item, amount);
    }
}

