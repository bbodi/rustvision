pub use std::libc::*;
use std::ptr::*;
use view::rect::Rect;
use view::pixel::Pixel;
use std::cell::RefCell;

mod rect;
mod pixel;

//static focused_view : Option<*mut View> = None;

pub struct View {
	rect: Rect,
	owner: Option<*mut View>,
	// TODO: Legyen private, es a view-n kivül senki ne erhesse el. Legyen ra getter()
	font_size: Option<Pixel>,
	executing:       bool,
	dirty:           bool,
	has_dirty_child:  bool,
	priv hidden:         bool,
	focused:        bool,
	//Widget          Widget
	//ExecutingResult ExecutingResult
	//views           list.List
	
	//focusedElement  *list.Element
	font: Option<Font>,
}

pub struct Font {
	char_width: Pixel,
	char_height: Pixel,
}

impl View {

	fn new() -> ~View {
		~View{rect: Rect{x: 0, y: 0, w: 0, h:0},
					owner: None,
					font_size: None,
					hidden: false,
					dirty: false,
					executing: false,
					has_dirty_child: false,
					focused: false,
					font: Some(Font{char_width: Pixel(12), char_height: Pixel(12)}),
				}
	}

	fn unsafe_add_view(&mut self, view: *mut View) {
		unsafe {
			(*view).owner = Some(to_mut_unsafe_ptr(self));
		}
	}

	fn add_view(&mut self, view: &mut View, x: int, y: int) {
		self.unsafe_add_view(to_mut_unsafe_ptr(view));
		view.rect.x = x;
		view.rect.y = y;
		view.modified();
	}

	fn get_parent<'s>(&'s self) -> Option<&'s mut View> {
		unsafe {
			match(self.owner) {
				Some(parentPtr) => Some(&'s mut *parentPtr),
				None() 			=> None,
			}
		}
	}

	fn font_size(&self) -> Pixel {
		match(self.font_size) {
			Some(pxl) 	=> return pxl,
			None() 		=> {
				match (self.get_parent()) {
					Some(owner) => return owner.font_size(), 
					None 		=> fail!("no font_size nor parent!"),
				}
			}
		}
	}

	fn hide(&mut self) {
		self.hidden = true;
		self.modified();
	}

	fn is_hidden(&self) -> bool {
		return self.hidden;
	}

	fn show(&mut self) {
		self.hidden = false;
		self.modified();
	}

	fn modified(&mut self) {
		self.dirty = true;
		let mut parent = self.get_parent();
		loop {
			match(parent) {
				Some(parent_view) => {
					parent_view.has_dirty_child = true;
					parent = parent_view.get_parent();
				}
				None() => return,
			}
		}
	}

	fn make_local(&self, x: Pixel, y: Pixel) -> (int, int) {
		let (off_x, off_y) = self.calc_offset();
		let self_x = x - off_x;
		let self_y = y - off_y;
        let local_x: int;
        if self_x < Pixel(0) {
            local_x = -1
        } else {
        	let font = Font{char_width: self.font_size(), char_height: self.font_size()};
        	let font_char_w = font.char_width;
        	local_x = (self_x / font_char_w).to_int();
        }
        let local_y: int;
        if (self_y < Pixel(0)) {
            local_y = -1
        } else {
        	let font = Font{char_width: self.font_size(), char_height: self.font_size()};
        	let font_char_h = font.char_height;
        	local_y = (self_y / font_char_h).to_int();
        }
        return (local_x, local_y)
	}

	fn calc_offset(&self) -> (Pixel, Pixel) {
		let font_size: Pixel;
		let mut parent_offset_x: Pixel = Pixel(0);
		let mut parent_offset_y: Pixel = Pixel(0);
		
		match (self.get_parent()) {
			Some(parent) 	=> {
				let (x, y) = parent.calc_offset();
				parent_offset_x = x;
				parent_offset_y = y;
				font_size = parent.font_size();
			}
			None => {
				font_size = self.font_size();
			}
		}
		let font = Font{char_width: font_size, char_height: font_size};
		let self_offset_x = font.char_width.pixelify(self.rect.x);
		let self_offset_y = font.char_height.pixelify(self.rect.y);
        return (parent_offset_x + self_offset_x, parent_offset_y + self_offset_y);
	}

	fn set_focused(&mut self, focused: bool) {
		self.focused = focused;
	}

	fn bring_to_front(&mut self) {
	}

	fn set_focused_view<'s>(maybe_view: Option<&'s mut View>) {
		//if focused_view.is_some() {
			//focused_view.unwrap().set_focused(false);
			//focused_view = None;
		//}
		unsafe {
		//	focused_view = match maybe_view {
		//		Some(view) => Some(to_mut_unsafe_ptr(view)),
		//		None => None,
		//	}
		}
		//focused_view = maybe_view;
		if maybe_view.is_some() {
			let mut view = maybe_view.unwrap();
			view.set_focused(true);
			let parent = view.get_parent();
			view.bring_to_front();
		}
	}
	
}

#[test]
fn test_set_focused() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();
	v0.add_view(v1, 0, 0);
	v1.add_view(v2, 0, 0);

	v2.set_focused(true);
	assert_eq!(true, v2.focused);
	assert_eq!(true, v1.focused);
	assert_eq!(true, v0.focused);
}

