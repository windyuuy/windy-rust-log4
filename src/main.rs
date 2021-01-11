mod log4;
use log4::Log4;

fn main() {
	let mut log=Log4::new();
	log.add_tag("klwjfelkj");
	log.add_tag("fe".to_string());
	log.info("klwjfl");
	log.info("klwjfl".to_string());
}
