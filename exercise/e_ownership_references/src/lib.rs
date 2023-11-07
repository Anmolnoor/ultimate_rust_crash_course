pub fn inspect(s: &String){
    if s.ends_with("s") == true {
        return println!("plural");
    }
    println!("singular")
}

pub fn change(s:&mut String){
    if s.ends_with("s") != true {
         s.push_str("s");
    }   
}

pub fn eat(s: String)-> bool{
    if s.starts_with("b") && s.contains("a"){
        return true;
    }
    false
}

pub fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}