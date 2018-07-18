
#[derive(Debug, Clone)]
pub enum ItemType {
    GOAL    = 0,
    ITEM1   = 1,
    ITEM2   = 2,
    ITEM3   = 3,
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

    pub fn init_items() -> Vec<Item> {
        vec![
            Item::new(
                ItemType::GOAL,
                (10f64, 4f64, 0f64)
            ),
            Item::new(
                ItemType::ITEM1,
                (-10f64, 4f64, 0f64)
            ),
            Item::new(
                ItemType::ITEM2,
                (0f64, 4f64, 10f64)
            ),
            Item::new(
                ItemType::ITEM3,
                (0f64, 4f64, -10f64)
            ),
        ]
    }
}

impl ToString for Item {
    #[inline]
    fn to_string(&self) -> String {
        // clone
        let json = format!("{{\"id\": {}, \"pos\": [{}, {}, {}]}}\n", self.item_type.clone() as i32, self.pos.0, self.pos.1, self.pos.2);

        json
    }
}