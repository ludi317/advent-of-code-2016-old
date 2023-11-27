fn main() {
    let input = include_str!("../input/day3.txt");
    let count = valid_triangle_count(input);
    println!("Part 1: Row-wise valid triangle count: {}", count);
    let count = valid_triangle_count_part2(input);
    println!("Part 2: Column-wise valid triangle count: {}", count);
}

fn valid_triangle_count_part2(s: &str) -> usize {
    s.lines()
        .collect::<Vec<_>>() // Collect lines into a Vec to allow chunking
        .chunks(3) // Process three lines at a time
        .flat_map(|chunk| {
            // Create an iterator of tuples, each representing a column from the input
            (0..3).map(|i| {
                chunk
                    .iter()
                    .map(|line| {
                        line.split_whitespace()
                            .nth(i)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap()
                    })
                    .collect()
            })
        })
        .filter(|v| is_valid_triangle(v)) // Use is_valid_triangle to filter valid triangles
        .count()
}

fn is_valid_triangle(v: &Vec<usize>) -> bool {
    return v[0] + v[1] > v[2] && v[0] + v[2] > v[1] && v[1] + v[2] > v[0];
}

fn valid_triangle_count(s: &str) -> usize {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect()
        })
        .filter(|v| is_valid_triangle(v))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_triangle_row_wise() {
        let s = r#"5 10 35"#;
        assert_eq!(valid_triangle_count(s), 0);
    }

    #[test]
    fn triangle_col_wise() {
        let s = r#"101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603"#;
        assert_eq!(valid_triangle_count_part2(s), 6);
    }
}
