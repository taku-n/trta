use trta::TrTA;
use trta::trta::I32;

fn main() {
    let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let v_window = v.window_under_head_with(3);
    println!("{:?}", v_window);
    //=> [[1.0, 1.0, 1.0], [1.0, 1.0, 2.0], [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]

    let w = vec![1, 2, 3, 4, 5];

    let w_window = w.window_under_head_with(3);

    println!("{:?}", w_window);  //=> [[1, 1, 1], [1, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5]]
}
