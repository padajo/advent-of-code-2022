use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::thread;

struct AOC {
    data: Vec<Sensor>,
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
                    // example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
                    let data_split: Vec<&str> = line.split(":").collect();
                    let mut sensor_data: Vec<&str> = data_split[0].split(",").collect();
                    let mut beacon_data: Vec<&str> = data_split[1].split(",").collect();
                    // get the bit after the "=" for each part
                    for i in 0..sensor_data.len() {
                        sensor_data[i] = sensor_data[i].split("=").collect::<Vec<&str>>()[1];
                    }
                    for i in 0..beacon_data.len() {
                        beacon_data[i] = beacon_data[i].split("=").collect::<Vec<&str>>()[1];
                    }

                    self.data.push(Sensor {
                        location: Point {
                            x: sensor_data[0].trim().parse::<i64>().unwrap(),
                            y: sensor_data[1].trim().parse::<i64>().unwrap(),
                        },
                        beacon: Point {
                            x: beacon_data[0].trim().parse::<i64>().unwrap(),
                            y: beacon_data[1].trim().parse::<i64>().unwrap(),
                        },
                    });
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Sensor {
    location: Point,
    beacon: Point,
}

impl Sensor {
    // using manhattan distance
    fn distance(&self) -> i64 {
        let x = self.location.x - self.beacon.x;
        let y = self.location.y - self.beacon.y;
        x.abs() + y.abs()
    }

    fn get_points_within_manhattan_distance(&self) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        let d = self.distance();
        for y in -d..=d {
            for x in -d..=d {
                if x.abs() + y.abs() <= d {
                    points.insert(Point {
                        x: self.location.x + x,
                        y: self.location.y + y,
                    });
                }
            }
        }
        points
    }

    fn get_num_of_points_on_y(&self, y: i64) -> (i64, Option<(i64, i64)>) {
        let d = self.distance();
        let dist_to_y = (self.location.y - y).abs();

        if y >= self.location.y - d && y <= self.location.y + d {
            // println!("y is in range");
        } else {
            // println!("y is not in range");
            return (0, None);
        }
        // this is distance times 2 from the sensor + 1 for the sensor point
        // then take away 1 for the sensor point
        // then every 1 further away from the beacon is 2 less
        let mut total = 0;
        let mut x_from = self.location.x - d;
        let mut x_to = self.location.x + d;
        if y == self.location.y {
            total = d * 2; // no + 1 here for the sensor point
        } else {
            total = d * 2 - (2 * dist_to_y) + 1; // e.g. if d = 5 and dist_to_y = 2, then 5 * 2 - (2 * 2) + 1 = 9
            x_from += dist_to_y;
            x_to -= dist_to_y;
        }
        if self.beacon.y == y {
            total -= 1;
        }
        // println!("total: {}", total);
        (total, Some((x_from, x_to)))
    }

    fn get_points_within_manhattan_distance_on_y(&self, y: i64) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();
        let d = self.distance();
        let y_diff = (y - self.location.y).abs();
        for x in -d + y_diff..=d - y_diff {
            points.insert(Point {
                x: self.location.x + x,
                y: y,
            });
        }
        points
    }
}

