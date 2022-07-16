fn sum_u32(a: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &val in a.iter(){
        match sum.checked_add(val) {
            Some(value) => {
                sum = value;
            }
            None => return None,
        };
    }
    Some(sum)
}

fn main() {
    let a: [u32; 5]= [3, 6, 4, 10, 15];
    let result = sum_u32(&a);
    println!("slice list sum is: {:?}", result);
}
