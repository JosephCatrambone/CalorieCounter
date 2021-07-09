use std::collections::BTreeMap;
use crate::food::{FoodID, Food};
use hashbrown::HashMap;

pub struct FoodSearchResult {
	id: FoodID,
	name: String,
	relevance: f32
}

// Do not derive serialize/deserialize.  Regenerate index on init.
pub struct SearchIndex<'a> {
	fulltext_index: BTreeMap<String, FoodID>,
	food_db_ref: &'a Vec<Food>
}

impl<'a> SearchIndex<'a> {
	pub fn new(food_db:&'a Vec<Food>) -> Self {
		SearchIndex {
			fulltext_index: BTreeMap::new(),
			food_db_ref: food_db,
		}
	}

	pub fn search(&self, food_name:String) -> Vec<FoodSearchResult> {
		todo!()
	}
}

pub struct PrefixTree {
	prefix: char,
	depth: usize,
	child_trees: HashMap<char, Box<PrefixTree>>,
	words: Vec<String>, // What full, finished words exist here?
}

impl PrefixTree {
	fn new() -> Self {
		PrefixTree {
			prefix: ' ',
			depth: 0,
			child_trees: HashMap::new(),
			words: vec![],
		}
	}

	fn add_word(&mut self, word:String) {
		// Is there another letter we need to store?
		if self.depth >= word.len() {
			// No.  Can just add the word at this level.
			self.words.push(word);
		} else {
			// We check above for length, so we can unwrap.
			let index_character:char = word.chars().nth(self.depth).unwrap();
			// If there isn't already a subtree for this character, create one.
			if !self.child_trees.contains_key(&index_character) {
				let mut new_tree = PrefixTree::new();
				new_tree.prefix = index_character;
				new_tree.depth = self.depth + 1;
				self.child_trees.insert(index_character, Box::new(new_tree));
			}
			// Use a mutable reference to recursively insert.
			if let Some(chld) = self.child_trees.get_mut(&index_character) {
				chld.add_word(word);
			}
		}
	}

	fn fuzzy_matches(&self, starting_string:String, max_results: u8) -> Vec<String> {
		let matches = Vec::<String>::with_capacity(max_results as usize);

		todo!();

		matches
	}
}


#[cfg(test)]
mod tests {
	use crate::*;
	use crate::search::SearchIndex;

	fn bootstrap_foods() -> Vec<Food> {
		let mut food_db = vec![];

		let mut sugar = Food::default();
		sugar.name = "Sugar".to_string();
		sugar.manufacturer = "Old Mill".to_string();
		food_db.push(sugar);

		food_db
	}

	#[test]
	fn test_prefix_tree() {

	}

	#[test]
	fn test_search() {
		let foods = bootstrap_foods();
		let index = SearchIndex::new(&foods);

		let search_results = index.search("sugar".to_string());
	}
}
