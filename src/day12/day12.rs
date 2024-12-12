use std::error::Error;
use std::fs::read_to_string;
use std::ops::Deref;

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

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Position {
    row: i32,
    column: i32,
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

#[derive(Debug)]
struct Garden(Vec<Vec<Plot>>);

impl Garden {
    fn plot_at(&self, x: i32, y: i32) -> Option<Plot> {
        if (y < 0) || (y as usize >= self.len()) {
            return None;
        }
        if (x < 0) || (x as usize >= self[y as usize].len()) {
            return None;
        }
        Some(self[y as usize][x as usize])
    }
}

impl Deref for Garden {
    type Target = Vec<Vec<Plot>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn collect_region(garden: &mut Garden, regions: &mut Vec<Region>, position: &Position) {
    let mut visited_positions: Vec<Position> = vec![];
    let mut search_front: Vec<Position> = vec![*position];
    // visited_positions.push(*position);

    let mut new_region = Region { area: 0, fences: 1 };
    let plant_type = garden
        .plot_at(position.column, position.row)
        .unwrap()
        .plant_type;
    // println!(
    //     "\nFilling a region from {:?} with type {}",
    //     position, plant_type
    // );

    while !search_front.is_empty() {
        let position = search_front.pop().unwrap();

        new_region.area += 1;
        visited_positions.push(position);
        garden.0[position.row as usize][position.column as usize].assigned = true;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let neighbor_position = Position {
                row: position.row + dy,
                column: position.column + dx,
            };
            if visited_positions.contains(&neighbor_position)
                || search_front.contains(&&neighbor_position)
            {
                // !! if the neighbor is already in the search front, we can also skip it
                // println!("  skipping position {:?} - already have that", neighbor_position);
                continue;
            }

            if let Some(neighbor) = garden.plot_at(neighbor_position.column, neighbor_position.row)
            {
                if neighbor.assigned {
                    // println!("  skipping position {:?} - already assigned to a different region", neighbor_position);
                    continue;
                }

                if neighbor.plant_type == plant_type {
                    search_front.push(neighbor_position);
                }
            }
        }
    }

    // println!(
    //     "Identified region {} with positions {:?}",
    //     plant_type, visited_positions
    // );
    // Go through all the plots of the previousy collected region
    new_region.fences = visited_positions
        .iter()
        .map(|position| {
            // Go through all the neighbors of one region and find those neighbors, that have
            // a different plant type or aren't even on the map
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter(|(dx, dy)| {
                    if let Some(plot) = garden.plot_at(position.column + dx, position.row + dy) {
                        plot.plant_type != plant_type
                    } else {
                        true
                    }
                })
                .count()
        })
        .sum();
    // println!(
    //     "  Region {}: {} fences, {} area",
    //     plant_type, new_region.fences, new_region.area
    // );

    regions.push(new_region);
}

fn challenge1(challenge_input: &str) -> i64 {
    let mut garden: Garden = Garden(
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
    // println!("{:?}", garden);

    let mut regions: Vec<Region> = vec![];
    for (line_no, line) in garden.clone().iter().enumerate() {
        for (pos, _plot) in line.iter().enumerate() {
            // We are not using the plot from iterating the garden because this is just
            // an unmodified clone of the garden
            if let Some(plot) = garden.plot_at(pos as i32, line_no as i32) {
                if !plot.assigned {
                    collect_region(
                        &mut garden,
                        &mut regions,
                        &Position {
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
