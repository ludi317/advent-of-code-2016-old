use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

fn main() {
    let input = include_str!("../input/day4.txt");
    let rooms = rooms(input).unwrap().1;
    let sum = rooms
        .iter()
        .filter(|r| checksum(r.name) == r.checksum)
        .map(|r| r.sector_id)
        .sum::<usize>();
    println!("Part 1: Sum of sector IDs of valid rooms: {}", sum);
    let r: Vec<_> = rooms
        .iter()
        .filter(|r| checksum(r.name) == r.checksum)
        .filter(|r| cipher(r.name, r.sector_id).contains("north"))
        .collect();

    println!(
        "Part 2: sector ID of north pole storage: {:?}",
        r[0].sector_id
    )
}

#[derive(Debug, PartialEq)]
struct Room<'a> {
    name: &'a str,
    sector_id: usize,
    checksum: &'a str,
}

#[derive(Copy, Clone)]
struct Letter {
    c: char,
    n: usize,
}

fn cipher(name: &str, mut rot: usize) -> String {
    rot %= 26;
    let rot_u8 = rot as u8;
    let a = 'a' as u8;
    name.chars()
        .map(|c| {
            if c.is_alphabetic() {
                ((c as u8 - a + rot_u8) % 26 + a) as char
            } else {
                ' '
            }
        })
        .collect()
}

fn checksum(name: &str) -> String {
    // get the frequency of each letter in the name
    let mut freq = name.chars().filter(|c| c.is_alphabetic()).fold(
        [Letter { c: ' ', n: 0 }; 26],
        |mut freq, c| {
            let idx = c as usize - 'a' as usize;
            freq[idx].n += 1;
            freq[idx].c = c;
            freq
        },
    );
    // sort the letters by frequency, then alphabetically
    freq.sort_by(|a, b| b.n.cmp(&a.n).then(a.c.cmp(&b.c)));

    freq.iter().take(5).map(|l| l.c).collect()
}

fn room(input: &str) -> IResult<&str, Room> {
    let (input, name) = take_while(|c: char| c.is_alphabetic() || c == '-')(input)?;
    let (input, sector_id) = digit1(input)?;
    let (input, checksum) = delimited(tag("["), alpha1, tag("]"))(input)?;

    let sector_id = sector_id.parse::<usize>().unwrap();

    Ok((
        input,
        Room {
            name,
            sector_id,
            checksum,
        },
    ))
}

fn rooms(input: &str) -> IResult<&str, Vec<Room>> {
    separated_list1(line_ending, room)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_room() {
        let s = r#"aaaaa-bbb-z-y-x-123[abxyz]"#;
        assert_eq!(
            room(s).unwrap().1,
            Room {
                name: "aaaaa-bbb-z-y-x-",
                sector_id: 123,
                checksum: "abxyz"
            }
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("aaaaa-bbb-z-y-x-"), "abxyz");
        assert_eq!(checksum("a-b-c-d-e-f-g-h-"), "abcde");
        assert_eq!(checksum("not-a-real-room-"), "oarel");
    }

    #[test]
    fn test_cipher() {
        assert_eq!(cipher("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
    }
}
