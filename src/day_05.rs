use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn day_05(ordering_rules: &[(usize, usize)], page_orders: &[Vec<usize>]) -> usize {
    let lookup = graph_from_ordering_rules(ordering_rules);
    page_orders
        .iter()
        .map(|page_order| {
            let is_valid = page_order.windows(2).all(|w| {
                let set = lookup.get(&w[0]);
                match set {
                    None => false,
                    Some(set) => set.contains(&w[1]),
                }
            });

            if is_valid {
                page_order[page_order.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn day_05_hard(ordering_rules: &[(usize, usize)], page_orders: &[Vec<usize>]) -> usize {
    let lookup = graph_from_ordering_rules(ordering_rules);
    page_orders.iter().map(|page_order| {
        let is_valid = page_order.windows(2).all(|w| {
            let set = lookup.get(&w[0]);
            match set {
                None => false,
                Some(set) => set.contains(&w[1]),
            }
        });

        if is_valid {
            0
        } else {
            let mut page_order = page_order.clone();
            sort_page_order(&mut page_order, &lookup);
            page_order[page_order.len() / 2]
        }
    }).sum()
}


fn graph_from_ordering_rules(ordering_rules: &[(usize, usize)]) -> HashMap<usize, HashSet<usize>> {
    let mut graph = HashMap::new();
    ordering_rules.iter().for_each(|(key, value)| {
        let mut set = match graph.get_mut(key) {
            Some(set) => set,
            None => {
                let set = HashSet::new();
                graph.insert(*key, set);
                graph.get_mut(key).unwrap()
            }
        };
        set.insert(*value);
    });
    graph
}
fn sort_page_order(page_order: &mut [usize], lookup: &HashMap<usize, HashSet<usize>>){
    page_order.sort_by(|a, b| {
        if let Some(set) = lookup.get(a)  {
            if set.contains(b) {
                return Ordering::Less
            }
        };

        if let Some(set) = lookup.get(b)  {
            if set.contains(a) {
                return Ordering::Greater
            }
        };
        Ordering::Equal
    })
}

#[cfg(test)]
mod tests {
    use crate::day_05::{day_05, day_05_hard};

    const INPUT: &str = include_str!("../resources/day_05/easy.txt");

    fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
        let mut split = input.split("\n\n");
        let ordering_rules = split.next().unwrap();
        let page_orders = split.next().unwrap();

        let ordering_rules = ordering_rules
            .split("\n")
            .map(|line| {
                let mut split_line = line.split("|");
                let a = split_line.next().unwrap().parse().unwrap();
                let b = split_line.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect();

        let page_orders = page_orders
            .split("\n")
            .map(|line| line.split(",").map(|i| i.parse().unwrap()).collect())
            .collect();

        (ordering_rules, page_orders)
    }

    #[test]
    fn easy() {
        let (ordering_rules, page_orders) = parse_input(INPUT);
        let left = day_05(&ordering_rules, &page_orders);
        let right = 4462;
        assert_eq!(left, right);
    }

    #[test]
    fn hard() {
        let (ordering_rules, page_orders) = parse_input(INPUT);
        let left = day_05_hard(&ordering_rules, &page_orders);
        let right = 6767;
        assert_eq!(left, right)
    }
}
