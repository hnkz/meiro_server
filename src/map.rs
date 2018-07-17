use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use item::Item;

#[derive(Debug)]
pub struct Map {
    wall: Vec<(i32, i32, i32)>,
    items: Vec<Item>
}

impl Map {
    pub fn new() -> Map {
        let wall = Map::load_file("map.csv");
        let mut items = Item::init_items();
        Map {
            wall: wall,
            items: items
        }
    }

    fn load_file(filename: &str) -> Vec<(i32, i32, i32)> {
        let f = File::open(filename).expect("cannnot open file");
        let reader = BufReader::new(f);

        let mut wall = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let mut points = (0i32, 0i32, 0i32);
            let mut splits = line.split(',');
            points.0 = splits.next().unwrap().parse::<i32>().unwrap();
            points.1 = splits.next().unwrap().parse::<i32>().unwrap();
            points.2 = splits.next().unwrap().parse::<i32>().unwrap();

            wall.push(points);
        }

        wall
    }

    pub fn item_to_string(&self) -> String {
        let mut json = ",\"item\": [\n".to_string();
        for item in &self.items {
            json.push_str(item.to_string().as_str());
            json.push_str(",");
        }
        json.pop();
        json.push_str("]\n");

        json
    }

    pub fn remove_item(&mut self, i: usize) {
        self.items.remove(i);
    }
}

impl ToString for Map {
    #[inline]
    fn to_string(&self) -> String {
        let mut json = ",\"map\": [\n".to_string();
        for vec in &self.wall {
            json.push_str(format!("[{},{},{}], ", vec.0, vec.1, vec.2).as_str());
        }
        json.pop();
        json.pop();
        json.push_str("\n],\n");

        json
    }
}