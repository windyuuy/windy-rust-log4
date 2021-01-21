use std::rc::Rc;

pub trait LogImpl {
    fn info(&self, ss: &String);
    fn warn(&self, ss: &String);
    fn error(&self, ss: &String);
}

pub struct DefaultLogImpl {}
impl LogImpl for DefaultLogImpl {
    fn info(&self, ss: &String) {
        println!("{}", ss);
    }

    fn warn(&self, ss: &String) {
        println!("{}", ss);
    }

    fn error(&self, ss: &String) {
        println!("{}", ss);
    }
}

pub struct Log4 {
    tags: Vec<String>,
    delegate: Rc<Box<dyn LogImpl>>,
}

impl Log4 {
    pub fn new() -> Self {
        let log = Log4 {
            tags: Vec::new(),
            delegate: Rc::new(Box::new(DefaultLogImpl {})),
        };
        return log;
    }

    pub fn set_log_impl(&mut self, delegate: Rc<Box<dyn LogImpl>>) -> &mut Self {
        self.delegate = delegate;
        return self;
    }

    fn to_log_line<T: std::fmt::Debug>(&self, ss: T) -> String {
        let tags_str: String = if (&self).tags.len() > 0 {
            format!("[{}]", (&self).tags.join("]["))
        } else {
            "".to_string()
        };
        let content = format!("{} {:?}", tags_str, ss);
        return content;
    }
    pub fn info<T: std::fmt::Debug>(&self, ss: T) {
        let content = self.to_log_line(ss);
        self.delegate.info(&content);
    }
    pub fn warn<T: std::fmt::Debug>(&self, ss: T) {
        let content = self.to_log_line(ss);
        self.delegate.warn(&content);
    }

    pub fn error<T: std::fmt::Debug>(&self, ss: T) {
        let content = self.to_log_line(ss);
        self.delegate.error(&content);
    }

    pub fn add_tag<T: std::fmt::Debug>(&mut self, tag: T) -> &mut Log4 {
        self.tags.push(format!("{:?}", tag));
        return self;
    }

    pub fn fork(&self) -> Log4 {
        let log4 = Log4 {
            tags: self.tags.clone(),
            delegate: self.delegate.clone(),
        };
        return log4;
    }
}

#[test]
fn test_log() {
    let mut log = Log4::new();
    let aaa: String = String::from("AAA");
    let bbb: &String = &String::from("BBB");
    log.add_tag(aaa).add_tag(bbb);
    log.info("CCC");
    // log.info("wklejweklfj");
    log.info(String::from("DDD"));
    log.info(bbb);

    let mut log2 = log.fork();
    log2.add_tag("fork1");
    log2.info("EEE");
    log2.warn("GGG");
    log2.error("HHH");

    log.info("III");
    log.warn("JJJ");
    log.error("KKK");
}
