use std::fmt;

struct Coordinate { 
    x: i32,
    y: i32
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate(x:{} y:{})", self.x, self.y)
    }
}

enum Direction {
    Horizontal,
    Vertical
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Direction::Horizontal => write!(f, "Horizontal"),
           Direction::Vertical => write!(f, "Vertical")
        }
    }
}

struct Wire {
    // wire start is always the x,y coordinates of the left point for horizontal wires
    // and bottom point for vertical wires.
    start : Coordinate,
    direction : Direction,
    length : i32
}

impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wire(start: {}, direction: {}, length {})", self.start, self.direction, self.length)
    }
}


fn wire_new(start_x: i32, start_y: i32, direction: Direction, length: i32) -> Wire
{
    Wire { 
        start: Coordinate {
            x : start_x,
            y : start_y
        },
        direction: direction,
        length: length
    }
}

fn perpindicular(wire1: &Wire, wire2: &Wire) -> bool
{
    match (&wire1.direction, &wire2.direction) {
        (Direction::Horizontal, Direction::Vertical) => true,
        (Direction::Vertical, Direction::Horizontal) => true,
        _ => false
    }
}

fn wire_intersection(wire1 : &Wire, wire2 : &Wire) -> Option<Coordinate> {
    if perpindicular(wire1, wire2) {
        println!("wire1 = {}\nwire2 = {}", wire1, wire2);
        let horiz = match wire1.direction {
             Direction::Horizontal =>  wire1,
             Direction::Vertical => wire2
        };
        let vert = match wire1.direction {
             Direction::Horizontal => wire2,
             Direction::Vertical => wire1
        };
        let horiz_end = wire_end(horiz);
        let vert_end = wire_end(vert);
        println!("horiz_end = {} vert_end = {}", horiz_end, vert_end);
        if (horiz.start.x <= vert.start.x && vert.start.x <= horiz_end.x) &&
           (vert.start.y <= vert.start.y && vert.start.y <= vert_end.y) {
               // vertical line x is between the start/end of horizontal line
               // and then horizontal line is between the start/end y of the vertical line
               Some(Coordinate { x : vert.start.x, y : horiz.start.y })
           }
        else
        {
            None
        }
    }
    else
    {
        None
    }
}

fn wire_end(wire : &Wire) -> Coordinate {
    match wire.direction {
        Direction::Horizontal => Coordinate { x: wire.start.x + wire.length, y: wire.start.y },
        Direction::Vertical => Coordinate { x: wire.start.x, y: wire.start.y + wire.length }
    }
}


fn main() {
    let wire1 = wire_new(5, 5, Direction::Horizontal, 10);
    let wire2 = wire_new(7, 1, Direction::Vertical, 10);
    let intersection = match wire_intersection(&wire1, &wire2) {
        Some(x) => x,
        None => panic!("No intersection")
    };
    println!("{}", intersection);
}
