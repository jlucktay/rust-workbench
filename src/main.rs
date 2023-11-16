fn main() {
	let my_number: u8 = 100; //  change my_number to my_number: u8
	println!("{}", my_number as char);

	println!("Hello, world!");

	let is_true = Bool::True;
	let is_false = Bool::False;
	println!("{is_true:?}");
	let are_same = is_true == is_false;
	let not_true = is_true.neg();

	let is_alive = HealthBar::Alive { life: 100 };

	let dead_player = Player::Dead;
	let knocked_out_player = Player::KnockedOut {
		life: 50,
		turns_to_wait: 3,
	};

	let possible = safe_division(4, 0);
	let also_possible = safe_division_result(4, 0);

	let two = Some(2).unwrap_or(0);
	let safe = None.unwrap_or(0);

	let one = safe_division_twice(4, 2, 2);
	let oops = safe_division_twice(4, 0, 2);

	let one = safe_division(4, 2).and_then(|x| safe_division(x, 2));
	let oops = safe_division(4, 0).and_then(|x| safe_division(x, 2));
}

#[derive(Debug, Eq, PartialEq)]
enum Bool {
	True,
	False,
}

enum HealthBar {
	Alive { life: i8 },
	Dead,
}

#[derive(Clone, Copy)]
enum Player {
	Alive { life: i8 },
	KnockedOut { life: i8, turns_to_wait: i8 },
	Dead,
}

impl Bool {
	const fn neg(self) -> Self {
		match self {
			Self::True => Self::False,
			Self::False => Self::True,
		}
	}
}

fn print_value(value: &Bool) {
	match value {
		Bool::True => println!("Clearly true!"),
		Bool::False => println!("Unfortunately false!"),
	}
}

fn print_and_return_value(value: &Bool) -> Bool {
	match value {
		Bool::True => {
			println!("Clearly true!");
			Bool::True
		}
		Bool::False => {
			println!("Unfortunately false!");
			Bool::False
		}
	}
}

const fn take_five(player: Player) -> Player {
	match player {
		Player::Alive { life } if life > 5 => Player::Alive { life: (life - 5) },
		Player::KnockedOut {
			life,
			turns_to_wait,
		} if life > 5 => Player::KnockedOut {
			life: (life - 5),
			turns_to_wait,
		},
		_ => Player::Dead,
	}
}

const fn safe_division(a: i32, b: i32) -> Option<i32> {
	match b {
		0 => None,
		_ => Some(a / b),
	}
}

#[derive(Debug, Clone)]
struct DivideByZero;

const fn safe_division_result(a: i32, b: i32) -> Result<i32, DivideByZero> {
	match b {
		0 => Err(DivideByZero),
		_ => Ok(a / b),
	}
}

fn safe_division_twice(a: i32, b: i32, c: i32) -> Option<i32> {
	let result = safe_division(a, b)?;
	safe_division(result, c)
}
