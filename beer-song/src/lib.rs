pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        3..=99 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
        _ => panic!("Invalid input: {}", n),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::from("");
    for num in (end..(1+start)).rev(){
        res.push_str(&verse(num));
        if num!=end{
            res.push_str(&"\n".to_string())
        }
    }
    return res;
}
