use crate::*;

#[near_bindgen]
impl Contract {
    pub fn increment_plays(&mut self, token_id: TokenId) {
        let plays = self.plays_by_song.get(&token_id).unwrap_or(0 as u64);
        self.plays_by_song.insert(&token_id, &(plays +1));
    }
}