fn main() {
    println!("Hello, world!");
    let m = Mat {row: 2, col: 2, arr: [1, 2, 3, 4].to_vec()};
    println!("{}", m.to_string());
}

struct Mat {
    row: usize,
    col: usize,
    arr: Vec<i64>
}


impl Mat {
    fn to_string(&self) -> String {
        return format!("row:{} col:{} arr:{:?}", self.row, self.col, self.arr)
    }
}
