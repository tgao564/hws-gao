pub fn hello() -> &'static str {
    return "Hello World!";
}

pub fn is_leap_year(yr: i32) -> bool {
    if yr % 4 == 0{
        if (yr % 400 != 0) && (yr % 100 == 0){
            return false
        }else{
            return true
        }
    }else{
        return false
    }
}
