use std::cell::RefCell;

struct Valami {
	parent: Option<RefCell<~Valami>>,
	x: int
}

impl Valami {
	fn new() -> RefCell<~Valami> {
		return RefCell::new(~Valami{parent: None, x: 0});
	}

	fn set_parent(&mut self, parent: RefCell<~Valami>) {
		self.parent = Some(parent);
	}

	fn set_x(&mut self, x: int) {
		self.x = x;
	}
}

#[test]
fn test_rc() {
	let vr0 = Valami::new();
	let vr1 = Valami::new();

	let mut v1 = vr1.borrow_mut();
	let mut a1 = v1.get();
	a1.set_parent(vr0);
	a1.set_x(3);

	let mut v0 = a1.parent.get_mut_ref().borrow_mut();
	v0.get().set_x(1);

	assert_eq!(3, a1.x);
	assert_eq!(1, v0.get().x);
}