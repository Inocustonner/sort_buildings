use std::collections::{HashMap, HashSet};

pub type BoxStr = Box<str>;
pub type NameFloorMap<'a> = HashMap<&'a str, HashSet<usize>>;
pub type FloorFloorIntMap<'a> = &'a [HashSet<usize>];

#[derive(Debug, thiserror::Error)]
pub enum LoadCaseError {
    #[error(transparent)]
    SerdeJsonErr(#[from] serde_json::Error),

    #[error(transparent)]
    FileError(#[from] std::io::Error),

    #[error("Case {0:?} was not found")]
    CaseNotFoundErr(Box<str>),

    #[error("Expected value of '{expected_type:?}', but got '{got_value:?}' value")]
    UnexpectedValueErr {
        expected_type: Box<str>,
        got_value: Box<str>,
    },
}

pub fn load_case_from_json(
    path: &std::path::Path,
    case_name: &str,
) -> Result<Box<[Box<[BoxStr]>]>, LoadCaseError> {
    let json_data: serde_json::Value = serde_json::from_reader(std::fs::File::open(path)?)?;
    let case = json_data[case_name]
        .as_array()
        .ok_or(LoadCaseError::CaseNotFoundErr(Box::from(case_name)))?;

    case.iter()
        .map(|floor| {
            let floor = floor.as_array().ok_or(LoadCaseError::UnexpectedValueErr {
                expected_type: "Array".into(),
                got_value: floor.to_string().into_boxed_str(),
            })?;

            let copied_floor = floor
                .iter()
                .map(|name| name.to_string().into_boxed_str())
                .collect();
            Ok(copied_floor)
        })
        .collect()
}

pub fn make_name_floor_map(bld: &[Box<[BoxStr]>]) -> HashMap<&str, HashSet<usize>> {
    let mut name_floor_map: HashMap<&str, HashSet<usize>> = HashMap::new();

    for (floor_i, floor) in bld.iter().enumerate() {
        for person in floor.as_ref() {
            if let Some(persons_floors) = name_floor_map.get_mut(person.as_ref()) {
                persons_floors.insert(floor_i);
            } else {
                name_floor_map.insert(&person, HashSet::from([floor_i]));
            }
        }
    }
    name_floor_map
}

pub fn make_compatible_floors_map(
    bld: &[Box<[BoxStr]>],
    name_floor_map: &NameFloorMap,
) -> Box<[HashSet<usize>]> {
    // TODO: optimze
    bld.iter()
        .map(|floor| {
            let incompatible_floors = floor.iter().fold(
                name_floor_map[floor[0].as_ref()].clone(),
                |floors_set, person| {
                    floors_set
                        .union(&name_floor_map[person.as_ref()])
                        .cloned()
                        .collect()
                },
            );
            HashSet::from_iter(
                HashSet::from_iter(0..bld.len())
                    .difference(&incompatible_floors)
                    .cloned(),
            )
        })
        .collect()
}

pub fn pprint_building(bld: &[Box<[Box<str>]>]) {
    for (i, floor) in bld.iter().enumerate() {
        print!("{} : {}", i, floor[0]);
        for person in &floor[1..] {
            print!(", {}", person);
        }
        println!("");
    }
}

fn make_floor_list_str<ContT>(floors: ContT) -> String
where
    ContT: ExactSizeIterator,
    ContT::Item: std::fmt::Display,
{
    let size = floors.len();

    let mut ln = String::new();
    ln.reserve(size * 4);

    for (i, floor) in floors.enumerate() {
        let s = floor.to_string();
        ln.push_str(&s);
        if i != size - 1 {
            ln.push_str(", ");
        }
    }
    return ln;
}

pub fn fmt_iter<ContT>(floors: ContT) -> String
where
    ContT: ExactSizeIterator,
    ContT::Item: std::fmt::Display,
{
    std::fmt::format(format_args!("[{}]", make_floor_list_str(floors)))
}

#[allow(dead_code)]
pub fn pprint_name_floor_map(map: &NameFloorMap) {
    for (name, floors) in map {
        println!("{}: {}", name, fmt_iter(floors.iter()));
    }
}

pub fn pprint_floor_floor_map(map: FloorFloorIntMap) {
    for (floor, floors) in map.iter().enumerate() {
        println!("{}: {}", floor, fmt_iter(floors.iter()));
    }
}
