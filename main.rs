use std::collections::LinkedList;
use std::fmt;

#[derive(Debug)]
enum Who {
    ColonelMustard,
    MissPeacock,
    ProfessorPlum,
    MrsWhite,
    MrGreen,
    MissScarlet,
}

impl std::fmt::Display for Who {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum What {
    Knife,
    Revolver,
    Wrench,
    Rope,
    Candlestick,
    LeadPipe,
}

#[derive(Debug)]
enum Where {
    Conservatory,
}

#[derive(Debug)]
struct Turn {
    player: Player,
    who: Who,
    what: What,
    place: Where,
    ended_by: Option<Player>,
}

#[derive(Debug)]
struct Player {
    who: Who,
}

trait Display {
    fn print(&self);
}

struct Matrix {}

impl Display for Matrix {
    fn print(&self) {
        println!("{}", "matrix");
    }
}

fn main() {
    println!("Clue Bot");

    game_loop();
}

fn game_loop() {
    let mut players: LinkedList<Player> = LinkedList::new();
    let mut turns: LinkedList<Turn> = LinkedList::new();
    let mut matrix: Matrix = Matrix {};
    let game_over = false;

    while !game_over {
        let turn = get_input();
        turns.push_back(turn);

        print_turns(&turns);
        matrix.print()
    }
}

fn get_input() -> Turn {
    let mut line = String::new();
    println!("What item was suggested?");
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", line);

    return Turn {
        player: Player {
            who: Who::ColonelMustard,
        },
        who: Who::ColonelMustard,
        what: What::LeadPipe,
        place: Where::Conservatory,
        ended_by: None,
    };
}

fn print_turns(turns: &LinkedList<Turn>) {
    for turn in turns {
        println!(
            "{} said \"{} did it in the {:?} with the {:?}\"",
            turn.player.who, turn.who, turn.place, turn.what
        );
    }
}
