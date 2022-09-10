use {
	reqwest::{ClientBuilder, Result},
	std::time::Duration,
};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
	let user = "jlucktay";
	let request_url = format!("https://api.github.com/users/{}", user);
	println!("{}", request_url);

	let timeout = Duration::new(5, 0);
	let user_agent = format!("watchd/{}", VERSION.unwrap_or("unknown"));
	let client = ClientBuilder::new()
		.timeout(timeout)
		.user_agent(user_agent)
		.build()?;
	let response = client.head(&request_url).send().await?;

	println!("response: {:?}", response);
	println!("response.status(): {:?}", response.status());

	if response.status().is_success() {
		println!("{} is a user!", user);
	} else {
		println!("{} is not a user!", user);
	}

	Ok(())
}
