mod util;
use util::intcode::*;
use crate::util::direction::Direction;

enum Command {
    Move(Direction),
    Take(String),
    Drop(String),
    Inv
}
use Command::*;
use Direction::*;
use petgraph::stable_graph::{StableGraph, NodeIndex};
use regex::Regex;
use std::convert::TryFrom;
use std::collections::HashMap;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use petgraph::algo::astar;
use itertools::Itertools;
use crate::util::intcode::Instruction::Output;
use petgraph::dot::{Dot, Config};
use std::net::Shutdown::Write;

impl Command {
    fn take(item: &str) -> Command {
        Take(item.to_owned())
    }

    fn drop(item: &str) -> Command {
        Drop(item.to_owned())
    }

     fn to_string(&self) -> String {
         match self {
             Move(dir) => dir.to_string().to_owned(),
             Take(item) => format!("take {}", item).to_string(),
             Drop(item) => format!("drop {}", item).to_string(),
             Inv => "inv".to_owned()
         }
     }
}

struct Droid {
    cpu: IntcodeCPU,

    location: String,
    doors: Vec<Direction>,

    status_parser: StatusParser,
}

impl Droid {
    fn new() -> Droid {
        let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
        cpu.output_mode = OutputMode::ASCII;

        cpu.start();

        let output: String = cpu.output.iter()
            .map(|&c| c as u8 as char)
            .collect();

        cpu.output.clear();

        let mut status = StatusParser::new().parse(&output);
        assert!(status.len() == 1);
        let status = status.pop().unwrap();

        dbg!(&status);

        Droid {
            cpu,
            doors: status.doors,
            location: status.location,
            status_parser: StatusParser::new(),
        }
    }

    fn set_status(&mut self, status: &Status) {
        self.location = status.location.clone();
        self.doors = status.doors.clone();
    }

    fn resume_command(&mut self, c: Command) -> String {
        if self.cpu.output_mode == OutputMode::ASCII {
            println!("{}", c.to_string());
        }

        self.cpu.resume_string(&format!("{}\n", c.to_string()));

        let output: String = self.cpu.output.iter()
            .map(|&c| c as u8 as char)
            .collect();

        self.cpu.output.clear();

        output
    }

    fn inventory(&mut self) -> Vec<String> {
        let output = self.resume_command(Inv);

        output
            .trim()
            .split('\n')
            .filter(|item| item.starts_with("-"))
            .map(|item| {
                (&item[2..]).to_string()
            })
            .collect()
    }

    fn go_direction(&mut self, dir: Direction) -> Vec<Status> {
        let output = self.resume_command(Move(dir));
        let statuses = self.status_parser.parse(&output);
        self.set_status(&statuses[statuses.len() - 1]);
        statuses
    }

    fn get_neighbour_locations(&mut self) -> HashMap<Direction, String> {
        let mut neighbour_locations = HashMap::new();
        for dir in self.doors.clone() {
            let statuses = self.go_direction(dir);

            let status_1 = &statuses[0];
            if status_1.is_ejected_back() {
                neighbour_locations.insert(dir, status_1.location.clone());
            } else {
                neighbour_locations.insert(dir, self.location.clone());
                self.go_direction(dir.mirror());
            }
        }
        neighbour_locations
    }

    fn follow_path(&mut self, path: Vec<Direction>) -> Vec<Status> {
        let mut statuses = vec![];
        for dir in path {
            let new_statuses  = self.go_direction(dir);
            statuses.extend(new_statuses);
        }
        statuses
    }
}

#[derive(Debug, Clone)]
struct Status {
    location: String,
    texts: Vec<String>,
    doors: Vec<Direction>,
    items: Vec<String>,
}

impl Status {
    fn is_ejected_back(&self) -> bool {
        self.texts.iter().any(|text| {
            text.contains("ejected back to the checkpoint")
        })
    }
}

//fn parse_status_once(text: String) -> Optional<Status> {
//
//}

struct StatusParser {
    regex: Regex,
}

impl StatusParser {
    fn new() -> StatusParser {
        StatusParser {
            regex: Regex::new(r"\s*== (?P<location>.*) ==
(?P<text1>.*)

Doors here lead:
(?P<doors>(?:- .*)(?:\n- .*)*)

(?:Items here:
(?P<items>(?:- .*)(?:\n- .*)*)

)?(?P<text2>.*)
").unwrap()
        }
    }

    fn parse(&self, text: &str) -> Vec<Status> {

//    println!("Parsing:\n--------------------\n{}\n---------------", text);

        let caps = self.regex.captures(&text).unwrap();

//    dbg!(&caps);

        let doors = caps["doors"].trim()
            .split('\n')
            .map(|dir| &dir[2..])
            .map(|dir| Direction::try_from(dir).unwrap())
            .collect();

        let items = if let Some(items)= caps.name("items") {
            items.as_str().trim().split('\n')
                .map(|item| (&item[2..]).to_string())
                .collect()
        } else {
            vec![]
        };

        let status = Status {
            location: caps["location"].to_string(),
            texts: vec![caps["text1"].to_string(), caps["text2"].to_string()],
            doors,
            items
        };

        if caps.get(0).unwrap().as_str().len() < text.len() {
//        println!("Leftover!");
//        println!("Message: ###{}###", text);
//        println!("Captured: ###{}###", caps.get(0).unwrap().as_str());

            let next_input = caps.get(0).unwrap().as_str();

            let mut statuses = self.parse(next_input);
            statuses.insert(0, status);
            statuses
        } else {
            vec![status]
        }
    }
}

struct Map {
    g: StableGraph<String, Direction>,
    locs: HashMap<String, NodeIndex<u32>>,
    locs_rev: HashMap<NodeIndex<u32>, String>,
}

#[derive(Clone)]
struct MyKey(String);

impl Map {
    fn new() -> Map {
        Map {
            g: StableGraph::new(),
            locs: HashMap::new(),
            locs_rev: HashMap::new(),
        }
    }

