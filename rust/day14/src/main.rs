use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;

struct AOC {
    data: Vec<Vec<(i32, i32)>>,
}

impl AOC {
    fn new() -> AOC {
        AOC { data: Vec::new() }
    }

    // a buffered reader is returned from here
    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file: File = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    // load all the data in one go
    fn load_all_data(&mut self, filename: &String) {
        if let Ok(lines) = self.read_lines(filename) {
            for l in lines {
                if let Ok(line) = l {
                    // each line is a list separated by "->"
                    let mut line_data: Vec<(i32, i32)> = Vec::new();
                    let vectors: Vec<&str> = line.split("->").collect();
                    // after trim, each string in the vec is a list of 2 numbers separated by ","
                    for s in vectors {
                        let mut nums: Vec<i32> = Vec::new();
                        if s.len() == 0 {
                            continue;
                        }
                        for n in s.trim().split(",") {
                            nums.push(n.parse::<i32>().unwrap());
                        }
                        line_data.push((nums[0], nums[1]));
                    }
                    self.data.push(line_data);
                }
            }
        }
    }

    fn get_filename(&self) -> Result<String, String> {
        let mut filename: String = "./test-data.txt".to_string();
        match std::env::args().nth(1) {
            Some(arg) => {
                filename = arg;
            }
            None => {
                println!("No filename given, using test data\n");
            }
        }
        Ok(filename)
    }
}

#[derive(Debug, Clone)]
struct Shape {
    outline: Vec<(i32, i32)>,
    points: Vec<(i32, i32)>,
}

impl Shape {
    fn new(outline: Vec<(i32, i32)>) -> Shape {
        Shape {
            outline: outline,
            points: Vec::new(),
        }
    }

    fn get_all_points_in_shape(&self) -> Vec<(i32, i32)> {
        let mut all_points = Vec::new();
        let mut points_data = self.outline.clone();
        points_data.reverse();
        let mut start = points_data.pop().unwrap(); // get the first
        while points_data.len() > 0 {
            let next = points_data.pop().unwrap();
            // println!("{},{} -> {},{}", start.0, start.1, next.0, next.1);
            let next_points = Shape::get_all_points_between_two_points(start, next).clone();
            // println!("{:?}", next_points);
            for (x, y) in next_points {
                all_points.push((x, y));
            }
            start = (next.0, next.1);
        }
        all_points
    }

    fn get_coords(&self) -> ((i32, i32), (i32, i32)) {
        let mut min_x: i32 = 5000;
        let mut max_x: i32 = -5000;
        let mut min_y: i32 = 5000;
        let mut max_y: i32 = -5000;

        for (x, y) in &self.outline {
            if x < &min_x {
                min_x = *x;
            }
            if x > &max_x {
                max_x = *x;
            }
            if y < &min_y {
                min_y = *y;
            }
            if y > &max_y {
                max_y = *y;
            }
        }

        // for drawing the shape
        let coords = ((min_x, min_y), (max_x, max_y));

        // println!("{:?}", coords);

        coords
    }

    fn get_all_points_between_two_points(p1: (i32, i32), p2: (i32, i32)) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = Vec::new();

        if p1.0 == p2.0 {
            if p2.1 > p1.1 {
                for y in p1.1..=p2.1 {
                    points.push((p1.0, y));
                }
            } else {
                for y in p2.1..=p1.1 {
                    points.push((p1.0, y));
                }
            }
        } else if p1.1 == p2.1 {
            if p2.0 > p1.0 {
                for x in p1.0..=p2.0 {
                    points.push((x, p1.1));
                }
            } else {
                for x in p2.0..=p1.0 {
                    points.push((x, p1.1));
                }
            }
        }

        points
    }
}

struct Cave {
    shapes: Vec<Shape>,
    coords: ((i32, i32), (i32, i32)),
    sand: Vec<(i32, i32)>,
    points: Vec<(i32, i32)>,
}

impl Cave {
    fn new(shapes: Vec<Shape>) -> Cave {
        let mut cave = Cave {
            shapes: shapes.clone(),
            coords: ((0, 0), (0, 0)),
            sand: vec![] as Vec<(i32, i32)>,
            points: vec![] as Vec<(i32, i32)>,
        };

        cave.coords = cave.get_total_size_of_all_shapes();
        cave.points = cave.get_all_points_in_cave();

        cave
    }

    fn drop_sand_until_blocked(&mut self) -> i32 {
        let max_y = self.coords.1 .1;
        let mut blocked = false;

        let mut blocks: HashSet<(i32, i32)> = HashSet::new();
        for p in &self.points {
            blocks.insert(*p);
        }
        let mut sand_blocks: HashSet<(i32, i32)> = HashSet::new();

        loop {
            let mut grain = Grain { x: 500, y: 0 };

            loop {
                // if it can move down, move down

                let points_to_check = vec![
                    (grain.x, grain.y + 1),
                    (grain.x - 1, grain.y + 1),
                    (grain.x + 1, grain.y + 1),
                ];

                let mut moved_down = false;

                for point in points_to_check {
                    if !blocks.contains(&(point.0, point.1))
                        && !sand_blocks.contains(&(point.0, point.1))
                    {
                        grain.move_to(point.0, point.1);
                        moved_down = true;
                        // println!("Moved down to {},{}", grain.x, grain.y);
                        break;
                    }
                }

                if moved_down {
                    continue;
                }

                // add to sand
                let sand_point = (grain.x, grain.y);
                self.sand.push(sand_point);
                sand_blocks.insert(sand_point);

                if grain.x == 500 && grain.y == 0 {
                    // it's at the first position and hasn't moved so it's blocked
                    blocked = true;
                }

                // self.draw_state();
                break;
            }

            if self.sand.len() % 100 == 0 {
                println!("Sand count: {}", self.sand.len());
                println!("blocked: {:?}", blocked);
                // self.draw_state();
                println!("Sand count: {}", self.sand.len());
            }

            if blocked {
                break;
            }
        }
        self.sand.len() as i32
    }

