fn main() {
    let input = include_str!("../input/day6.txt");
    println!("Part 1: {}", code_most(input));
    println!("Part 2: {}", code_least(input));
}

fn code_least(s: &str) -> String {
    let line_length = s.lines().next().unwrap().chars().count();
    let mut freqs: Vec<Vec<usize>> = vec![vec![0; 26]; line_length];

    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            freqs[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    let code: String = freqs
        .iter()
        .map(|frequencies| {
            frequencies
                .iter()
                .enumerate()
                .filter(|&(_, f)| f != &0usize)
                .min_by_key(|&(_, freq)| freq)
                .map(|(index, _)| (index as u8 + b'a') as char)
                .unwrap()
        })
        .collect();

    code
}

fn code_most(s: &str) -> String {
    let line_length = s.lines().next().unwrap().chars().count();
    let mut freqs: Vec<Vec<usize>> = vec![vec![0; 26]; line_length];

    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            freqs[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    let code: String = freqs
        .iter()
        .map(|frequencies| {
            frequencies
                .iter()
                .enumerate()
                .max_by_key(|&(_, freq)| freq)
                .map(|(index, _)| (index as u8 + b'a') as char)
                .unwrap()
        })
        .collect();

    code
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_code_most() {
        assert_eq!(
            code_most(
                r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#
            ),
            "easter"
        )
    }

    #[test]
    fn test_code_least() {
        assert_eq!(
            code_least(
                r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#
            ),
            "advent"
        )
    }
}
