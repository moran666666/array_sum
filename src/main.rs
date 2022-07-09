fn sum_u32(a: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for val in a.iter(){
        sum = sum + val;
        if sum < std::u32::MAX {
            continue;
        } else {
            return None;
        }
    }
    Some(sum)
}

fn main() {
    let a: [u32; 5]= [3, 6, 4, 10, 15];
    let result = sum_u32(&a);
    println!("slice list sum is: {:?}", result);
}
