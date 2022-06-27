pub const INSTRUCTIONS: &str = "WELCOME TO MY CALCULUS APP";

const LINKS: [[&str; 3]; 8] = [
	["/math", "back to", " math APIs page"],
	["", "back to", " calculus page"],
	["/differentiation", "differentiation", ""],
	["/integration", "integration", ""],
	["/root-finding", "root-finding", ""],
	["/max-finding", "max-finding", ""],
	["/ode", "1st order", " differential equations"],
	["/ode2", "2nd order", " differential equations"],
];

pub fn general_page() -> String {format!("<p align=center>{}</p><p align=center>{}</p>", INSTRUCTIONS, links(1))}

pub fn links(n: i32) -> String {
	let mut links = "".to_string();
	for j in 0..8 {
		let i = j as usize;
		if i != n as usize {
			links = format!("{}
				<a href='https://basic-calculus.herokuapp.com{}'>{}</a>{}<br>",
				links, LINKS[i][0], LINKS[i][1], LINKS[i][2]
			);
		}
	}
	links
}
