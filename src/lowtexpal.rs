
// use image::DynamicImage;
use regex::Regex;

#[derive(Debug,Copy,Clone)]
struct Color {
	rgba: [u8;4],
}

impl From<[u8;4]> for Color {

	fn from(src: [u8;4]) -> Self {
		Color {
			rgba: src,
		}
	}
}

impl From<&image::Rgba<u8>> for Color {
	fn from(src: &image::Rgba<u8> ) -> Self {
		Color {
			rgba: [src[0],src[1],src[2],src[3]],
		}
	}
}

impl Color {
	pub fn rgba(&self) -> [u8;4] {
		self.rgba
	}

	pub fn is_empty( &self ) -> bool {
		self.rgba == [0u8;4]
	}

	pub fn from_string( color_string: &str ) -> Option< Color > {
		//css_color::Rgba
		// :TODO: add error handling
		let hashcolor_re = Regex::new( r"^#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})$" ).expect("");
		let hashcolor_caps = hashcolor_re.captures( color_string );
//		dbg!( &hashcolor_caps );

		match hashcolor_caps {
			Some( hashcolor_caps ) => {
				let r = hex::decode( &hashcolor_caps[ 1 ] ).unwrap_or([0u8;1].to_vec())[0];	// we know if is 00-ff, so the _or( ... ) should never trigger
				let g = hex::decode( &hashcolor_caps[ 2 ] ).unwrap_or([0u8;1].to_vec())[0];
				let b = hex::decode( &hashcolor_caps[ 3 ] ).unwrap_or([0u8;1].to_vec())[0];
//				dbg!("Match", &r, &g, &b);

				let a = 0xff;

				let color = [ r, g, b, a ].into();
				return Some( color )
			},
			None => {},
		}
		// :TODO: add parsers for other formats

		None		
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
			let rgba = color.rgba();

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

}

