#![feature(vec_remove_item)]

use std::fs::read_to_string;
use std::ops::Mul;
use std::collections::HashMap;

///////////////
// Datatypes //
///////////////

#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Debug)]
struct Ingredient<'a>(i64, &'a str);

impl<'a> PartialEq<str> for Ingredient<'a> {
    fn eq(&self, other: &str) -> bool {
        self.1 == other
    }
}

impl<'a> std::ops::Mul<i64> for Ingredient<'a> {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient(self.0 * rhs, self.1)
    }
}

#[derive(Clone, Eq, Ord, PartialOrd, PartialEq, Debug)]
struct Reaction<'a>(Vec<Ingredient<'a>>, Ingredient<'a>);

impl<'a> Reaction<'a> {
    fn produces(&self, ingredient: &Ingredient) -> bool {
        self.1 == *ingredient.1
    }
}

/////////////
// Parsing //
/////////////

fn parse_ingredients(ingredients: &str) -> Vec<Ingredient> {
    ingredients.trim().split(",").map(|elem| elem.trim()).map(|elem| {
        let elems: Vec<_> = elem.split(" ").collect();
        Ingredient(elems[0].parse().expect(""), elems[1])
    }).collect()
}

fn parse_reactions(reactions: &String) -> Vec<Reaction> {
    reactions.trim().split("\n").map(|line| {
        let sides: Vec<_> = line.split("=>").collect();

        Reaction(
            parse_ingredients(sides[0]),
            parse_ingredients(sides[1])[0]
        )
    }).collect()
}

///////////
// Logic //
///////////

fn reverse_react<'a>(reactions: &Vec<Reaction<'a>>, target_ingredient: &Ingredient) -> Vec<Ingredient<'a>> {
    let reaction = find_producing_reaction(reactions, target_ingredient);

    // Multiplier is equal to the number of times we need the reactions to be applied
    let multiplier = (target_ingredient.0 as f64 / ((reaction.1).0 as f64)).ceil() as i64;

    reaction.0
        .iter().map(|ingredient| *ingredient * multiplier).collect()
}

fn compress(pool: Vec<Ingredient>) -> Vec<Ingredient> {
    let mut pool_map : HashMap<&str, i64> = HashMap::new();

    for ingredient in pool {
        let amount = pool_map.entry(ingredient.1).or_default();
        *amount += ingredient.0;
    }

    pool_map.iter().map(|(name, value)| Ingredient(*value, name)).collect()
}

fn find_producing_reaction<'a, 'b>(reactions: &'b Vec<Reaction<'a>>, ingredient: &Ingredient) -> &'b Reaction<'a> {
    reactions.iter().find(|reaction| reaction.produces(ingredient)).unwrap()
}

fn ore_distance(reactions: &Vec<Reaction>, ingredient: &Ingredient) -> i64 {
    if ingredient == "ORE" {
        0
    } else {
        let producing_reaction = find_producing_reaction(reactions, ingredient);
        producing_reaction.0.iter().map(|ingredient| ore_distance(reactions, ingredient)).max().unwrap() + 1
    }
}

fn calculate_ore_distances<'a>(reactions: &Vec<Reaction<'a>>) -> HashMap<&'a str, i64> {
    let mut map = HashMap::new();

    map.insert("ORE", 0);

    for reaction in reactions {
        let ingredient = reaction.1;
        map.insert(ingredient.1, ore_distance(reactions, &ingredient));

        for reaction in &reaction.0 {
            map.insert(ingredient.1, ore_distance(reactions, &ingredient));
        }
    }

    map
}

fn calculate_ore_footprint(reactions: &Vec<Reaction>, ore_distances: &HashMap<&str, i64>, ingredient: Ingredient) -> i64 {
    let mut pool = vec![ingredient];

    while !pool.iter().all(|ingredient| ingredient == "ORE") {
        let reactee = *pool.iter().max_by_key(|ingredient| {
            ore_distances.get(ingredient.1).unwrap()
        }).unwrap();

        pool.remove_item(&reactee);
        pool.extend(reverse_react(&reactions, &reactee));

        pool = compress(pool);
    }

    pool.iter().map(|ingredient| ingredient.0).sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
//    let input = read_to_string("test_input1.txt").unwrap();
    let reactions = parse_reactions(&input);

    let ore_distances = calculate_ore_distances(&reactions);
    assert!(ore_distances.get("FUEL").is_some());
    assert!(ore_distances.get("ORE").is_some());

    let res1 = calculate_ore_footprint(&reactions, &ore_distances, Ingredient(1, "FUEL"));
    assert_eq!(res1, 469536);
    println!("Ore needed: {}", res1);

    // 122356 too low
    // 252934162514 too high
    // 469536 ok

    let mut lower_bound = 1;
    let mut upper_bound = 10000000;
    let target = 1000000000000;
    dbg!(lower_bound);
    dbg!(upper_bound);

    loop {
        let fuel_count = (lower_bound + upper_bound) / 2;
        let ore_footprint = calculate_ore_footprint(&reactions, &ore_distances, Ingredient(fuel_count, "FUEL"));
        let next_ore_footprint = calculate_ore_footprint(&reactions, &ore_distances, Ingredient(fuel_count + 1, "FUEL"));
        if ore_footprint <= target && next_ore_footprint > target {
            break;
        } else if ore_footprint < target {
            lower_bound = fuel_count;
        } else {
            upper_bound = fuel_count;
        }

        println!("-------");
        dbg!(fuel_count);
        dbg!(ore_footprint);
        dbg!(lower_bound);
        dbg!(upper_bound);
    }

    let fuel_count = (lower_bound + upper_bound) / 2;
    let ore_footprint = calculate_ore_footprint(&reactions, &ore_distances, Ingredient(fuel_count, "FUEL"));
    assert_eq!(fuel_count, 3343477);
    println!("{} fuel for {} ore", fuel_count, ore_footprint);
}
