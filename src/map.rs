use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
pub struct Map {
    wall: Vec<(i32, i32, i32)>
}

impl Map {
    pub fn new() -> Map {
        let wall = Map::load_file("map.csv");
        Map {
            wall: wall
        }
    }

    fn load_file(filename: &str) -> Vec<(i32, i32, i32)> {
        let f = File::open(filename).expect("cannnot open file");
        let mut reader = BufReader::new(f);

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
}

impl ToString for Map {
    #[inline]
    fn to_string(&self) -> String {
        let mut json = "{\n".to_string();
        for vec in &self.wall {
            json.push_str(format!("[{},{},{}], ", vec.0, vec.1, vec.2).as_str());
        }
        json.pop();
        json.pop();
        json.push_str("\n}\n");

        json
    }
}