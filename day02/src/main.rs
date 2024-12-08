use aoc::aoc;
use utils::tuple_split_parse;

#[aoc(2015, 2, 1)]
fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| tuple_split_parse::<i32, _>(line, "x"))
        .map(|(l, w, h)| {
            let a = l * w;
            let b = w * h;
            let c = h * l;

            let smallest_side = a.min(b).min(c);

            2 * a + 2 * b + 2 * c + smallest_side
        })
        .sum()
}
