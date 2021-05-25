pub fn binary_search_core<T: PartialOrd>(vec: &Vec<T>, target: T) -> Option<usize> {
    let mut lb = -1;
    let mut ub = vec.len() as i32;
    while ub - lb > 1 {
        let mid = (lb+ub)/2;
        if vec[mid as usize] == target {
            return Some(mid as usize);
        }
        if vec[mid as usize] >= target {
            ub = mid;
        } else {
            lb = mid;
        }
    }
    return None;
}

pub fn binary_search<T: PartialOrd>(vec: &Vec<T>, target: T) -> i64 {
    let res = binary_search_core(vec, target);
    match res {
        Some(n) => return n as i64,
        None => return -1,
    }
}

#[test]
fn binary_search_test() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    assert_eq!(0,binary_search(&vec, 1));
    assert_eq!(1,binary_search(&vec, 2));
    assert_eq!(2,binary_search(&vec, 3));
    assert_eq!(3,binary_search(&vec, 4));
    assert_eq!(4,binary_search(&vec, 5));
    assert_eq!(-1,binary_search(&vec, -10000));
    assert_eq!(-1,binary_search(&vec, 10));
}

#[test]
fn binary_search_str_test() {
    let mut vec = Vec::new();
    vec.push("america");
    vec.push("black");
    vec.push("coffee");
    vec.push("dely");
    vec.push("egg");
    println!("{:?}", vec);

    assert_eq!(0,binary_search(&vec, "america"));
    assert_eq!(1,binary_search(&vec, "black"));
    assert_eq!(2,binary_search(&vec, "coffee"));
    assert_eq!(3,binary_search(&vec, "dely"));
    assert_eq!(4,binary_search(&vec, "egg"));
    assert_eq!(-1,binary_search(&vec, "fdklsafjlkadj"));
    assert_eq!(-1,binary_search(&vec, "hello"));
}