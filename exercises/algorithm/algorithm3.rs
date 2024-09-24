/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::thread;

fn sort<T>(array: &mut [T])
where
    T: Ord + Clone + Send,
{
    if array.len() == 1 {
        return;
    }
    let mid = array.len() / 2;
    let (l_array, r_array) = array.split_at_mut(mid);
    let (mut l, mut r) = (0, 0);
    // thread::spawn(move || {
    sort(l_array);
    // });
    // thread::spawn(move || {
    sort(r_array);
    // });
    let mut res_array = Vec::new();
    while l < l_array.len() && r < r_array.len() {
        if l_array[l] < r_array[r] {
            res_array.push(l_array[l].clone());
            l += 1;
        } else {
            res_array.push(r_array[r].clone());
            r += 1;
        }
    }
    while l < l_array.len() {
        res_array.push(l_array[l].clone());
        l += 1;
    }
    while r < r_array.len() {
        res_array.push(r_array[r].clone());
        r += 1;
    }
    array.clone_from_slice(&res_array);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
