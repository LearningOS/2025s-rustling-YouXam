/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::PartialOrd;

fn sort<T: PartialOrd>(array: &mut [T]){
    if array.len() < 2 {
        return;
    }
    let len = array.len();
	let mut middle = len / 2;
    let mut l = 0;
    let mut r = len - 1;
    loop {
        while array[l] < array[middle] && l < middle {
            l += 1;
        }
        while array[r] > array[middle] && r > middle {
            r -= 1;
        }
        if l >= middle && r <= middle {
            break;
        }
        if l < middle && r > middle {
            array.swap(l, r);
            l += 1;
            r -= 1;
        } else if r == middle {
            array.swap(l, middle);
            middle = l;
            l += 1;
        } else {
            array.swap(r, middle);
            middle = r;
            r -= 1;
        }
    }
    sort(&mut array[0..middle]);
    sort(&mut array[middle..len]);
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