    fn get_all_points_in_cave(&mut self) -> Vec<(i32, i32)> {
        let mut all_points: Vec<(i32, i32)> = vec![];

        for shape in &self.shapes {
            let points = shape.get_all_points_in_shape();
            for p in points {
                all_points.push(p);
            }
        }

        all_points
    }

    fn get_cave_bounds(&self) -> ((i32, i32), (i32, i32)) {
        let min_x: i32 = self.coords.0 .0 - 1;
        let max_x: i32 = self.coords.1 .0 + 1;
        let min_y: i32 = 0;
        let max_y: i32 = self.coords.1 .1 + 1;

        ((min_x, min_y), (max_x, max_y))
    }

    fn get_total_size_of_all_shapes(&self) -> ((i32, i32), (i32, i32)) {
        let mut min_x: i32 = 5000;
        let mut max_x: i32 = -5000;
        let mut min_y: i32 = 5000;
        let mut max_y: i32 = -5000;

        for shape in &self.shapes {
            let coords = shape.get_coords();
            let min = coords.0;
            let max = coords.1;
            if min.0 < min_x {
                min_x = min.0;
            }
            if max.0 > max_x {
                max_x = max.0;
            }
            if min.1 < min_y {
                min_y = min.1;
            }
            if max.1 > max_y {
                max_y = max.1;
            }
        }

        ((min_x, min_y), (max_x, max_y))
    }

    fn draw_state(&self) -> (((i32, i32), (i32, i32)), Vec<(i32, i32)>) {
        print!("\x1B[2J\x1B[1;1H");
        let coords = self.coords;

        println!("{:?}", coords);

        let all_points = self.points.clone();

        let mut blocks: HashSet<(i32, i32)> = HashSet::new();
        for p in &self.points {
            blocks.insert(*p);
        }
        let mut sand_blocks: HashSet<(i32, i32)> = HashSet::new();
        for p in &self.sand {
            sand_blocks.insert(*p);
        }

        let min_x: i32 = coords.0 .0 - 1;
        let max_x: i32 = coords.1 .0 + 1;
        let min_y: i32 = 0;
        let max_y: i32 = coords.1 .1 + 1;

        let mut s = "".to_string();

        for y in min_y - 3..=max_y {
            if y < min_y {
                s.push_str("   ");
            } else {
                s.push_str(format!("{:03}", y).as_str());
            }
            for x in min_x..=max_x {
                if x == 500 && y == 0 {
                    s.push_str("+");
                    continue;
                } else if y < min_y {
                    if y == min_y - 3 {
                        s.push_str(format!("{}", x.to_string().chars().nth(0).unwrap()).as_str());
                    } else if y == min_y - 2 {
                        s.push_str(format!("{}", x.to_string().chars().nth(1).unwrap()).as_str());
                    } else if y == min_y - 1 {
                        s.push_str(format!("{}", x.to_string().chars().nth(2).unwrap()).as_str());
                    }
                } else {
                    if blocks.contains(&(x, y)) {
                        s.push_str("#");
                        continue;
                    } else if sand_blocks.contains(&(x, y)) {
                        s.push_str("o");
                        continue;
                    }
                    s.push_str(".");
                }
            }
            s.push_str("\n");
        }

        println!("{}", s);

        thread::sleep(std::time::Duration::from_millis(50));
        (coords, all_points)
    }
}

#[derive(Debug, Clone)]
struct GrainOfSand {
    x: i32,
    y: i32,
}

impl GrainOfSand {
    fn new(x: i32, y: i32) -> GrainOfSand {
        GrainOfSand { x, y }
    }

    fn get_coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn move_down_left(&mut self) {
        self.y += 1;
        self.x -= 1;
    }

    fn move_down_right(&mut self) {
        self.y += 1;
        self.x -= 1;
    }
}

type Grain = GrainOfSand;

fn main() {
    println!("Advent of code 2022, Day 14 Part 1\n");
    let mut aoc = AOC::new();
    let filename = aoc.get_filename().unwrap();
    aoc.load_all_data(&filename);

    // println!("{:?}", aoc.data);

    let mut shapes: Vec<Shape> = vec![];

    for shape in aoc.data {
        // println!("Shape: {:?}", shape);
        let s = Shape::new(shape.clone());
        shapes.push(s);
    }

    let mut cave = Cave::new(shapes.clone());

    println!("Cave: {:?}", cave.coords);

    let mut floor_shape = Shape::new(vec![
        (500 - cave.coords.1 .1 - 30, cave.coords.1 .1 + 2),
        (500 + cave.coords.1 .1 + 30, cave.coords.1 .1 + 2),
    ]);

    cave.shapes.push(floor_shape);

    cave.coords = cave.get_total_size_of_all_shapes();
    cave.points = cave.get_all_points_in_cave();

    cave.draw_state();

    let total = cave.drop_sand_until_blocked();

    println!("Total: {}", total);
}
