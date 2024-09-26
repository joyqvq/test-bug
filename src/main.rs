use std::{collections::{HashMap, HashSet}, sync::{Arc, RwLock}};

fn main() {
    let state = Arc::new(AppState {
        values: Default::default(),
    });

    for keys in vec![vec![("p1_key1".to_string(), "p1_val1".to_string()), ("p1_key2".to_string(), "p1_val2".to_string())], vec![("p2_key1".to_string(), "p2_val1".to_string())]] {
        let state_clone = state.clone();
        update_keys(state_clone, keys);
    }
    println!("{:?}", state.values.read().unwrap());
}

/// Application state that contains the seed and JWKs.
#[derive(Clone, Debug)]
pub struct AppState {
    pub values: Arc<RwLock<HashMap<String, String>>>,
}

pub fn update_keys(state_clone: Arc<AppState>, keys: Vec<(String, String)>) {
    let ids = keys.iter().map(|(k, _)| k.clone()).collect::<HashSet<_>>();
    let mut values = state_clone.values.write().unwrap();
    for k in values.clone().keys() {
        if !ids.contains(k) {
            values.remove(k);
        }
    }
    for (jwk_id, jwk) in keys {
        if values.contains_key(&jwk_id) {
            continue;
        }

        values.insert(jwk_id, jwk.clone());
    }
}