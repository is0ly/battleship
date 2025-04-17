use std::collections::HashSet;
use std::io;

#[derive(Debug)]
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

fn main() {
    let mut my_ship = Ship::new(
        String::from("Battleship-1"),
        ShipType::Battleship,
        HashSet::new(),
        true,
    );

    for _n in 0..my_ship.decks_amount {
        my_ship.occupied.insert(get_user_coordinates());
        print_field(&my_ship);
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
