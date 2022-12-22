use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
}

// given a set of sensor locations (Points), you want to know what points are not covered by any sensor
fn get_points_not_covered_by_sensors(
    sensors: &Vec<Sensor>,
    min_y: i64,
    max_y: i64,
    min_x_to_find: i64,
    max_x_to_find: i64,
) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();

    let mut sensor_data: HashMap<Point, i64> = HashMap::new();
    let mut sensor_locations: Vec<Point> = Vec::new();
    let mut beacon_data: HashMap<Point, Point> = HashMap::new();

    // println!("Getting sensor data... ({} sensors)", sensors.len());

    // avoid having to do multiple loops
    for sensor in sensors {
        let d = sensor.distance();
        sensor_data.insert(sensor.location, d);
        beacon_data.insert(sensor.location, sensor.beacon);
        sensor_locations.push(sensor.location.clone());
    }

    let sensor_locations_iterator = sensor_locations.iter();

    println!("Running...");

    for y in min_y..=max_y {
        if y % 10000 == 0 {
            println!("y: {}", y);
        }
        let mut ranges: Vec<(i64, i64)> = vec![];
        for sensor in sensor_locations_iterator.clone() {
            let d = sensor_data.get(sensor).unwrap();

            // work out the points on y that are covered by this sensor
            if sensor.y >= y - *d && sensor.y <= y + *d {
                // println!("\ny({}) is in range", y);
                // now work out how far away from the sensor we are
                let dist_to_sensor = (sensor.y - y).abs();
                let mut x_min = sensor.x - d + dist_to_sensor;
                let mut x_max = sensor.x + d - dist_to_sensor;
                // println!("{:?} - {}", sensor, d);
                // println!("x_min: {}, x_max: {}", x_min, x_max);
                if x_min < min_x_to_find {
                    x_min = min_x_to_find;
                }
                if x_max > max_x_to_find {
                    x_max = max_x_to_find;
                }
                ranges.push((x_min, x_max));
            } else {
                // println!("\ny({}) is NOT in range", y);
                continue;
            }
            // print_sensor_map(sensor, beacon_data.get(sensor).unwrap(), Some(y));
        }
        let range_for_y = get_uncovered_range(ranges.clone(), min_x_to_find, max_x_to_find);
        if range_for_y.is_some() {
            let range_for_y_data = range_for_y.unwrap();
            for r in range_for_y_data {
                if r.0 <= r.1 {
                    for x in r.0..=r.1 {
                        println!("x: {}", x);
                        points.insert(Point { x, y });
                    }
                }
            }
        }

        // given ranges... work out what ranges are still left within min_x and max_x
    }

    points
}

fn get_uncovered_range(ranges: Vec<(i64, i64)>, min_x: i64, max_x: i64) -> Option<Vec<(i64, i64)>> {
    let mut uncovered_ranges: Vec<(i64, i64)> = vec![];
    let mut x_min = min_x;
    let mut x_max = max_x;
    // sort by x_min
    let mut from_to_data = ranges.clone();
    // println!("from_to_data: {:?}", from_to_data);
    from_to_data.sort_by(|a, b| (a.1 - a.0).cmp(&(b.1 - b.0)));
    // println!("from_to_data: {:?}", from_to_data);
    from_to_data.reverse();
    // println!("from_to_data: {:?}", from_to_data);

    let mut new_from_to_data = consolidate_number_ranges(from_to_data, min_x, max_x);

    if new_from_to_data.len() == 1 {
        return None;
    }

    // sort by x
    new_from_to_data.sort_by(|a, b| (a.0).cmp(&(b.0)));

    let mut gaps = vec![];

    let mut last_range: (i64, i64) = new_from_to_data[0];
    // now get the gaps
    for r in 1..new_from_to_data.len() {
        gaps.push((last_range.1 + 1, new_from_to_data[r].0 - 1));
        last_range = new_from_to_data[r];
    }

    Some(gaps)
}

