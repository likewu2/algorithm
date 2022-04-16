use std::collections::HashMap;

pub fn intersect_list(l1: Option<Vec<i32>>, l2: Option<Vec<i32>>) -> Option<Vec<i32>> {
    match (&l1, &l2) {
        (Some(n1), Some(n2)) => {
          let mut n3 = Vec::new();
          let mut n4 = n1.clone();
          let mut n5 = n2.clone();
          n4.sort();
          n5.sort();
          let mut i=0;
          let mut j=0;
          while i<n4.len() && j<n5.len(){
            if n4[i]>n5[j] {
              j+=1;
            } else if n4[i]<n5[j] {
              i+=1;
            } else {
              n3.push(n4[i]);
              i+=1;
              j+=1;
            }
          }
          Some(n3)
        }
        (Some(n1), None) => Some(n1.clone()),
        (None, Some(n2)) => Some(n2.clone()),
        _ => None
    }
}
