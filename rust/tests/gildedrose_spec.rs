use rust::{GildedRose, Item};

fn setup_item(name: &str, sell_in: i32, quality: i32) -> Item {
    Item::new(name, sell_in, quality)
}

#[test]
fn normal_item_quality_degrades_by_1() {
    let items = vec![setup_item("normal", 10, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 19);
    assert_eq!(rose.items[0].sell_in, 9);
}

#[test]
fn normal_item_quality_degrades_twice_as_fast_after_sellin() {
    let items = vec![setup_item("normal", 0, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 18);
    assert_eq!(rose.items[0].sell_in, -1);
}

#[test]
fn quality_never_negative() {
    let items = vec![setup_item("normal", 5, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert!(rose.items[0].quality >= 0);
}

#[test]
fn aged_brie_increases_quality() {
    let items = vec![setup_item("Aged Brie", 2, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 1);
}

#[test]
fn aged_brie_increases_quality_twice_as_fast_after_sellin() {
    let items = vec![setup_item("Aged Brie", 0, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 2);
}

#[test]
fn quality_never_exceeds_50() {
    let items = vec![setup_item("Aged Brie", 2, 50)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert!(rose.items[0].quality <= 50);
}

#[test]
fn sulfuras_never_changes() {
    let items = vec![setup_item("Sulfuras, Hand of Ragnaros", 0, 80)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 80);
    assert_eq!(rose.items[0].sell_in, 0);
}

#[test]
fn backstage_increases_quality_by_1_when_more_than_10_days() {
    let items = vec![setup_item("Backstage passes to a TAFKAL80ETC concert", 15, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 21);
}

#[test]
fn backstage_increases_quality_by_2_when_10_days_or_less() {
    let items = vec![setup_item("Backstage passes to a TAFKAL80ETC concert", 10, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 22);
}

#[test]
fn backstage_increases_quality_by_3_when_5_days_or_less() {
    let items = vec![setup_item("Backstage passes to a TAFKAL80ETC concert", 5, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 23);
}

#[test]
fn backstage_quality_drops_to_0_after_concert() {
    let items = vec![setup_item("Backstage passes to a TAFKAL80ETC concert", 0, 20)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    assert_eq!(rose.items[0].quality, 0);
}

// TDD: Conjured items (expected to fail until implemented)
#[test]
fn conjured_items_degrade_twice_as_fast() {
    let items = vec![setup_item("Conjured Mana Cake", 3, 6)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    // Should degrade by 2
    assert_eq!(rose.items[0].quality, 4);
}

#[test]
fn conjured_items_degrade_four_times_as_fast_after_sellin() {
    let items = vec![setup_item("Conjured Mana Cake", 0, 6)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();
    // Should degrade by 4
    assert_eq!(rose.items[0].quality, 2);
}
