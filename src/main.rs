use TerimalRtdm::*;

struct Creature {
    name: String,
    health: f32,
    xp: f32,
    defense: f32,
    attack: f32,
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
        }
    }

    raw_mode(false);
}
