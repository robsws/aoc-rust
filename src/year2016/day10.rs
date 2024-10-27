use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let mut system = balance_bots::System::new_from_rules(&lines);
    println!("{}", system.run(17, 61));
}

pub fn part2(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let mut system = balance_bots::System::new_from_rules(&lines);
    system.run(-1, -1);
    println!("{}", system.get_output(0) * system.get_output(1) * system.get_output(2))
}

mod balance_bots {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::cmp::{min, max};

    lazy_static! {
        static ref CHIP_START_REGEX: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        static ref BOT_RULE_REGEX: Regex = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
    }

    #[derive (Clone)]
    enum Destination {
        Bot(i32),
        Output(i32)
    }

    fn type_and_id_to_destination(dest_type: &str, dest_id: i32) -> Destination {
        match dest_type {
            "bot" => Destination::Bot(dest_id),
            "output" => Destination::Output(dest_id),
            _ => {
                panic!("Unsupported destination type: {}", dest_type)
            }
        }
    }

    #[derive (Clone)]
    struct Bot {
        chips: Vec<i32>,
        low_dest: Destination,
        high_dest: Destination
    }

    impl Bot {

        pub fn new(low_dest: Destination, high_dest: Destination) -> Bot {
            Bot {
                chips: Vec::<i32>::new(),
                low_dest,
                high_dest
            }
        }

        pub fn add_chip(&mut self, chip: i32) {
            if self.chips.len() >= 2 {
                panic!("Tried to give bot more than two chips.");
            }
            self.chips.push(chip);
        }

        pub fn ready_to_proceed(&self) -> bool {
            return self.chips.len() == 2;
        }

    }

    pub struct System {
        bots: HashMap<i32, Bot>,
        outputs: HashMap<i32, i32>
    }

    impl System {

        pub fn new() -> System {
            System {
                bots: HashMap::<i32, Bot>::new(),
                outputs: HashMap::<i32, i32>::new()
            }
        }

        pub fn new_from_rules(rules: &Vec<String>) -> System {
            
            let mut system = System::new();

            // parse all of the bot rules first so we can initialise them
            for rule in rules {
                
                if let Some(captures) = BOT_RULE_REGEX.captures(rule) {

                    // Extract values from the regex captures
                    let bot_id = captures.get(1).unwrap().as_str();
                    let bot_id: i32 = bot_id.parse().expect("Bot ID value not an integer.");
                    let low_dest_type = captures.get(2).unwrap().as_str();
                    let low_dest_id = captures.get(3).unwrap().as_str();
                    let low_dest_id: i32 = low_dest_id.parse().expect("Lower destination ID value not an integer.");
                    let high_dest_type = captures.get(4).unwrap().as_str();
                    let high_dest_id = captures.get(5).unwrap().as_str();
                    let high_dest_id: i32 = high_dest_id.parse().expect("Upper destination ID value not an integer.");
                    let low_dest = type_and_id_to_destination(low_dest_type, low_dest_id);
                    let high_dest = type_and_id_to_destination(high_dest_type, high_dest_id);

                    // Create any new outputs in the system
                    if let Destination::Output(id) = low_dest {
                        system.outputs.insert(id, -1);
                    }
                    if let Destination::Output(id) = high_dest {
                        system.outputs.insert(id, -1);
                    }
                    
                    // Create the bot and add it to the system
                    let bot = Bot::new(low_dest, high_dest);
                    system.bots.insert(bot_id, bot);
                }
            }
            
            // now parse all of the initialisation rules
            for rule in rules {
                
                if let Some(captures) = CHIP_START_REGEX.captures(rule) {

                    // Extract values from the regex captures
                    let chip = captures.get(1).unwrap().as_str();
                    let chip: i32 = chip.parse().expect("Chip value not an integer.");
                    let bot_id = captures.get(2).unwrap().as_str();
                    let bot_id: i32 = bot_id.parse().expect("Bot ID value not an integer.");

                    // Add the chip into the system
                    system.bots.get_mut(&bot_id).expect("Invalid bot ID when trying to assign chip.").add_chip(chip);
                }
            }
            system
        }

        pub fn run(&mut self, stop_low: i32, stop_high: i32) -> i32 {
            // On each pass of the loop, check for bots that are ready to proceed and resolve them.
            let mut changed_state = true;
            while changed_state {
                changed_state = false;
                let mut new_bots = self.bots.clone();
                for (bot_id, bot) in self.bots.iter() {
                    if bot.ready_to_proceed() {
                        let low_chip = min(bot.chips[0], bot.chips[1]);
                        let high_chip = max(bot.chips[0], bot.chips[1]);

                        if low_chip == stop_low && high_chip == stop_high {
                            return *bot_id;
                        }

                        match bot.low_dest {
                            Destination::Bot(id) => {
                                new_bots
                                    .get_mut(&id)
                                    .expect("Invalid bot ID in rule")
                                    .add_chip(low_chip);
                            },
                            Destination::Output(id) => {
                                self.outputs.insert(id, low_chip);
                            }
                        }
                        match bot.high_dest {
                            Destination::Bot(id) => {
                                new_bots
                                    .get_mut(&id)
                                    .expect("Invalid bot ID in rule")
                                    .add_chip(high_chip);
                            },
                            Destination::Output(id) => {
                                self.outputs.insert(id, high_chip);
                            }
                        }

                        new_bots.get_mut(bot_id).unwrap().chips.clear();
                        changed_state = true;
                    }
                }
                self.bots = new_bots;
            }
            return -1;
        }

        pub fn get_output(&self, id: i32) -> i32 {
            *self.outputs.get(&id).expect("Asked for non-existent output.")
        }
    }

}
