use pp2a_asg2::{OrderedLinearArray, WordCollection};

const NAMES: [&str; 8] = [
	"Peter", "Sathish", "Wade", "Don", "Indrajit", "Rahul", "Sam", "Kevin",
];

#[test]
fn driver_ord_array_linear() {
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

	println!(
		"Current Implementation based on: {}",
		std::any::type_name::<OrderedLinearArray>()
	);

	let mut ola = OrderedLinearArray::default();
	assert_eq!(ola.size_collection(), 0);

	for name in NAMES {
		ola.add_collection(name);
	}

	assert_eq!(ola.size_collection(), 8);

	println!("The following names are in the Collection:");
	ola.display_collection();

	assert_eq!(EXPECTED_OUTPUT, ola.to_string());

	assert!(ola.search_collection("Sathish"));
	assert!(!ola.search_collection("Kratos"));
}
