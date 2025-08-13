use dotenvy::dotenv;

fn main() {
	if let Ok(path) = dotenv() {
		println!("cargo:rerun-if-changed={}", path.display())
	}
}
