use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

fn main() {
    Problem::read_from_stdin().unwrap().solve();
}

struct Problem {
    inferior_constraints: Vec<(Vec<i32>, Vec<i32>)>,
    superior_constraints: Vec<(Vec<i32>, Vec<i32>)>,
    constant_constraints: Vec<(Vec<i32>, Vec<i32>)>,
}
impl Problem {
    fn read_from_stdin() -> Result<Self, Box<dyn Error>> {
        let stdin = std::io::stdin();
        let mut inferior_constraints = Vec::new();
        let mut superior_constraints = Vec::new();
        let mut constant_constraints = Vec::new();
        let mut vars = HashSet::new();
        for line in stdin.lines() {
            let line = line?;
            let Some((left, right)) = line.split_once("b.w.") else {
                return Err(format!("wrong format: {line}").into());
            };
            let parse = |side: &str| {
                let mut terms = side.split_whitespace().peekable();
                let mut var = None;
                if let Some(s) = terms.peek() {
                    if s.starts_with(char::is_alphabetic) {
                        var = Some(s.to_string());
                        terms.next();
                    }
                }
                let rem = terms.map(|s| s.parse().unwrap()).collect();
                (var, rem)
            };
            let (left_var, left_rem) = parse(left);
            let (right_var, right_rem) = parse(right);
            match (&left_var, &right_var) {
                (Some(_), None) => inferior_constraints.push((left_rem, right_rem)),
                (None, Some(_)) => superior_constraints.push((left_rem, right_rem)),
                _ => constant_constraints.push((left_rem, right_rem)),
            }
            vars.extend(left_var);
            vars.extend(right_var);
        }
        if vars.len() >= 2 {
            return Err(format!("multiple variables: {vars:?}").into());
        }
        Ok(Problem {
            inferior_constraints,
            superior_constraints,
            constant_constraints,
        })
    }
    fn solve(&self) {
        for (left, right) in &self.constant_constraints {
            if !left.starts_with(&right) {
                println!(
                    "constraint never satisfied: {} b.w. {}",
                    left.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                    right
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                return;
            }
        }
        for (prefix, has_arbitrary_tail) in self.solve_impl() {
            let mut vec: Vec<_> = prefix.iter().map(ToString::to_string).collect();
            if has_arbitrary_tail {
                vec.push("t (t is arbitrary)".to_string());
            }
            if vec.is_empty() {
                println!("epsilon")
            } else {
                println!("{}", vec.join(" "))
            }
        }
    }
    fn solve_impl(&self) -> Vec<(VecDeque<i32>, bool)> {
        let mut ret = Vec::new();
        if self
            .inferior_constraints
            .iter()
            .chain(&self.superior_constraints)
            .all(|(left, right)| left.starts_with(right))
        {
            ret.push((VecDeque::new(), false));
        }
        let mut heads = HashSet::new();
        let mut next_inferior_constraints = Vec::new();
        for (left, right) in &self.inferior_constraints {
            if let Some((&head, tail)) = right.split_first() {
                heads.insert(head);
                next_inferior_constraints.push((left.to_vec(), tail.to_vec()));
            }
        }
        let mut next_superior_constraints = Vec::new();
        for (left, right) in &self.superior_constraints {
            if let Some((&head, tail)) = left.split_first() {
                heads.insert(head);
                next_superior_constraints.push((tail.to_vec(), right.to_vec()));
            } else {
                return ret;
            }
        }
        match heads.len() {
            0 => return vec![(VecDeque::new(), true)],
            1 => {
                let head = heads.into_iter().next().unwrap();
                let mut tmp = Problem {
                    inferior_constraints: next_inferior_constraints,
                    superior_constraints: next_superior_constraints,
                    constant_constraints: Vec::new(),
                }
                .solve_impl();
                for (ans, _) in &mut tmp {
                    ans.push_front(head);
                }
                ret.extend(tmp);
            }
            _ => {}
        }
        ret
    }
}
