use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Entity {
    Bot(usize),
    Output(usize),
}

#[derive(Clone, Copy)]
struct PassRule {
    high_target: Entity,
    low_target: Entity,
}

struct Bot {
    id: usize,
    chips: Vec<u32>,
    pass_rules: Option<PassRule>,
}

impl Bot {
    fn new(id: usize) -> Self {
        Bot { id, chips: Vec::new(), pass_rules: None }
    }
}

struct Factory {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, Vec<u32>>,
}

impl Factory {
    fn new() -> Self {
        Factory {
            bots: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    /// Define a rule for bot `bot_id` about where to send its high/low chips.
    fn set_rule(&mut self, bot_id: usize, high: Entity, low: Entity) {
        let bot = self.bots.entry(bot_id).or_insert_with(|| Bot::new(bot_id));
        bot.pass_rules = Some(PassRule {
            high_target: high,
            low_target: low,
        });
    }

    /// Give a chip to a bot; if it now has 2, process it immediately (recursively).
    fn give_chip(&mut self, bot_id: usize, chip: u32) {
        // 1) Push the chip
        let bot = self.bots.entry(bot_id).or_insert_with(|| Bot::new(bot_id));
        bot.chips.push(chip);

        // 2) If exactly two, pull data out in its own scope so the borrow ends
        if bot.chips.len() == 2 {
            let (high, low, high_target, low_target) = {
                let bot = self.bots.get_mut(&bot_id).unwrap();
                // determine high/low
                let high = *bot.chips.iter().max().unwrap();
                let low  = *bot.chips.iter().min().unwrap();

                if high == 61 && low == 17 {
                    println!("bot {} has 61 and 17", bot_id);
                }
                // clear for next time
                bot.chips.clear();
                let rules = bot.pass_rules.unwrap();
                (high, low, rules.high_target, rules.low_target)
            }; // ← mutable borrow of this bot ends right here

            // 3) Recursively send them on
            self.dispatch(high_target, high);
            self.dispatch(low_target,  low);
        }
        else if bot.chips.len() > 2 {
            panic!("Bot {} has more than 2 chips", bot_id);
        }
    }

    /// Helper to send a chip to either a bot or an output
    fn dispatch(&mut self, target: Entity, chip: u32) {
        match target {
            Entity::Bot(id)    => self.give_chip(id, chip),
            Entity::Output(id) => {
                self.outputs.entry(id).or_default().push(chip);
            }
        }
    }
}

// —— example usage ——
fn main() {
    let mut factory = Factory::new();

    // set up rules: bot 2 gives high to bot 1, low to output 0
    factory.set_rule(2, Entity::Bot(1), Entity::Output(0));
    // bot 1 gives both to outputs
    factory.set_rule(1, Entity::Output(1), Entity::Output(2));

    // feed chips
    factory.give_chip(2, 5);
    factory.give_chip(2, 3);

    println!("outputs: {:?}", factory.outputs);
}

