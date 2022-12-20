pub fn ord(x: char) -> u32 {
    x.into()
}

#[allow(dead_code)]
fn paths<T>(xs: &Vec<Vec<T>>, p: &(usize, usize)) -> impl Iterator {
    let m = xs.len();
    let n = xs[0].len();
    let (i, j) = *p;
    (i + 1..m)
        .map(move |a| (a, j))
        .chain((0..i).rev().map(move |a| (a, j)))
        .chain((j + 1..n).map(move |a| (i, a)))
        .chain((0..j).rev().map(move |a| (i, a)))
}
