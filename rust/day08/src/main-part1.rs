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

fn main() {
    println!("Advent of code 2022, Day 8 Part 1\n");

    let mut total_trees_visible: u32 = 0;

    let mut v_grid: Vec<Vec<i16>> = Vec::new();
    let mut v_grid_initialised = false;
    let mut h: i16 = 0;
    // trees visible will hold the trees in the horizontal (east-west)
    let mut trees_visible: Vec<Vec<i8>> = Vec::new();

    // if let Ok(lines) = read_lines("./test-grid.txt") {
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
                if !v_grid_initialised {
                    v_grid.push(Vec::new());
                }
                let v_vec_ref: &mut Vec<i16> = &mut v_grid[i];
                let num: i16 = c.to_string().parse::<i16>().unwrap();
                tree_heights_in_grid.push(num);
                v_vec_ref.push(num);
            }
            if !v_grid_initialised {
                v_grid_initialised = true;
            }

            let visible_trees = get_visible_trees_both_ways(&mut tree_heights_in_grid, &h);
            trees_visible.push(visible_trees);
            h += 1;
        }
    }

    // now we've looped over the lines, loop over the grid
    // (which is the passed in lines as a grid, rotated by 90 degrees)
    // println!("VERTICAL");
    let _v: i16;
    for _v in 0..v_grid.len() {
        let column: &mut Vec<i16> = &mut v_grid[_v];
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

    for trees in trees_visible {
        println!("{}", join_visible_trees(&trees));
        total_trees_visible += trees.clone().into_iter().sum::<i8>() as u32;
    }

    println!("Total trees visible: {}", total_trees_visible);
}
