extern crate clipboard;

/// From a Vec<f32> returns the average,min,max and mean (in that order)
/// ```
/// stat(vec![vec![4.0, 2.0, 1.0, 3.0]])
/// >>> (2.5, 1.0, 4.0, 3.0)
/// ```
fn stats(mut x: Vec<f32>) -> (f32, f32, f32, f32) {
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let sum: f32 = {
        let mut total: f32 = 0.0;
        for i in &x {
            total += i;
        }
        total
    };
    let len: i32 = x.len() as i32;

    println!("{:?}", x);

    let average: f32 = sum / len as f32;
    let min: f32 = x[0];
    let max: f32 = x[(len - 1) as usize];
    let mean: f32 = x[{ ((len as f32) / 2.0) as i32 } as usize];

    (average, min, max, mean)
}

fn main() {
    //todo! create a system for parsing a file and getting stats
    //todo! get average of clipboard

}

#[cfg(test)]
mod tests {
    use crate::stats;
    #[test]
    fn status_test() {
        assert_eq!(stats(vec![1.0, 2.0, 3.0, 4.0]), (2.5, 1.0, 4.0, 3.0));
    }
}