fn main() {
    println!("Advent of code 2022, Day 15\n");
    let mut aoc = AOC::new();
    let filename = aoc.get_filename().unwrap();
    aoc.load_all_data(&filename);

    println!("data rows: {:?}", aoc.data.len());

    let mut all_points: HashSet<(i64, i64)> = HashSet::new();
    let mut all_points: HashSet<Point> = HashSet::new();
    let mut sensor_locations: HashSet<Point> = HashSet::new();
    let mut beacon_locations: HashSet<Point> = HashSet::new();

    let mut from_to_data: Vec<(i64, i64)> = Vec::new();

    let mut num_of_points_on_y = 0;

    let mut min_x = 50000000;
    let mut max_x = -50000000;
    let mut min_y = 50000000;
    let mut max_y = -50000000;

    let y_to_find = 2000000;

    // let mut y_to_find = 10;
    let mut sensor_num_on_y = 0;
    let mut beacon_num_on_y = 0;

    for sensor in &aoc.data {
        if !sensor_locations.contains(&sensor.location) {
            sensor_locations.insert(sensor.location.clone());
            if sensor.location.y == y_to_find {
                sensor_num_on_y += 1;
            }
        }
        if !beacon_locations.contains(&sensor.beacon) {
            beacon_locations.insert(sensor.beacon.clone());
            if sensor.beacon.y == y_to_find {
                beacon_num_on_y += 1;
            }
        }
        let d = sensor.distance();

        // println!("sensor y: {}", sensor.location.y);
        // println!("d: {}", d);
        // println!("sensor.location.y + d: {}", sensor.location.y + d);
        // println!("sensor.location.y - d: {}", sensor.location.y - d);
        // println!("y_to_find            : {}", y_to_find);
        if sensor.location.y + d >= y_to_find && sensor.location.y - d <= y_to_find {
            // println!("sensor y is in range");
        } else {
            // println!("sensor y is not in range");
            continue;
        }

        let (n, from_to) = sensor.get_num_of_points_on_y(y_to_find);
        num_of_points_on_y += n;

        if from_to.is_some() {
            let (x_from, x_to) = from_to.unwrap();
            from_to_data.push((x_from, x_to));
        }
    }

    // sort longest to shortest
    from_to_data.sort_by(|a, b| (a.1 - a.0).cmp(&(b.1 - b.0)));
    from_to_data.reverse();

    let from_to_len = from_to_data.len();

    let mut wholly_inside: HashSet<usize> = HashSet::new();

    // change the data so that it removes the overlaps (and ignores any wholly inside)
    for i in 0..from_to_len {
        let (a1, a2) = from_to_data[i];
        // println!("\n\n{} (a1,a2) {:?}", i, (a1, a2));
        // let mut add_to_new = true;
        for j in i + 1..from_to_len {
            if i == j {
                continue;
            }
            // check if b is contained in a
            let (b1, b2) = from_to_data[j];

            // println!(
            //     "\t\tchecking {} (a1,a2) {:?} for overlaps with {} (b1,b2) {:?}",
            //     i,
            //     (a1, a2),
            //     j,
            //     (b1, b2)
            // );

            if wholly_inside.contains(&j) {
                continue;
            } else if b1 >= a1 && b2 <= a2 {
                // println!("WHOLLY INSIDE");
                wholly_inside.insert(j);
            } else if b2 < a1 {
                // println!("Wholly to the left - no change");
            } else if b1 > a2 {
                // println!("Wholly to the right - no change");
            } else if b1 < a1 && b2 >= a1 {
                // println!("Overlaps to the left");
                // println!("Change to {},{}", b1, a1 - 1);
                let mut new_data = from_to_data.get_mut(j).unwrap();
                new_data.0 = b1;
                new_data.1 = a1 - 1;
            } else if b1 <= a2 && b2 > a2 {
                // println!("Overlaps to the right");
                // println!("Change to {},{}", a2 + 1, b2);
                let mut new_data = from_to_data.get_mut(j).unwrap();
                new_data.0 = a2 + 1;
                new_data.1 = b2;
            }
        }
    }

    num_of_points_on_y = 0;

    println!("\n\nLooping over the data");
    // for (a1, a2) in from_to_data {
    for i in 0..from_to_data.len() {
        let (a1, a2) = from_to_data[i];
        println!("({},{})", a1, a2);
        if wholly_inside.contains(&i) {
            println!("wholly inside, skipping");
            continue;
        }
        println!("({},{}) -> {} - {} + 1 = {}", a1, a2, a2, a1, a2 - a1 + 1);
        num_of_points_on_y += a2 - a1 + 1;
    }

    println!("\n\nnum on y: {:?}", num_of_points_on_y);
    println!("sensors on y: {:?}", sensor_num_on_y);
    println!("beacons on y: {:?}", beacon_num_on_y);
    println!(
        "total points on y: {:?}",
        num_of_points_on_y - sensor_num_on_y - beacon_num_on_y
    );
}
