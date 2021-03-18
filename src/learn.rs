use std::fs;

fn parse_csv(csv_file: &str) -> Vec<[f64; 2]> {
	fs::read_to_string(csv_file)
		.expect("Error while trying to read from file")
		.trim()
		.split('\n')
		.collect::<Vec<&str>>()
		.get(1..)
		.expect("Couldn't retrieve datas from vector")
		.iter()
		.map(|truc| truc.split(',').collect::<Vec<&str>>())
		.collect::<Vec<Vec<&str>>>()
		.iter()
		.map(|pair| {
			[
				pair[0].parse::<f64>().expect("Couldn't parse value"),
				pair[1].parse::<f64>().expect("Couldn't parse value"),
			]
		})
		.collect::<Vec<[f64; 2]>>()
}

fn find_min_max(data: &Vec<[f64; 2]>) -> [f64; 2] {
	let mut min_max = [std::f64::MAX, std::f64::MIN];

	for pair in data {
		if min_max[0] > pair[0] {
			min_max[0] = pair[0];
		}
		if min_max[1] < pair[0] {
			min_max[1] = pair[0];
		}
	}
	min_max
}

fn normalize_input(data: &Vec<[f64; 2]>, min_max: &[f64; 2]) -> Vec<[f64; 2]> {
	data.iter()
		.map(|pair| [(pair[0] - min_max[0]) / (min_max[1] - min_max[0]), pair[1]])
		.collect::<Vec<[f64; 2]>>()
}

fn cost_function(data: &Vec<[f64; 2]>, theta: &[f64; 2]) -> f64 {
	data.iter().fold(0.0, |error, &pair| {
		error + (pair[1] - (theta[0] + theta[1] * pair[0]))
	}) / data.len() as f64
}

fn gradient_step(data: &Vec<[f64; 2]>, theta: &[f64; 2]) -> [f64; 2] {
	let length = data.len() as f64;
	let new_theta = data.iter().fold([0.0, 0.0], |new_theta, &pair| {
		[
			new_theta[0] + ((theta[0] + theta[1] * pair[0]) - pair[1]),
			new_theta[1] + ((theta[0] + theta[1] * pair[0]) - pair[1]) * pair[0],
		]
	});
	[
		theta[0] - (0.01 * new_theta[0] / length),
		theta[1] - (0.01 * new_theta[1] / length),
	]
}

fn learn(data: &Vec<[f64; 2]>, min_max: &[f64; 2]) -> [f64; 2] {
	let mut theta = [0.0, 0.0];
	let mut error = std::f64::MAX;

	loop {
		if error < cost_function(&data, &gradient_step(&data, &theta)) {
			break;
		} else {
			error = cost_function(&data, &theta);
		}
		theta = gradient_step(&data, &theta);
	}
	theta[1] /= min_max[1] - min_max[0];
	theta
}

fn main() {
	let data = parse_csv("ressources/data.csv");
	let min_max = find_min_max(&data);
	let theta = learn(&normalize_input(&data, &min_max), &min_max);

	fs::write(
		"ressources/theta_result.csv",
		[theta[0].to_string(), theta[1].to_string()].join(","),
	)
	.expect("Error while trying to write to file");
}
