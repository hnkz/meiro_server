
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
    pos: (i32, i32, i32)
}

impl Item {
    pub fn new(item_type: ItemType, pos: (i32, i32, i32)) -> Item {
        Item {
            item_type: item_type,
            pos: pos
        }
    }

    pub fn init_items() -> Vec<Item> {
        vec![
            Item {
                item_type: ItemType::GOAL,
                pos: (0, 0, 0)
            },
            Item {
                item_type: ItemType::ITEM1,
                pos: (0, 0, 0)
            },
            Item {
                item_type: ItemType::ITEM2,
                pos: (0, 0, 0)
            },
            Item {
                item_type: ItemType::ITEM3,
                pos: (0, 0, 0)
            },
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