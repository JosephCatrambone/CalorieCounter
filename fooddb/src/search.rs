
use hashbrown::HashMap;
use mathru::*;
use mathru::algebra::linear::{Vector, Matrix};
//use mathru::statistics::distrib::{Distribution, Normal};
//use mathru::optimization::{Optim, LevenbergMarquardt};
use regex::Regex;
use std::ops::Mul;
use mathru::algebra::linear::matrix::Transpose;

const MAX_FEATURES:usize = 1000;
const EMBEDDING_SIZE:usize = 64; // For conversion to a u64.

pub fn calculate_embedding(food: &str, lsi_matrix: &Matrix<f32>) -> u64 {
	todo!()
}

pub fn build_ngram_lsi(foods: &Vec<&str>) -> (HashMap<String, usize>, Matrix<f32>) {
	// In LSI, a COLUMN is a document and a row is a word.
	// If we build our big matrix NxM of N words and M documents,
	// then compute U, S, V, U will be...
	// U -> N words by N words,
	// S -> min(M,N) ( scale back by allocating a matrix of zeros of compatible size )
	// V -> M docs by M docs,
	// Normally, you can get back to the matrix A by...
	// dot(dot(U, diag_to_zeros(S, M, N)), V)

	// Calculate character, bichar, and trichar features,
	// then split all words and calculate the TF-IDF matrix,
	// then compute the LSI embedding in 64-dim space.
	println!("Learning feature map...");
	let features = learn_feature_map(foods);

	// Allocate a HUUUUUGE vector.  Would be nice if this were sparse, but...
	println!("Allocating big matrix...");
	let mut document_matrix:Matrix<f32> = Matrix::zero(MAX_FEATURES, foods.len());
	println!("Vectorizing foods...");
	for (cidx, f) in foods.iter().enumerate() {
		let v = compute_counts_vector(f, &features);
		document_matrix.set_column(&v, cidx);
	}

	// Embed via doc_embed = S_inv * U^T * doc_input.
	// S_inv is just the negation of every element, since it's a diagonal matrix.
	println!("Computing SVD...");
	dbg!("{:?}", &document_matrix);
	let (u, s, _v): (Matrix<f32>, Matrix<f32>, Matrix<f32>) = document_matrix.dec_sv();
	let mut s_reduced = Matrix::zero(64, 64);
	for i in 0..EMBEDDING_SIZE {
		*s_reduced.get_mut(i, i) = -s.get(i, i); // s^-1 = -s_i_j for diagonal matrices.
	}
	println!("Building embedding matrix...");
	let embedding:Matrix<f32> = s_reduced * u.transpose();

	return (features, embedding);
}

/// Calculate the MAX_FEATURES-wide vector of counts.
fn compute_counts_vector(name: &str, feature_map:&HashMap<String, usize>) -> Vector<f32> {
	let mut result = Vector::zero(MAX_FEATURES);
	let name_letters:Vec<char> = name.chars().collect();
	// First, calculate all the single, double, and triple character features.
	for chrs in name_letters.windows(2) {
		let ntuple:String = chrs.iter().collect();
		if let Some(idx) = feature_map.get(&ntuple) {
			*result.get_mut(*idx) += 1.0;
		}
	}

	// Now convert the name to lower-case and split out all the words.
	// Make sure this stays in sync with the method in learn_feature_map.
	// Note that here we're doing binary features, rather than counts.
	let split_criteria = Regex::new(r"\W").unwrap();
	let lowercase_food = name.to_lowercase();
	let tokens: Vec<String> = split_criteria.split(&lowercase_food).map(|s|{String::from(s)}).collect();
	for token in tokens {
		if let Some(idx) = feature_map.get(&token) {
			*result.get_mut(*idx) = 1.0;
		}
	}

	result
}

