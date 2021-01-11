pub struct Log4 {
	tags: Vec<String>,
}

impl Log4 {
	pub fn new() -> Self {
		let log = Log4 { tags: Vec::new() };
		return log;
	}

	pub fn info<T: std::fmt::Debug>(&self, ss: T) {
		let tags_str: String = if (&self).tags.len() > 0 {
			format!("[{}]", (&self).tags.join("]["))
		} else {
			"".to_string()
		};
		println!("{} {:?}", tags_str, ss);
	}

	pub fn add_tag<T: std::fmt::Debug>(&mut self, tag: T) {
		self.tags.push(format!("{:?}", tag));
	}
}

#[test]
fn test_log() {
	let mut log = Log4::new();
	let aaa: String = String::from("lkwfj");
	let bbb: &String = &String::from("lkwfj");
	log.add_tag(aaa);
	log.add_tag(bbb);
	log.info("wklejweklfj");
	// log.info("wklejweklfj");
	log.info(String::from("lkwfj"));
	log.info(bbb);
}
