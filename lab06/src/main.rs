use std::fs;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]);
}
struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _rest: &[&str]) {
        println!("pong!");
    }
}
struct CountCommand;

impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, rest: &[&str]) {
        println!("counted {} arguments", rest.len());
    }
}
struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _rest: &[&str]) {
        self.count += 1;
        println!("called {} times", self.count);
    }
}

struct SumCommand;

impl Command for SumCommand {
    fn get_name(&self) -> &str {
        "sum"
    }

    fn exec(&mut self, rest: &[&str]) {
        let mut total = 0.0;
        let mut ok = true;

        for arg in rest {
            match arg.parse::<f64>() {
                Ok(num) => total += num,
                Err(_) => {
                    println!("'{}' not a number", arg);
                    ok = false;
                }
            }
        }

        if ok {
            println!("sum = {}", total);
        }
    }
}
struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal { commands: vec![] }
    }

    fn register(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    fn run(&mut self) {
        let mut s = String::new();

        if let Ok(cont) = fs::read_to_string("fisier.txt") {
            s = cont;
        } else {
            println!("nu s-a putut citi fisierul");
        }

        for line in s.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let mut name = "";
            let mut rest = String::new();

            for (k, c) in line.split(" ").enumerate() {
                if k == 0 {
                    name = c;
                } else {
                    rest.push_str(c);
                    rest.push(' ');
                }
            }

            if name == "stop" {
                println!("stopping");
                break;
            }

            let mut ok = false;
            let rest1: Vec<&str> = rest.split_whitespace().collect();
            for cmd in self.commands.iter_mut() {
                if cmd.get_name() == name {
                    cmd.exec(&rest1);
                    ok = true;
                    break;
                }
            }

            if !ok {
                //comanda va fi inlocuita cu cea corecta daca are scrierea corecta dar uppercase sau o sg litera gresita
                let mut new = String::new();
                for cmd in self.commands.iter() {
                    let corect = cmd.get_name();

                    if corect == name.to_lowercase().as_str() {
                        new = corect.to_string();
                        break;
                    }

                    if corect.len() == name.len() {
                        let mut dif = 0;
                        let char_corect: Vec<char> = corect.chars().collect();
                        let char: Vec<char> = name.chars().collect();

                        for i in 0..char_corect.len() {
                            if char_corect[i] != char[i] {
                                dif += 1;
                            }
                        }
                        if dif == 1 {
                            new = corect.to_string();
                            break;
                        }
                    }
                }

                if new.is_empty() {
                    println!("invalid command found");
                } else {
                    println!("suggested command {}", new);
                    let rest1: Vec<&str> = rest.split_whitespace().collect();
                    for cmd in self.commands.iter_mut() {
                        if cmd.get_name() == new {
                            cmd.exec(&rest1);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(SumCommand {}));

    terminal.run();
}
