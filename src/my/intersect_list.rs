use std::collections::HashMap;

pub fn intersect_list(l1: Option<Vec<i32>>, l2: Option<Vec<i32>>) -> Option<Vec<i32>> {
    match (&l1, &l2) {
        (Some(n1), Some(n2)) => {
          let mut m0: HashMap<i32, i32> = HashMap::new();
          let mut n3 = Vec::new();
          for pat in n1 {
            if let Some(n)=m0.get(pat) {
              m0.insert(*pat, *n+1);
            } else {
              m0.insert(*pat, 1);
            }
          }
          for pat in n2 {
            if let Some(n)=m0.get(pat) {
              if *n>0 {
                m0.insert(*pat, *n-1);
                n3.push(*pat);
              }
            }
          }
          Some(n3)
        }
        (Some(n1), None) => Some(n1.clone()),
        (None, Some(n2)) => Some(n2.clone()),
        _ => None
    }
}
