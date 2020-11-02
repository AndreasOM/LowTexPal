
#[derive(Debug,Copy,Clone)]
struct Color {
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

		let img = img.into_rgba();

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
		match ( Color::from_string( &start_color_string ), Color::from_string( &end_color_string ) ) {
			( Some( start_color) , Some( end_color ) ) => {
				let delta = ( end_color - start_color ) / ( steps - 1 );

				let mut color = start_color;
				let mut indices = Vec::new();
				for _s in 0..steps {
					dbg!(color);
					indices.push( self.add_color( &color ) );
					color += delta;
				}
//				dbg!(color);
				Some( indices )
			},
			_ => None,
		}
	}

}

