
// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
//         let sub_arr = [6,8,10]; 


fn main() {
    let org = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub1 = [6,8,10];
    let sub2 = [6,9,10];

    // check if sub_arr is a subarray of org_arr

    println!("sub1 is subarray of org_arr ?: {}", check(&sub1, &org));
    println!("sub2 is subarray of org_arr ?: {}", check(&sub2, &org));
}

fn check(sub_arr: &[i32], org_arr: &[i32]) -> bool {
    let mut is_subarray = true;
    let mut i = 0;
    let mut j = 0;
    while i < org_arr.len() && j < sub_arr.len() {
        if org_arr[i] == sub_arr[j] {
            i+=1;
            j+=1;
        } else {
            i+=1;
        }
    }

    if j < sub_arr.len() {
        is_subarray = false;
    }
    
    is_subarray
}
