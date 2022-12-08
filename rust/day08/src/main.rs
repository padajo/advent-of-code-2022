use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_visible_trees(tree_heights: &mut Vec<i16>) -> Vec<u8> {
    let mut current_visible_height: i16 = -1; // start below 0
    let mut visible_grid: Vec<u8> = Vec::new();
    for tree_height in tree_heights {
        if *tree_height > current_visible_height {
            // set the current visible height to this height
            current_visible_height = tree_height.clone();
            // increment the number of visible trees
            visible_grid.push(1);
        } else {
            visible_grid.push(0);
        }
    }
    visible_grid
}

fn get_visible_trees_both_ways(trees: &mut Vec<i16>, _h: &i16) -> Vec<i8> {
    let west_visible = get_visible_trees(trees);
    trees.reverse();
    let mut east_visible = get_visible_trees(trees);
    let mut visible_trees: Vec<i8> = Vec::new();
    east_visible.reverse();
    for i in 0..trees.len() {
        if west_visible[i] == 1 || east_visible[i] == 1 {
            visible_trees.push(1);
        } else {
            visible_trees.push(0);
        }
    }
    trees.reverse(); // just for completeness so we haven't just left it reversed

    visible_trees
}

// a function that takes a vec of i8 of values and joins them and returns a string
// and each value that is a 1 is returned as a T and anything else is a .
fn join_visible_trees(visible_trees: &Vec<i8>) -> String {
    let mut visible_trees_str: String = String::new();
    for tree in visible_trees {
        if *tree == 1 {
            visible_trees_str.push('T');
        } else {
            visible_trees_str.push('.');
        }
    }
    visible_trees_str
}

fn get_scenic_score(tree_data: &[i16], start_height: &i16) -> i32 {
    let mut score: i32 = 0;

    // this is the number of trees the current tree can "see"
    // the tree_data is the height of the trees leading away from the
    // tree in whatever direction is required
    for i in 0..tree_data.len() {
        score += 1;
        // if we hit a tree that is greater than or equal to the current
        // start_height then we can't see any more trees
        if tree_data[i] >= *start_height {
            break;
        }
    }

    score
}

fn get_tree_data_sets(data: &Vec<i16>, pos: usize) -> (Vec<i16>, Vec<i16>) {
    // take in a list of tree heights and a position
    // and return two sets from the position
    // that are the tree heights moving away from that position
    let mut reverse_set = data.clone();
    let forward_set = reverse_set.split_off(pos + 1);
    if reverse_set.len() > 0 {
        reverse_set.pop();
    }
    reverse_set.reverse();
    (reverse_set, forward_set)
}

