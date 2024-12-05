use std::{fs::read_to_string, ops::Div};
use petgraph::algo::toposort;
use petgraph::prelude::*;

pub fn part_01(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let (rules, updates) = source.split_once("\n\n").unwrap();

    let rules: Vec<(u32, u32)> = rules.lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    updates.iter()
        .filter(|update| {
            obeys_rules(update, &rules)
        })
        .map(|update| {
            update[(update.len() - 1).div(2)]
        })
        .sum()
}

pub fn part_02(path: &str) -> u32 {
    let source = read_to_string(path).unwrap();

    let (rules, updates) = source.split_once("\n\n").unwrap();

    let rules: Vec<(u32, u32)> = rules.lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    

    let mut sum = 0;
    for update in updates.iter().filter(|update| !obeys_rules(update, &rules)) {
        let mut graph = DiGraphMap::new();

        for &node in update {
            graph.add_node(node);
        }

        for &(before, after) in &rules {
            if update.contains(&before) && update.contains(&after) {
                graph.add_edge(before, after, ());
            }
        }

        let result = toposort(&graph, None).unwrap();
        
        sum += result[(result.len() - 1).div(2)]
    }

    sum as u32
}

fn obeys_rules(update: &[u32], rules: &[(u32, u32)]) -> bool {
    rules.iter().all(|&(before, after)| {
        let before_idx = update.iter().position(|&num| num == before);
        let after_idx = update.iter().position(|&num| num == after);

        match (before_idx, after_idx) {
            (Some(b_idx), Some(a_idx)) => b_idx < a_idx,
            (None, _) | (_, None) => true,
        }
    })
}
