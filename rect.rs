
pub struct Rect {
	x: int,
	y: int,
	w: int,
	h: int
}

impl Rect {
	fn move(&self, add_x: int, add_y: int) -> Rect {
		return Rect{x: self.x + add_x,
					y: self.y + add_y,
					w: self.w,
					h: self.h}
	}

	fn grow(&self, add_x: int, add_y: int) -> Rect {
		return Rect{x: self.x,
					y: self.y,
					w: self.w + add_x,
					h: self.h + add_y}
	}

	fn contains(&self, x: int, y: int) -> bool {
		let contains_x = x >= self.x && x < self.x+self.w;
		let contains_y = y >= self.y && y < self.y+self.h;
		return contains_x && contains_y;
	}
}

#[test]
fn test_move() {
	let r = Rect{x: 0, y: 0, w: 0, h:0};
	let moved_rect = r.move(10, 15);
	assert_eq!(moved_rect.x, 10);
	assert_eq!(moved_rect.y, 15);
	assert_eq!(moved_rect.w, 0);
	assert_eq!(moved_rect.h, 0);
}

#[test]
fn test_grow() {
	let r = Rect{x: 0, y: 0, w: 0, h:0};
	let growed_rect = r.grow(10, 15);
	assert_eq!(growed_rect.x, 0);
	assert_eq!(growed_rect.y, 0);
	assert_eq!(growed_rect.w, 10);
	assert_eq!(growed_rect.h, 15);
}

#[test]
fn test_contains() {
	let r = Rect{x: 1, y: 2, w: 5, h:6};
	assert!(r.contains(1, 2));
	assert!(r.contains(3, 3));
	assert!(r.contains(5, 3));
	assert!(r.contains(3, 7));
	assert!(r.contains(5, 7));
	assert!(r.contains(1, 1) == false);
	assert!(r.contains(7, 7) == false);
	assert!(r.contains(6, 8) == false);
}