use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub struct Entry<V>{
    pub value: Arc<V>,
    pub duration: Instant //non Duration se no non capisci quando è scaduto
}
pub struct Cache<K: Eq + Hash, V>{
    pub data: RwLock<HashMap<K, Entry<V>>>,
}
//guarda cosa viene tornato

impl<K: Eq + Hash, V> Cache<K, V>{
    pub fn new() -> Self{
        Cache{
            data: RwLock::new(HashMap::new())
        }
    }
    pub fn size(&self) -> usize{
        self.data.read().unwrap().len()
    }
    pub fn put(&self, k: K, v:V, d:Duration){
        let lifetime = Instant::now() + d;
        let entry = Entry{
            value: Arc::new(v),
            duration: lifetime
        };
        let mut map_guard = self.data.write().unwrap();
        map_guard.insert(k, entry);
        //ciclo di pulizia
        map_guard.retain(|_, v| v.duration > Instant::now());
        //con ciclo for
        /*for (k, v) in map_guard.iter(){
            if v.duration < Instant::now(){
                map_guard.remove(k);
            }
        }*/
    }
    pub fn renew(&self, k: &K, d:Duration) -> bool{
        let mut map = self.data.write().unwrap();
        match map.get_mut(k){
            Some(value) =>{ //se la chiave esiste
                //se la chiave è scaduta
                if value.duration < Instant::now(){
                    return false;
                }
                //se la chiave è ancora valida
                else{
                    value.duration = Instant::now() + d;
                    return true;
                }
            }
            None => return false
        }
    }
    pub fn get(&self, k:&K) -> Option<Arc<V>>{
        let map = self.data.read().unwrap();
        match map.get(k){//controllo se la chiave esiste
            Some(entry) => {
                //se la chiave è scaduta
                if entry.duration< Instant::now(){
                    None
                }
                //se la chiave è ancora valida
                else{
                    Some(Arc::clone(&entry.value))
                }
            }
            None => None

        }
    }

}