fn learn_feature_map(foods: &Vec<&str>) -> HashMap<String, usize> {
	// 26^2 = 676
	// 26^3 = 17576 reserved for n-gram characters.  Remaining 50k for full-words.
	let mut features = HashMap::with_capacity(MAX_FEATURES);
	let mut index:usize = 0;
	// First, allocate space for all three letter words.
	for character_a in 'a'..'z' {
		for character_b in 'a'..'z' {
			//for character_c in 'a'..'z' {
				//features.insert(format!("{}{}{}", character_a, character_b, character_c), index);
				features.insert(format!("{}{}", character_a, character_b), index);
				index += 1;
			//}
		}
	}

	// For each of the foods, convert to lower-case, convert all non-characters to spaces, and split on space.
	//let non_az = Regex::new(r"[^a-z]").unwrap();
	//let split_criteria = Regex::new(r"\W+").unwrap(); // Match all 'not-word'.  Maybe consider \s to match all spaces?  \W will match the ( and ) in (Foo).
	let split_criteria = Regex::new(r"\W").unwrap();
	// Rather than replace non [a-z] with spaces and split on space, just split on non-a-z!
	let mut word_count = HashMap::<String, u32>::new();  // We don't have separate TF/IDF matrices because each doc is only a few words, so we can assume there's only _one_.
	for food in foods {
		let lowercase_food = food.to_lowercase();
		let tokens: Vec<String> = split_criteria.split(&lowercase_food).map(|s|{String::from(s)}).collect();
		for token in tokens {
			let next_value = match word_count.get(&token) {
				Some(value) => *value,
				None => 0
			}+1;
			word_count.insert(token, next_value);
		}
	}

	// Make a bunch of tuples from the frequency to the word, then filter the top n.
	let mut frequency_word_tuples:Vec<(u32, String)> = word_count.iter().map(|(word, count)|{(*count, word.clone())}).collect();
	frequency_word_tuples.sort_by(|a, b|{ a.0.cmp(&b.0) });
	frequency_word_tuples.reverse();
	for (_count, word) in frequency_word_tuples {
		if index > MAX_FEATURES {
			break;
		}
		if word == "" {
			continue;
		}
		if !features.contains_key(&word) {
			features.insert(word, index);
			index += 1;
		}
	}

	features
}

fn make_tokens(food: &str, split_criteria:Option<Regex>) -> Vec<String> {
	let split_criteria = match split_criteria {
		Some(sc) => sc,
		None => Regex::new(r"\W").unwrap()
	};

	let lowercase_food = food.to_lowercase();
	let tokens: Vec<String> = split_criteria.split(&lowercase_food).map(|s|{String::from(s)}).collect();
	tokens
}

#[cfg(test)]
mod tests {
	use crate::*;
	use crate::search::{learn_feature_map, make_tokens, build_ngram_lsi};

	fn bootstrap_foods() -> Vec<&'static str> {
		vec![
			"Chicken and Waffles (900 Grayson)",
			"boring vegetables (boring vegetables company)",
			"EXCITING! vegetables",
			"generic protein",
			"Delicious Cake (Tasty Cake Bakery)",
			"Carrots",
			"Potatoes (Mashed)",
			"Fetuccini",
			"Tomatoes",
			"Power Shake Protein",
		]
	}

	#[test]
	fn test_learned_feature_map() {
		let foods = bootstrap_foods();
		let feature_map = learn_feature_map(&foods);
		assert!(feature_map.contains_key("generic"));
		assert!(feature_map.contains_key("vegetables"));
		assert!(feature_map.contains_key("exciting"));
		assert!(feature_map.contains_key("and"));
		dbg!("{:?}", feature_map);
	}

	#[test]
	fn test_tokenization() {
		let tokens = make_tokens("GeNeRiC Foooods? I'm Ron Burgundy?", None);
		assert!(tokens.contains(&"generic".to_string()));
		assert!(tokens.contains(&"foooods".to_string()));
		assert!(tokens.contains(&"i".to_string()));
		assert!(tokens.contains(&"m".to_string()));
		assert!(tokens.contains(&"ron".to_string()));
		assert!(tokens.contains(&"burgundy".to_string()));
	}

	#[test]
	fn test_lsi() {
		let foods = bootstrap_foods();
		let (feature_map, embedding_mat) = build_ngram_lsi(&foods);
		dbg!("{:?}", feature_map);
		dbg!("{:?}", embedding_mat);
	}
}
