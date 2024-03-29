use pp2a_asg2::{
	binary::OrderedBinaryArray, linear::OrderedLinearArray, linked::OrderedLinkedList,
	ubst::UnbalancedBinarySearchTree, WordCollection,
};

const NAMES: [&str; 8] = [
	"Peter", "Sathish", "Wade", "Don", "Indrajit", "Rahul", "Sam", "Kevin",
];

#[test]
fn driver() {
	// The output should be sorted in alphabetical order.
	const EXPECTED_OUTPUT: &str =
		r#"["Don", "Indrajit", "Kevin", "Peter", "Rahul", "Sam", "Sathish", "Wade"]"#;

	assert_eq!(NAMES.len(), 8);

	let ord_linear = OrderedLinearArray::new();
	let ord_binary = OrderedBinaryArray::new();
	let ord_linked = OrderedLinkedList::new();
	let bin_tree = UnbalancedBinarySearchTree::new();

	let boxed_collections: Vec<Box<dyn WordCollection>> = vec![
		Box::new(ord_linear),
		Box::new(ord_binary),
		Box::new(ord_linked),
		Box::new(bin_tree),
	];

	for mut collection in boxed_collections {
		assert_eq!(collection.size(), 0);

		for name in NAMES {
			collection.add(name);
		}

		assert_eq!(collection.size(), 8);

		let actual_output = collection.to_string();

		println!(
			"The following names are in the Collection:\n{}",
			actual_output
		);

		assert_eq!(EXPECTED_OUTPUT, actual_output);

		assert!(collection.search("Sathish"));
		assert!(!collection.search("Kratos"));
	}
}
