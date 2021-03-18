use std::fs;

fn main() {
	let data = fs::read_to_string("ressources/theta_result.csv")
		.expect("Error while trying to read from file");
	let data = data.trim().split(',').collect::<Vec<&str>>();
	let theta = [
		data[0].parse::<f64>().expect("Couldn't parse value"),
		data[1].parse::<f64>().expect("Couldn't parse value"),
	];
	let mut string = String::new();
	let mut input: f64;
	println!("Veuillez entrer un kilométrage");
	loop {
		std::io::stdin()
			.read_line(&mut string)
			.expect("Erreur pendant la lecture de l'entrée standard");
		input = match string.trim().parse::<f64>() {
			Ok(input) => {
				if input >= 0.0 {
					input
				} else {
					-1.0
				}
			}
			Err(_) => -1.0,
		};
		if input != -1.0 {
			break;
		} else {
			println!("Veuillez entrer un nombre positif");
			string.clear();
		}
	}
	println!(
		"L'estimation du prix d'après votre kilométrage est de {:.2} euros",
		theta[0] + theta[1] * input
	);
}
