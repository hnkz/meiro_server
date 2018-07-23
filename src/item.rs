use num::traits::FromPrimitive;

#[derive(Debug, Clone)]
pub enum ItemType {
    GOAL    = 0,
    ITEM1   = 1,
    ITEM2   = 2,
    ITEM3   = 3,
}

impl FromPrimitive for ItemType {
    fn from_i64(n: i64) -> Option<ItemType> {
        match n {
            0 => Some(ItemType::GOAL),
            1 => Some(ItemType::ITEM1),
            2 => Some(ItemType::ITEM2),
            3 => Some(ItemType::ITEM3),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<ItemType> {
        match n {
            0 => Some(ItemType::GOAL),
            1 => Some(ItemType::ITEM1),
            2 => Some(ItemType::ITEM2),
            3 => Some(ItemType::ITEM3),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Item {
    item_type: ItemType,
    pos: (f64, f64, f64)
}

impl Item {
    pub fn new(item_type: ItemType, pos: (f64, f64, f64)) -> Item {
        Item {
            item_type: item_type,
            pos: pos
        }
    }

    #[allow(dead_code)]
    pub fn init_items() -> Vec<Item> {
        vec![
            Item::new(
                ItemType::GOAL,
                (142f64, 4f64, -23f64)
            ),
            Item::new(
                ItemType::ITEM1,
                (58f64, 4f64, -2f64)
            ),
            Item::new(
                ItemType::ITEM2,
                (75f64, 4f64, -22f64)
            ),
            Item::new(
                ItemType::ITEM3,
                (95f64, 4f64, -3f64)
            ),
        ]
    }
}

impl ToString for Item {
    #[inline]
    fn to_string(&self) -> String {
        // clone
        let json = format!("{{\"type\": {}, \"pos\": [{}, {}, {}]}}\n", self.item_type.clone() as i32, self.pos.0, self.pos.1, self.pos.2);

        json
    }
}

