use crate::parser::{Stmt, StmtKind};
use std::collections::BTreeMap;
use std::collections::BTreeSet;


type SolveResult = BTreeMap<String, BTreeSet<String>>;

pub fn solve(stmts: &Vec<Stmt>) -> SolveResult {
    let mut result = SolveResult::new();

    for stmt in stmts.iter() {
        if !result.contains_key(&stmt.lhs) {
            result.insert(stmt.lhs.clone(), BTreeSet::new());
        }
        if !result.contains_key(&stmt.rhs) {
            result.insert(stmt.rhs.clone(), BTreeSet::new());
        }
    }

    let mut change_ocurred = true;

    while change_ocurred {
        change_ocurred = false;
        for stmt in stmts.iter() {
            match stmt.kind {
                StmtKind::Ref => {
                    let lhs_set = result.get_mut(&stmt.lhs).unwrap();
                    if !lhs_set.contains(&stmt.rhs) {
                        lhs_set.insert(stmt.rhs.clone());
                        change_ocurred = true;
                    }
                }

                StmtKind::Alias => {
                    let lhs_set = &result[&stmt.lhs];
                    let rhs_set = &result[&stmt.rhs];
                    let mut new_lhs_set = lhs_set.clone();
                    insert_set_into(&mut new_lhs_set, &rhs_set);
                    if lhs_set.len() < new_lhs_set.len() {
                        change_ocurred = true;
                    }
                    result.insert(stmt.lhs.clone(), new_lhs_set);
                }

                StmtKind::DerefRead => {
                    let lhs_set = &result[&stmt.lhs];
                    let rhs_set = &result[&stmt.rhs];
                    let mut new_lhs_set = lhs_set.clone();
                    for v in rhs_set.iter() {
                        let v_set = result[v].clone();
                        insert_set_into(&mut new_lhs_set, &v_set);
                    }
                    if lhs_set.len() < new_lhs_set.len() {
                        change_ocurred = true;
                    }
                    result.insert(stmt.lhs.clone(), new_lhs_set);
                }

                StmtKind::DerefWrite => {
                    let lhs_set = result[&stmt.lhs].clone();
                    for v in lhs_set.iter() {
                        let v_set = &result[v];
                        let mut new_v_set = v_set.clone();
                        let rhs_set = result[&stmt.rhs].clone();
                        insert_set_into(&mut new_v_set, &rhs_set);
                        if v_set.len() < new_v_set.len() {
                            change_ocurred = true;
                            result.insert(v.clone(), new_v_set);
                        }
                    }
                }
            }
        }
    }
    return result;
}

fn insert_set_into(target: &mut BTreeSet<String>, src: &BTreeSet<String>) {
    for s in src.iter() {
        if !target.contains(s) {
            target.insert(s.clone());
        }
    }
}

pub fn print_solve_result(result: &SolveResult) {
    for (k, v) in result {
        print!("{} -> [", k);
        let mut i = 0;
        for s in v {
            if i != v.len() - 1 {
                print!("{}, ", s);
            } else {
                print!("{}", s);
            }
            i += 1;
        }
        println!("]");
    }
}
