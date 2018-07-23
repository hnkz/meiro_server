use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use rand::prelude::*;

const WALL_LENGTH: i32 = 10;

#[derive(PartialEq)]
enum ElmType {
    AISLE   = 0, // 通路
    WALL    = 1,
    START   = 2,
    GOAL    = 3,
    EMPTY   = 4,
}

pub struct Map {
    width   : i32,
    height  : i32,
    map     : Vec<Vec<ElmType>>,
    wall    : Vec<(f64, f64, f64)>,
    start   : (f64, f64, f64),
    goal    : (f64, f64, f64)
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            width   : width,
            height  : height,
            map     : Vec::new(),
            wall    : Vec::new(),
            start   : (0f64, 0f64, 0f64),
            goal    : (0f64, 0f64, 0f64)
        };

        map.create_map();

        map
    }

    fn create_map(&mut self) {
        let width = self.width;
        let height = self.height;

        let mv: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut map = Vec::new();
        
        let mut search_vec: Vec<(i32, i32)>     = Vec::new();
        let mut searched_vec: Vec<(i32, i32)>   = Vec::new();

        for i in 0..height {
            let mut map_vec = Vec::new();
            for j in 0..width {
                if i == 0 || j == 0 || i == (height-1) || j == (width-1) {
                    map_vec.push(ElmType::WALL);
                } else if i % 2 == 0 && j % 2 == 0 {
                    search_vec.push((j as i32, i as i32));
                    map_vec.push(ElmType::AISLE);
                } else {
                    map_vec.push(ElmType::AISLE);
                }
            }
            map.push(map_vec);
        }

        map[1][1] = ElmType::START;
        map[(width-2) as usize][(width-2) as usize] = ElmType::GOAL;

        self.start = ((1 * WALL_LENGTH) as f64, 4f64, (1 * WALL_LENGTH) as f64);
        self.goal = (((width-2) * WALL_LENGTH) as f64, 4f64, ((width-2) * WALL_LENGTH) as f64);

        // 柱が空になるまで
        while !search_vec.is_empty() {
            // get random search_vec's item
            let search_idx = random::<usize>() % search_vec.len();
            let (mut x, mut y) = search_vec.remove(search_idx);

            if map[x as usize][y as usize] == ElmType::AISLE {
                searched_vec.clear();
                while map[x as usize][y as usize] != ElmType::WALL {
                    map[x as usize][y as usize] = ElmType::WALL;

                    // 方向をランダムで選定
                    let m_idx = random::<usize>() % mv.len();
                    let m = mv[m_idx];

                    // 拡張中の壁に当たるか？
                    let mut collision_count = 0;
                    for searched in &searched_vec {
                        for m in mv.iter() {
                            if  searched.0 == x + (m.0 * 2) && searched.1 == y + (m.1 * 2) &&
                                map[(y + (m.1 * 2)) as usize][(x + (m.0 * 2)) as usize] == ElmType::WALL
                            {
                                collision_count += 1;
                                break;
                            }
                        }
                    }

                    if collision_count == 4 {
                        break;
                    }
                    
                    map[(y + m.1) as usize][(x + m.0) as usize] = ElmType::WALL;
                    searched_vec.push((x + (m.0 * 2), y + (m.1 * 2)));
                    x = x + (m.0 * 2);
                    y = y + (m.1 * 2);
                }
            }
        }

        let mut wall_vec = Vec::new();
        for i in 0..height {
            for j in 0..width {
                if map[i as usize][j as usize] == ElmType::WALL {
                    wall_vec.push(((j * WALL_LENGTH) as f64, 0f64, (i * WALL_LENGTH) as f64));
                }
            }
        }

        self.wall   = wall_vec;
        self.map    = map;
    }

    #[allow(dead_code)]
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

    pub fn get_start_pos(&self) -> (f64, f64, f64) {
        self.start
    }

    #[allow(dead_code)]
    pub fn get_goal_pos(&self) -> (f64, f64, f64) {
        self.goal
    }

    pub fn get_random_pos(&mut self) -> (f64, f64, f64) {
        loop {
            let x = random::<usize>() % self.width as usize;
            let y = random::<usize>() % self.height as usize;

            if self.map[y][x] == ElmType::AISLE {
                self.map[y][x] = ElmType::EMPTY;
                return ((x * WALL_LENGTH as usize) as f64, 4f64, (y * WALL_LENGTH as usize) as f64)
            }
        }
    }
}

impl ToString for Map {
    #[inline]
    fn to_string(&self) -> String {
        let mut json = "\"map\": [\n".to_string();
        for vec in &self.wall {
            json.push_str(format!("[{},{},{}], ", vec.0, vec.1, vec.2).as_str());
        }
        json.pop();
        json.pop();
        json.push_str("\n]\n");

        json
    }
}