// OKLab color space conversion matrices and constants
// Based on Björn Ottosson's OKLab specification (bottosson.github.io/posts/oklab)

// M1: Linear sRGB to LMS
const M1: [[f32; 3]; 3] = [
	[0.4122214708, 0.5363325363, 0.0514459929],
	[0.2119034982, 0.6806995451, 0.1073969566],
	[0.0883024619, 0.2817188376, 0.6299787005],
];

// M1^-1: LMS to Linear sRGB
const M1_INV: [[f32; 3]; 3] = [
	[ 4.0767245293, -3.3077216883,  0.2309759054],
	[-1.2681437731,  2.6093323231, -0.3411344290],
	[-0.0041119885, -0.7034763098,  1.7068625689],
];

// M2: L'M'S' to OKLab
const M2: [[f32; 3]; 3] = [
	[ 0.2104542553,  0.7936177850, -0.0040720468],
	[ 1.9779984951, -2.4285922050,  0.4505937099],
	[ 0.0259040371,  0.7827717662, -0.8086757660],
];

// M2^-1: OKLab to L'M'S'
const M2_INV: [[f32; 3]; 3] = [
	[1.0000000000,  0.3963377774,  0.2158037573],
	[1.0000000000, -0.1055613458, -0.0638541728],
	[1.0000000000, -0.0894841775, -1.2914855480],
];

#[derive(Debug,Copy,Clone)]
pub struct Color {
	rgba: [f32;4],
}

impl From<[u8;4]> for Color {

	fn from(src: [u8;4]) -> Self {
		Color {
			rgba: [
				( src[ 0 ] as f32 ) / 255.0,
				( src[ 1 ] as f32 ) / 255.0,
				( src[ 2 ] as f32 ) / 255.0,
				( src[ 3 ] as f32 ) / 255.0,
			],
		}
	}
}

impl From<&image::Rgba<u8>> for Color {
	fn from(src: &image::Rgba<u8> ) -> Self {
		Color {
			rgba: [
				( src[ 0 ] as f32 ) / 255.0,
				( src[ 1 ] as f32 ) / 255.0,
				( src[ 2 ] as f32 ) / 255.0,
				( src[ 3 ] as f32 ) / 255.0,
			],
		}
	}
}

impl std::ops::Sub for Color {
	type Output = Color;
	fn sub(self, other: Color) -> <Self as std::ops::Sub<Color>>::Output {
		Color{
			rgba: [
				self.rgba[ 0 ] - other.rgba[ 0 ],
				self.rgba[ 1 ] - other.rgba[ 1 ],
				self.rgba[ 2 ] - other.rgba[ 2 ],
				self.rgba[ 3 ] - other.rgba[ 3 ],
			],
		}
	}
}

impl std::ops::Div<u32> for Color {
	type Output = Color;
	fn div(self, other: u32) -> <Self as std::ops::Div<u32>>::Output {
		Color{
			rgba: [
				self.rgba[ 0 ] / ( other as f32 ),
				self.rgba[ 1 ] / ( other as f32 ),
				self.rgba[ 2 ] / ( other as f32 ),
				self.rgba[ 3 ] / ( other as f32 ),
			],
		}
	}
}

impl std::ops::AddAssign for Color {
	fn add_assign(&mut self, other: Color) {
		/*
		self.rgba[ 0 ] = self.rgba[ 0 ].saturating_add(other.rgba[ 0 ] );
		self.rgba[ 1 ] = self.rgba[ 1 ].saturating_add(other.rgba[ 1 ] );
		self.rgba[ 2 ] = self.rgba[ 2 ].saturating_add(other.rgba[ 2 ] );
		self.rgba[ 3 ] = self.rgba[ 3 ].saturating_add(other.rgba[ 3 ] );
		*/
		self.rgba[ 0 ] += other.rgba[ 0 ];
		self.rgba[ 1 ] += other.rgba[ 1 ];
		self.rgba[ 2 ] += other.rgba[ 2 ];
		self.rgba[ 3 ] += other.rgba[ 3 ];
	}
}

impl Color {
	pub fn rgba_u8(&self) -> [u8;4] {
		[
			( self.rgba[ 0 ] * 255.0 ) as u8,
			( self.rgba[ 1 ] * 255.0 ) as u8,
			( self.rgba[ 2 ] * 255.0 ) as u8,
			( self.rgba[ 3 ] * 255.0 ) as u8,
		]
	}

	pub fn rgba(&self) -> [f32;4] {
		self.rgba
	}

	pub fn is_empty( &self ) -> bool {
		self.rgba == [0f32;4]
	}

	// sRGB gamma correction: sRGB to linear RGB
	fn srgb_to_linear(c: f32) -> f32 {
		if c <= 0.04045 {
			c / 12.92
		} else {
			((c + 0.055) / 1.055).powf(2.4)
		}
	}

