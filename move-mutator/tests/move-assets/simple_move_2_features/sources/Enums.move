module TestAccount::Enums {
	use std::string::{String, utf8};

    enum Shape has drop {
        Rectangle { width: u64, height: u64 }
    }

    fun rectangle_area(shape: Shape): u64 {
		assert!(shape is Shape::Rectangle);
		shape.width*shape.height
    }

	enum Colour { Red, Green, Blue }

	fun colour_enum_to_string(colour: Colour): String {
		match (colour) {
			Red => utf8(b"red"),
			Green => utf8(b"green"),
			Blue => utf8(b"blue"),
		}
	}

    #[test]
    fun area_of_rectangle() {
        assert!(rectangle_area(Shape::Rectangle{width: 5, height: 5}) == 25);
    }

    #[test]
    fun print_color_from_enum() {
        assert!(colour_enum_to_string(Colour::Red) == utf8(b"red"));
    }
}
