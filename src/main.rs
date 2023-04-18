use std::{collections::HashSet, env, fmt};

const DEFAULT_SETS: usize = 3;
const DEFAULT_VARIANTS: usize = 4;
const DEFAULT_ITEMS: usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (items, set_size, variant_count) = parse_args(args);
    let solution = solve(items, set_size, variant_count);
    println!("{}", solution);
}

#[test]
fn test_simple() {
    let args = vec![String::new(), "100010202101122011210212011212210101102102112201".to_string()];
    let (items, set_size, variant_count) = parse_args(args);
    let solution = solve(items, set_size, variant_count);
    let expected = vec![
        vec![1, 2, 5],
        vec![1, 6, 11],
        vec![2, 9, 10],
        vec![3, 5, 11],
        vec![4, 7, 9],
        vec![7, 10, 11],
    ];
    assert_eq!(solution.0, expected);
}

fn parse_args(args: Vec<String>) -> (Vec<Vec<u8>>, usize, usize) {
    let item_string = args
        .get(1)
        .expect("Must input string representing items to solve");
    let set_size = if args.len() > 2 {
        args[2].parse().expect("Expected number for first argument")
    } else {
        DEFAULT_SETS
    };
    let variant_count = if args.len() > 3 {
        args[3]
            .parse()
            .expect("Expected number for second argument")
    } else {
        DEFAULT_VARIANTS
    };
    let item_count = if args.len() > 4 {
        args[4].parse().expect("Expected number for third argument")
    } else {
        DEFAULT_ITEMS
    };

    assert!(
        item_string.len() % item_count == 0,
        "Should be `item_count` items"
    );
    assert!(
        item_string.len() % variant_count == 0,
        "Should be equal amounts of variants for each item"
    );

    let items = item_string
        .chars()
        .map(|char| {
            char.to_digit(10)
                .expect("Expects all numeric characters in item_string") as u8
        })
        .collect::<Vec<u8>>();
    let items = items
        .chunks(variant_count)
        .map(|slice| slice.to_vec())
        .collect::<Vec<Vec<u8>>>();

    (items, set_size, variant_count)
}

fn solve(items: Vec<Vec<u8>>, set_size: usize, variant_count: usize) -> Solution {
    let mut solver = Solver {
        items,
        set_size,
        variant_count,
        sets: vec![],
    };
    let sets = solver.solve();
    Solution(sets)
}

struct Solution(Vec<Vec<usize>>);

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, set) in self.0.iter().enumerate() {
            let mut first = true;
            for num in set.iter() {
                if !first {
                    write!(f, "\t")?;
                } else {
                    first = false;
                }
                write!(f, "{}", num)?;
            }
            if i != self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

struct Solver {
    items: Vec<Vec<u8>>,
    set_size: usize,
    variant_count: usize,
    sets: Vec<Vec<usize>>,
}

impl Solver {
    fn solve(&mut self) -> Vec<Vec<usize>> {
        for i in 0..(self.items.len() - self.set_size + 1) {
            self.solve_helper(&vec![], i);
        }
        self.sets.clone()
    }

    fn solve_helper(&mut self, candidate_set: &[usize], current_item: usize) {
        if current_item >= self.items.len() {
            return;
        }
        let mut new_candidate_set = candidate_set.to_owned();
        new_candidate_set.push(current_item);
        if new_candidate_set.len() == self.set_size {
            let candidate_items = new_candidate_set
                .iter()
                .map(|i| self.items.get(*i).unwrap())
                .collect::<Vec<&Vec<u8>>>();
            for i in 0..self.variant_count {
                let mut set = HashSet::new();
                for x in candidate_items.iter().map(|item| item[i]) {
                    set.insert(x);
                }
                if set.len() != 1 && set.len() != self.set_size {
                    return;
                }
            }
            self.sets.push(new_candidate_set.clone());
        } else {
            for i in (current_item + 1)..self.items.len() {
                self.solve_helper(&new_candidate_set, i);
            }
        }
    }
}
