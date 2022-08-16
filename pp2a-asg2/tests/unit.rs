use pp2a_asg2::{OrderedLinearArray, WordCollection};

const NAMES: [&str; 8] = [
	"Peter", "Sathish", "Wade", "Don", "Indrajit", "Rahul", "Sam", "Kevin",
];

#[test]
fn driver_ord_array_linear() {
	assert_eq!(NAMES.len(), 8);

	let make_result = OrderedLinearArray::make_collection(NAMES.as_slice());

	assert!(make_result.is_ok());
}
