use clap::{Command, Arg};

use lowtexpal::LowTexPal;

fn main() {

	let matches = Command::new("lowtexpal")
					.version("0.1")
					.author("Andreas N. <andreas@omni-mad.com")
					.about("Manipulates low poly palette textures")
					.arg(
						Arg::new("file")
							.long("file")
							.short('f')
							.value_name("FILE")
							.help("Set the name of the file to be manipulated.")
					)
					.subcommand(
						Command::new("add-color")
						.arg(
							Arg::new("color")
								.long("color")
								.short('c')
								.value_name("COLOR")
								.help("Set the color to be added.")
						)
						.arg(
							Arg::new("force")
								.long("force")
								.help("Force the color to be added even if it already exists.")
								.action(clap::ArgAction::SetTrue)
						)
					)
					.subcommand(
						Command::new("add-gradient")
						.arg(
							Arg::new("start-color")
								.long("start-color")
								.value_name("START COLOR")
								.help("Set the start color of the gradient to be added.")
						)
						.arg(
							Arg::new("end-color")
								.long("end-color")
								.value_name("END COLOR")
								.help("Set the end color of the gradient to be added.")
						)
						.arg(
							Arg::new("steps")
								.long("steps")
								.value_name("STEPS")
								.help("Set the number of steps.")
						)
						.arg(
							Arg::new("force")
								.long("force")
								.help("Force the colors to be added even if they already exist.")
								.action(clap::ArgAction::SetTrue)
						)
					)
					.get_matches();

//	dbg!(&matches);

	let file = matches.get_one::<String>("file").map(|s| s.as_str()).unwrap_or("").to_string();

//	dbg!(&file);

	let mut lowtexpal = LowTexPal::new( &file );

//	dbg!(&lowtexpal);

	lowtexpal.load(); // :TODO: error handling

//	dbg!(&lowtexpal);

	// :TODO: handle sub commmands

	if let Some(("add-color", sub_matches)) = matches.subcommand() {
		let color = sub_matches.get_one::<String>("color").map(|s| s.as_str()).unwrap_or("").to_string();
		dbg!(&color);
		if color != "" {
			match lowtexpal.add_color_string( &color ) {
				Some( i ) => println!("Added {} at {}", &color, i ),
				None => println!("Couldn't add {}", &color ),
			}
		}
	} else if let Some(("add-gradient", sub_matches)) = matches.subcommand() {
		let start_color = sub_matches.get_one::<String>("start-color").map(|s| s.as_str()).unwrap_or("").to_string();
		let end_color = sub_matches.get_one::<String>("end-color").map(|s| s.as_str()).unwrap_or("").to_string();
		let steps = sub_matches.get_one::<String>("steps").map(|s| s.as_str()).unwrap_or("").parse::<u32>().unwrap_or( 0 );
		dbg!(&start_color, &end_color, &steps);
		if start_color != "" && end_color != "" && steps != 0 {
			match lowtexpal.add_gradient_strings( &start_color, &end_color, steps ) {
				Some( i ) => println!("Added {} - {} at {:#?}", &start_color, &end_color, &i ),
				None => println!("Couldn't add {} - {} with {} steps", &start_color, &end_color, steps ),
			}
		}
	} else {

	}

//	dbg!(&lowtexpal);

	if lowtexpal.was_modified() {
		lowtexpal.save();
	}
}
