use aoc2024::{Field, Location};
use std::error::Error;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const SIMPLE_INPUT2: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn test_simple_input_part1() {
        assert_eq!(challenge1(SIMPLE_INPUT), 140);
    }

    #[test]
    fn test_simple_input_part1_slightly_bigger() {
        assert_eq!(challenge1(SIMPLE_INPUT2), 1930);
    }

    #[test]
    fn test_simple_input_part2() {
        assert_eq!(challenge2(SIMPLE_INPUT), 0);
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Plot {
    plant_type: char,
    assigned: bool, // marker, to indicate whether this plot was assigned to a region
}

#[derive(PartialEq, Debug)]
struct Region {
    fences: usize,
    area: usize,
}

type Garden = Field<Plot>;

fn collect_region(garden: &mut Garden, regions: &mut Vec<Region>, position: &Location) {
    let mut visited_positions: Vec<Location> = vec![];
    let mut search_front: Vec<Location> = vec![*position];

    let mut new_region = Region { area: 0, fences: 1 };
    let plant_type = garden.at(&position).unwrap().plant_type;

    while !search_front.is_empty() {
        let position = search_front.pop().unwrap();

        new_region.area += 1;
        visited_positions.push(position);
        garden.at_mut(&position).unwrap().assigned = true;

        for (location, neighbor) in garden.actual_neighbors(&position) {
            if visited_positions.contains(&location) || search_front.contains(&location) {
                // !! if the neighbor is already in the search front, we can also skip it
                continue;
            }

            if neighbor.assigned {
                // This neighbor already belongs to a region
                continue;
            }

            if neighbor.plant_type == plant_type {
                search_front.push(location);
            }
        }
    }

    // Go through all the plots of the previousy collected region
    new_region.fences = visited_positions
        .iter()
        .map(|position| {
            // Go through all the neighbors of one region and find those neighbors, that have
            // a different plant type or aren't even on the map
            garden
                .all_neighbors(position)
                .filter(|(_location, maybe_plot)| {
                    if let Some(plot) = maybe_plot {
                        plot.plant_type != plant_type
                    } else {
                        true
                    }
                })
                .count()
        })
        .sum();

    regions.push(new_region);
}

fn challenge1(challenge_input: &str) -> i64 {
    let mut garden: Garden = Field::new(
        challenge_input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|plant_id| Plot {
                        plant_type: plant_id,
                        assigned: false,
                    })
                    .collect()
            })
            .collect(),
    );

    let mut regions: Vec<Region> = vec![];
    for (line_no, line) in garden.clone().iter().enumerate() {
        for (pos, _plot) in line.iter().enumerate() {
            // We are not using the plot from iterating the garden because this is just
            // an unmodified clone of the garden
            if let Some(plot) = garden.at(&Location {
                column: pos as i32,
                row: line_no as i32,
            }) {
                if !plot.assigned {
                    collect_region(
                        &mut garden,
                        &mut regions,
                        &Location {
                            row: line_no as i32,
                            column: pos as i32,
                        },
                    );
                }
                // Otherwise the plot is already assigned
            }
        }
    }
    regions
        .iter()
        .map(|region| (region.area * region.fences) as i64)
        .sum()
}

fn challenge2(_challenge_input: &str) -> i64 {
    42
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_input = read_to_string("input_data/day12/input.txt")?;

    let result1 = challenge1(&test_input);
    let result2 = challenge2(&test_input);

    println!("Answer part 1: {}", result1);
    println!("Answer part 2: {}", result2);

    Ok(())
}
