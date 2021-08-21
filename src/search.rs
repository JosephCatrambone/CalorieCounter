use std::collections::BTreeMap;
use crate::food::{FoodID, Food};
use hashbrown::HashMap;

pub struct FoodSearchResult {
	pub id: FoodID,
	pub name: String,
	relevance: f32
}

// Do not derive serialize/deserialize.  Regenerate index on init.
pub struct SearchIndex {
	fulltext_index: BTreeMap<String, FoodID>,
	autocomplete_index: PrefixTree,
}

impl Default for SearchIndex {
	fn default() -> Self {
		SearchIndex {
			fulltext_index: BTreeMap::new(),
			autocomplete_index: PrefixTree::new(),
		}
	}
}

impl SearchIndex {
	pub fn empty() -> Self {
		SearchIndex::default()
	}

	pub fn reindex(&mut self, food_db:&Vec<Food>) {
		self.fulltext_index = BTreeMap::new();
		self.autocomplete_index = PrefixTree::new();
		food_db.iter().for_each(|f|{
			self.fulltext_index.insert(f.name.clone(), f.id);
			self.autocomplete_index.add_word(f.name.clone());
		});
	}

	pub fn new(food_db:&Vec<Food>) -> Self {
		let mut new_index = SearchIndex::empty();
		new_index.reindex(food_db);
		new_index
	}

	pub fn search(&self, food_name:&String, max_results:Option<u8>) -> Vec<FoodSearchResult> {
		let mut matches = vec![];
		let max_results = max_results.unwrap_or(10);

		// If we have an exact match, return it.
		if let Some(id) = self.fulltext_index.get(food_name) {
			matches.push(FoodSearchResult {
				id: *id,
				name: food_name.clone(),
				relevance: 1.0f32,
			});
		}
		
		// Append other matches.
		for name in self.autocomplete_index.fuzzy_matches(food_name, max_results) {
			if let Some(id) = self.fulltext_index.get(&name) {
				matches.push(FoodSearchResult {
					id: *id,
					name: name,
					relevance: 0.0f32,
				})
			}
		}

		matches
	}
}

pub struct PrefixTree {
	depth: usize,
	child_trees: HashMap<char, Box<PrefixTree>>,
	words: Vec<String>, // What full, finished words exist here?
}

impl PrefixTree {
	pub fn new() -> Self {
		PrefixTree {
			depth: 0,
			child_trees: HashMap::new(),
			words: vec![],
		}
	}

	pub fn add_word(&mut self, word:String) {
		// Is there another letter we need to store?
		if self.depth >= word.len() {
			// No.  Can just add the word at this level.
			self.words.push(word);
		} else {
			// We check above for length, so we can unwrap.
			let index_character:char = word.to_lowercase().chars().nth(self.depth).unwrap();
			// If there isn't already a subtree for this character, create one.
			if !self.child_trees.contains_key(&index_character) {
				let mut new_tree = PrefixTree::new();
				new_tree.depth = self.depth + 1;
				self.child_trees.insert(index_character, Box::new(new_tree));
			}
			// Use a mutable reference to recursively insert.
			if let Some(chld) = self.child_trees.get_mut(&index_character) {
				chld.add_word(word);
			}
		}
	}

	pub fn fuzzy_matches(&self, starting_string:&String, max_results: u8) -> Vec<String> {
		let mut matches = Vec::<String>::with_capacity(max_results as usize);

		// If we are past our depth, i.e., there are no more characters in the autocomplete sequence, give back everything at this level.
		if self.depth >= starting_string.len() {
			for w in &self.words {
				matches.push(w.clone());
			}
			for c in self.child_trees.values() {
				matches.extend(c.fuzzy_matches(starting_string, max_results));
				if matches.len() > max_results as usize {
					break;
				}
			}
		} else { // Otherwise, grab the token at the given depth and recurse into this tree.
			let index_char = starting_string.to_lowercase().chars().nth(self.depth).unwrap();
			if let Some(c) = self.child_trees.get(&index_char) {
				matches = c.fuzzy_matches(starting_string, max_results);
			}
		}

		matches
	}
}


#[cfg(test)]
mod tests {
	use crate::*;
	use crate::search::{SearchIndex, PrefixTree};

	fn bootstrap_foods() -> Vec<Food> {
		let mut food_db = vec![];

		let mut sugar = Food::default();
		sugar.name = "Sugar".to_string();
		sugar.manufacturer = "Old Mill".to_string();
		food_db.push(sugar);

		let mut splenda = Food::default();
		splenda.name = "splenda".to_string();
		food_db.push(splenda);

		let mut generic_food = Food::default();
		generic_food.name = "food".to_string();
		food_db.push(generic_food);

		food_db
	}

	#[test]
	fn test_prefix_tree() {
		let mut autocomplete = PrefixTree::new();
		let foods = bootstrap_foods();
		for f in &foods {
			autocomplete.add_word(f.name.clone());
		}

		// Should find both spelenda and sugar, but not 'food'.
		let res:Vec<String> = autocomplete.fuzzy_matches(&"s".to_string(), 10);
		assert!(res.contains(&"Sugar".to_string()));
		assert!(res.contains(&"splenda".to_string()));
		assert!(!res.contains(&"food".to_string()));

		let empty:Vec<String> = autocomplete.fuzzy_matches(&"asdf".to_string(), 10);
		assert!(empty.is_empty());

		let just_food:Vec<String> = autocomplete.fuzzy_matches(&"food".to_string(), 10);
		assert_eq!(vec!["food"], just_food);
	}

	#[test]
	fn test_search() {
		let foods = bootstrap_foods();
		let index = SearchIndex::new(&foods);

		let search_results = index.search(&"sugar".to_string(), None);
	}
}
