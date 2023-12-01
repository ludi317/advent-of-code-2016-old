use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

fn main() {
    let input = include_str!("../input/day7.txt");
    println!("Part 1: Number that support TLS: {}", num_tls(input));
    println!("Part 2: Number that support SSL: {}", num_ssl(input))
}

fn num_ssl(s: &str) -> usize {
    let addrs = addresses(s).unwrap().1;
    addrs
        .iter()
        .filter(|a| {
            a.brackets.iter().any(|b| {
                let babs = get_babs(b);
                a.no_brackets
                    .iter()
                    .any(|n| babs.iter().any(|bab| n.contains(bab)))
            })
        })
        .count()
}

fn num_tls(s: &str) -> usize {
    let addrs = addresses(s).unwrap().1;
    addrs
        .iter()
        .filter(|a| a.no_brackets.iter().any(is_abba))
        .filter(|a| !a.brackets.iter().any(is_abba))
        .count()
}

#[derive(Debug, PartialEq)]
struct Addr<'a> {
    brackets: Vec<&'a str>,
    no_brackets: Vec<&'a str>,
}

fn get_babs(s: &str) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let bs = s.as_bytes();
    for i in 0..=s.len() - 3 {
        // find aba pattern
        if bs[i] != bs[i + 1] && bs[i] == bs[i + 2] {
            let a: char = s.chars().nth(i).unwrap();
            let b: char = s.chars().nth(i + 1).unwrap();
            // bab pattern
            let bab = format!("{}{}{}", b, a, b);
            out.push(bab);
        }
    }
    out
}

fn is_abba(s: &&str) -> bool {
    let b = s.as_bytes();
    for i in 1..=s.len() - 3 {
        if b[i] == b[i + 1] && b[i - 1] == b[i + 2] && b[i] != b[i - 1] {
            return true;
        }
    }
    false
}

fn address(mut input: &str) -> IResult<&str, Addr> {
    let mut brackets = vec![];
    let mut no_brackets = vec![];

    let mut a = "";
    loop {
        (input, a) = take_while(|c: char| c != '[' && c != '\n')(input)?;
        no_brackets.push(a);

        if input.starts_with('[') {
            (input, a) = delimited(tag("["), alpha1, tag("]"))(input)?;
            brackets.push(a);
        } else {
            break;
        }
    }

    Ok((
        input,
        Addr {
            no_brackets,
            brackets,
        },
    ))
}

fn addresses(input: &str) -> IResult<&str, Vec<Addr>> {
    separated_list1(line_ending, address)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let s = r#"abba[mnop]qrst
abc[def]ghi"#;
        assert_eq!(
            addresses(s).unwrap().1,
            vec![
                Addr {
                    brackets: vec!["mnop"],
                    no_brackets: vec!["abba", "qrst"]
                },
                Addr {
                    brackets: vec!["def"],
                    no_brackets: vec!["abc", "ghi"]
                },
            ]
        )
    }

    #[test]
    fn test_num_support_tls() {
        assert_eq!(num_tls("abba[mnop]qrst"), 1);
        assert_eq!(num_tls("abcd[bddb]xyyx"), 0);
        assert_eq!(num_tls("aaaa[qwer]tyui"), 0);
        assert_eq!(num_tls("ioxxoj[asdfgh]zxcvbn"), 1);
    }

    #[test]
    fn test_num_support_ssl() {
        assert_eq!(num_ssl("aba[bab]xyz"), 1);
        assert_eq!(num_ssl("xyx[xyx]xyx"), 0);
        assert_eq!(num_ssl("aaa[kek]eke"), 1);
        assert_eq!(num_ssl("zazbz[bzb]cdb"), 1);
    }
}
