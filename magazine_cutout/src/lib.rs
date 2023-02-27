use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
        
    let mut map_maga = HashMap::new();
    for i in magazine {
        let count = map_maga.entry(i).or_insert(0);
        *count += 1;
    }

    let mut map_note = HashMap::new();
    for i in note {
        let count = map_note.entry(i).or_insert(0);
        *count += 1;
    }
    
    let mut noword = 0;
    for k in map_note.keys() {
        if map_maga.get(k) < map_note.get(k) {
            noword+=1;
        }
    }
    
    if noword == 0 {
        true
    }
    else {
        false
    }
    
}