    fn add_neighbour(&mut self, location: &str, dir: Direction, neighbour: &str) {
        self.add_edge(location, dir, neighbour);
        self.add_edge(neighbour, dir.mirror(), location);
    }

    fn add_neighbours(&mut self, location: &str, neighbours: &HashMap<Direction, String>) {
        for (dir, neighbour) in neighbours {
            self.add_neighbour(location, *dir, neighbour);
        }
    }

    fn add_edge(&mut self, location: &str, dir: Direction, neighbour: &str) {
        let location = self.add_node(location);
        let neighbour = self.add_node(neighbour);

        if !self.g.contains_edge(location, neighbour) {
            self.g.add_edge(location, neighbour, dir);
        }
    }

    fn add_node(&mut self, location: &str) -> NodeIndex<u32> {
        if let Some(ni) = self.locs.get(location) {
            *ni
        } else {
            let ni = self.g.add_node(location.to_string());
            self.locs.insert(location.to_string(), ni);
            self.locs_rev.insert(ni, location.to_string());
            ni
        }
    }

    fn has_node(&self, location: &str) -> bool {
        self.locs.get(location).is_some()
    }

    fn get_node(&self, location: &str) -> NodeIndex<u32> {
        if let Some(ni) = self.locs.get(location) {
            *ni
        } else {
            panic!()
        }
    }

    fn find_path(&self, from: &str, to: &str) -> Vec<Direction> {
        let a = self.get_node(from);
        let b = self.get_node(to);

        let path = astar(&self.g, a, |finish| finish == b, |_| 1, |_| 0).unwrap().1;

        let path = path.iter().map(|ni| self.locs_rev.get(ni).unwrap().clone()).collect();

        self.convert_path(path)
    }

    fn convert_path(&self, path: Vec<String>) -> Vec<Direction> {
        (0..path.len() - 1)
            .map(|i| {
                let a = self.get_node(&path[i]);
                let b = self.get_node(&path[i + 1]);
                let ei = self.g.find_edge(a, b).unwrap();
                self.g.edge_weight(ei).unwrap().clone()
            })
            .collect()
    }
}

fn main() {
    println!("Exploring map!");

    let mut d = Droid::new();

    let mut map = Map::new();
    map.add_node(&d.location);

    let mut unexplored = vec![d.location.clone()];
    let mut explored = vec!["Pressure-Sensitive Floor".to_string()];

    while unexplored.len() > 0 {
        println!("----");

        let target = unexplored.pop().unwrap();
        let path = map.find_path(&d.location, &target);

        println!("Going from {} to {} via: {:?}", d.location, target, path);

        d.follow_path(path);

        println!("Visiting direct neighbours...");

        let neighbours = d.get_neighbour_locations();

        map.add_neighbours(&d.location, &neighbours);

        explored.push(d.location.clone());

        for (_, neighbour) in &neighbours {
            if !explored.contains(neighbour) {
                unexplored.push(neighbour.clone());
            }
        }
        println!("Locations left: {:?}", unexplored);
    }

    println!("Locations: {:?}", explored);

    println!("Gathering all items!");

    for location in &explored {
        if &location == &"Pressure-Sensitive Floor" {
            continue;
        }

        let path = map.find_path(&d.location, location);

        println!("------");

        let status = d.follow_path(path).pop().unwrap();

        for item in status.items {
            match item.as_str() {
                "infinite loop" => continue,
                "photons" => continue,
                "escape pod" => continue,
                "molten lava" => continue,
                "giant electromagnet" => continue,
                _ => ()
            }

            println!("Taking {}", item);
            d.resume_command(Take(item));
        }
    }

    println!("Items: {:?}", d.inventory());

    println!("Brute forcing way through the pressure pad");

    let path = map.find_path(&d.location, "Security Checkpoint");
    d.follow_path(path);

    let items = d.inventory();

    for item in &items {
        d.resume_command(Drop(item.clone()));
    }

    let mut made_it_through = false;
    for k in 0..items.len() {
        for items in items.iter().combinations(k) {
            println!("Pick up {:?}", items);
            for &item in &items {
                d.resume_command(Take(item.clone()));
            }

            println!("Go to pressure pad");
            let statuses = d.go_direction(East);
            if !statuses.iter().any(|status| status.is_ejected_back()) {
                made_it_through = true;
                break;
            }

            println!("Dropping items...");
            for &item in &items {
                d.resume_command(Drop(item.clone()));
            }
        }

        if made_it_through {
            break;
        }
    }

    let dot = Dot::with_config(&map.g, &[]);
    println!("{:?}", dot);

    std::fs::write("graph.dot", format!("{}", dot));
}
