use itertools::Itertools;

#[allow(unused)]
fn part_2() -> u32 {
    "199
200
208
210
200
207
240
269
260
263"
    .lines()
    .map(str::parse::<u32>)
    .map(Result::unwrap)
    .tuple_windows::<(_, _, _)>()
    .map(|window| window.0 + window.1 + window.2)
    .tuple_windows::<(_, _)>()
    .fold(
        0,
        |acc, (first, second): (u32, u32)| {
            if first < second {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 5);
}
