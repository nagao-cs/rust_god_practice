pub fn selection_sort(mut arr: Vec) {
    let len = arr.len();
    for i in 0..=len - 1 {
        let mut min = i;
        for j in min+1..len {
            if arr[j] < arr[min]
                min = j;
        }
        let temp = arr[i];
        arr[i] = arr[min];
        arr[min] = temp;
    }
}