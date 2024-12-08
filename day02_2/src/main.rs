use aoc::aoc;
use utils::tuple_split_parse;

#[aoc(2015, 2, 2)]
fn main(input: &str) -> i32 {
    input
        .lines()
        .map(|line| tuple_split_parse::<i32, _>(line, "x"))
        .map(|(l, w, h)| {
            let biggest = l.max(w).max(h);
            let ribbon = 2 * l + 2 * w + 2 * h - 2 * biggest;
            let bow = l * w * h;

            ribbon + bow
        })
        .sum()
}
