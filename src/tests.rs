use super::lowtexpal::{Color, LowTexPal};

// Helper function for float comparison with tolerance
fn assert_f32_near(a: f32, b: f32, tolerance: f32) {
	assert!(
		(a - b).abs() < tolerance,
		"Expected {} to be close to {} (tolerance: {}), diff: {}",
		a, b, tolerance, (a - b).abs()
	);
}

// ===== Color Conversion Tests =====

#[test]
fn test_color_from_u8_black() {
	let color = Color::from([0u8, 0, 0, 0]);
	let rgba = color.rgba();
	assert_eq!(rgba, [0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_color_from_u8_white() {
	let color = Color::from([255u8, 255, 255, 255]);
	let rgba = color.rgba();
	assert_f32_near(rgba[0], 1.0, 0.001);
	assert_f32_near(rgba[1], 1.0, 0.001);
	assert_f32_near(rgba[2], 1.0, 0.001);
	assert_f32_near(rgba[3], 1.0, 0.001);
}

#[test]
fn test_color_from_u8_mid_values() {
	let color = Color::from([128u8, 64, 192, 255]);
	let rgba = color.rgba();
	assert_f32_near(rgba[0], 128.0 / 255.0, 0.001);
	assert_f32_near(rgba[1], 64.0 / 255.0, 0.001);
	assert_f32_near(rgba[2], 192.0 / 255.0, 0.001);
	assert_f32_near(rgba[3], 1.0, 0.001);
}

#[test]
fn test_color_to_u8_black() {
	let color = Color::from([0u8, 0, 0, 0]);
	assert_eq!(color.rgba_u8(), [0, 0, 0, 0]);
}

#[test]
fn test_color_to_u8_white() {
	let color = Color::from([255u8, 255, 255, 255]);
	assert_eq!(color.rgba_u8(), [255, 255, 255, 255]);
}

#[test]
fn test_color_round_trip_black() {
	let original = [0u8, 0, 0, 0];
	let color = Color::from(original);
	let result = color.rgba_u8();
	assert_eq!(result, original);
}

#[test]
fn test_color_round_trip_white() {
	let original = [255u8, 255, 255, 255];
	let color = Color::from(original);
	let result = color.rgba_u8();
	assert_eq!(result, original);
}

#[test]
fn test_color_round_trip_various() {
	// Test various values - allow ±1 tolerance due to rounding
	let test_cases = vec![
		[128u8, 64, 192, 255],
		[255, 0, 0, 255],
		[0, 255, 0, 255],
		[0, 0, 255, 255],
	];

	for original in test_cases {
		let color = Color::from(original);
		let result = color.rgba_u8();
		for i in 0..4 {
			let diff = (result[i] as i16 - original[i] as i16).abs();
			assert!(diff <= 1, "Round-trip error too large for {:?}: got {:?}", original, result);
		}
	}
}

// ===== CSS Color Parsing Tests =====

#[test]
fn test_parse_hex_red() {
	let color = Color::from_string("#ff0000").unwrap();
	assert_eq!(color.rgba_u8(), [255, 0, 0, 255]);
}

#[test]
fn test_parse_hex_green() {
	let color = Color::from_string("#00ff00").unwrap();
	assert_eq!(color.rgba_u8(), [0, 255, 0, 255]);
}

#[test]
fn test_parse_hex_blue() {
	let color = Color::from_string("#0000ff").unwrap();
	assert_eq!(color.rgba_u8(), [0, 0, 255, 255]);
}

#[test]
fn test_parse_named_black() {
	let color = Color::from_string("black").unwrap();
	assert_eq!(color.rgba_u8(), [0, 0, 0, 255]);
}

#[test]
fn test_parse_named_white() {
	let color = Color::from_string("white").unwrap();
	assert_eq!(color.rgba_u8(), [255, 255, 255, 255]);
}

#[test]
fn test_parse_named_red() {
	let color = Color::from_string("red").unwrap();
	assert_eq!(color.rgba_u8(), [255, 0, 0, 255]);
}

#[test]
fn test_parse_named_lime() {
	let color = Color::from_string("lime").unwrap();
	assert_eq!(color.rgba_u8(), [0, 255, 0, 255]);
}

#[test]
fn test_parse_named_blue() {
	let color = Color::from_string("blue").unwrap();
	assert_eq!(color.rgba_u8(), [0, 0, 255, 255]);
}

#[test]
fn test_parse_invalid_returns_none() {
	assert!(Color::from_string("invalid").is_none());
	assert!(Color::from_string("").is_none());
	assert!(Color::from_string("#gg0000").is_none());
}

// ===== LowTexPal Tests =====

#[test]
fn test_lowtexpal_new() {
	let ltp = LowTexPal::new("test.png");
	assert!(!ltp.was_modified());
}

#[test]
fn test_add_color_rgb() {
	let mut ltp = LowTexPal::new("test.png");
	assert!(!ltp.was_modified());

	ltp.add_color_rgb(255, 0, 0);
	assert!(ltp.was_modified());
}

#[test]
fn test_add_color_string_hex() {
	let mut ltp = LowTexPal::new("test.png");
	let result = ltp.add_color_string("#ff0000");
	assert!(result.is_some());
	assert_eq!(result.unwrap(), 1);
}

#[test]
fn test_add_color_string_named() {
	let mut ltp = LowTexPal::new("test.png");
	let result = ltp.add_color_string("lime");
	assert!(result.is_some());
}

#[test]
fn test_add_color_string_invalid() {
	let mut ltp = LowTexPal::new("test.png");
	let result = ltp.add_color_string("invalid");
	assert!(result.is_none());
}

#[test]
fn test_add_gradient_strings() {
	let mut ltp = LowTexPal::new("test.png");
	let result = ltp.add_gradient_strings("black", "white", 4);
	assert!(result.is_some());

	let indices = result.unwrap();
	assert_eq!(indices.len(), 4);
}

#[test]
fn test_modification_tracking() {
	let mut ltp = LowTexPal::new("test.png");
	assert!(!ltp.was_modified());

	ltp.add_color_rgb(255, 0, 0);
	assert!(ltp.was_modified());
}

// ===== OKLab Color Space Tests =====

#[test]
fn test_oklab_round_trip_black() {
	let black = Color::from([0u8, 0, 0, 255]);
	let lab = black.to_oklab();
	let back = Color::from_oklab(lab);

	// Black should convert to/from OKLab cleanly
	let rgb = back.rgba_u8();
	assert!(rgb[0] <= 1); // Allow tiny rounding error
	assert!(rgb[1] <= 1);
	assert!(rgb[2] <= 1);
}

#[test]
fn test_oklab_round_trip_white() {
	let white = Color::from([255u8, 255, 255, 255]);
	let lab = white.to_oklab();
	let back = Color::from_oklab(lab);

	// White should convert to/from OKLab cleanly
	let rgb = back.rgba_u8();
	assert!(rgb[0] >= 254); // Allow tiny rounding error
	assert!(rgb[1] >= 254);
	assert!(rgb[2] >= 254);
}

#[test]
fn test_oklab_round_trip_red() {
	let red = Color::from([255u8, 0, 0, 255]);
	let lab = red.to_oklab();
	let back = Color::from_oklab(lab);

	let rgb = back.rgba_u8();
	assert!(rgb[0] >= 254);
	assert!(rgb[1] <= 1);
	assert!(rgb[2] <= 1);
}

#[test]
fn test_oklch_round_trip() {
	let red = Color::from([255u8, 0, 0, 255]);
	let lch = red.to_oklch();
	let back = Color::from_oklch(lch);

	let rgb = back.rgba_u8();
	assert!(rgb[0] >= 254);
	assert!(rgb[1] <= 1);
	assert!(rgb[2] <= 1);
}

#[test]
fn test_oklab_gradient() {
	let mut ltp = LowTexPal::new("test.png");
	let result = ltp.add_gradient_colorspace("red", "lime", 8, "oklab");
	assert!(result.is_some());

	let indices = result.unwrap();
	assert_eq!(indices.len(), 8);
}

#[test]
fn test_oklch_gradient() {
	let mut ltp = LowTexPal::new("test.png");
	let result = ltp.add_gradient_colorspace("red", "blue", 8, "oklch");
	assert!(result.is_some());

	let indices = result.unwrap();
	assert_eq!(indices.len(), 8);
}

#[test]
fn test_rgb_gradient_backward_compat() {
	let mut ltp = LowTexPal::new("test.png");
	// Old method should still work
	let result1 = ltp.add_gradient_strings("black", "white", 4);
	assert!(result1.is_some());

	// New method with RGB should give same result
	let mut ltp2 = LowTexPal::new("test2.png");
	let result2 = ltp2.add_gradient_colorspace("black", "white", 4, "rgb");
	assert!(result2.is_some());
}
