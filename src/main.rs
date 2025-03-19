use TerimalRtdm::*;

struct Creature {
    name: String,
    emoji: String,
    health: f32,
    xp: f32,
    attack: f32,
    special: f32,
    power: f32,
}

struct Item {
    id: f32,
    name: String,
    health_benefit: f32,
    cost: f32,
}

struct Player_data {
    name: String,
    creatures: Vec<Creature>,
    xp: f32,
    inventory: Vec<Item>,
}

fn print_keybind() {
    line(Position { x: 0, y: 0 }, "i - 💰 Inventory", "blue");
    line(Position { x: 0, y: 20 }, "m - 🏃 Move", "red");
    line(Position { x: 0, y: 32 }, "s - 🛒 Shop", "yellow");
}

fn main() {
    let catalog: Vec<Creature> = vec![
        Creature {
            name: "Tikashoe".to_string(),
            emoji: "👞".to_string(),
            health: 10.0,
            xp: 0.0,
            attack: 1.0,
            special: 2.0,
            power: 20.0,
        },
        Creature {
            name: "Troy".to_string(),
            emoji: "🧞‍♂️".to_string(),
            health: 5.0,
            xp: 0.0,
            attack: 3.0,
            special: 8.0,
            power: 20.0,
        },
        Creature {
            name: "Dalius".to_string(),
            emoji: "🐮".to_string(),
            health: 15.0,
            xp: 0.0,
            attack: 2.0,
            special: 4.0,
            power: 10.0,
        },
        ]

    let mut playing: bool = false;
    let mut has_starter: bool = false;

    clear();
    let mut app = App::new();

    raw_line("q 👈 to quit 🏃");
    raw_line("p 👈 to play 🎲");

    raw_mode(true);

    // app loop
    loop {
        clear();
        collect_presses(&mut app);

        if key_press(&app, "q") {
            clear();
            break;
        }

        if key_press(&app, "p") {
            print_keybind();
            playing = true;
        }
        if playing && !has_starter {
            line(Position { x: 0, y: 0 }, "Choose your pal!", "white");
            line(Position { x: 1, y: 0 }, "👞 Tikashoe", "red");
            line(Position { x: 1, y: 20 }, "Enjoy balance? ⚖️ (j)", "red");
            line(Position { x: 2, y: 0 }, "🧞‍♂️ Troy", "blue");
            line(Position { x: 2, y: 20 }, "Do you prefer war? 🔫 (k)", "blue");
            line(Position { x: 3, y: 0 }, "🐮 Dalius", "yellow");
            line(Position { x: 3, y: 20 }, "Are you a coward? (l)", "yellow");
        }
    }

    raw_mode(false);
}
