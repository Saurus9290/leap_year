fn is_leap_year(year:u32) -> bool{

    (year % 4 == 0)&&((year % 100!= 0) || (year %400 == 0))
}
fn main(){
    let year = 2024;

    if is_leap_year(year){
        println!("{} is a leap year.",year);
    } else {
        println!("{} is not a leap year.",year);
    }
}