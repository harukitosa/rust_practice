pub fn binary_search<T: PartialOrd>(vec: &Vec<T>, target: T) -> usize {
    let mut lb = 0 as usize;
    let mut ub = vec.len();
    while ub - lb > 1 {
        let mid = (lb+ub)/2;
        if vec[mid] >= target {
            ub = mid;
        } else {
            lb = mid;
        }
    }
    return ub;
}



#[test]
fn it_works() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    assert_eq!(1, binary_search(&vec, 2));
    assert_eq!(3, binary_search(&vec, 4));
    println!("{}", binary_search(&vec, 0));
    // assert_eq!(0, );
}