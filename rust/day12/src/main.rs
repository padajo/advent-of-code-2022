use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct AOC {
    data: Vec<String>,
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
                    self.data.push(line);
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

fn lines_to_graph(
    lines: Vec<String>,
) -> (
    HashMap<(i32, i32), i32>,
    HashMap<(i32, i32), Vec<(i32, i32, i32)>>,
    (i32, i32),
    (i32, i32),
) {
    let mut graph: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    lines.iter().enumerate().for_each(|(ey, line)| {
        line.chars().enumerate().for_each(|(ex, c)| {
            let y = ey as i32;
            let x = ex as i32;
            let mut n = c as u32;
            if c == 'S' {
                start = (x, y);
                n = 1;
            } else if c == 'E' {
                end = (x, y);
                n = 26;
            } else {
                n = n - 'a' as u32 + 1;
            }
            grid.insert((x, y), n as i32);
        });
    });

    // now build the graph
    let grid_clone = grid.clone();
    for i in 0..grid.len() {
        let pos: (i32, i32) = grid.keys().nth(i).unwrap().clone();
        let height = *grid.get(&pos).unwrap();
        let x = pos.0;
        let y = pos.1;
        let mut edges: Vec<(i32, i32, i32)> = Vec::new();
        let moves: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for m in moves {
            let neighbour_x: i32 = x + m.0;
            let neighbour_y: i32 = y + m.1;
            let neighbour = grid_clone.get(&(neighbour_x, neighbour_y));
            if neighbour.is_some() {
                let neighbour_height = neighbour.unwrap();
                if neighbour_height < &(height + 2) {
                    // the edge exists and it can be moved to
                    edges.push((neighbour_x, neighbour_y, *neighbour_height));
                }
            }
        }
        graph.insert((x, y), edges);
    }

    (grid, graph, start, end)
}

/*
Declare a queue and insert the starting vertex.
Initialize a visited array and mark the starting vertex as visited.
Follow the below process till the queue becomes empty:
Remove the first vertex of the queue.
Mark that vertex as visited.
Insert all the unvisited neighbours of the vertex into the queue.

--> Additionally if you set the *level* for each node, then 
you only need to do the run through once, and you can get the
shortest path to the end node.
--> level is the number of steps taken to get to the node
*/


use std::thread;

fn bfs(
    graph: &HashMap<(i32, i32), Vec<(i32, i32, i32)>>,
    start: &(i32, i32),
    end: &(i32, i32),
    grid: &HashMap<(i32, i32), i32>,
) -> (Vec<((i32, i32), i32)>, i32, bool) {
    let mut q: VecDeque<((i32, i32), i32)> = VecDeque::new();
    let mut v: Vec<((i32, i32), i32)> = vec![];
    // make a quick visited list
    let mut visited: HashMap<String,bool> = HashMap::new();
    for (k,_) in grid {
        visited.insert(format!("{},{}",k.0,k.1), false);
    }

    let mut l: i32 = 0;

    q.push_back(((start.0, start.1), l));

    let mut end_l = 0;
    let mut end_reached = false;

    while !q.is_empty() {
        let (p, level) = q.pop_front().unwrap();
        l = level;
        // move the position to the next in the queue
        // check if visited again...
        let pos = format!("{},{}",p.0, p.1);
        let mut is_visited = visited.get(&pos).unwrap();
        if *is_visited {
            continue;
        }
        v.push(((p.0, p.1), l));
        // set the point as visited
        let point = visited.get_mut(&pos).unwrap();
        *point = true; // set the pointer to true
        if p.0 == end.0 && p.1 == end.1 {
            end_l = l;
            end_reached = true;
            break;
        }
        l += 1;
        let edges = graph.get(&p).unwrap();
        for e in edges {
            // if not visited, this will be false
            is_visited = visited.get(&format!("{},{}",e.0, e.1)).unwrap();
            // println!("{} {} {} {}", e.0, e.1, e.2, is_visited);
            // thread::sleep(std::time::Duration::from_millis(50));
            if !*is_visited {
                q.push_back(((e.0, e.1), l));
            }
        }
    }
    (v, end_l, end_reached)
}

fn main() {
    println!("Advent of code 2022, Day 12 Part 1\n");
    let mut aoc = AOC::new();
    let filename = aoc.get_filename().unwrap();
    aoc.load_all_data(&filename);

    // use breadth-first-search
    // just have to build the tree
    // https://programming-idioms.org/idiom/128/breadth-first-traversing-of-a-tree/2490/rust

    let (grid, graph, start, end) = lines_to_graph(aoc.data);

    // build a graph from the grid
    // https://www.geeksforgeeks.org/breadth-first-search-or-bfs-for-a-graph/
    // traversing a graph is different to traversing a tree


    let mut min_distance = 500;
    let mut a_points = vec![];
    for (k,v) in grid.clone() {
        if v == 1 {
            a_points.push((k.0, k.1));
        }
    }

    let mut data = vec![];
    for k in a_points {
        println!("{},{}", k.0, k.1);
        let (route, level, end_reached) = bfs(&graph, &k, &end, &grid);
        data.push((format!("{},{}", k.0,k.1), level, route));
        println!("{},{}: {} ({:?})", k.0, k.1, level, end_reached);
        if level < min_distance && end_reached {
            min_distance = level;
        }
    }

    println!("{:?}", data);

    println!("Min distance: {}", min_distance);

}
