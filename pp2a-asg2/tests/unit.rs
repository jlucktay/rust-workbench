use pp2a_asg2::WordCollection;

const NAMES: [&str; 8] = [
	"Peter", "Sathish", "Wade", "Don", "Indrajit", "Rahul", "Sam", "Kevin",
];

#[test]
fn driver_ord_array_linear() {
	assert_eq!(NAMES.len(), 8);
	// sort using normal methods (for comparison)
	let mut sorted: Vec<String> = NAMES.iter().map(|s| s.to_string()).collect();
	sorted.sort();

	// create
	let mut make_result = WordCollection::with_capacity(8);
	// add all words (random order)
	NAMES.iter().for_each(|s| make_result.add_word(s));

	// Check sorted
	assert_eq!(make_result.to_vec(), sorted);
	// Check search
	assert_eq!(make_result.search("Wade"), true);
	assert_eq!(make_result.search("Sam"), true);
	assert_eq!(make_result.search("Rahhh"), false);

	// println!("{}", make_result); // or make_result.to_string()
	/*
		Element #1:     Don
		Element #2:     Indrajit
		Element #3:     Kevin
		Element #4:     Peter
		Element #5:     Rahul
		Element #6:     Sam
		Element #7:     Sathish
		Element #8:     Wade
	*/
	// println!("{:?}", make_result);
	/* WordCollection { words: ["Don", "Indrajit", "Kevin", "Peter", "Rahul", "Sam", "Sathish", "Wade"] } */
}
