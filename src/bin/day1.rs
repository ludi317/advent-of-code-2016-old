use nom::{branch::alt, sequence::preceded, IResult};
use nom::{bytes::complete::tag, character::complete::digit1};
use nom::{
    combinator::{map, map_res},
    multi::separated_list1,
};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day1.txt");
    // parse
    let dirs = directions(input).unwrap().1;
    // compute
    let (dist, visited) = walk(&dirs);
    println!("Part 1: Easter Bunny HQ is {} blocks away.", dist);
    println!(
        "Part 2: First twice-visited location is {} blocks away.",
        l1_dist(visited.unwrap())
    )
}

fn l1_dist(c: (isize, isize)) -> isize {
    c.0.abs() + c.1.abs()
}

fn walk(dirs: &Vec<Direction>) -> (isize, Option<(isize, isize)>) {
    let way: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut way_idx = 0;
    let mut pos = (0, 0);
    let mut pos_set: HashSet<(isize, isize)> = HashSet::new();
    pos_set.insert(pos);
    let mut visited: Option<(isize, isize)> = None;
    for dir in dirs {
        let (rotation, steps) = match dir {
            Direction::L(n) => (3, n),
            Direction::R(n) => (1, n),
        };
        way_idx = (way_idx + rotation) % 4;
        for _ in 1..=*steps {
            pos.0 += way[way_idx][0];
            pos.1 += way[way_idx][1];
            // println!("{:?}", pos);
            if visited.is_none() {
                if pos_set.contains(&pos) {
                    visited = Some(pos);
                } else {
                    pos_set.insert(pos);
                }
            }
        }
    }
    (l1_dist(pos), visited)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(
            preceded(tag("L"), map_res(digit1, |s: &str| s.parse::<isize>())),
            Direction::L,
        ),
        map(
            preceded(tag("R"), map_res(digit1, |s: &str| s.parse::<isize>())),
            Direction::R,
        ),
    ))(input)
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(tag(", "), direction)(input)
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    L(isize),
    R(isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            directions("R2, L3"),
            Ok(("", vec![Direction::R(2), Direction::L(3)]))
        );
        assert_eq!(
            directions("R2, R2, R2"),
            Ok(("", vec![Direction::R(2), Direction::R(2), Direction::R(2)]))
        );
        assert_eq!(
            directions("R5, L5, R5, R3"),
            Ok((
                "",
                vec![
                    Direction::R(5),
                    Direction::L(5),
                    Direction::R(5),
                    Direction::R(3)
                ]
            ))
        )
    }

    #[test]
    fn walking() {
        assert_eq!(walk(&vec![Direction::R(2), Direction::L(3)]).0, 5);
        assert_eq!(
            walk(&vec![Direction::R(2), Direction::R(2), Direction::R(2)]).0,
            2
        );
        assert_eq!(
            walk(&vec![
                Direction::R(5),
                Direction::L(5),
                Direction::R(5),
                Direction::R(3)
            ])
            .0,
            12
        )
    }

    #[test]
    fn visited_twice() {
        assert_eq!(
            l1_dist(
                walk(&vec![
                    Direction::R(8),
                    Direction::R(4),
                    Direction::R(4),
                    Direction::R(8)
                ])
                .1
                .unwrap()
            ),
            4
        )
    }
}
