use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredients = HashMap::new();

    stdin.read_to_string(&mut input).unwrap();
    input.lines().for_each(|line| {
        let mut set = HashSet::new();
        let mut allergens_start = false;
        line.split(' ').map(|block| block.trim()).for_each(|block| {
            if block.contains("(") {
                allergens_start = true;
            } else if allergens_start {
                let mut allergen = String::from(block);
                allergen.pop();
                if allergens.contains_key(&allergen) {
                    let intersection: HashSet<_> = set
                        .intersection(allergens.get(&allergen).unwrap())
                        .map(|item| item.clone())
                        .collect();
                    allergens.insert(allergen, intersection);
                } else {
                    allergens.insert(allergen, set.iter().cloned().collect());
                }
            } else {
                set.insert(String::from(block));
                if ingredients.contains_key(&String::from(block)) {
                    ingredients.insert(
                        String::from(block),
                        ingredients.get(&String::from(block)).unwrap() + 1,
                    );
                } else {
                    ingredients.insert(String::from(block), 1);
                }
            }
        });
    });
    let mut safe_ingredients: HashMap<String, i32> = ingredients
        .iter()
        .map(|item| (item.0.clone(), *item.1))
        .collect();
    allergens.iter().for_each(|(_, ingredients)| {
        ingredients.iter().for_each(|ingredient| {
            safe_ingredients.remove(ingredient);
        });
    });
    println!(
        "{}",
        safe_ingredients.iter().map(|(_, num)| num).sum::<i32>()
    );
    while allergens.iter().filter(|(_, set)| set.len() == 1).count() > 0
        && allergens.iter().filter(|(_, set)| set.len() == 1).count() != allergens.len()
    {
        let items: Vec<_> = allergens
            .iter()
            .filter(|(_, set)| set.len() == 1)
            .map(|(_, set)| {
                return set.iter().next().unwrap().clone();
            })
            .collect();
        for item in items {
            for (_, set) in allergens.iter_mut() {
                if set.len() > 1 {
                    set.remove(&item);
                }
            }
        }
    }
    let mut list = allergens
        .iter()
        .map(|(allergen, set)| (allergen, set.iter().next().unwrap()))
        .collect::<Vec<_>>();
    list.sort();
    let mut list = list
        .iter()
        .map(|(_, item)| item)
        .fold(String::default(), |acc, item| acc + item + ",");
    list.pop();
    println!("{}", list);
}
