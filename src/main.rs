mod helpers;

use std::collections::HashSet;

use crate::helpers::{
    FloorFloorIntMap, fmt_iter, load_case_from_json, make_compatible_floors_map,
    make_name_floor_map, pprint_building, pprint_floor_floor_map,
};

#[derive(Debug, Clone)]
struct Solution {
    floors: HashSet<usize>,
}

fn walk_solution_from(
    start_floor: usize,
    ff_map: FloorFloorIntMap,
    from_solution: Option<Solution>,
) -> Solution {
    let mut sol = if let Some(sol_) = from_solution {
        sol_
    } else {
        Solution {
            floors: HashSet::new(),
        }
    };

    let mut allowed_floors = ff_map[start_floor].clone();
    // filter out all floors incompatible with start_floor
    sol.floors.retain(|floor| allowed_floors.contains(floor));
    
    // it colud be the case where 
    // allowed_floors may contain floors incompatible with some floors in sol.floors
    // so we have to additionally filter them
    for &sol_floor in &sol.floors {
        allowed_floors.retain(|floor| ff_map[sol_floor].contains(floor));
    }

    while let Some(&next_floor) = allowed_floors.iter().next() {
        sol.floors.insert(next_floor);
        allowed_floors.retain(|floor| ff_map[next_floor].contains(floor));
    }
    sol.floors.insert(start_floor);
    sol
}

fn find_rest_of_solutions(ff_map: FloorFloorIntMap) -> Vec<Solution> {
    let mut sols = Vec::from_iter([walk_solution_from(0, ff_map, None)]);
    let mut floors_to_check = HashSet::<usize>::from_iter(
        (0..ff_map.len()).filter(|floor| !&sols[0].floors.contains(floor)),
    );
    // go thru floors that did not fit into any solution
    while let Some(&start_floor) = floors_to_check.iter().next() {
        for i in 0..sols.len() {
            let sol = walk_solution_from(start_floor, ff_map, Some(sols[i].clone()));
            // if a floor has already been added to solution, then we don't need to search for it
            // bcs it will belong to all solutions it can be in
            // bcs we only need to check the floors we haven't ever tried yet
            floors_to_check.retain(|floor| !sol.floors.contains(floor));
            sols.push(sol);
        }
    }

    sols
}

fn main() {
    // Input:
    // You have a table with rows of different lenght containing names
    // In a row names are unique, but they may repeat across the rows
    // Your task is to find a combination of rows, such that this combination
    // will hold the most number of unique name
    //
    // Important: In the final combination of rows names should be unique
    //
    // Example:
    //
    // Input:
    // ["Jake", "Mike"],
    // ["Daniel", "Jake", "Mike"],
    // ["Jimmy"]
    //
    // Output:
    // ["Daniel", "Jake", "Mike"],
    // ["Jimmy"]
    //
    // Explanation:
    // In total we have only 5 valid combination of rows
    // 1: ["Jake", "Mike"]
    // 2: ["Daniel", "Jake", "Mike"],
    // 3: ["Jimmy"]
    // 4: ["Jake", "Mike"], ["Jimmy"]
    // 5: ["Daniel", "Jake", "Mike"], ["Jimmy"]
    //
    // They're valid because name do not repeat in them.
    // Out of them we see that the 5th has most names. Hence, it is our answer

    let case = "case 5";

    let building = load_case_from_json(std::path::Path::new("data/cases.json"), case).unwrap();

    println!("{} Bulding:", case);
    pprint_building(&building);
    println!("");

    // solution
    let name_floor_map = make_name_floor_map(&building);
    // println!("NFM: ");
    // pprint_name_floor_map(&name_floor_map);
    // println!("");

    let floor_floor_map = make_compatible_floors_map(&building, &name_floor_map);
    println!("FFM: ");
    pprint_floor_floor_map(&floor_floor_map);
    println!("");

    let sols = find_rest_of_solutions(&floor_floor_map);

    let names_in_solution = |sol: &&Solution| {
        sol.floors.iter().fold(0, |sum, &floor| {
            sum + building.as_ref()[floor].as_ref().len()
        })
    };
    let best_solution = sols
        .iter()
        .reduce(|best, c| std::cmp::max_by_key(best, c, &names_in_solution))
        .unwrap();
    println!(
        "Best Solution: {}({})",
        fmt_iter(best_solution.floors.iter()),
        names_in_solution(&best_solution)
    );

    for (i, sol) in sols.iter().enumerate() {
        println!(
            "Solution {}: {}({})",
            i,
            fmt_iter(sol.floors.iter()),
            names_in_solution(&sol)
        );
    }
}