	// sRGB gamma correction: linear RGB to sRGB
	fn linear_to_srgb(c: f32) -> f32 {
		if c <= 0.0031308 {
			c * 12.92
		} else {
			1.055 * c.powf(1.0 / 2.4) - 0.055
		}
	}

	// Matrix multiplication helper: 3x3 matrix * 3x1 vector
	fn matrix_mul_3x3(matrix: &[[f32; 3]; 3], vec: [f32; 3]) -> [f32; 3] {
		[
			matrix[0][0] * vec[0] + matrix[0][1] * vec[1] + matrix[0][2] * vec[2],
			matrix[1][0] * vec[0] + matrix[1][1] * vec[1] + matrix[1][2] * vec[2],
			matrix[2][0] * vec[0] + matrix[2][1] * vec[1] + matrix[2][2] * vec[2],
		]
	}

	// Convert sRGB (0-1) to OKLab
	pub fn to_oklab(&self) -> [f32; 3] {
		// 1. sRGB to linear RGB
		let r_lin = Self::srgb_to_linear(self.rgba[0]);
		let g_lin = Self::srgb_to_linear(self.rgba[1]);
		let b_lin = Self::srgb_to_linear(self.rgba[2]);

		// 2. Linear RGB to LMS
		let lms = Self::matrix_mul_3x3(&M1, [r_lin, g_lin, b_lin]);

		// 3. Apply cube root to each LMS component
		let l_prime = lms[0].cbrt();
		let m_prime = lms[1].cbrt();
		let s_prime = lms[2].cbrt();

		// 4. L'M'S' to OKLab
		Self::matrix_mul_3x3(&M2, [l_prime, m_prime, s_prime])
	}

	// Create Color from OKLab (L, a, b)
	pub fn from_oklab(lab: [f32; 3]) -> Self {
		// 1. OKLab to L'M'S'
		let lms_prime = Self::matrix_mul_3x3(&M2_INV, lab);

		// 2. Cube each component
		let l = lms_prime[0].powi(3);
		let m = lms_prime[1].powi(3);
		let s = lms_prime[2].powi(3);

		// 3. LMS to linear RGB
		let rgb_lin = Self::matrix_mul_3x3(&M1_INV, [l, m, s]);

		// 4. Linear RGB to sRGB with clamping
		Color {
			rgba: [
				Self::linear_to_srgb(rgb_lin[0].max(0.0).min(1.0)),
				Self::linear_to_srgb(rgb_lin[1].max(0.0).min(1.0)),
				Self::linear_to_srgb(rgb_lin[2].max(0.0).min(1.0)),
				1.0, // full alpha
			],
		}
	}

	// Convert OKLab to OKLCH
	pub fn oklab_to_oklch(lab: [f32; 3]) -> [f32; 3] {
		let l = lab[0];
		let a = lab[1];
		let b = lab[2];
		let c = (a * a + b * b).sqrt();
		let h = b.atan2(a); // radians
		[l, c, h]
	}

	// Convert OKLCH to OKLab
	pub fn oklch_to_oklab(lch: [f32; 3]) -> [f32; 3] {
		let l = lch[0];
		let c = lch[1];
		let h = lch[2]; // radians
		let a = c * h.cos();
		let b = c * h.sin();
		[l, a, b]
	}

	// Convert to OKLCH
	pub fn to_oklch(&self) -> [f32; 3] {
		Self::oklab_to_oklch(self.to_oklab())
	}

	// Create Color from OKLCH
	pub fn from_oklch(lch: [f32; 3]) -> Self {
		Self::from_oklab(Self::oklch_to_oklab(lch))
	}

	pub fn from_string( color_string: &str ) -> Option< Color > {
		match color_string.parse() as Result<css_color::Rgba, css_color::ParseColorError> {
			Err( _e ) => None,
			Ok( css_color ) => {
//				dbg!(&css_color);

				// :TODO: improve
				let r = ( 255.0 * css_color.red as f32 ) as u8;
				let g = ( 255.0 * css_color.green as f32 ) as u8;
				let b = ( 255.0 * css_color.blue as f32 ) as u8;
				let a = ( 255.0 * css_color.alpha as f32 ) as u8;

				let color = [ r, g, b, a ].into();
				Some( color )
			},
		}
	}
}

#[derive(Debug)]
pub struct LowTexPal {
	filename: String,
	was_modified: bool,
	colors: Vec<Color>,
}

impl LowTexPal {

	pub fn new(
		filename: &str,
	) -> Self {
		LowTexPal {
			filename: filename.to_string(),
			was_modified: false,
			colors:Vec::new(),
		}
	}

	pub fn load( &mut self ) {
		// :TODO: load
		let img = match image::open( &self.filename ) {
			Err( _e ) => return,	// :TODO: error handling // Note: This is not even an error, since we might be creating a new image later. Maybe?!
			Ok( img ) => img,
		};
//		self.image = Some( img );
		// :TODO: iterate through pixels to get colors

		let img = img.into_rgba8();

		for rgba in img.pixels() {
//			dbg!(&rgba);
			let color: Color = rgba.into();
			if !color.is_empty() {
				self.colors.push( color );
			}
		};

		self.was_modified = false;
	}

