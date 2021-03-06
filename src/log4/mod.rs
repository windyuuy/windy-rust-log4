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
    /// 日志对象是否启用
    pub enable: bool,
    /// 日志标签列表
    tags: Vec<String>,
    /// 日志IO代理
    delegate: Rc<Box<dyn LogImpl>>,
}

unsafe impl Sync for Log4 {}

impl Log4 {
    pub fn new() -> Self {
        let log = Log4 {
            enable: true,
            tags: Vec::new(),
            delegate: Rc::new(Box::new(DefaultLogImpl {})),
        };
        return log;
    }

    pub fn set_log_impl(&mut self, delegate: Box<dyn LogImpl>) -> &mut Self {
        self.delegate = Rc::new(delegate);
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
        if self.enable {
            let content = self.to_log_line(ss);
            self.delegate.info(&content);
        }
    }
    pub fn warn<T: std::fmt::Debug>(&self, ss: T) {
        if self.enable {
            let content = self.to_log_line(ss);
            self.delegate.warn(&content);
        }
    }

    pub fn error<T: std::fmt::Debug>(&self, ss: T) {
        if self.enable {
            let content = self.to_log_line(ss);
            self.delegate.error(&content);
        }
    }

    pub fn add_tag<T: std::fmt::Debug>(&mut self, tag: T) -> &mut Log4 {
        self.tags.push(format!("{:?}", tag));
        return self;
    }

    pub fn fork(&self) -> Log4 {
        let log4 = Log4 {
            tags: self.tags.clone(),
            delegate: self.delegate.clone(),
            enable: self.enable,
        };
        return log4;
    }
}

#[cfg(test)]
mod test_log4 {
    use super::{DefaultLogImpl, Log4};
    use once_cell::sync::Lazy;

    static mut MYLOG: Lazy<Log4> = Lazy::new(|| Log4::new());

    static mut MYLOG22: Lazy<Log4> = Lazy::new(|| unsafe { MYLOG.fork() });

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

        let mut log3 = log2.fork();
        log3.info("log enabled");
        log3.enable = false;
        log3.info("this log shall not appear");
        log3.enable = true;
        log3.info("log enabled again");
    }

    #[test]
    fn test_delete() {
        let mut log = Log4::new();
        log.set_log_impl(Box::new(DefaultLogImpl {}));
        log.info("test delegate");
    }

    #[test]
    fn test_static() {
        unsafe {
            MYLOG.info("static info");
            MYLOG.warn("static warn");
            MYLOG.error("static error");

            let log2 = MYLOG.fork();
            log2.info("fork static info");

            MYLOG22.info("static fork info");
        }
    }
}
