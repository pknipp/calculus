pub const INSTRUCTIONS: &str = "WELCOME TO MY CALCULUS APP";

struct Link<'a> {
	url: &'a str,
	inner: &'a str,
	outer: &'a str,
}

const LINKS: [Link; 8] = [
	Link{
		url: "/math",
		inner: "back to",
		outer: " math APIs page",
	},
	Link{
		url: "",
		inner: "back to",
		outer: " calculus page",
	},
	Link{
		url: "/differentiation",
		inner: "differentiation",
		outer: "",
	},
	Link{
		url: "/integration",
		inner: "integration",
		outer: "",
	},
	Link{
		url: "/root-finding",
		inner: "root-finding",
		outer: "",
	},
	Link{
		url: "/max-finding",
		inner: "max-finding",
		outer: "",
	},
	Link{
		url: "/ode",
		inner: "1st order",
		outer: " differential equations",
	},
	Link{
		url: "/ode2",
		inner: "2nd order",
		outer: " differential equations",
	},
];

pub fn general_page() -> String {format!("<p align=center>{}</p><p align=center>{}</p>", INSTRUCTIONS, links(1))}

pub fn links(n: i32) -> String {
	let mut links = "".to_string();
	for i in 0..8 {
		if i != n {
			links = format!("{}
				<a href='https://basic-calculus.herokuapp.com{}'>{}</a>{}<br>",
				links,
			  	LINKS[i as usize].url,
			  	LINKS[i as usize].inner,
			  	LINKS[i as usize].outer,
			);
		}
	}
	links
}
