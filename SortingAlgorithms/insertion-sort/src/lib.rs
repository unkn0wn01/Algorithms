#![allow(unused_comparisons)]
pub fn insertion_sort(items: &mut [u32]) {
    for j in 1..items.len() {
        let item = items[j];
        let mut i = j - 1;

        while i >= 0 && items[i] > item {
            items[i + 1] = items[i]; // swapping
            items[i] = item; // swapping
            if i != 0 {
                i = i - 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_vec() {
        let mut items = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut items);
        assert_eq!(vec![1, 2, 3, 4, 5], items)
    }
}
