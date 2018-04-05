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
const LAST_EXECUTED_COMMAND: &str = "LAST_EXECUTED_COMMAND";
const UNDO: &str = "UNDO";
const HOW_MANY_BOXES: &str = "HOW_MANY_BOXES";
const HOW_MUCH_TIME: &str = "HOW_MUCH_TIME";

fn next<'a, 'b, T, O>(iterator: &'a mut T) -> O
    where T: Iterator<Item=&'b str>, O: FromStr, <O as FromStr>::Err: Debug
{
    iterator.next().unwrap().parse().unwrap()
}

#[derive(Clone)]
enum CommandType {
    Get,
    Drop,
}

#[derive(Clone)]
struct Command {
    command_type: CommandType,
    robot_id: usize,
    x: usize,
    y: usize,
    num_boxes: usize,
    priority: usize,
    time: usize
}

enum ExecutedCommand {
    Command(Command),
    Execute,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.command_type {
            CommandType::Get => write!(f, "GET {} {} {}", self.x, self.y, self.num_boxes),
            CommandType::Drop => write!(f, "DROP {} {} {}", self.x, self.y, self.num_boxes)
        }
    }
}

#[derive(Clone)]
struct Robot {
    num_boxes: usize,
    commands: VecDeque<Command>,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            num_boxes: 0,
            commands: VecDeque::new(),
        }
    }

    fn add_command(&mut self, command: Command) {
        match command.priority {
            0 => self.commands.push_front(command),
            1 => self.commands.push_back(command),
            _ => panic!("Unexpected priority")
        }
    }

    fn update_time(&mut self) {
        for command in &mut self.commands {
            command.time += 1;
        }
    }
}

fn main() {
    let input = BufReader::new(File::open(INPUT_FILE).unwrap());
    let mut output = BufWriter::new(OpenOptions::new().create(true).truncate(true).write(true).open(OUTPUT_FILE).unwrap());
    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();
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
        let line = lines.next().unwrap().unwrap();
        let mut tokens = line.split_whitespace();
        for j in 0..col {
            map[i][j] = next(&mut tokens);
        }
    }
    let mut robots: Vec<Robot> = vec![Robot::new(); n];
    let mut executed_commands: Vec<Command> = Vec::new();
    let mut all_executed_commands: Vec<ExecutedCommand> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some(ADD_GET_BOX) => {
                let robot_id: usize = next(&mut tokens);
                let command = Command {
                    command_type: CommandType::Get,
                    robot_id,
                    x: next(&mut tokens),
                    y: next(&mut tokens),
                    num_boxes: next(&mut tokens),
                    priority: next(&mut tokens),
                    time: 0
                };
                robots[robot_id].add_command(command.clone());
                all_executed_commands.push(ExecutedCommand::Command(command))
            }
            Some(ADD_DROP_BOX) => {
                let robot_id: usize = next(&mut tokens);
                let command = Command {
                    command_type: CommandType::Drop,
                    robot_id,
                    x: next(&mut tokens),
                    y: next(&mut tokens),
                    num_boxes: next(&mut tokens),
                    priority: next(&mut tokens),
                    time: 0
                };
                robots[robot_id].add_command(command.clone());
                all_executed_commands.push(ExecutedCommand::Command(command))
            }
            Some(EXECUTE) => {
                let robot_id: usize = next(&mut tokens);
                let robot = &mut robots[robot_id];
                match robot.commands.pop_front() {
                    Some(mut command) => {
                        match command.command_type {
                            CommandType::Get => {
                                let effective_num_boxes = min(map[command.x][command.y], command.num_boxes);
                                map[command.x][command.y] -= effective_num_boxes;
                                robot.num_boxes += effective_num_boxes;
                                command.num_boxes = effective_num_boxes;
                                executed_commands.push(command)
                            }
                            CommandType::Drop => {
                                let effective_num_boxes = min(robot.num_boxes, command.num_boxes);
                                robot.num_boxes -= effective_num_boxes;
                                map[command.x][command.y] += effective_num_boxes;
                                command.num_boxes = effective_num_boxes;
                                executed_commands.push(command)
                            }
                        }
                        all_executed_commands.push(ExecutedCommand::Execute);
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
            Some(LAST_EXECUTED_COMMAND) => {
                writeln!(output, "{}: {}", LAST_EXECUTED_COMMAND, match executed_commands.len() {
                    0 => "No command was executed".to_string(),
                    _ => {
                        let last_executed_command = executed_commands.last().unwrap();
                        format!("{}: {}", last_executed_command.robot_id, last_executed_command)
                    }
                }).unwrap();
            }
            Some(UNDO) => {
                match all_executed_commands.len() {
                    0 => writeln!(output, "{}: No history", UNDO).unwrap(),
                    _ => match all_executed_commands.pop().unwrap() {
                        ExecutedCommand::Command(command) => {
                            match command.priority {
                                0 => { robots[command.robot_id].commands.pop_front(); }
                                1 => { robots[command.robot_id].commands.pop_back(); }
                                _ => panic!("Unexpected priority")
                            }
                        }
                        ExecutedCommand::Execute => {
                            let mut command = executed_commands.pop().unwrap();
                            match command.command_type {
                                CommandType::Get => {
                                    robots[command.robot_id].num_boxes -= command.num_boxes;
                                    map[command.x][command.y] += command.num_boxes;
                                }
                                CommandType::Drop => {
                                    map[command.x][command.y] -= command.num_boxes;
                                    robots[command.robot_id].num_boxes += command.num_boxes;
                                }
                            }
                            command.time = 0;
                            command.priority = 0;
                            robots[command.robot_id].commands.push_front(command);
                        }
                    }
                }
            }
            Some(HOW_MANY_BOXES) => {
                let robot_id: usize = next(&mut tokens);
                writeln!(output, "{}: {}", HOW_MANY_BOXES, robots[robot_id].num_boxes).unwrap();
            }
            Some(HOW_MUCH_TIME) => {
                writeln!(output, "{}: {}", HOW_MUCH_TIME, match executed_commands.len() {
                    0 => "No command was executed".to_string(),
                    _ => {
                        let last_executed_command = executed_commands.last().unwrap();
                        last_executed_command.time.to_string()
                    }
                }).unwrap();
            }
            _ => {}
        }
        for robot in &mut robots {
            robot.update_time();
        }
    }
}
