use std::collections::HashSet;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum ShipType {
    Battleship,
    Cruiser,
}

#[derive(Debug)]
struct Ship {
    name: String,
    ship_type: ShipType,
    decks_amount: u8,
    occupied: HashSet<(u8, u8)>,
    is_live: bool,
}

impl Ship {
    fn new(name: String, ship_type: ShipType, occupied: HashSet<(u8, u8)>, is_live: bool) -> Self {
        let decks_amount = match ship_type {
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
        };
        Ship {
            name,
            ship_type,
            decks_amount,
            occupied,
            is_live,
        }
    }
}

struct FleetConfig {
    ship_counts: HashMap<ShipType, usize>,
}

impl FleetConfig {
    fn standart() -> Self {
        let mut ship_counts = HashMap::new();
        ship_counts.insert(ShipType::Battleship, 1);
        ship_counts.insert(ShipType::Cruiser, 2);
        FleetConfig {
            ship_counts
        }
    }
}

struct NavalFleet {
    ships: Vec<Ship>,
}

impl NavalFleet {
    fn new(config: &FleetConfig) -> Self {
        let mut fleet = Vec::new();
        
        for(ship_type, count) in &config.ship_counts {
            for i in 0..*count {
                let name = format!("{:?} {}", ship_type, i + 1);
                let ship = Ship::new(
                    name,
                    ship_type.clone(),
                    HashSet::new(),
                    true,
                );
                fleet.push(ship)
            }
        }
        NavalFleet { ships: fleet }
    }
}

fn main() {
    let mut user_1_fleet = NavalFleet::new(&FleetConfig::standart());
    
    for ship in &user_1_fleet.ships {
    
        for i in 0..ship.decks_amount {
            println!("{}", i);
        }
    }
    

}

fn print_field(ship: &Ship) {
    for n in (0..10).rev() {
        for o in 0..10 {
            let coords = (n, o);
            if ship.occupied.contains(&coords) {
                print!("■ ");
                continue;
            }
            print!("· ");
            if o == 9 {
                println!();
                continue;
            }
        }
    }
}

fn get_user_coordinates() -> (u8, u8) {
    let mut point = String::new();

    println!("Enter coordinates-> ");

    io::stdin()
        .read_line(&mut point)
        .expect("Failed to read line");

    let digits: Vec<u8> = point
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    (digits[0], digits[1])
}
