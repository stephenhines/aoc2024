use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn read_network_map(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let toks: Vec<&str> = line.split('-').collect();
        assert_eq!(toks.len(), 2);
        let computer1 = toks[0].to_string();
        let computer2 = toks[1].to_string();
        if map.contains_key(&computer1) {
            let mut entry = map.get(&computer1).unwrap().clone();
            entry.push(computer2.clone());
            map.insert(computer1.clone(), entry);
        } else {
            let mut entry = Vec::new();
            entry.push(computer2.clone());
            map.insert(computer1.clone(), entry);
        }
        if map.contains_key(&computer2) {
            let mut entry = map.get(&computer2).unwrap().clone();
            entry.push(computer1.clone());
            map.insert(computer2.clone(), entry);
        } else {
            let mut entry = Vec::new();
            entry.push(computer1.clone());
            map.insert(computer2.clone(), entry);
        }
    }
    //dbg!(&map);
    map
}

fn find_trios(map: &HashMap<String, Vec<String>>) -> usize {
    let mut set: HashSet<(&String, &String, &String)> = HashSet::new();
    let mut labels = map.keys().collect::<Vec<&String>>();
    labels.sort();
    for label_1 in labels {
        let first_values = map.get(label_1).unwrap();
        for label_2 in first_values {
            if label_2 != label_1 {
                let second_values = map.get(label_2).unwrap();
                for label_3 in second_values {
                    let third_values = map.get(label_3).unwrap();
                    if third_values.contains(label_1) {
                        let mut val_vec = vec![label_1, label_2, label_3];
                        val_vec.sort();
                        set.insert((val_vec[0], val_vec[1], val_vec[2]));
                        //println!("Found trio: {label_1} {label_2} {label_3}");
                    }
                }
            }
        }
    }
    let trios = set
        .iter()
        .filter(|x| x.0.starts_with('t') || x.1.starts_with('t') || x.2.starts_with('t'))
        .collect::<Vec<_>>()
        .len();
    println!("Trios: {trios}");
    trios
}

fn get_combos(map: &HashMap<String, Vec<String>>, start: &String) -> Vec<Vec<String>> {
    let mut combos = Vec::new();
    let connections = map.get(start).unwrap();
    let c_len = connections.len() as u32;

    let total = 2u64.pow(c_len);
    for i in 0..total {
        let mut new_vec = Vec::new();
        new_vec.push(start.clone());
        for j in 0..c_len {
            let p = 1 << j;
            if i & p != 0 {
                new_vec.push(connections[j as usize].clone());
            }
        }
        new_vec.sort(); // Always sort to keep it easier to check what we've tried already
        combos.push(new_vec);
    }
    combos
}

fn check_clique(map: &HashMap<String, Vec<String>>, clique: &Vec<String>) -> bool {
    for m in clique {
        let connections = map.get(m).unwrap();
        for n in clique {
            if m != n {
                if !connections.contains(n) {
                    return false;
                }
            }
        }
    }
    true
}

fn find_cliques(map: &HashMap<String, Vec<String>>) -> String {
    let mut tried: HashSet<Vec<String>> = HashSet::new();
    let mut max_clique = Vec::new();
    let mut max_clique_len = 0;
    for key in map.keys() {
        //println!("Trying key {}: {} {:?}", key, map.get(key).unwrap().len(), map.get(key).unwrap());
        let try_vec = get_combos(map, key);
        for clique in try_vec {
            if !tried.contains(&clique) {
                if clique.len() > max_clique_len {
                    if check_clique(map, &clique) {
                        max_clique = clique.clone();
                        max_clique_len = max_clique.len();
                        //dbg!(&clique);
                    }
                }
                tried.insert(clique);
            }
        }
    }
    //dbg!(&max_clique);
    let password = max_clique.join(",");
    println!("Password: {password}");
    password
}

#[test]
fn test_prelim() {
    let trios = find_trios(&read_network_map(&get_input("prelim.txt")));
    assert_eq!(trios, 7);
}

#[test]
fn test_part1() {
    let trios = find_trios(&read_network_map(&get_input("input.txt")));
    assert_eq!(trios, 1476);
}

#[test]
fn test_prelim2() {
    let password = find_cliques(&read_network_map(&get_input("prelim.txt")));
    assert_eq!(password, "co,de,ka,ta");
}

#[test]
fn test_part2() {
    let password = find_cliques(&read_network_map(&get_input("input.txt")));
    assert_eq!(password, "ca,dw,fo,if,ji,kg,ks,oe,ov,sb,ud,vr,xr");
}

fn main() {
    find_trios(&read_network_map(&get_input("prelim.txt")));
    find_trios(&read_network_map(&get_input("input.txt")));
    find_cliques(&read_network_map(&get_input("prelim.txt")));
    find_cliques(&read_network_map(&get_input("input.txt")));
}