#[test]
fn test_set_focused_generate_event() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();
	v0.add_view(v1, 0, 0);
	v1.add_view(v2, 0, 0);

	v2.set_focused(true);
	// TODO: csinalni egy TestWidgetet, aminek meghivodik az eventhandlere!
}

#[test]
fn test_bring_to_front() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();
	v0.add_view(v1, 0, 0);
	v1.add_view(v2, 0, 0);
	// TODO: childs
}

#[test]
fn test_set_focused_view_unfocuses_old_view() {
	let mut v0 = View::new();
	let mut v1 = View::new();

	View::set_focused_view(Some(v1));
	assert!(v1.focused);
	assert!(v0.focused == false);

	View::set_focused_view(Some(v0));
	assert!(v1.focused == false);
	assert!(v0.focused);

	View::set_focused_view(None);
	assert!(v1.focused == false);
	assert!(v0.focused == false);
}

#[test]
fn test_set_focused_view() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();

	v0.add_view(v1, 0, 0);
	v1.add_view(v2, 0, 0);
	//assert!(focused_view.is_none());
	View::set_focused_view(Some(v2));
	assert_eq!(true, v2.focused);
	assert_eq!(true, v1.focused);
	assert_eq!(true, v0.focused);
	v2.rect.x = 2;
	//assert_eq!(2, focused_view.unwrap().rect.x);
}

#[test]
fn test_add_view() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();

	assert_eq!(false, v1.dirty);
	assert_eq!(false, v0.has_dirty_child);
	v0.add_view(v1, 20, 22);
	assert_eq!(true, v1.dirty);
	assert_eq!(true, v0.has_dirty_child);

	assert_eq!(false, v2.dirty);
	assert_eq!(false, v1.has_dirty_child);
	v1.add_view(v2, 30, 33);
	assert_eq!(true, v2.dirty);
	assert_eq!(true, v1.has_dirty_child);
}

#[test]
fn test_calc_offset() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();

	v0.add_view(v1, 20, 22);
	v1.add_view(v2, 30, 33);

	v0.rect.x = 10;
	v0.rect.y = 11;
	v0.font_size = Some(Pixel(1));
	v1.font_size = Some(Pixel(2));

	let (x, y) = v2.calc_offset();
	assert_eq!(Pixel(((2*30 + 20*1 + 10))), x);
	assert_eq!(Pixel(((2*33 + 22*1 + 11))), y);
}

#[test]
fn test_make_local() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();

	v0.add_view(v1, 20, 22);
	v1.add_view(v2, 30, 33);

	v0.rect.x = 10;
	v0.rect.y = 11;
	v0.font_size = Some(Pixel(1));
	v1.rect.x = 20;
	v1.rect.y = 22;
	v1.font_size = Some(Pixel(2));
	v2.rect.x = 30;
	v2.rect.y = 33;

	let (x, y) = v2.make_local(Pixel(100), Pixel(200));
	assert_eq!(((100 - (2*30 + 20*1 + 10)) / 2), x);
	assert_eq!(((200 - (2*33 + 22*1 + 11)) / 2), y);
}

#[test]
fn test_modified() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();

	v0.add_view(v1, 0, 0);
	v1.add_view(v2, 0, 0);

	v0.dirty = false;
	v1.dirty = false;
	v2.dirty = false;
	v0.has_dirty_child = false;
	v1.has_dirty_child = false;
	v2.has_dirty_child = false;
	v2.modified();
	assert!(v2.dirty);
	assert!(v1.dirty == false);
	assert!(v0.dirty == false);

	assert!(v2.has_dirty_child == false);
	assert!(v1.has_dirty_child);
	assert!(v0.has_dirty_child);
}

#[test]
fn test_defaults() {
	let mut v = View::new();
	assert_eq!(None, v.owner);
	assert_eq!(None, v.font_size);
	assert_eq!(false, v.hidden);
	assert_eq!(false, v.dirty);
	assert_eq!(false, v.executing);
	assert_eq!(false, v.has_dirty_child);
	assert_eq!(false, v.focused);
}

#[test]
fn test_font_size() {
	let mut v0 = View::new();
	let mut v1 = View::new();
	let mut v2 = View::new();

	v0.add_view(v1, 0, 0);
	v1.add_view(v2, 0, 0);

	v0.font_size = Some(Pixel(1));
	v1.font_size = Some(Pixel(2));
	v2.font_size = None;

	assert_eq!(Pixel(1), v0.font_size());
	assert_eq!(Pixel(2), v1.font_size());
	assert_eq!(Pixel(2), v2.font_size());
}

#[test]
fn test_hide() {
	let mut v = View::new();
	assert_eq!(v.dirty, false);
	assert_eq!(v.is_hidden(), false);
	v.hide();
	assert_eq!(v.is_hidden(), true);
	assert_eq!(v.dirty, true);
}

#[test]
fn test_show() {
	let mut v = View::new();
	v.hidden = true;
	assert_eq!(v.dirty, false);
	v.show();
	assert_eq!(v.is_hidden(), false);
	assert_eq!(v.dirty, true);
}

#[test]
fn test_multiple_ptr_to_a_view() {
	let mut v = View::new();
	let mut v1 = View::new();
	
	v.add_view(v1, 0, 0);
	v1.rect.x = 3;
	match(v1.get_parent()) {
		Some(parent) => {
			parent.rect.x = 4;
		}
		None() => {},
	}
	assert_eq!(3, v1.rect.x);
	assert_eq!(4, v.rect.x);
}