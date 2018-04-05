use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use std::cmp::min;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

const INPUT_FILE: &str = "robots.in";
const OUTPUT_FILE: &str = "robots.out";
const ADD_GET_BOX: &str = "ADD_GET_BOX";
const ADD_DROP_BOX: &str = "ADD_DROP_BOX";
const EXECUTE: &str = "EXECUTE";
const PRINT_COMMANDS: &str = "PRINT_COMMANDS";

fn next<'a, 'b, T, O>(iterator: &'a mut T) -> O
    where T: Iterator<Item=&'b str>, O: FromStr, <O as FromStr>::Err: Debug
{
    iterator.next().unwrap().parse().unwrap()
}

#[derive(Clone)]
enum Command {
    GetBox { x: usize, y: usize, num_boxes: usize },
    DropBox { x: usize, y: usize, num_boxes: usize },
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Command::GetBox {x, y, num_boxes} => write!(f, "GET {} {} {}", x, y, num_boxes),
            Command::DropBox {x, y, num_boxes} => write!(f, "DROP {} {} {}", x, y, num_boxes)
        }
    }
}

#[derive(Clone)]
struct Robot {
    num_boxes: usize,
    commands: VecDeque<Command>,
    executed_commands: Vec<Command>
}

impl Robot {
    fn new() -> Robot {
        Robot {
            num_boxes: 0,
            commands: VecDeque::new(),
            executed_commands: Vec::new()
        }
    }

    fn add_command(&mut self, command: Command, priority: u32) {
        match priority {
            0 => self.commands.push_front(command),
            1 => self.commands.push_back(command),
            _ => panic!("Unexpected priority")
        }
    }
}

fn main() {
    let mut input = BufReader::new(File::open(INPUT_FILE).unwrap());
    let mut output = BufWriter::new(OpenOptions::new().create(true).truncate(true).write(true).open(OUTPUT_FILE).unwrap());
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let n: usize;
    let lin: usize;
    let col: usize;
    {
        let mut tokens = line.split_whitespace();
        n = next(&mut tokens);
        lin = next(&mut tokens);
        col = next(&mut tokens);
    }
    let mut map: Vec<Vec<usize>> = vec![vec![0; col]; lin];
    for i in 0..lin {
        input.read_line(&mut line).unwrap();
        let mut tokens = line.split_whitespace();
        for j in 0..col {
            map[i][j] = tokens.next().unwrap().parse().unwrap();
        }
    }
    let mut robots: Vec<Robot> = vec![Robot::new(); n];

    for line in input.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some(ADD_GET_BOX) => {
                let robot_id: usize = next(&mut tokens);
                robots[robot_id].add_command(Command::GetBox {
                    x: next(&mut tokens),
                    y: next(&mut tokens),
                    num_boxes: next(&mut tokens),
                },
                                             next(&mut tokens));
            }
            Some(ADD_DROP_BOX) => {
                let robot_id: usize = next(&mut tokens);
                robots[robot_id].add_command(Command::DropBox {
                    x: next(&mut tokens),
                    y: next(&mut tokens),
                    num_boxes: next(&mut tokens),
                },
                                             next(&mut tokens));
            }
            Some(EXECUTE) => {
                let robot_id: usize = next(&mut tokens);
                let robot = &mut robots[robot_id];
                match robot.commands.pop_front() {
                    Some(command) => {
                        match command {
                            Command::GetBox { x, y, num_boxes } => {
                                map[x][y] -= min(map[x][y], num_boxes);
                                robot.num_boxes += num_boxes;
                            }
                            Command::DropBox { x, y, num_boxes } => {
                                robot.num_boxes -= min(robot.num_boxes, num_boxes);
                                map[x][y] += num_boxes;
                            }
                        }
                    }
                    None => { writeln!(output, "{}: No command to execute", EXECUTE).unwrap(); }
                }
            }
            Some(PRINT_COMMANDS) => {
                let robot_id: usize = next(&mut tokens);
                let commands = &robots[robot_id].commands;
                writeln!(output, "{}: {}", PRINT_COMMANDS, match commands.len() {
                    0 => "No command found".to_string(),
                    _ => format!("{}: {}", robot_id, commands.iter().map(|command| command.to_string())
                        .collect::<Vec<String>>().join("; "))
                }).unwrap();
            }
            _ => {}
        }
    }
}
