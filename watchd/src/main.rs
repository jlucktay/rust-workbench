use {
	clap::Parser,
	reqwest::{ClientBuilder, Result},
	std::{
		hash::{Hash, Hasher},
		time::Duration,
	},
};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// Simple program to look up a user through a web API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// User to look up on GitHub
	#[clap(short, long, value_parser)]
	user: String,
}

/// [Further reading](https://www.reddit.com/r/rust/comments/xmc91c/comment/ipnsjxr).
impl Hash for Args {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let Self { user: _ } = self;
		self.user.hash(state);
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	let request_url = format!("https://api.github.com/users/{}", args.user);
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
		println!("{} is a user!", args.user);
	} else {
		println!("{} is not a user!", args.user);
	}

	Ok(())
}
