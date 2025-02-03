#[path = "lc.rs"]
pub mod lc_mod;

pub mod test {
    pub fn vec_capa_matches_len<T>(v: &Vec<T>) -> bool {
        v.len() == v.capacity()
    }
}