fn consolidate_number_ranges(ranges: Vec<(i64, i64)>, min_x: i64, max_x: i64) -> Vec<(i64, i64)> {
    let mut ranges_to_return = vec![];
    let mut ranges_to_loop = ranges.clone();
    let mut is_updated = false;

    loop {
        let mut new_ranges: HashSet<(i64, i64)> = HashSet::new();
        for r in &ranges_to_loop {
            new_ranges.insert(*r);
        }
        let num = ranges_to_loop.len();

        // println!("new ranges: {:?}", new_ranges);

        is_updated = false;

        for i in 0..num {
            let mut a = ranges_to_loop[i];
            for j in 0..num {
                if i == j || a.1 < min_x || a.0 > max_x {
                    continue;
                }
                if a.0 < min_x {
                    a.0 = min_x;
                }
                if a.1 > max_x {
                    a.1 = max_x;
                }
                let mut b = ranges_to_loop[j];
                if b.1 < min_x || b.0 > max_x {
                    continue;
                }
                if b.0 < min_x {
                    b.0 = min_x;
                }
                if b.1 > max_x {
                    b.1 = max_x;
                }
                if (a.0 == min_x && a.1 == max_x) || (b.0 == min_x && b.1 == max_x) {
                    // return the whole range because we've finished
                    return vec![(min_x, max_x)];
                }
                if number_range_inside(&a, &b) {
                    // println!("\t\t\ta WHOLLY INSIDE b - remove a");
                    // wholly_inside.insert(j);
                    new_ranges.insert(b);
                    new_ranges.remove(&a);
                    is_updated = true;
                } else if number_range_inside(&b, &a) {
                    // println!("\t\t\tb WHOLLY INSIDE a - remove b");
                    // println!("\t\t\tChange to {},{}", b1, a1 - 1);
                    // println!("
                    new_ranges.insert(a);
                    new_ranges.remove(&b);
                    is_updated = true;
                } else if number_ranges_overlap(&a, &b) {
                    // println!("\t\t\tnumber ranges overlap {:?} {:?} - join them", a, b);
                    let new_range = join_number_ranges(&a, &b);
                    new_ranges.remove(&a);
                    new_ranges.remove(&b);
                    let new_range = (new_range.0, new_range.1);
                    if (new_range.0 == min_x && new_range.1 == max_x) {
                        // return the whole range because we've finished
                        return vec![(min_x, max_x)];
                    }
                    // println!("\t\t\tnew range: {:?}", new_range);
                    new_ranges.insert(new_range);
                    is_updated = true;
                } else if (b.0 - a.1).abs() == 1 {
                    // if they are 1 apart, join them
                    // println!("\t\t\tnumber ranges one apart {:?} {:?} - join them", a, b);
                    if (a.0 == min_x && b.1 == max_x) {
                        // return the whole range because we've finished
                        return vec![(min_x, max_x)];
                    }
                    new_ranges.remove(&a);
                    new_ranges.remove(&b);
                    new_ranges.insert((a.0, b.1));
                    is_updated = true;
                } else if (a.0 - b.1).abs() == 1 {
                    // println!("\t\t\tnumber ranges one apart {:?} {:?} - join them", b, a);
                    if (b.0 == min_x && a.1 == max_x) {
                        // return the whole range because we've finished
                        return vec![(min_x, max_x)];
                    }
                    new_ranges.remove(&a);
                    new_ranges.remove(&b);
                    new_ranges.insert((a.0, b.1));
                    is_updated = true;
                } else {
                    // just leave both ranges where they are
                }
            }
        }

        // println!("new ranges (updated): {:?}", new_ranges);

        ranges_to_return.clear();

        for r in new_ranges.clone() {
            ranges_to_return.push((r.0, r.1));
        }

        // keep looping until we have a stable set of ranges
        if !is_updated {
            // println!("STOP LOOPING");
            break;
        } else {
            // println!("CONTINUE LOOPING");
            ranges_to_loop = ranges_to_return.clone();
        }
    }

    ranges_to_return
}

fn number_range_inside(r1: &(i64, i64), r2: &(i64, i64)) -> bool {
    let (r1_min, r1_max) = r1;
    let (r2_min, r2_max) = r2;
    if r1_min >= r2_min && r1_max <= r2_max {
        true
    } else {
        false
    }
}

fn number_ranges_overlap(r1: &(i64, i64), r2: &(i64, i64)) -> bool {
    let (r1_min, r1_max) = r1;
    let (r2_min, r2_max) = r2;
    if r1_min <= r2_min && r1_max >= r2_min {
        true
    } else if r2_min <= r1_min && r2_max >= r1_min {
        true
    } else {
        false
    }
}

fn join_number_ranges(r1: &(i64, i64), r2: &(i64, i64)) -> (i64, i64) {
    let (r1_min, r1_max) = r1;
    let (r2_min, r2_max) = r2;
    let min = if r1_min < r2_min { r1_min } else { r2_min };
    let max = if r1_max > r2_max { r1_max } else { r2_max };
    (*min, *max)
}

fn print_sensor_map(sensor_ref: &Point, beacon_ref: &Point, y_to_find: Option<i64>) {
    let sensor = Sensor {
        location: sensor_ref.clone(),
        beacon: beacon_ref.clone(),
    };

    let points = sensor.get_points_within_manhattan_distance();

    for y in 0..=20 {
        print!("{:02} ", y);
        for x in 0..=20 {
            let p = Point { x: x, y: y };
            if sensor.location.x == x && sensor.location.y == y {
                print!("S");
            } else if sensor.beacon.x == x && sensor.beacon.y == y {
                print!("B");
            } else if points.contains(&p) {
                print!("#");
            } else if y_to_find.is_some() && y == y_to_find.unwrap() {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn get_tuning_frequency(x: i64, y: i64) -> i128 {
    ((x * 4000000) + y) as i128
}

fn main() {
    println!("Advent of code 2022, Day 15\n");
    let mut aoc = AOC::new();
    let filename = aoc.get_filename().unwrap();
    aoc.load_all_data(&filename);

    println!("data rows: {:?}", aoc.data.len());

    let mut sensor_locations: HashSet<Point> = HashSet::new();
    let mut beacon_locations: HashSet<Point> = HashSet::new();
    let mut points: HashSet<Point> = HashSet::new();

    for s in &aoc.data {
        sensor_locations.insert(s.location.clone());
        beacon_locations.insert(s.beacon.clone());
        // let no_beacon_points = s.get_points_within_manhattan_distance();
        // for p in no_beacon_points {
        //     points.insert(p);
        // }
    }

    println!("sensor locations: {:?}", sensor_locations);
    println!("beacon locations: {:?}", beacon_locations);

    let min_y = 0;
    let mut max_y = 20;
    let min_x = 0;
    let mut max_x = 20;

    if filename.eq("input.txt") {
        max_y = 4000000;
        max_x = 4000000;
    }

    let data = get_points_not_covered_by_sensors(&aoc.data, min_y, max_y, min_x, max_x);

    println!("data: {:?}", data);

    let p = data.iter().nth(0).unwrap();

    println!("tuning frequency: {:?}", get_tuning_frequency(p.x, p.y));

}
