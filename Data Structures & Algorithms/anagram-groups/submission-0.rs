impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hashMap:HashMap<[u32;26],Vec<String>> = HashMap::new();
         for word in strs {
        let key = getArr(&word);
        hashMap.entry(key).or_default().push(word.clone());
        }

    hashMap.into_values().collect()

    }


}
     fn getArr(str: &str) -> [u32;26]{
    let mut counts = [0u32; 26];

       for c in str.chars() {
        counts[(c as usize) - ('a' as usize)] += 1;  
        } 
    counts
    }