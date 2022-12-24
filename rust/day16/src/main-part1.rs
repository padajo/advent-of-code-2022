use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct AOC {
    lines: Vec<String>,
}

impl AOC {
    fn new() -> AOC {
        AOC { lines: Vec::new() }
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
    fn load_all_lines(&mut self, filename: &String) {
        self.lines = Vec::new();
        if let Ok(lines) = self.read_lines(filename) {
            for l in lines {
                if let Ok(line) = l {
                    let line_data = line.trim();
                    if line_data.len() > 0 {
                        self.lines.push(line_data.to_string());
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

    fn parse_line(line: &String) -> Option<(String, i16, Vec<String>)> {
        let valve_tunnel_split: Vec<&str> = line.split(";").collect();
        let valve = valve_tunnel_split[0].split(' ').collect::<Vec<&str>>()[1];
        let flow_rate = valve_tunnel_split[0].split('=').collect::<Vec<&str>>()[1];
        let tunnel_data = valve_tunnel_split[1].split("valve").collect::<Vec<&str>>()[1];
        // we will always be able to remove the first character
        let mut tunnels_string = tunnel_data.to_string();
        tunnels_string.remove(0);
        let mut tunnels: Vec<String> = Vec::new();
        for t in tunnels_string.split(",") {
            let s = t.to_string();
            tunnels.push(s.trim().to_string());
        }
        Some((
            valve.to_string(),
            flow_rate.parse::<i16>().unwrap(),
            tunnels,
        ))
    }
}

// type SavedSolution = (Vec<String>, VecDeque<String>, HashMap<String, i16>);

struct TunnelSystem {
    valves: HashMap<String, Valve>,
    one_minute_to: HashMap<String, Vec<String>>,
}

impl TunnelSystem {
    fn new() -> TunnelSystem {
        TunnelSystem {
            valves: HashMap::new(),
            one_minute_to: HashMap::new(),
        }
    }

    fn build_tunnel_system(&mut self, lines: &Vec<String>) -> Result<(), String> {
        for line in lines {
            let data = AOC::parse_line(&line);
            if data.is_some() {
                let (valve, flow_rate, tunnels) = data.unwrap();
                let v = Valve::new(valve.clone(), flow_rate);
                // save the data
                self.valves.insert(valve.clone(), v);
                self.one_minute_to.insert(valve.clone(), tunnels.clone());
                // println!(
                //     "V {} - fr={}; to V(s) {}",
                //     valve,
                //     flow_rate,
                //     tunnels.join(", ")
                // );
            }
        }

        Ok(())
    }

    fn get_non_zero_valve_names(&self) -> Result<HashMap<String, i16>, String> {
        let mut non_zero_valves: HashMap<String, i16> = HashMap::new();
        // println!("{:?}", self.valves);
        for (valve_name, valve) in &self.valves {
            // println!("{}: {}", valve_name, valve.flow_rate);
            if valve.flow_rate > 0 {
                non_zero_valves.insert(valve_name.clone(), valve.flow_rate);
            }
        }
        Ok(non_zero_valves)
    }

    fn get_shortest_route_between_valves(
        &self,
        v1: &String,
        v2: &String,
    ) -> Result<Vec<String>, String> {
        let mut route = Vec::new();
        // queue stores (current, parents) valve names where parents is a comma delimited list of parents
        let mut queue: VecDeque<(String, String)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();
        queue.push_back((v1.clone(), "".to_string())); // first one has no parent so use empty string
        visited.insert(v1.clone());
        let mut found: (String, String) = ("".to_string(), "".to_string());
        while !queue.is_empty() {
            let (current, parent) = queue.pop_front().unwrap();
            if current.eq(v2) {
                found = (current, parent);
                break;
            }
            for next in self.one_minute_to.get(&current).unwrap() {
                if !visited.contains(next) {
                    let new_parent = format!("{},{}", parent, current);
                    queue.push_back((next.clone(), new_parent));
                }
            }
        }
        // now get the parent list (comma delimited list)
        for v in found.1.split(",") {
            if v.len() > 0 {
                route.push(v.to_string());
            }
        }
        // and add the last one on the end
        route.push(found.0);

        /*
        // from wikipedia: BFS pseudocode
         1  procedure BFS(G, root) is
         2      let Q be a queue
         3      label root as explored
         4      Q.enqueue(root)
         5      while Q is not empty do
         6          v := Q.dequeue()
         7          if v is the goal then
         8              return v
         9          for all edges from v to w in G.adjacentEdges(v) do
        10              if w is not labeled as explored then
        11                  label w as explored
        12                  w.parent := v
        13                  Q.enqueue(w)
                 */
        Ok(route)
    }

    fn get_shortest_routes_between_valves(
        &self,
        valves: &NonZeroValveData,
    ) -> Result<TunnelRoutes, String> {
        let mut routes: TunnelRoutes = HashMap::new();
        let valve_names = valves.keys().cloned().collect::<Vec<String>>();
        let num = valve_names.len();

        for i in 0..num {
            for j in 0..num {
                if i != j {
                    let v1 = valve_names[i].clone();
                    let v2 = valve_names[j].clone();
                    let route_result = self.get_shortest_route_between_valves(&v1, &v2);
                    if route_result.is_ok() {
                        let route = route_result.unwrap();
                        let current_valve_routes = routes.get(&v1);
                        if current_valve_routes.is_none() {
                            let mut route_map: HashMap<String, Vec<String>> = HashMap::new();
                            route_map.insert(v2.clone(), route.clone());
                            routes.insert(v1.clone(), route_map);
                        } else {
                            let mut route_map = current_valve_routes.unwrap().clone();
                            route_map.insert(v2.clone(), route.clone());
                            routes.insert(v1.clone(), route_map);
                        }
                    }
                }
            }
        }

        for (k, v) in &routes {
            let valve = self.valves.get(k).unwrap();
            println!("{} -> flow rate: {}", k, valve.flow_rate);
            for (k2, v2) in v {
                println!("\t{} -> {:?}", k2, v2);
            }
        }

        Ok(routes)
    }

    fn get_all_possible_routes(
        &self,
        routes: &TunnelRoutes,
        start: &String,
    ) -> Result<AllRoutes, String> {
        let mut all_routes: Vec<Vec<(String, i16)>> = Vec::new();
        // need to store how many steps (minutes) it is between each pair of valves
        let mut distances: HashMap<(String, String), i16> = HashMap::new();
        for (k, v) in routes {
            for (k2, v2) in v {
                // the steps are stored as ["v1", ... , "v2"] so the length is the number of steps
                // because at the end the last one will need to be waited at for 1 minute
                // to open the valve, so appending will mean there will be two entries for the
                // valve ["v1", ... , "v2"] + ["v2", ... , "v3"] because v2 is the start of the next
                // entry and v1 becomes the parent of v2
                distances.insert((k.clone(), k2.clone()), v2.len() as i16);
            }
        }
        // queue stores (current, parents) valve names where parents is a comma delimited list of parents
        let mut queue: VecDeque<(String, Vec<(String, i16)>)> = VecDeque::new();

        queue.push_back((start.clone(), vec![])); // first one has no parent so use empty string
        while !queue.is_empty() {
            let (current, parents) = queue.pop_front().unwrap();
            // the end state is 30 minutes
            // this is a "distance" calculation
            let mut distance_to_here: i16 = parents.clone().into_iter().map(|n| n.1).sum();
            // get last point in parents
            let mut distance_for_this_step: i16 = 0;
            if parents.len() > 0 {
                let last_parent = parents.last().unwrap();
                let distance_result = distances.get(&(current.clone(), last_parent.0.to_string()));
                distance_for_this_step += distance_result.unwrap();
            }

            let next = (current.clone(), distance_for_this_step);

            // if it's more than 30, don't add any more, just add it to the possible routes
            if distance_to_here + distance_for_this_step > 30 {
                let mut new_route = parents.clone();
                new_route.push(next);
                all_routes.push(new_route);
            } else {
                let mut new_parents = parents.clone();
                new_parents.push(next);
                // we can just do this each time for each route
                // and because each loop carries it's own parents with it
                // there is no need to have a separate visited list
                let visited: HashSet<String> =
                    new_parents.clone().into_iter().map(|n| n.0).collect();
                for next_valve in routes.get(&current).unwrap().keys() {
                    if !visited.contains(next_valve) {
                        queue.push_back((next_valve.clone(), new_parents.clone()));
                    }
                }
            }
        }
        Ok(all_routes)
    }

    fn calculate_pressure_released(&self, solution: &Vec<String>) -> i32 {
        let mut pressure_released: i32 = 0;
        let mut flow_rate: i32 = 0;
        let mut current_valve = "AA";
        let mut i: i32 = 1;
        for valve in solution {
            // println!("\nMinute {}", i);
            i += 1;
            pressure_released += flow_rate;
            // println!("Pressure released: {}", pressure_released);
            if valve.eq(current_valve) {
                let valve_flow_rate: i32 = self.valves.get(valve).unwrap().flow_rate as i32;
                // println!("Opening valve {} - flow rate: {}", valve, valve_flow_rate);
                flow_rate += valve_flow_rate;
            } else {
                // println!("Moved to {}", valve);
            }
            current_valve = valve;
        }
        pressure_released
    }
}

#[derive(Debug, Clone)]
struct Valve {
    valve_name: String,
    flow_rate: i16,
}

impl Valve {
    fn new(valve_name: String, flow_rate: i16) -> Valve {
        Valve {
            valve_name,
            flow_rate,
        }
    }
}

type NonZeroValveData = HashMap<String, i16>;
type TunnelRoutes = HashMap<String, HashMap<String, Vec<String>>>;
type AllRoutes = Vec<Vec<(String, i16)>>;

fn main() {
    println!("Advent of Code 2022 Day 16");
    let mut aoc = AOC::new();
    let filename = aoc.get_filename().unwrap();
    aoc.load_all_lines(&filename);

    println!("Building tunnel system...");
    let mut ts = TunnelSystem::new();
    let built = ts.build_tunnel_system(&aoc.lines);

    // ts.print_tunnels();

    if built.is_ok() {
        // get all the non-zero valves
        println!("Finding shortest routes between valves with non-zero flow rate...");
        let valves_path_data_result: Result<NonZeroValveData, String> =
            ts.get_non_zero_valve_names();

        let mut routes: TunnelRoutes = HashMap::new();
        if valves_path_data_result.is_ok() {
            let mut valves_path_data: NonZeroValveData = valves_path_data_result.unwrap();
            valves_path_data.insert("AA".to_string(), 0);
            let routes_result: Result<TunnelRoutes, String> =
                ts.get_shortest_routes_between_valves(&valves_path_data);
            if routes_result.is_ok() {
                routes = routes_result.unwrap();
            }
        }

        println!("Finding all possible routes in 30 minutes...");
        // starting at AA, run every possible route that is less than 30 minutes
        let all_routes_result = ts.get_all_possible_routes(&routes, &"AA".to_string());

        if all_routes_result.is_ok() {
            let all_routes = all_routes_result.unwrap();
            println!("Count of all routes: {}", all_routes.len());

            let mut max_pressure = 0;
            let mut max_route = vec![];
            let mut max_full_route = vec![];

            for r in all_routes {
                let mut route_vec: Vec<String> = vec![];
                // a.append(&mut b.clone())
                let waypoints: Vec<String> = r.clone().into_iter().map(|n| n.0).collect();
                let waypoint_len = waypoints.len();
                for i in 1..waypoint_len {
                    let from = waypoints.get(i - 1).unwrap();
                    let to = waypoints.get(i).unwrap();
                    // now get the route from the routes map
                    let route = routes.get(from).unwrap().get(to).unwrap();
                    route_vec.append(&mut route.clone());
                }
                // remove iniial "AA" from the route
                route_vec.remove(0);
                // now get just the first 30 minutes
                route_vec.truncate(30);
                let pressure_released = ts.calculate_pressure_released(&route_vec);
                if pressure_released > max_pressure {
                    max_pressure = pressure_released.clone();
                    max_route = waypoints.clone();
                    max_full_route = route_vec.clone();
                }
                // now we have a full route, calculate the pressure released
                // println!("Route        : {:?}", waypoints.clone());
                // println!("   Full Route: {:?}", route_vec.clone());
                // println!("       Length: {:?}", route_vec.len());
                // println!("");
            }

            println!("Max pressure released: {}", max_pressure);
            println!("Max route: {:?}", max_route);
            println!("Max full route: {:?}", max_full_route);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let line = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string();
        let data = AOC::parse_line(&line);
        assert!(data.is_some());
        if data.is_some() {
            let (valve, flow_rate, tunnels) = data.unwrap();
            assert_eq!(valve, "AA");
            assert_eq!(flow_rate, 0);
            assert_eq!(tunnels.len(), 3);
            assert_eq!(tunnels[0], "DD");
            assert_eq!(tunnels[1], "II");
            assert_eq!(tunnels[2], "BB");
        }
        // note: valve not valves, flow rate is not zero, and only has a single tunnel
        let line2 = "Valve HH has flow rate=22; tunnel leads to valve GG".to_string();
        let data2 = AOC::parse_line(&line2);
        assert!(data2.is_some());
        if data2.is_some() {
            let (valve2, flow_rate2, tunnels2) = data2.unwrap();
            assert_eq!(valve2, "HH");
            assert_eq!(flow_rate2, 22);
            assert_eq!(tunnels2.len(), 1);
            assert_eq!(tunnels2[0], "GG");
        }
    }

    #[test]
    fn test_new_valve() {
        let valve_name: String = "AA".to_string();
        let flow_rate: i16 = 20;
        let valve = Valve::new(valve_name, flow_rate);

        assert_eq!(valve.valve_name, "AA");
        assert_eq!(valve.flow_rate, 20);
    }

    #[test]
    fn test_build_tunnel_system() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
            "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
        ];
        let mut ts = TunnelSystem::new();
        let result = ts.build_tunnel_system(&lines);
        assert!(result.is_ok());
        assert_eq!(ts.valves.len(), 2);
        assert_eq!(ts.one_minute_to.len(), 2);
        assert_eq!(ts.valves.contains_key(&"AA".to_string()), true);
        assert_eq!(ts.valves.contains_key(&"DD".to_string()), false);
        assert_eq!(ts.one_minute_to.contains_key(&"HH".to_string()), true);
        assert_eq!(ts.one_minute_to.contains_key(&"II".to_string()), false);
        assert_eq!(
            ts.one_minute_to
                .get(&"HH".to_string())
                .unwrap()
                .contains(&"GG".to_string()),
            true
        );
    }

    #[test]
    fn test_get_valve_names_with_flow_rate_greater_than_zero() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves BB, CC, DD".to_string(),
            "Valve BB has flow rate=1; tunnel leads to valve AA".to_string(),
            "Valve CC has flow rate=0; tunnel leads to valve DD".to_string(),
            "Valve DD has flow rate=20; tunnel leads to valve EE, FF".to_string(),
            "Valve EE has flow rate=0; tunnel leads to valve DD, FF".to_string(),
            "Valve FF has flow rate=15; tunnel leads to valve CC".to_string(),
        ];
        let mut ts = TunnelSystem::new();
        let result = ts.build_tunnel_system(&lines);
        // now the tunnel system is built
        let non_zero_valves_result: Result<HashMap<String, i16>, String> =
            ts.get_non_zero_valve_names();
        assert!(non_zero_valves_result.is_ok());
        let non_zero_valves = non_zero_valves_result.unwrap();
        assert_eq!(non_zero_valves.len(), 3);
        assert_eq!(non_zero_valves.contains_key(&"BB".to_string()), true);
        assert_eq!(non_zero_valves.contains_key(&"DD".to_string()), true);
        assert_eq!(non_zero_valves.contains_key(&"FF".to_string()), true);
        assert_eq!(non_zero_valves.get(&"BB".to_string()).unwrap(), &1);
        assert_eq!(non_zero_valves.get(&"DD".to_string()).unwrap(), &20);
        assert_eq!(non_zero_valves.get(&"FF".to_string()).unwrap(), &15);
    }

    #[test]
    fn test_shortest_path() {
        let lines = vec![
            "Valve AA has flow rate=0; tunnels lead to valves BB, CC, DD".to_string(),
            "Valve BB has flow rate=1; tunnel leads to valve AA".to_string(),
            "Valve CC has flow rate=0; tunnel leads to valve DD".to_string(),
            "Valve DD has flow rate=20; tunnel leads to valve EE, FF".to_string(),
            "Valve EE has flow rate=0; tunnel leads to valve DD, FF".to_string(),
            "Valve FF has flow rate=15; tunnel leads to valve CC".to_string(),
        ];
        let mut ts = TunnelSystem::new();
        let result = ts.build_tunnel_system(&lines);
        // now the tunnel system is built
        assert!(result.is_ok());

        if result.is_ok() {
            // paths will be from AA (start) to all the valves with non-zero flow rates
            let non_zero_valves_result: Result<HashMap<String, i16>, String> =
                ts.get_non_zero_valve_names();
            assert!(non_zero_valves_result.is_ok());
            let mut non_zero_valves = non_zero_valves_result.unwrap();

            non_zero_valves.insert("AA".to_string(), 0);

            let mut shortest_path_result =
                ts.get_shortest_route_between_valves(&"AA".to_string(), &"BB".to_string());
            assert!(shortest_path_result.is_ok());
            assert_eq!(shortest_path_result.unwrap().len(), 2); // AA -> BB

            shortest_path_result =
                ts.get_shortest_route_between_valves(&"AA".to_string(), &"DD".to_string());
            assert!(shortest_path_result.is_ok());
            assert_eq!(shortest_path_result.unwrap().len(), 2); // AA -> DD

            shortest_path_result =
                ts.get_shortest_route_between_valves(&"AA".to_string(), &"FF".to_string());
            assert!(shortest_path_result.is_ok());
            assert_eq!(shortest_path_result.unwrap().len(), 3); // AA -> DD -> FF
        }
    }
}
