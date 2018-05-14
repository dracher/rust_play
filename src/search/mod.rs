pub fn binary_search(list: &Vec<i32>, item: i32) -> i32 {
    let mut start = 0;
    let mut end = list.len() - 1;

    while start <= end {
        let mid = (start + end) / 2;
        if list[mid] == item {
            return mid as i32;
        } else if list[mid] > item {
            end = mid - 1
        } else {
            start = mid + 1
        }
    }
    -1
}
