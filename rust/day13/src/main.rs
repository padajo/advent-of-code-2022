use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct AOC {
    data: Vec<(String, String)>,
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
        let mut left_right: Vec<String> = Vec::new();
        if let Ok(lines) = self.read_lines(filename) {
            for l in lines {
                if let Ok(line) = l {
                    // println!("Line: {}", line);
                    if line.len() == 0 {
                        continue;
                    } else {
                        left_right.push(line);
                        if left_right.len() == 2 {
                            // add the left and right to the data
                            self.data
                                .push((left_right[0].clone(), left_right[1].clone()));
                            // remove all the data from left_right
                            left_right.clear();
                        }
                    }
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

// https://rust-unofficial.github.io/too-many-lists/second-option.html

#[derive(Debug, Clone)]
struct DataItem {
    s: String,            // the original string
    index: usize,         // the index of this item in the data store
    parent: DParent,      // if root, then no parent
    children: Vec<usize>, // if empty, then no children
    int: DInt,            // if it's just a value, then this is the value
}

// yay type aliases!
type DParent = Option<Box<usize>>;
type DInt = Option<Box<i32>>;
// type DList = Option<Box<Vec<usize>>>;

#[derive(Debug)]
struct DataParseError {}

type DataParseResult<T> = Result<T, DataParseError>;

enum DataCompare {
    RightOrder,
    WrongOrder,
    ContinueComparing,
}

type DataComparisonResult = DataCompare;

impl DataItem {
    fn new(s: String) -> DataItem {
        DataItem {
            s: s,
            index: 0,
            parent: None,
            children: vec![],
            int: None,
        }
    }

    fn get_index(&self) -> usize {
        self.index.clone()
    }

    fn get_integer(&self) -> Option<i32> {
        if self.int.is_some() {
            return Some(*self.int.as_ref().unwrap().clone());
        }
        None
    }

    fn get_parent(&self) -> Option<usize> {
        if self.parent.is_some() {
            return Some(*self.parent.as_ref().unwrap().clone());
        }
        None
    }

    fn set_parent(&mut self, p: usize) {
        self.parent = Some(Box::new(p));
    }

    fn has_children(&self) -> bool {
        !self.is_integer()
        //self.children.len() > 0
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }

    fn is_integer(&self) -> bool {
        self.int.is_some()
    }

    fn set_int(&mut self, i: i32) {
        self.int = Some(Box::new(i));
    }
}

#[derive(Debug)]
struct DataStore {
    left_data: Vec<DataItem>,  // all data items for the left side
    right_data: Vec<DataItem>, // all data items for the right side
    original_string_left: String,
    original_string_right: String,
}

impl DataStore {
    fn new(l: String, r: String) -> DataStore {
        DataStore {
            left_data: Vec::new(),
            right_data: Vec::new(),
            original_string_left: l.clone(), // make sure we clone these
            original_string_right: r.clone(), // make sure we clone these
        }
    }

    // fn is_in_right_order(&mut self) -> bool {
    //     let right_order = self.compare(0, 0, 0);
    //     match right_order {
    //         Some(DataCompare::RightOrder) => {
    //             println!("Right order");
    //             return true;
    //         }
    //         Some(DataCompare::WrongOrder) => {
    //             println!("Wrong order");
    //             return false;
    //         }
    //         Some(DataCompare::ContinueComparing) => {
    //             println!("Continue comparing");
    //             return false;
    //         }
    //         None => {
    //             println!("None");
    //             return false;
    //         }
    //     }
    // }

    fn compare(&mut self, l: usize, r: usize, depth: i32) -> Option<DataComparisonResult> {
        // this is recursive
        // start at l=0 and r=0 and work through the data
        let l_item = &self.left_data.get(l).unwrap();
        let r_item = &self.right_data.get(r).unwrap();

        if l_item.is_integer() && r_item.is_integer() {
            let left_int = l_item.get_integer().unwrap();
            let right_int = r_item.get_integer().unwrap();
            return self.compare_integers(&left_int, &right_int, depth + 1);
        }

        if l_item.has_children() && r_item.has_children() {
            // get items...
            return self.compare_lists(l, r, depth + 1);
        }

        // println!("We get here...");
        // println!("l_item: {:?}", l_item);
        // println!("r_item: {:?}", r_item);
        if l_item.has_children() && r_item.is_integer() {
            // println!("Mixed Types... left is list, right is integer... fixing");
            // create a new data item so that the r_item as a child and the new item has a list of 1
            let num = r_item.get_integer().unwrap();
            let mut new_data_item = DataItem::new(format!("[{}]", num.to_owned()));
            new_data_item.set_int(num.clone());
            let parent = r_item.get_parent();
            if parent.is_some() {
                new_data_item.set_parent(parent.unwrap());
            }
            new_data_item.index = self.right_data.len();
            new_data_item.children.push(r_item.get_index());
            // push the data item to the right data
            self.right_data.push(new_data_item);
            // now do a new compare with this new item as it is a list that can be used for a comparison
            return self.compare_lists(l, self.right_data.len() - 1, depth + 1);
        } else if r_item.has_children() && l_item.is_integer() {
            // println!("Mixed Types... right is list, left is integer... fixing");
            // create a new data item so that the l_item as a child and the new item has a list of 1
            let num = l_item.get_integer().unwrap();
            let mut new_data_item = DataItem::new(format!("[{}]", num.to_owned()));
            new_data_item.set_int(num.clone());
            let parent = l_item.get_parent();
            if parent.is_some() {
                new_data_item.set_parent(parent.unwrap());
            }
            new_data_item.index = self.left_data.len();
            new_data_item.children.push(l_item.get_index());
            // push the data item to the right data
            self.left_data.push(new_data_item);
            // now do a new compare with this new item as it is a list that can be used for a comparison
            return self.compare_lists(self.left_data.len() - 1, r, depth + 1);
        }

        Some(DataCompare::ContinueComparing)
    }

    fn compare_lists(
        &mut self,
        left_list: usize,
        right_list: usize,
        depth: i32,
    ) -> Option<DataComparisonResult> {
        let l_item = self.left_data.get(left_list).unwrap().clone();
        let r_item = self.right_data.get(right_list).unwrap().clone();

        // println!("\t - Compare {:?} vs {:?}", l_item.s, r_item.s);

        let left_children_indexes = &l_item.children;
        let right_children_indexes = &r_item.children;

        if left_children_indexes.len() == 0 && right_children_indexes.len() > 0 {
            // println!("left list is empty and right list is not, so has to be right order");
            return Some(DataCompare::RightOrder);
        }

        let mut i = left_children_indexes.len();
        if right_children_indexes.len() > i {
            i = r_item.child_count();
        }
        // loop up to max child count
        for j in 0..i {
            let l = l_item.clone();
            let r = r_item.clone();
            let l_child_index = l.children.get(j).clone();
            let r_child_index = r.children.get(j).clone();
            if l_child_index.is_none() && r_child_index.is_some() {
                // println!("\t\tleft list ran out");
                // If the left list runs out of items first, the inputs are in the right order
                return Some(DataCompare::RightOrder);
            } else if r_child_index.is_none() && l_child_index.clone().is_some() {
                // println!("\t\tright list ran out");
                // If the right list runs out of items first, the inputs are not in the right order.
                return Some(DataCompare::WrongOrder);
            } else if l_child_index.is_some() && r_child_index.is_some() {
                let lci = l_child_index.unwrap();
                let rci = r_child_index.unwrap();
                match self.compare(*lci, *rci, depth + 1) {
                    Some(DataCompare::ContinueComparing) => {
                        // continue comparing
                    }
                    Some(DataCompare::RightOrder) => {
                        // right order
                        return Some(DataCompare::RightOrder);
                    }
                    Some(DataCompare::WrongOrder) => {
                        // wrong order
                        return Some(DataCompare::WrongOrder);
                    }
                    None => {
                        // continue comparing
                    }
                }
            }
        }

        Some(DataCompare::ContinueComparing)
    }

    fn compare_integers(
        &mut self,
        left_int: &i32,
        right_int: &i32,
        _depth: i32,
    ) -> Option<DataComparisonResult> {
        // println!("\t - Compare {} vs {}", left_int, right_int);
        if left_int == right_int {
            // println!("\t\t - continue comparing");
            return Some(DataCompare::ContinueComparing);
        } else if left_int < right_int {
            // println!("\t\t - right order");
            return Some(DataCompare::RightOrder);
        } else {
            // println!("\t\t - wrong order");
            return Some(DataCompare::WrongOrder);
        }
    }

    fn parse(&mut self) {
        // println!(
        //     "\n\nParsing {} and {}",
        //     self.original_string_left, self.original_string_right
        // );
        // take left and right and parse them
        let l = self.parse_string(true, &mut self.original_string_left.clone(), None);
        if l.is_err() {
            println!("Error parsing left string");
        }

        let r = self.parse_string(false, &mut self.original_string_right.clone(), None);
        if r.is_err() {
            println!("Error parsing right string");
        }
    }

    // return a list of Strings for the specific items
    fn parse_string(
        &mut self,
        is_left: bool,
        s: &mut String,
        parent: Option<usize>,
    ) -> DataParseResult<usize> {
        // println!("parse_string: {:?}", s);
        // create a data item
        let mut data_item = DataItem::new(s.clone());
        if parent.is_some() {
            data_item.parent = Some(Box::new(parent.unwrap()));
        }
        let item_index;
        if is_left {
            item_index = self.left_data.len();
            data_item.index = item_index.clone();
            self.left_data.push(data_item.clone());
        } else {
            item_index = self.right_data.len();
            data_item.index = item_index.clone();
            self.right_data.push(data_item.clone());
        }

        // now we have a data_item... we can add it to the
        if DataStore::is_list(&s) {
            let mut items: Vec<String> = Vec::new();
            // now parse the list
            // get each of the parts as a string
            // get a DataItem for each
            let mut data_string: &mut String = s;
            DataStore::strip_list_brackets(&mut data_string);
            let mut bracket_count = 0;
            let mut item_string = String::new();

            for _c in data_string.chars() {
                if _c == '[' {
                    //     // start of a list
                    bracket_count += 1;
                    //     brackets_found = true;
                    item_string.push(_c);
                } else if _c == ']' {
                    //     // end of a list
                    bracket_count -= 1;
                    item_string.push(_c);
                } else if _c == ',' {
                    if bracket_count == 0 {
                        items.push(item_string.clone());
                        item_string.clear();
                    } else {
                        item_string.push(_c);
                    }
                } else {
                    item_string.push(_c);
                }
            }
            // catch the case where there is zero string length
            if item_string.len() > 0 {
                // data_items.add(DataStore::parse_item(&item_string));
                items.push(item_string.clone());
                item_string.clear();
            }

            // now we have items!!!!
            for item in items.iter() {
                let ii = Some(item_index).as_ref().cloned();
                let _new_item_index = self.parse_string(is_left, &mut item.clone(), ii);
                // set children in parent
                let index = _new_item_index.unwrap();
                if is_left {
                    self.left_data
                        .get_mut(ii.unwrap())
                        .unwrap()
                        .children
                        .push(index);
                } else {
                    self.right_data
                        .get_mut(ii.unwrap())
                        .unwrap()
                        .children
                        .push(index);
                }
            }
        } else {
            if s.len() > 0 {
                let num_result = s.parse::<i32>();
                if num_result.is_ok() {
                    let num = num_result.unwrap();
                    if is_left {
                        self.left_data[item_index].set_int(num);
                    } else {
                        self.right_data[item_index].set_int(num);
                    }
                }
            }
        }

        Ok(item_index)
    }

    fn strip_list_brackets(data: &mut String) {
        if DataStore::is_list(data) {
            data.remove(0);
            data.pop();
        }
    }

    fn is_list(data: &String) -> bool {
        data.starts_with("[") && data.ends_with("]")
    }
}

use std::cmp::*;
fn compare_packets(a: &String, b: &String) -> Ordering {
    // println!("compare_packets: {:?} {:?}", a, b);
    let mut lr = DataStore::new(a.clone(), b.clone());
    lr.parse();
    let order = lr.compare(0, 0, 0);
    match order {
        Some(DataCompare::RightOrder) => {
            return Ordering::Less;
        }
        Some(DataCompare::ContinueComparing) => {
            return Ordering::Equal;
        }
        Some(DataCompare::WrongOrder) => {
            return Ordering::Greater;
        }
        _ => {
            println!("compare_packets: shouldn't ever get here");
            panic!("something wrong"); // shouldn't ever get here
        }
    }
}

fn main() {
    println!("Advent of code 2022, Day 13 Part 1\n");
    let mut aoc = AOC::new();
    let filename = aoc.get_filename().unwrap();
    aoc.load_all_data(&filename);

    println!("Data loaded");
    let mut packets: Vec<String> = vec![];
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());
    for (l, r) in &aoc.data {
        packets.push(l.clone());
        packets.push(r.clone());
        // println!("{:?}", (l, r));
    }

    println!("Packets loaded");

    packets.sort_by(compare_packets);

    let mut d1 = 0;
    let mut d2 = 0;
    let mut i = 0;
    for packet in packets.iter() {
        i += 1;
        println!("packet: {:?}", packet);
        if packet.eq("[[2]]") {
            d1 = i;
        } else if packet.eq("[[6]]") {
            d2 = i;
        }
    }

    println!("d1: {:?}", d1);
    println!("d2: {:?}", d2);
    println!("d1 * d2: {:?}", d1 * d2);
}
