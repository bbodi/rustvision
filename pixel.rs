#[deriving(Eq)]
pub struct Pixel(int);

impl Pixel {
	pub fn pixelify(&self, num: int) -> Pixel {
		let Pixel(n) = *self;
		return Pixel(n * num);
	}

	pub fn to_int(&self) -> int {
		let Pixel(n) = *self;
		return n;	
	}
}

#[test]
fn test_pixel_pixelify() {
	assert_eq!(Pixel(32), Pixel(8).pixelify(4));
}

impl Ord for Pixel {
	fn lt(&self, other: &Pixel) -> bool {
		let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return self_num < other_num;
	}

	fn le(&self, other: &Pixel) -> bool {
		let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return self_num <= other_num;
	}

	fn gt(&self, other: &Pixel) -> bool {
		let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return self_num > other_num;
	}

	fn ge(&self, other: &Pixel) -> bool {
		let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return self_num >= other_num;
	}
}

impl Add<Pixel, Pixel> for Pixel {
    fn add(&self, other: &Pixel) -> Pixel {
    	let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return Pixel(self_num + other_num);
    }
}

impl Sub<Pixel, Pixel> for Pixel {
    fn sub(&self, other: &Pixel) -> Pixel {
        let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return Pixel(self_num - other_num);
    }
}

impl Div<Pixel, Pixel> for Pixel {
    fn div(&self, other: &Pixel) -> Pixel {
        let Pixel(self_num) = *self;
    	let Pixel(other_num) = *other;
        return Pixel(self_num / other_num);
    }
}