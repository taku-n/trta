// main.rs

use trta::TrTA;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let window = v.window_under_zero_value(3);

    println!("{:?}", window);  //=> [[0, 0, 1], [0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5]]
}
