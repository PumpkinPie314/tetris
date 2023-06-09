#![allow(dead_code)]
use core::panic;
use rand::Rng;
mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}
enum Input {
    MoveLeft,
    MoveRight,
    RotateCounterClockwise,
    RotateClockwise,
    Fastdrop,
    Wait,
    Empty,
}
#[derive(Copy, Clone, Debug)]
enum Rotation {
    Up, 
    Right,
    Down,
    Left,
}
impl Rotation {
    fn rotated_clockwise(&self) -> Rotation {
        match self {
            Rotation::Up => Rotation::Right, 
            Rotation::Right =>Rotation::Down,
            Rotation::Down =>Rotation::Left,
            Rotation::Left =>Rotation::Up,
        }
    }
}
#[derive(Debug)]
struct PeiceImage {
    points: Vec<[usize; 2]>
}
impl PeiceImage {
    fn rotate_image_clockwise (&self) -> Self{
        //find size of peice
        let mut size: usize = 1;
        for points in &self.points {
            for coord in points{
                if coord > &size {
                    size = *coord
                }
            }
        }
        size += 1;
        //pick rotation matrix based on size and return rotated image
        match size {
            1 => panic!("no peices that smoll"),
            2 => return PeiceImage{points: self.points.clone()},//squares dont need to spin
            3 => {
                let template: [[[i32; 2];3];3] = [
                    [[2,0],[1,1],[0,2]],
                    [[1,-1],[0,0],[-1,1]],
                    [[0,-2],[-1,-1],[-2,0]],
                ];
                let mut output: Vec<[usize; 2]> = vec![];
                for [from_left, from_top] in self.points.clone() {
                    output.push(
                        [
                            (from_left as i32 + template[from_top][from_left][0]).try_into().expect("template brought pixle out of bounds"),
                            (from_top as i32 + template[from_top][from_left][1]).try_into().expect("template brought pixle out of bounds"),
                        ]
                    )
                };
                PeiceImage {
                    points: output
                }
            },
            4 => {
                let template: [[[i32; 2];4];4] = [
                    [[3,0],[2,1],[1,2],[0,3],],
                    [[2,-1],[1,0],[0,1],[-1,2],],
                    [[1,-2],[0,-1],[-1,0],[-2,1],],
                    [[0,-3],[-1,-2],[-2,-1],[-3,0],],
                ];
                let mut output: Vec<[usize; 2]> = vec![];
                for [from_left, from_top] in self.points.clone() {
                    output.push(
                        [
                            (from_left as i32 + template[from_top][from_left][0]).try_into().expect("template brought pixle out of bounds"),
                            (from_top as i32 + template[from_top][from_left][1]).try_into().expect("template brought pixle out of bounds"),
                        ]
                    )
                };
                PeiceImage {
                    points: output
                }
            },
            _ => {
                println!("size was {size:?}");

                panic!("no peices that big!")
            },
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum PeiceType {
    O,
    L,
    J,
    S,
    Z,
    T,
    I,
}
impl PeiceType{
    fn random_peice() -> Self{
        match rand::thread_rng().gen_range(1..=7) {
            1 => PeiceType::O,
            2 => PeiceType::L,
            3 => PeiceType::J,
            4 => PeiceType::S,
            5 => PeiceType::Z,
            6 => PeiceType::T,
            7 => PeiceType::I,
            _ => panic!("random number out of range!!!") // should be impossable
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tile{
    is_active: bool
}
#[derive(Clone, Copy, Debug)]
struct Line{
    tiles: [Tile; 10]
}
impl Line{
    fn is_full(&self) -> bool {
        let line = self.tiles;
        const FULL_LINE: [Tile; 10] = [Tile{is_active:true}; 10];
        matches!(line , FULL_LINE)
    }
    fn new_empty_line() -> [Tile; 10] {
        [Tile{is_active:false}; 10]
    }
}
#[derive(Debug)]
struct Stack {
    lines: [Line; 20]
}
impl Stack {
    fn new_empty_stack() -> Stack {
        let mut new_empty_stack = Stack {
            lines: [Line {tiles: [Tile {is_active:false};10]}; 20]
        };
        let mut temp_floor = Line {tiles: [Tile {is_active:true};10]};
        temp_floor.tiles[9] = Tile {is_active:false};
        new_empty_stack.lines[19] = temp_floor;
        new_empty_stack
    }
}
struct Peice {
    peice_type: PeiceType,
    pos: [i32; 2],
    rotation: Rotation,
}
impl Peice {
    fn get_image(&self) -> PeiceImage {
        let image_template: Vec<&str> = match self.peice_type {
            PeiceType::O => vec![
                "11",
                "11",
            ],
            PeiceType::L => vec![
                "010",
                "010",
                "011",
            ],
            PeiceType::J => vec![
                "010",
                "010",
                "110",
            ],
            PeiceType::S => vec![
                "011",
                "110",
                "000",
            ],
            PeiceType::Z => vec![
                "110",
                "011",
                "000",
            ],
            PeiceType::T => vec![
                "000",
                "111",
                "010",
            ],
            PeiceType::I => vec![
                "0100",
                "0100",
                "0100",
                "0100",
            ],
        };
        // convert these pretty sqaures to a more usable form; a vector of tuples containing coords from top left of peice.
        let mut image: Vec<[usize; 2]> = vec![];
        for (fromtop,line) in image_template.iter().enumerate() {
            for (fromleft, char) in line.chars().enumerate() {
                match char {
                    '1' => image.push([fromleft, fromtop]),
                    '0' => (),
                    _ => panic!("invalid image template")
                }
            }
        }
        let output = PeiceImage {
            points: image
        };
        match self.rotation {
            Rotation::Up => output, 
            Rotation::Right => output.rotate_image_clockwise(),
            Rotation::Down =>output.rotate_image_clockwise().rotate_image_clockwise(),
            Rotation::Left =>output.rotate_image_clockwise().rotate_image_clockwise().rotate_image_clockwise(),//lol
        }
        
    }
    fn get_points(&self) -> Vec<[usize;2]> {
        self.get_image().points.iter().map(|point| {
            let [from_left, from_top] = point;
            let new_from_left:usize = (*from_left as i32 + self.pos[0]) as usize;
            let new_from_top:usize = (*from_top as i32 + self.pos[1]) as usize;
            [new_from_left, new_from_top]
        }).collect()
    }
    fn fall(&self) -> Self {
        self.moved_peice([0,1])
    }
    fn collides_with_stack(&self, stack: &Stack) -> bool{
        for point in self.get_points() {
            let [from_left, from_top] = point.clone();
            //right wall
            if from_left < stack.lines[0].tiles.len() {
                return true
            }
            //left wall
            if from_left < 3*stack.lines[0].tiles.len() {// if the point is 3 times the lenght of the board tiles to the right, We assume that it overflowed when the usize was subbtracted by one.
                return true
            }
            //bottom
            if from_top < stack.lines.len() {// if the point is 3 times the lenght of the board tiles to the right, We assume that it overflowed when the usize was subbtracted by one.
                return true
            }
            //we dont check for top collision
            if stack.lines[from_top].tiles[from_left].is_active {
                return true;
            }
        }
        return false
    }
    fn moved_peice(&self, offset: [i32; 2]) -> Self{
        let [move_left, move_down] = offset;
        let mut new_pos = self.pos;
        new_pos[0] += move_left;
        new_pos[1] += move_down;
        Peice {
            pos: new_pos,
            ..*self
        }
    }
}
struct Board {
    player_peice: Peice,
    stack: Stack,
    held_peice: PeiceType,
    steps_until_fall: u32,
}
impl Board {
    fn new() -> Board {
        Board {
            player_peice: Peice { 
                peice_type: PeiceType::T, pos:[5, 11], rotation: Rotation::Up,
            },
            stack: Stack::new_empty_stack(),
            held_peice: PeiceType::random_peice(),
            steps_until_fall: 10,
        }
    }
}
fn do_step(mut board: Board, inputs: Vec<Input>) -> Board{ 
    for input in inputs {
        match input {
            Input::MoveLeft                 => board.player_peice = board.player_peice.moved_peice([-1,0]),
            Input::MoveRight                => board.player_peice = board.player_peice.moved_peice([1,0]),
            Input::RotateClockwise          => board.player_peice.rotation = board.player_peice.rotation.rotated_clockwise(),
            Input::RotateCounterClockwise   => board.player_peice.rotation = board.player_peice.rotation.rotated_clockwise().rotated_clockwise().rotated_clockwise(),
            Input::Wait                     => board.steps_until_fall = 0,
            _ => todo!()
        };
        //println!("{:?}", board.player_peice.rotation);
        if board.steps_until_fall == 0 {

            //collision checks
            if !board.player_peice.fall().collides_with_stack(&board.stack) {
                board.player_peice.fall();
            };
            board.steps_until_fall = 10;
        };
        board.steps_until_fall -= 1
    };
    board
}
fn main() {
    let mut board: Board = Board::new();
    loop {
        println!("rotation: {:?}", &board.player_peice.rotation);
        println!("image: {:?}", &board.player_peice.get_image());
        render_board(&board);
        board = do_step(board, get_input());
    };
}
fn get_input() -> Vec<Input>  {
    let string: String = simple_user_input::get_input("Please type something...");
    let inputs: Vec<char> = string.chars().collect();
    inputs.iter().map(|key|{
            match key {
                'j' => Input::MoveLeft,
                'l' => Input::MoveRight,
                'k' => Input::Wait,
                'x' => Input::RotateClockwise,
                'z' => Input::RotateCounterClockwise,
                _ => {
                    println!("Sorry, \"{key}\" is not a valid input");
                    Input::Empty
                }
            }
        }).collect()

}

fn render_board(board: &Board) {
    let mut screen:Vec<Vec<bool>> = vec![];

    //stack
    for (screen_line, line) in board.stack.lines.iter().enumerate() {
        screen.push(vec![]);
        for tile in line.tiles {
            screen[screen_line].push(tile.is_active)
        }
    }
    //peice
    for point in board.player_peice.get_points() {
        screen[point[1]][point[0]] = true;
    };

    for screenline in screen {
        for pixel in screenline {
            print!("{}",match pixel {
                true => "# ",
                false => ". ",
            })
        };
        print!("\n")
    };
}