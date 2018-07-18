use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use item::Item;

#[derive(Debug)]
pub struct Map {
    width: i32,
    height: i32,
    wall: Vec<(f64, f64, f64)>,
    items: Vec<Item>
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let wall = Map::load_file("map.csv");
        let items = Item::init_items();
        Map {
            width: width,
            height: height,
            wall: wall,
            items: items
        }
    }

    fn create_map(&mut self) {

    }

    fn load_file(filename: &str) -> Vec<(f64, f64, f64)> {
        let f = File::open(filename).expect("cannnot open file");
        let reader = BufReader::new(f);

        let mut wall = Vec::new();
        for line in reader.lines().map(|l| l.unwrap()) {
            let mut points = (0f64, 0f64, 0f64);
            let mut splits = line.split(',');
            points.0 = splits.next().unwrap().parse::<f64>().unwrap();
            points.1 = splits.next().unwrap().parse::<f64>().unwrap();
            points.2 = splits.next().unwrap().parse::<f64>().unwrap();

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
        json.push_str("\n]\n");

        json
    }
}