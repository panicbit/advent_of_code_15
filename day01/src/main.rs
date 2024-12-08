use aoc::aoc;

#[aoc(2015, 1, 1)]
fn main(input: &str) -> i32 {
    let directions = parse_directions(input);

    visited_floors(directions).last().unwrap()
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
