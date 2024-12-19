use aoc2024::CharacterField;
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const SIMPLE_INPUT2: &str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 18);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT2), 9);
    }

    #[test]
    fn test_getting_main_diagonals() {
        let input: &str = "abcd\nefgh\nijkl";

        assert_eq!(
            get_main_diagonals(input),
            &["d", "ch", "bgl", "afk", "ej", "i"]
        );
    }

    #[test]
    fn test_getting_cross_diagonals() {
        let input: &str = "abcd\nefgh\nijkl";

        assert_eq!(
            get_cross_diagonals(input),
            &["a", "eb", "ifc", "jgd", "kh", "l"]
        );
    }

    #[test]
    fn test_transposed_text() {
        let input: &str = "abcd\nefgh\nijkl";

        assert_eq!(get_transposed(input), &["aei", "bfj", "cgk", "dhl"]);
    }
}

fn get_main_diagonals(text: &str) -> Vec<String> {
    let lines: Vec<String> = text
        .lines()
        .filter(|line| line.trim() != "")
        .map(|x| x.to_string())
        .collect();
    let mut diagonals: Vec<String> = vec![];
    let x_iterator = (0..lines[0].len())
        .rev()
        .chain(std::iter::repeat(0).take(lines.len() - 1));
    let y_iterator = std::iter::repeat(0)
        .take(lines[0].len() - 1)
        .chain(0..lines.len());
    for (start_x, start_y) in x_iterator.zip(y_iterator) {
        let mut diagonal: String = "".to_string();
        for (x, y) in (start_x..lines[0].len()).zip(start_y..lines.len()) {
            diagonal.push(lines[y].as_bytes()[x] as char)
        }
        diagonals.push(diagonal);
    }
    diagonals
}

fn get_cross_diagonals(text: &str) -> Vec<String> {
    let lines: Vec<String> = text
        .lines()
        .filter(|line| line.trim() != "")
        .map(|x| x.to_string())
        .collect();
    let mut diagonals: Vec<String> = vec![];

    let x_iterator = std::iter::repeat(0)
        .take(lines.len() - 1)
        .chain(0..lines[0].len());
    let y_iterator =
        (0..lines.len()).chain(std::iter::repeat(lines.len() - 1).take(lines[0].len() - 1));

    for (start_x, start_y) in x_iterator.zip(y_iterator) {
        let mut diagonal: String = "".to_string();
        for (x, y) in (start_x..lines[0].len()).zip((0..start_y + 1).rev()) {
            diagonal.push(lines[y].as_bytes()[x] as char)
        }
        diagonals.push(diagonal);
    }
    diagonals
}

fn get_transposed(text: &str) -> Vec<String> {
    let lines: Vec<String> = text
        .lines()
        .filter(|line| line.trim() != "")
        .map(|x| x.to_string())
        .collect();

    let line_length = lines[0].len();
    let mut line_iterators: Vec<_> = lines.iter().map(|n| n.chars()).collect();

    (0..line_length)
        .map(|_| {
            let res: String = line_iterators
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<char>>()
                .into_iter()
                .collect();
            res
        })
        .collect::<Vec<String>>()
}

fn challenge1(challenge_input: &str) -> i32 {
    let mut hits: usize = challenge_input
        .lines()
        .filter(|line| line.trim() != "")
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum();

    let transposed_input = get_transposed(challenge_input);
    hits += transposed_input
        .iter()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum::<usize>();

    let rotated_input = get_main_diagonals(challenge_input);
    hits += rotated_input
        .iter()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum::<usize>();

    let rotated_input = get_cross_diagonals(challenge_input);
    hits += rotated_input
        .iter()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum::<usize>();

    hits as i32
}

fn challenge2(challenge_input: &str) -> i32 {
    let char_field: Vec<&str> = challenge_input.lines().collect();

    let mut mas_count = 0;
    for x in 0..char_field[0].len() as i32 {
        for y in 0..char_field.len() as i32 {
            if char_field.char_at(x, y).unwrap_or('.') == 'A' {
                let corners: Vec<char> = [(-1, -1), (1, 1), (-1, 1), (1, -1)]
                    .iter()
                    .map(|(dx, dy)| char_field.char_at(x + dx, y + dy).unwrap_or('.'))
                    .collect();

                if ((corners[0] == 'M' && corners[1] == 'S')
                    || (corners[0] == 'S' && corners[1] == 'M'))
                    && ((corners[2] == 'M' && corners[3] == 'S')
                        || (corners[2] == 'S' && corners[3] == 'M'))
                {
                    mas_count += 1;
                }
            }
        }
    }
    mas_count
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day4/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
