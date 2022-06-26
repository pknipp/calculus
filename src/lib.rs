use std::f64::consts::PI;
use rocket::http::RawStr;
use serde::{Serialize, Deserialize};

pub const INSTRUCTIONS: &str = "WELCOME TO MY CALCULUS APP";

struct Link<'a> {
	url: &'a str,
	inner: &'a str,
	outer: &'a str,
}

const LINKS: [Link; 8] = [
	Link{
		url: "https://pknipp.github.io/math",
		inner: "back to",
		outer: " math APIs page",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com",
		inner: "back to",
		outer: " calculus page",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/differentiation",
		inner: "differentiation",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/integration",
		inner: "integration",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/root-finding",
		inner: "root-finding",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/max-finding",
		inner: "max-finding",
		outer: "",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/ode",
		inner: "1st order",
		outer: " differential equations",
	},
	Link{
		url: "https://basic-calculus.herokuapp.com/ode2",
		inner: "2nd order",
		outer: " differential equations",
	},
];

pub fn general_page() -> String {format!("<p align=center>{}</p><p align=center>{}</p>", INSTRUCTIONS, links(1))}

pub fn links(n: i32) -> String {
	let mut links = "".to_string();
	for i in 0..8 {
		if i != n {
			links = format!("{}<a href='{}'>{}</a>{}<br>", links,
			  	LINKS[i as usize].url,
			  	LINKS[i as usize].inner,
			  	LINKS[i as usize].outer,
			);
		}
	}
	links
}
