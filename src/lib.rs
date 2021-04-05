
pub fn is_armstrong_number(num: u32) -> bool {

    let vec: Vec<u32> = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let square_sum = vec.iter().map(|x| x.pow(vec.len() as u32)).sum();
    
    num == square_sum
}
