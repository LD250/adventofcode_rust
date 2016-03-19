use std::cmp;

fn main() {
    const INGREDIENTS: [[i32; 5]; 4] = [
        [2, 0, -2, 0, 3],
        [0, 5, -3, 0, 3],
        [0, 0, 5, -1, 8],
        [0, -1, 0, 5, 8],
    ];

    fn prop_score(prop_index: usize, s: i32, b:i32, c:i32, a:i32, ) -> i32 {
        cmp::max(0, [(0, s), (1, b), (2, c), (3, a)].iter().map(|i| INGREDIENTS[i.0][prop_index] * i.1).fold(0, |sum, x| sum + x))
    };
    let mut score: i32 = 0;
    let mut score_cal: i32 = 0;
    for s in 1..100 {
        for b in 1..(100 - s) {
            for c in 1..(100 - s - b) {
                let a = 100 - s - b - c;
                let new_score = [0, 1, 2, 3].iter().map(|prop_index| prop_score(*prop_index as usize, s, b, c, a)).fold(1, |prod, x| prod * x);
                score = cmp::max(score, new_score);
                if prop_score(4, s, b, c, a) == 500 {
                    score_cal = cmp::max(score_cal, new_score);
                }
            }
        }
    };

    println!("{:?}", score);
    println!("{:?}", score_cal);

}
