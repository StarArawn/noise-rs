pub fn checkerboard<const DIM: usize>(point: [f64; DIM], size: u32) -> f64 {
    let mask = 1 << size;

    let result = point
        .iter()
        .map(|&a| a.floor() as usize)
        .fold(0, |a, b| (a & mask) ^ (b & mask));

    if result > 0 {
        -1.0
    } else {
        1.0
    }
}
