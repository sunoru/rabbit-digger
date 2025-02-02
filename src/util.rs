use std::{collections::HashMap, hash::Hash};

use topological_sort::TopologicalSort;

pub fn topological_sort<K, V, D, E>(
    mut map: HashMap<K, V>,
    get_deps: D,
) -> Result<Option<Vec<(K, V)>>, E>
where
    K: Hash + Eq + Clone,
    D: Fn(&V) -> Result<Vec<K>, E>,
{
    let mut ts = TopologicalSort::<K>::new();

    for (k, v) in map.iter() {
        for d in get_deps(v)?.into_iter() {
            ts.add_dependency(d, k.clone());
        }
    }

    let mut list = Vec::<K>::new();
    while let Some(k) = ts.pop() {
        list.push(k.clone());
    }

    if ts.len() > 0 {
        return Ok(None);
    }

    Ok(Some(
        list.into_iter()
            .map(|k| {
                let v = map.remove(&k);
                v.map(|v| (k, v))
            })
            .filter_map(|i| i)
            .collect(),
    ))
}
