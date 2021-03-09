use std::fmt::Debug;

//O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("v: {:?}", v);
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut v = vec![23, 11, 22, 10, 11, 4, 3];
        bubble_sort(&mut v);
        let sorted_v = vec![3, 4, 10, 11, 11, 22, 23];
        assert_eq!(v, sorted_v);
    }
}
