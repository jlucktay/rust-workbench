use pp2a_asg2::{
	ord_array_binary::OrderedBinaryArray, ord_array_linear::OrderedLinearArray, WordCollection,
};

const NAMES: [&str; 8] = [
	"Peter", "Sathish", "Wade", "Don", "Indrajit", "Rahul", "Sam", "Kevin",
];

#[test]
fn driver() {
	const EXPECTED_OUTPUT: &str = r#"Element #1:	Don
Element #2:	Indrajit
Element #3:	Kevin
Element #4:	Peter
Element #5:	Rahul
Element #6:	Sam
Element #7:	Sathish
Element #8:	Wade
"#;

	assert_eq!(NAMES.len(), 8);

	let ord_linear = OrderedLinearArray::default();
	let ord_binary = OrderedBinaryArray::default();

	let boxed_collections: Vec<Box<dyn WordCollection>> =
		vec![Box::new(ord_linear), Box::new(ord_binary)];

	for mut collection in boxed_collections {
		assert_eq!(collection.size_collection(), 0);

		for name in NAMES {
			collection.add_collection(name);
		}

		assert_eq!(collection.size_collection(), 8);

		println!("The following names are in the Collection:");
		collection.display_collection();

		assert_eq!(EXPECTED_OUTPUT, collection.to_string());

		assert!(collection.search_collection("Sathish"));
		assert!(!collection.search_collection("Kratos"));
	}
}
