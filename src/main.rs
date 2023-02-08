use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let vecky = ["eat", "tea", "tan", "ate", "nat", "bat", "atb"];

    let mut sorted_word: String;

    let mut hash: HashMap<String, Vec<String>> = HashMap::new();

    for word in vecky.iter() {
        sorted_word = word.chars().sorted().collect::<String>();
        if hash.contains_key(&sorted_word) {
            let mut vecku: Vec<String> = hash.get_mut(&sorted_word).unwrap().to_vec();
            vecku.push(word.to_string());
            hash.insert(sorted_word.clone(), vecku);
        } else {
            hash.insert(sorted_word.clone(), vec![word.to_string()]);
        }
    }
    println!("{:#?}", &hash);
}
