use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(PartialEq, Copy, Clone, Debug)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

impl Coords {
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<f64> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();

        if parts.len() == 3 {
            Some(Coords {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct WirePairs {
    item1: Coords,
    item2: Coords,
    dis: f64,
}

fn distance(a: Coords, b: Coords) -> f64 {
    let dx = (a.x - b.x).powf(2.0);
    let dy = (a.y - b.y).powf(2.0);
    let dz = (a.z - b.z).powf(2.0);
    (dx + dy + dz).sqrt()
}

fn part1(input_file: &str) -> u64{
   if let Ok(lines) = read_lines(input_file) {
        let coords_list: Vec<_> = lines
            .filter_map(|line| Coords::from_str(&line.ok()?))
            .collect();
        let mut junction_boxs: Vec<Vec<WirePairs>> = Vec::new();
        let mut pairs: Vec<WirePairs> = Vec::new();
        for (i, ca) in coords_list.iter().enumerate() {
            for (j, cb) in coords_list.iter().enumerate() {
                if i < j {  // Only create each pair once
                    let some_pair = WirePairs {
                        item1: *ca,
                        item2: *cb,
                        dis: distance(*ca, *cb),
                    };
                    pairs.push(some_pair);
                }
            }
        }
        pairs.sort_by(|a, b| a.dis.partial_cmp(&b.dis).expect("Something"));
        
        // Connect only the 10 shortest pairs (test) / 1000 (real input)
        //If this number is wrong then then the problem doesn't work
        for p in pairs.into_iter().take(10) {
            // Find the index of the circuit containing item1
            let idx1 = junction_boxs.iter().position(|circuit| {
                circuit
                    .iter()
                    .any(|pair| pair.item1 == p.item1 || pair.item2 == p.item1)
            });
        
            // Find the index of the circuit containing item2
            let idx2 = junction_boxs.iter().position(|circuit| {
                circuit
                    .iter()
                    .any(|pair| pair.item1 == p.item2 || pair.item2 == p.item2)
            });
        
            // Make the right move based on where the items are
            match (idx1, idx2) {
                // Neither item is in a circuit. Make a new one.
                (None, None) => {
                    junction_boxs.push(vec![p]);
                }
        
                // item1 is in a circuit, but item2 is brand new.
                (Some(i), None) => {
                    junction_boxs[i].push(p);
                }
        
                // item2 is in a circuit, but item1 is brand new.
                (None, Some(j)) => {
                    junction_boxs[j].push(p);
                }
        
                // BOTH items are already in a circuit.
                (Some(i), Some(j)) => {
                    if i == j {
                        // They are in the SAME circuit - skip (nothing happens)
                        continue;
                    } else {
                        // They are in DIFFERENT circuits - merge them
                        let (keep_idx, remove_idx) = if i < j { (i, j) } else { (j, i) };
                        let mut circuit_to_merge = junction_boxs.remove(remove_idx);
                        junction_boxs[keep_idx].push(p);
                        junction_boxs[keep_idx].append(&mut circuit_to_merge);
                    }
                }
            }
        }

        println!("Results section");
        
        // Count unique junction boxes in each circuit
        let mut circuit_sizes: Vec<usize> = junction_boxs
            .iter()
            .map(|circuit| {
                let mut unique_boxes = std::collections::HashSet::new();
                for pair in circuit {
                    unique_boxes.insert((pair.item1.x.to_bits(), pair.item1.y.to_bits(), pair.item1.z.to_bits()));
                    unique_boxes.insert((pair.item2.x.to_bits(), pair.item2.y.to_bits(), pair.item2.z.to_bits()));
                }
                unique_boxes.len()
            })
            .collect();
        
        // Add individual junction boxes that have no connections
        let total_coords = coords_list.len();
        let mut connected_boxes = std::collections::HashSet::new();
        for circuit in &junction_boxs {
            for pair in circuit {
                connected_boxes.insert((pair.item1.x.to_bits(), pair.item1.y.to_bits(), pair.item1.z.to_bits()));
                connected_boxes.insert((pair.item2.x.to_bits(), pair.item2.y.to_bits(), pair.item2.z.to_bits()));
            }
        }
        
        // Each unconnected junction box is its own circuit of size 1
        let unconnected_count = total_coords - connected_boxes.len();
        println!("Unconnected junction boxes: {}", unconnected_count);
        for _ in 0..unconnected_count {
            circuit_sizes.push(1);
        }
        
        circuit_sizes.sort();
        circuit_sizes.reverse();

        circuit_sizes.iter().enumerate().for_each(|(i, size)| {
            println!("Circuit {}: {} junction boxes", i + 1, size);
        });
        let answer: u64 = circuit_sizes.iter().take(3).map(|&x| x as u64).product();
        answer
    } else {
        0
    }
}


fn part2(input_file: &str) -> u64 {
   let mut answer: u64 = 0;
   if let Ok(lines) = read_lines(input_file) {
        let coords_list: Vec<_> = lines
            .filter_map(|line| Coords::from_str(&line.ok()?))
            .collect();
        let mut junction_boxs: Vec<Vec<WirePairs>> = Vec::new();
        let mut pairs: Vec<WirePairs> = Vec::new();
        for (i, ca) in coords_list.iter().enumerate() {
            for (j, cb) in coords_list.iter().enumerate() {
                if i < j {  // Only create each pair once
                    let some_pair = WirePairs {
                        item1: *ca,
                        item2: *cb,
                        dis: distance(*ca, *cb),
                    };
                    pairs.push(some_pair);
                }
            }
        }
        pairs.sort_by(|a, b| a.dis.partial_cmp(&b.dis).expect("Something"));
        
        let mut last_added: Option<(Coords, Coords)> = None;
        
        //don't want to take just want to merge into mega circutit
        for p in pairs {
            // Find the index of the circuit containing item1
            let idx1 = junction_boxs.iter().position(|circuit| {
                circuit
                    .iter()
                    .any(|pair| pair.item1 == p.item1 || pair.item2 == p.item1)
            });
        
            // Find the index of the circuit containing item2
            let idx2 = junction_boxs.iter().position(|circuit| {
                circuit
                    .iter()
                    .any(|pair| pair.item1 == p.item2 || pair.item2 == p.item2)
            });
        
            // Make the right move based on where the items are
            match (idx1, idx2) {
                // Neither item is in a circuit. Make a new one.
                (None, None) => {
                    junction_boxs.push(vec![p]);
                    last_added = Some((p.item1, p.item2));
                }
        
                // item1 is in a circuit, but item2 is brand new.
                (Some(i), None) => {
                    junction_boxs[i].push(p);
                    last_added = Some((p.item1, p.item2));
                }
        
                // item2 is in a circuit, but item1 is brand new.
                (None, Some(j)) => {
                    junction_boxs[j].push(p);
                    last_added = Some((p.item1, p.item2));
                }
        
                //This is the tricky part of the problem. So much headache 
                // BOTH items are already in a circuit.
                (Some(i), Some(j)) => {
                    if i == j {
                        // They are in the SAME circuit.
                        // Do nothing to avoid making a redundant loop
                        // need for match statement
                        continue;
                    } else {
                        // They are in DIFFERENT circuits. merge them
        
                        // Figure out which index is smaller/larger so we don't
                        // mess up the array ordering when we remove one.
                        let (keep_idx, remove_idx) = if i < j { (i, j) } else { (j, i) };
        
                        // Pull the second circuit completely out of the list
                        let mut circuit_to_merge = junction_boxs.remove(remove_idx);
        
                        // Add the new connecting wire to the kept circuit
                        junction_boxs[keep_idx].push(p);
        
                        // Dump all wires from the removed circuit into the kept one
                        junction_boxs[keep_idx].append(&mut circuit_to_merge);
                        
                        last_added = Some((p.item1, p.item2));
                    }
                }
            }
        }

        println!("Results section");
        if let Some((box1, box2)) = last_added {
                answer = (box1.x * box2.x) as u64;
        }
}
answer
}



fn main() {
    let answer = part1("./test.txt");
    println!("Part 1: {}", answer);
    let answer2 = part2("./test.txt");
    println!("Part 2: {}", answer2);
}
