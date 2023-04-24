use std::collections::{BTreeSet, HashMap, VecDeque};

use crate::parser::{Stmt, IR};

type PointedSet = BTreeSet<String>;
type PtrDict = HashMap<String, PointedSet>;

pub struct Node {
    stmt: Option<Stmt>,
    next: Vec<usize>,
    data: PtrDict,
}

pub fn solve(ir_list: Vec<IR>) -> Vec<Node> {
    let mut nodes = Vec::<Node>::with_capacity(ir_list.len());

    let mut i = 0;
    for ir in ir_list {
        let mut stmt = None;
        let mut next = Vec::<usize>::with_capacity(2);
        next.push(i + 1);
        match ir {
            IR::Stmt(s) => stmt = Some(s),
            IR::Branch(x) => next.push(x),
            _ => (),
        };
        nodes.push(Node {
            stmt,
            next,
            data: PtrDict::new(),
        });
        i += 1;
    }

    let mut queue: VecDeque<usize> = (0..nodes.len()).into_iter().collect();

    while !queue.is_empty() {
        let node_idx = queue.pop_front().unwrap();
        let node_mut = nodes.get_mut(node_idx).unwrap();
        transfer(&mut node_mut.data, &node_mut.stmt);

        let node = &nodes[node_idx];
        let out = &node.data;
        for succ_idx in node.next.iter() {
            let succ_node = &nodes[*succ_idx];
            let try_data = transfer_clone(out, &succ_node.stmt);
            if succ_node.data != try_data {
                queue.push_back(*succ_idx);
            }
        }
    }

    return nodes;
}

fn transfer_clone(data: &PtrDict, stmt: &Option<Stmt>) -> PtrDict {
    let mut result = data.clone();
    transfer(&mut result, stmt);
    return result;
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
                if !data.contains_key(rhs) {
                    return;
                }
                let r_set = data[rhs].clone();
                if !data.contains_key(lhs) {
                    data.insert(lhs.clone(), r_set);
                } else {
                    let l_set = data[lhs].clone();
                    for v in l_set {
                        if let Some(v_set) = data.get(&v) {
                            let mut new_v_set = v_set.clone();
                            move_all_into(&mut new_v_set, r_set.clone());
                            data.insert(v, new_v_set);
                        }
                    }
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
