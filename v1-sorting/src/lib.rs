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

pub fn merge_sort<T: PartialOrd + Debug> (mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    } 

    println!("v: {:?}", v);
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    //merge
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_next = a_it.next();
    let mut b_next = b_it.next();

    loop {
        match a_next  {
            Some(ref a_val) => match b_next {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_next.take().unwrap());
                        b_next = b_it.next();
                    } else {
                        res.push(a_next.take().unwrap());
                        a_next = a_it.next();
                    }
                }
                None => {
                    res.push(a_next.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_next {
                    res.push(b_val)
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![23, 11, 22, 10, 11, 4, 3];
        bubble_sort(&mut v);
        let sorted_v = vec![3, 4, 10, 11, 11, 22, 23];
        assert_eq!(v, sorted_v);
    }
    
    #[test]
    fn test_merge_sort() {
        let v = vec![23, 11, 22, 10, 11, 4, 3];
        let v = merge_sort(v);
        assert_eq!(v,  vec![3, 4, 10, 11, 11, 22, 23]);
    }
}
