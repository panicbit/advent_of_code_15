use aoc::aoc;

#[aoc(2015, 1, 2)]
fn main(input: &str) -> usize {
    let directions = parse_directions(input);

    visited_floors(directions)
        .enumerate()
        .find(|(_, floor)| *floor == -1)
        .map(|(i, _)| i)
        .unwrap()
        + 1
}

fn visited_floors(directions: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    directions.scan(0, |floor, direction| {
        *floor += direction;
        Some(*floor)
    })
}

fn parse_directions(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.chars().map(|ch| match ch {
        '(' => 1,
        ')' => -1,
        _ => unreachable!(),
    })
}
