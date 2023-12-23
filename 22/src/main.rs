use std::{fs::read_to_string, collections::BTreeMap};

fn parse_coords(coords: &str) -> (i32, i32, i32) {
    let mut coords = coords.split(",").map(|coord| coord.parse::<i32>().unwrap());
    let x = coords.next().unwrap();
    let y = coords.next().unwrap();
    let z = coords.next().unwrap();
    (x, y, z)
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    id: usize,
    coords1: (i32, i32, i32),
    coords2: (i32, i32, i32),
}

impl Brick {
    fn new(id: usize, coords1: (i32, i32, i32), coords2: (i32, i32, i32)) -> Self {
        Self { id, coords1, coords2 }
    }

    fn intersects(&self, other: &Self) -> bool {
        let (x1, y1, z1) = self.coords1;
        let (x2, y2, z2) = self.coords2;
        let highx = x1.max(x2);
        let lowx = x1.min(x2);
        let highy = y1.max(y2);
        let lowy = y1.min(y2);
        let highz = z1.max(z2);
        let lowz = z1.min(z2);
        let (x3, y3, z3) = other.coords1;
        let (x4, y4, z4) = other.coords2;
        let otherhighx = x3.max(x4);
        let otherlowx = x3.min(x4);
        let otherhighy = y3.max(y4);
        let otherlowy = y3.min(y4);
        let otherhighz = z3.max(z4);
        let otherlowz = z3.min(z4);
        let xintersects = (otherlowx >= lowx && otherlowx <= highx) || (otherhighx >= lowx && otherhighx <= highx) || (lowx >= otherlowx && lowx <= otherhighx) || (highx >= otherlowx && highx <= otherhighx);
        let yintersects = (otherlowy >= lowy && otherlowy <= highy) || (otherhighy >= lowy && otherhighy <= highy) || (lowy >= otherlowy && lowy <= otherhighy) || (highy >= otherlowy && highy <= otherhighy);
        let zintersects = (otherlowz >= lowz && otherlowz <= highz) || (otherhighz >= lowz && otherhighz <= highz) || (lowz >= otherlowz && lowz <= otherhighz) || (highz >= otherlowz && highz <= otherhighz);
        xintersects && yintersects && zintersects
    }

    fn move_by(&self, x: i32, y: i32, z: i32) -> Self {
        let (x1, y1, z1) = self.coords1;
        let (x2, y2, z2) = self.coords2;
        Self {
            id: self.id,
            coords1: (x1 + x, y1 + y, z1 + z),
            coords2: (x2 + x, y2 + y, z2 + z),
        }
    }

    fn get_height_above_ground(&self) -> i32 {
        let (_, _, z1) = self.coords1;
        let (_, _, z2) = self.coords2;
        z1.min(z2)
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once("~").unwrap())
        .enumerate()
        .map(|(id, (coords1, coords2))| Brick::new(id, parse_coords(coords1), parse_coords(coords2)))
        .collect::<Vec<_>>();

    let mut bricks = input.clone();

    let mut change = true;
    while change {
        let mut new_bricks = Vec::with_capacity(bricks.len());
        change = false;
        for brick in &bricks {
            let new_brick = if brick.get_height_above_ground() > 1 {
                let potential_brick = brick.move_by(0, 0, -1);
                let mut intersects = false;
                for other_brick in bricks.iter().filter(|b| b.id != brick.id) {
                    if potential_brick.intersects(other_brick) {
                        intersects = true;
                        break;
                    }
                }
                if intersects {
                    brick.clone()
                } else {
                    change = true;
                    potential_brick
                }
            } else {
                brick.clone()
            };
            new_bricks.push(new_brick);
        }
        bricks = new_bricks;
    }

    let mut supported_by: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut supports: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        let down_brick = brick.move_by(0, 0, -1);
        for (j, other_brick) in bricks.iter().enumerate() {
            if i != j && down_brick.intersects(other_brick) {
                supported_by.entry(i).or_default().push(j);
                supports.entry(j).or_default().push(i);
            }
        }
    }

    // print supports
    /*for (i, brick) in bricks.iter().enumerate() {
        println!("{}: {:?}", i, brick);
        println!("  Supported by: {:?}", supported_by.get(&i));
        println!("  Supports: {:?}", supports.get(&i));
    }*/

    let result1 = (0..bricks.len())
        .filter(|i| supports.get(i)
            .map_or(true, |s| s.iter().all(|j| supported_by[j].len() > 1)))
        .count();

    println!("Part 1: {}", result1);

    let mut result2 = 0;

    // add imaginary "ground brick" to supported by for all bricks with height = 1
    for (i, brick) in bricks.iter().enumerate() {
        if brick.get_height_above_ground() == 1 {
            supported_by.entry(i).or_default().push(usize::MAX);
        }
    }

    for brick in &bricks {
        let mut supported_by_temp = supported_by.clone();
        // remove current brick from supported_by
        supported_by_temp.iter_mut().for_each(|(_, l)| l.retain(|i| *i != brick.id));
        let mut change = true;
        while change {
            change = false;
            // Check if any bricks are now unsupported and remove them from supports
            let unsupported = supported_by_temp.iter().filter(|(_, l)| l.is_empty()).map(|(i, _)| *i).collect::<Vec<_>>();
            for brick_id in &unsupported {
                supported_by_temp.iter_mut().for_each(|l| l.1.retain(|i| *i != *brick_id));
                supported_by_temp.remove(&brick_id);
                change = true;
            }
            result2 += unsupported.len();
        }
    }

    println!("Part 2: {}", result2);

}
