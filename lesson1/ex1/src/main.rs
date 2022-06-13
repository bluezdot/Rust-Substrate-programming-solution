
// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
//         let sub_arr = [6,8,10]; 
// 🥑 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

fn main() {
    let org = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub1 = [6,8,10];
    let sub2 = [6,9,10];
    let emp_sub = [];

    // check if sub_arr is a subarray of org_arr
    println!("sub1 is subarray of org_arr ?: {}", is_subset(&sub1, &org));
    println!("sub2 is subarray of org_arr ?: {}", is_subset(&sub2, &org));
    println!("emp_sub is subarray of org_arr ?: {}", is_subset(&emp_sub, &org));
    println!("emp_sub is subarray of emp_sub ?: {}", is_subset(&emp_sub, &emp_sub));
}

fn is_subset(sub_arr: &[i32], org_arr: &[i32]) -> bool {
    let mut is_subarray = true;
    let mut i = 0;
    let mut j = 0;

    if (sub_arr.len() == 0 && org_arr.len() == 0) {
        is_subarray = false;
        is_subarray;
    }
    else if (sub_arr.len() == 0 && org_arr.len() != 0) {
        is_subarray = true;
        is_subarray;
    }
    else {
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
    }

    is_subarray
}
