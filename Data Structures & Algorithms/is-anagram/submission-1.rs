impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        if s.len() != t.len() {
            return false;
        }

   let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    
    for k in t.chars(){
        if let Some(count) = counts.get_mut(&k) {
            *count -= 1;
             if *count == 0 {
             counts.remove(&k);
    
                            }

        }
    }

 counts.is_empty() 
    }
}
