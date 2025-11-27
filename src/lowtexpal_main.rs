use clap::{Parser, Subcommand};

use lowtexpal::LowTexPal;

#[derive(Parser)]
#[command(name = "lowtexpal")]
#[command(version = "0.1")]
#[command(author = "Andreas N. <andreas@omni-mad.com")]
#[command(about = "Manipulates low poly palette textures")]
struct Cli {
	/// Set the name of the file to be manipulated
	#[arg(short = 'f', long, value_name = "FILE")]
	file: Option<String>,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Add a single color to the palette
	AddColor {
		/// Set the color to be added
		#[arg(short = 'c', long, value_name = "COLOR")]
		color: Option<String>,

		/// Force the color to be added even if it already exists
		#[arg(long)]
		force: bool,
	},

	/// Add a color gradient to the palette
	AddGradient {
		/// Set the start color of the gradient to be added
		#[arg(long, value_name = "START COLOR")]
		start_color: Option<String>,

		/// Set the end color of the gradient to be added
		#[arg(long, value_name = "END COLOR")]
		end_color: Option<String>,

		/// Set the number of steps
		#[arg(long, value_name = "STEPS")]
		steps: Option<u32>,

		/// Colorspace for gradient interpolation (rgb, oklab, oklch)
		#[arg(long, value_name = "COLORSPACE", default_value = "rgb")]
		colorspace: String,

		/// Force the colors to be added even if they already exist
		#[arg(long)]
		force: bool,
	},
}

fn main() {
	let cli = Cli::parse();

	let file = cli.file.unwrap_or_default();

//	dbg!(&file);

	let mut lowtexpal = LowTexPal::new( &file );

//	dbg!(&lowtexpal);

	lowtexpal.load(); // :TODO: error handling

//	dbg!(&lowtexpal);

	// :TODO: handle sub commmands

	match &cli.command {
		Some(Commands::AddColor { color, force: _ }) => {
			if let Some(color) = color {
				dbg!(&color);
				if !color.is_empty() {
					match lowtexpal.add_color_string( &color ) {
						Some( i ) => println!("Added {} at {}", &color, i ),
						None => println!("Couldn't add {}", &color ),
					}
				}
			}
		}
		Some(Commands::AddGradient { start_color, end_color, steps, colorspace, force: _ }) => {
			if let (Some(start_color), Some(end_color), Some(steps)) = (start_color, end_color, steps) {
				dbg!(&start_color, &end_color, &steps, &colorspace);
				if !start_color.is_empty() && !end_color.is_empty() && *steps != 0 {
					match lowtexpal.add_gradient_colorspace( &start_color, &end_color, *steps, colorspace ) {
						Some( i ) => println!("Added {} - {} ({}) at {:#?}", &start_color, &end_color, colorspace, &i ),
						None => println!("Couldn't add {} - {} with {} steps", &start_color, &end_color, steps ),
					}
				}
			}
		}
		None => {
			// No subcommand provided
		}
	}

//	dbg!(&lowtexpal);

	if lowtexpal.was_modified() {
		lowtexpal.save();
	}
}