fn get_row_as_string(row: &Vec<i16>) -> String {
    row.clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    println!("Advent of code 2022, Day 8 Part 2\n");

    // let mut total_trees_visible: u32 = 0;

    let mut tree_grid_h: Vec<Vec<i16>> = Vec::new();
    let mut tree_grid_v: Vec<Vec<i16>> = Vec::new();
    let mut v_grid_initialised = false;
    let mut h: i16 = 0;
    // trees visible will hold the trees in the horizontal (east-west)
    let mut trees_visible: Vec<Vec<i8>> = Vec::new();

    // if let Ok(lines) = read_lines("./test-grid.txt") {
    // if let Ok(lines) = read_lines("./test-grid-part2.txt") {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            // get the tree heights
            let char_tree_heights_in_grid: Vec<char> = Ok::<String, String>(line.unwrap())
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            let mut tree_heights_in_grid: Vec<i16> = Vec::new();
            for i in 0..char_tree_heights_in_grid.len() {
                // for c in char_tree_heights_in_grid {
                let c: char = char_tree_heights_in_grid[i];
                // this only runs on the first loop
                if !v_grid_initialised {
                    tree_grid_v.push(Vec::new());
                }
                let v_vec_ref: &mut Vec<i16> = &mut tree_grid_v[i];
                let num: i16 = c.to_string().parse::<i16>().unwrap();
                tree_heights_in_grid.push(num);
                v_vec_ref.push(num);
            }
            if !v_grid_initialised {
                v_grid_initialised = true;
            }

            let visible_trees = get_visible_trees_both_ways(&mut tree_heights_in_grid, &h);
            tree_grid_h.push(tree_heights_in_grid.clone());
            trees_visible.push(visible_trees);
            h += 1;
        }
    }

    // now we've looped over the lines, loop over the grid
    // (which is the passed in lines as a grid, rotated by 90 degrees)
    // println!("VERTICAL");
    let _v: i16;
    for _v in 0..tree_grid_v.len() {
        let column: &mut Vec<i16> = &mut tree_grid_v[_v];
        let num: i16 = _v as i16;
        let mut visible_trees_v = get_visible_trees_both_ways(&mut column.clone(), &num);

        for h in 0..trees_visible.len() {
            let row: &mut Vec<i8> = trees_visible.get_mut(h).unwrap();
            let is_visible_vertically: &i8 = visible_trees_v.get_mut(h).unwrap();
            if *is_visible_vertically > 0 {
                row[_v] = 1;
            }
        }
    }

    // for trees in trees_visible {
    //     // println!("{}", join_visible_trees(&trees));
    //     total_trees_visible += trees.clone().into_iter().sum::<i8>() as u32;
    // }

    // println!("Total trees visible: {}", total_trees_visible);

    println!("\n--- Part 2 ---\n");

    let v_size = tree_grid_h.len();
    let h_size = tree_grid_v.len();

    let mut north_score: i32 = 0;
    let mut south_score: i32 = 0;
    let mut east_score: i32 = 0;
    let mut west_score: i32 = 0;

    let print_full: bool = false;

    let mut max_scenic_score: u32 = 0;
    let mut max_point: (usize, usize) = (0, 0);

    for h in 0..h_size {
        for v in 0..v_size {
            let tree_height = tree_grid_h[h][v];
            if print_full {
                println!("h: {}, v: {}", h, v);
                let mut s: String = format!("******\nTree Height: {}\n\n", tree_height);
                for inside_h in 0..h_size {
                    let mut data = tree_grid_h[inside_h].clone();
                    if inside_h != h {
                        s.push_str(&get_row_as_string(&data));
                    } else {
                        let data_end = data.split_off(v + 1);
                        data.pop();
                        s.push_str(&get_row_as_string(&data));
                        s.push_str(".");
                        s.push_str(&get_row_as_string(&data_end));
                    }
                    s.push_str("\n");
                }
                println!("{}", s);
            }

            // generate north and south tree data
            let (north_tree_data, south_tree_data) = get_tree_data_sets(&tree_grid_v[v], h);
            north_score = get_scenic_score(&north_tree_data, &tree_height);
            south_score = get_scenic_score(&south_tree_data, &tree_height);
            if print_full {
                println!(
                    "    North score: {:>3} {}-{}",
                    north_score,
                    tree_height,
                    get_row_as_string(&north_tree_data)
                );
                println!(
                    "    South score: {:>3} {}-{}",
                    south_score,
                    tree_height,
                    get_row_as_string(&south_tree_data)
                );
            }

            // generate east and west tree data
            let (west_tree_data, east_tree_data) = get_tree_data_sets(&tree_grid_h[h], v);
            east_score = get_scenic_score(&east_tree_data, &tree_height);
            west_score = get_scenic_score(&west_tree_data, &tree_height);
            if print_full {
                println!(
                    "     East score: {:>3} {}-{}",
                    east_score,
                    tree_height,
                    get_row_as_string(&east_tree_data)
                );
                println!(
                    "     West score: {:>3} {}-{}",
                    west_score,
                    tree_height,
                    get_row_as_string(&west_tree_data)
                );
            }

            let point_scenic_score: u32 =
                (north_score * south_score * east_score * west_score) as u32;

            if print_full {
                println!("Scenic Score ({},{}): {:>3}", h, v, point_scenic_score);
            }

            if point_scenic_score > max_scenic_score {
                println!(
                    "New max scenic score: {} ({},{}) N {} * E {} * S {} * W {}",
                    point_scenic_score, h, v, north_score, east_score, south_score, west_score
                );
                max_scenic_score = point_scenic_score;
                max_point = (h, v);
            }
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
    println!("Max point: ({}, {})", max_point.0, max_point.1);
}
