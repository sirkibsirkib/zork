use hashbrown::HashMap;
use regex::Captures;
use regex::Match;

use regex::Regex;
use std::io;

trait CapAll {
    fn cap_all<'a, 'b: 'a>(&'a self, s: &'b str) -> Option<Captures>;
}
impl CapAll for Regex {
    fn cap_all<'a, 'b: 'a>(&'a self, s: &'b str) -> Option<Captures> {
        let c = self.captures(s);
        if c.as_ref()
            .and_then(|c| c.get(0))
            .map(|x: Match| x.end() == s.len())
            .unwrap_or(false)
        {
            c
        } else {
            None
        }
    }
}
struct CmdParser {
    go_regex: Regex,
    exit_regex: Regex,
    time_regex: Regex,
    wait_regex: Regex,
}
impl CmdParser {
    fn new() -> Self {
        Self {
            go_regex: Regex::new(
                r"(go to|go|walk|stroll|amble|move|head|meander|saunter|jog) (.*)",
            )
            .unwrap(),
            exit_regex: Regex::new(r"exit|quit").unwrap(),
            time_regex: Regex::new(r"(check)? (time|clock|watch)").unwrap(),
            wait_regex: Regex::new(r"wait|idle|chill|stay").unwrap(),
        }
    }
    fn parse(&self, world: &World, s: &str) -> Option<Cmd> {
        self.go_regex
            .cap_all(s)
            .and_then(|captures| captures.get(2))
            .and_then(|_match| {
                world
                    .locations
                    .get(&world.at)
                    .expect("unknown location!")
                    .connections
                    .iter()
                    .filter(|conn| conn.direction == _match.as_str())
                    .map(|conn| Cmd::GoTo(conn.destination))
                    .next()
            })
            .or_else(|| self.exit_regex.cap_all(s).map(|_| Cmd::Exit))
            .or_else(|| self.time_regex.cap_all(s).map(|_| Cmd::CheckTime))
            .or_else(|| self.wait_regex.cap_all(s).map(|_| Cmd::Wait))
    }
}

enum Cmd {
    GoTo(LocKey),
    Exit,
    CheckTime,
    Wait,
}

fn main() {
    let mut world = World::new();
    let mut buffer = String::new();
    let parser = CmdParser::new();

    loop {
        buffer.clear();
        println!();
        world.print_state();
        print!(" > ");
        use std::io::Write;
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).expect("IO FAILED");
        let said = &buffer.trim();
        let parsed = parser.parse(&world, said);
        match parsed {
            Some(Cmd::GoTo(dest)) => {
                world.my_time.progress(5);
                world.at = dest
            }
            Some(Cmd::Exit) => {
                println!("Until next time!");
                return;
            }
            Some(Cmd::CheckTime) => {
                println!(
                    "it is currently {}:{}.",
                    world.my_time.hrs, world.my_time.mins
                );
            }
            Some(Cmd::Wait) => {
                world.my_time.progress(15);
                println!("... you waited.");
            }
            None => println!(" Didn't understand {:?}", said),
        }
    }
}

#[derive(derive_new::new)]
struct MyTime {
    hrs: u32,
    mins: u32,
}
impl MyTime {
    fn progress(&mut self, mins: u32) {
        self.mins += mins;
        self.hrs += self.mins / 60;
        self.mins %= 60;
    }
}

struct World {
    my_time: MyTime,
    locations: HashMap<LocKey, Location>,
    at: LocKey,
}
impl World {
    fn new() -> Self {
        // start state
        use LocKey::*;

        let locations = vec![
            (
                Hallway,
                Location::new(vec![Connection::new(
                    Passageway,
                    "a large doorway",
                    "north",
                )]),
            ),
            (
                Passageway,
                Location::new(vec![Connection::new(Hallway, "a wooden archway", "south")]),
            ),
        ]
        .into_iter()
        .collect();
        Self {
            my_time: MyTime::new(16, 10),
            locations,
            at: Hallway,
        }
    }

    fn print_state(&self) {
        println!("You are in {:?}.", self.at);
        let loc = self.locations.get(&self.at).expect("AHHH");
        for connection in loc.connections.iter() {
            println!(
                "there is {} to the {}.",
                connection.description, connection.direction,
            );
        }
    }
}

struct Location {
    connections: Vec<Connection>,
}
impl Location {
    fn new(connections: Vec<Connection>) -> Self {
        Self {
            // people: vec![],
            connections,
        }
    }
}

#[derive(derive_new::new, Clone, Debug)]
struct Connection {
    destination: LocKey,
    description: &'static str,
    direction: &'static str,
}

struct Person {
    name: &'static str,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum LocKey {
    Hallway,
    Passageway,
}
