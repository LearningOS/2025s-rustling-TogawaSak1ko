/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM DONE

fn pivot_partition<T: std::cmp::PartialOrd>(array: &mut [T]) -> usize{
    let pivot = 0;
    let l = array.len();
    array.swap(l / 2,pivot);
    let mut lt_index = 1;
    for i in 1..l {
        if array[i] < array[pivot] {
            array.swap(i,lt_index);
            lt_index+=1;
        }
    }
    // lt_index -> pointer the first element that greater or equal to the array[pivot],so use the lt_index - 1 
    array.swap(pivot,lt_index - 1);
    lt_index - 1
}

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    //TODO
    // bubble
    // let l = array.len();
    // for i in 0..l {
    //     for j in 0..l -1 - i {
    //         if array[j] >= array[j + 1] {
    //             array.swap(j,j + 1);
    //         }
    //     }
    // }

    // quick sort
    if array.len() < 2 {
        return ;
    }
    let pivot = pivot_partition(array);
    sort(&mut array[..pivot]);
    sort(&mut array[pivot + 1..]);
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
