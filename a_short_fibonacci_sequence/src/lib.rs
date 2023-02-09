pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

pub fn create_buffer(count : usize) -> Vec<u8> {
    let mut v = Vec::new();
    
    let mut number = 0;
    
    while number < count {
        v.push(0);
        number += 1
    }
    
    v
}

pub fn fibonacci() -> Vec<u8> {
    let mut v = Vec::new();
    
    v.push(1);
    v.push(1);
    
    while v.len() < 5 {
        let m = v.len()-1;
        let n = v.len()-2;
        let new = &v[m] + &v[n];
        v.push(new);
    }
    
    v
    
}