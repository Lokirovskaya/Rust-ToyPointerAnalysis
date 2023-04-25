use std::collections::{BTreeSet, HashMap, VecDeque};

use crate::parser::{Stmt, IR};

type PointedSet = BTreeSet<String>;
type PtrDict = HashMap<String, PointedSet>;

#[derive(Debug)]
pub struct Node {
    stmt: Option<Stmt>,
    next: Vec<usize>,
    data: PtrDict,
    tag: Option<String>,
}

pub fn solve(ir_list: Vec<IR>) {
    let mut nodes = Vec::<Node>::with_capacity(ir_list.len());

    let mut i = 0;
    let len = ir_list.len();
    for ir in ir_list {
        let mut stmt = None;
        let mut next = Vec::<usize>::with_capacity(2);
        let mut tag = None;
        if i + 1 < len {
            next.push(i + 1);
        }
        match ir {
            IR::Stmt(s) => stmt = Some(s),
            IR::Branch(x) => next.push(x),
            IR::Check(t) => tag = Some(t),
            _ => (),
        };
        nodes.push(Node {
            stmt,
            next,
            data: PtrDict::new(),
            tag,
        });
        i += 1;
    }

    let mut queue: VecDeque<usize> = (0..nodes.len()).into_iter().collect();

    while !queue.is_empty() {
        let node_idx = queue.pop_front().unwrap();
        let node = nodes.get_mut(node_idx).unwrap();
        transfer(&mut node.data, &node.stmt);

        let node = &nodes[node_idx];
        let out = node.data.clone();
        let next = node.next.clone();
        for succ_idx in next {
            let succ_node = nodes.get_mut(succ_idx).unwrap();
            let data_before = succ_node.data.clone();
            meet(&mut succ_node.data, &out);
            let data_after = &succ_node.data;
            if data_before != *data_after {
                queue.push_back(succ_idx);
            }
        }
    }

    print_on_tag(nodes);
}

fn meet(data: &mut PtrDict, src: &PtrDict) {
    for (var, set) in src {
        if !data.contains_key(var) {
            data.insert(var.clone(), set.clone());
        } else {
            for s in set {
                data.get_mut(var).unwrap().insert(s.clone());
            }
        }
    }
}

fn transfer(data: &mut PtrDict, stmt: &Option<Stmt>) {
    if let Some(stmt) = stmt {
        match stmt {
            Stmt::Ref { lhs, rhs } => {
                if !data.contains_key(lhs) {
                    data.insert(lhs.clone(), PointedSet::new());
                }
                let l_set = data.get_mut(lhs).unwrap();
                l_set.insert(rhs.clone());
            }
            Stmt::Alias { lhs, rhs } => {
                if !data.contains_key(rhs) {
                    return;
                }
                let r_set = data[rhs].clone();
                if !data.contains_key(lhs) {
                    data.insert(lhs.clone(), PointedSet::new());
                }
                let l_set = data.get_mut(lhs).unwrap();
                move_all_into(l_set, r_set);
            }
            Stmt::DerefRead { lhs, rhs } => {
                if !data.contains_key(rhs) {
                    return;
                }
                let r_set = &data[rhs];
                let mut union_of_v_set = PointedSet::new();
                for v in r_set.iter() {
                    if let Some(v_set) = data.get(v) {
                        move_all_into(&mut union_of_v_set, v_set.clone());
                    }
                }
                if !data.contains_key(lhs) {
                    data.insert(lhs.clone(), PointedSet::new());
                }
                let l_set = data.get_mut(lhs).unwrap();
                move_all_into(l_set, union_of_v_set);
            }
            Stmt::DerefWrite { lhs, rhs } => {
                if !data.contains_key(rhs) || !data.contains_key(lhs) {
                    return;
                }
                let r_set = data[rhs].clone();
                let l_set = data[lhs].clone();
                for v in l_set {
                    if data.contains_key(&v) {
                        let mut new_v_set = data.get_mut(&v).unwrap();
                        move_all_into(&mut new_v_set, r_set.clone());
                        let new_v_set = new_v_set.clone(); // it's strange that i have to do this
                        data.insert(v, new_v_set);
                    } else {
                        data.insert(v, r_set.clone());
                    };
                }
            }
        }
    }
}

fn move_all_into(target: &mut PointedSet, src: PointedSet) {
    for p in src.iter() {
        target.insert(p.clone());
    }
}

pub fn print_on_tag(nodes: Vec<Node>) {
    for node in nodes {
        if let Some(tag) = node.tag {
            println!("# {}:", tag);
            if node.data.is_empty() {
                println!("  null");
            } else {
                let mut keys: Vec<(String, PointedSet)> = node.data.into_iter().collect();
                keys.sort_by_key(|k| k.0.to_string());
                for (var, set) in keys {
                    print!("  {} -> [", var);
                    let mut i = 0;
                    let len = set.len();
                    for s in set {
                        if i < len - 1 {
                            print!("{}, ", s);
                        } else {
                            print!("{}", s);
                        }
                        i += 1;
                    }
                    println!("]");
                }
            }
            println!();
        }
    }
}
