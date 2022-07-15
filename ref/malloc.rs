const ARR_LEN: usize = 10;

pub fn main() {
    let mut array = vec![0; ARR_LEN];

    for (i, v) in array.iter_mut().enumerate() {
        *v = i * 2;
    }

    for v in array.iter() {
        println!("{}", v);
    }
}
