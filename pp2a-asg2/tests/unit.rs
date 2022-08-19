use pp2a_asg2::{OrderedLinearArray, WordCollection};

const NAMES: [&str; 8] = [
	"Peter", "Sathish", "Wade", "Don", "Indrajit", "Rahul", "Sam", "Kevin",
];

#[test]
fn driver_ord_array_linear() {
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

	assert!(ola.search_collection("Sathish"));
	assert!(!ola.search_collection("Kratos"));
}
