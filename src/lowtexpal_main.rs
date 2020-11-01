use clap::{App, Arg, SubCommand};

use lowtexpal::LowTexPal;

fn main() {

	let matches = App::new("lowtexpal")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com")
					.about("Manipulates low poly palette textures")
					.arg(
						Arg::with_name("file")
							.long("file")
							.short("f")
							.value_name("FILE")
							.help("Set the name of the file to be manipulated.")
							.takes_value(true)
					)
					.subcommand(
						SubCommand::with_name("add-color")
						.arg(
							Arg::with_name("color")
								.long("color")
								.short("c")
								.value_name("COLOR")
								.help("Set the color to be added.")
								.takes_value(true)
						)
						.arg(
							Arg::with_name("force")
								.long("force")
								.help("Force the color to be added even if it already exists.")
						)
					)
					.get_matches();

//	dbg!(&matches);

	let file = matches.value_of("file").unwrap_or("").to_string();

//	dbg!(&file);

	let mut lowtexpal = LowTexPal::new( &file );

//	dbg!(&lowtexpal);

	lowtexpal.load(); // :TODO: error handling

//	dbg!(&lowtexpal);

	// :TODO: handle sub commmands

	if let( "add-color" , Some( sub_matches ) ) = matches.subcommand() {
		let color = sub_matches.value_of("color").unwrap_or("").to_string();
		dbg!(&color);
		if color != "" {
			match lowtexpal.add_color_string( &color ) {
				Some( i ) => println!("Added {} at {}", &color, i ),
				None => println!("Couldn't add {}", &color ),
			}
		}
	} else {

	}

//	dbg!(&lowtexpal);

	if lowtexpal.was_modified() {
		lowtexpal.save();
	}
}