	pub fn save( &mut self ) {
		// :TODO: save
		if self.colors.len() == 0 {
			println!("No colors. Not saving.");
			return
		}

		let size = match self.colors.len() {
			1 => 1,
			2 => 2, //  <= 4 -> 2x2
			3 => 2, //  <= 4 -> 2x2
			4 => 2, //  <= 4 -> 2x2
			n if n <= 16 => 4,
			n if n <= 64 => 8,
			n if n <= 256 => 16,
			// :TODO: could use round_up_to_power_of_2(sqrt(n))
			n => panic!("Not handling size for {} entries", n ),
		};

		dbg!(&size);

		let mut imgbuf = image::ImageBuffer::new( size, size );

		let mut y = 0;
		let mut x = 0;

		for color in &self.colors {

			if x >= size && y >= size {
				panic!("Tried to write to many pixels to image");	// should never trigger				
			}
			let rgba = color.rgba_u8();

			let p = imgbuf.get_pixel_mut( x, y );

			*p = image::Rgba( rgba );

			x += 1;
			if x >= size {
				y += 1;
				x = 0;
			}
		}
		match imgbuf.save( &self.filename ) {
			Err( e ) => {
				println!("Error saving image {}", &e );
				return
			},
			_ => {},
		}
		self.was_modified = false;
	}

	pub fn was_modified( &self ) -> bool {
		self.was_modified
	}

	fn add_color( &mut self, color: &Color ) -> usize {
		self.was_modified = true;
		self.colors.push( *color );
		self.colors.len()
	}

	pub fn add_color_rgb( &mut self, r: u8, g: u8, b: u8 ) -> usize {
		let a = 0xff;

		let color = [ r, g, b, a ].into();
		self.add_color( &color )
	}

	pub fn add_color_string( &mut self, color_string: &str ) -> Option< usize > {
		match Color::from_string( &color_string ) {
			None => {
						dbg!("No Match");
						None
			},
			Some( color ) => {
				return Some( self.add_color( &color ) )				
			}
		}
	}

	pub fn add_gradient_strings( &mut self, start_color_string: &str, end_color_string: &str, steps: u32 ) -> Option< Vec< usize > > {
		// Call new method with RGB colorspace for backward compatibility
		self.add_gradient_colorspace(start_color_string, end_color_string, steps, "rgb")
	}

	pub fn add_gradient_colorspace( &mut self, start_color_string: &str, end_color_string: &str, steps: u32, colorspace: &str ) -> Option< Vec< usize > > {
		match ( Color::from_string( &start_color_string ), Color::from_string( &end_color_string ) ) {
			( Some( start_color) , Some( end_color ) ) => {
				let mut indices = Vec::new();

				match colorspace.to_lowercase().as_str() {
					"oklab" => {
						// Interpolate in OKLab space
						let start_lab = start_color.to_oklab();
						let end_lab = end_color.to_oklab();

						for i in 0..steps {
							let t = i as f32 / (steps - 1) as f32;
							let interpolated_lab = [
								start_lab[0] + t * (end_lab[0] - start_lab[0]),
								start_lab[1] + t * (end_lab[1] - start_lab[1]),
								start_lab[2] + t * (end_lab[2] - start_lab[2]),
							];
							let color = Color::from_oklab(interpolated_lab);
							indices.push( self.add_color( &color ) );
						}
					},
					"oklch" => {
						// Interpolate in OKLCH space
						let start_lch = start_color.to_oklch();
						let end_lch = end_color.to_oklch();

						// Handle hue interpolation (shortest path around color wheel)
						let mut hue_diff = end_lch[2] - start_lch[2];
						if hue_diff > std::f32::consts::PI {
							hue_diff -= 2.0 * std::f32::consts::PI;
						} else if hue_diff < -std::f32::consts::PI {
							hue_diff += 2.0 * std::f32::consts::PI;
						}

						for i in 0..steps {
							let t = i as f32 / (steps - 1) as f32;
							let interpolated_lch = [
								start_lch[0] + t * (end_lch[0] - start_lch[0]),
								start_lch[1] + t * (end_lch[1] - start_lch[1]),
								start_lch[2] + t * hue_diff,
							];
							let color = Color::from_oklch(interpolated_lch);
							indices.push( self.add_color( &color ) );
						}
					},
					_ => {
						// Default: RGB interpolation (existing behavior)
						let delta = ( end_color - start_color ) / ( steps - 1 );
						let mut color = start_color;
						for _s in 0..steps {
							indices.push( self.add_color( &color ) );
							color += delta;
						}
					}
				}

				Some( indices )
			},
			_ => None,
		}
	}

}

