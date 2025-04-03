pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter {
    logger: StderrLogger,
    filter_func: Box<dyn Fn(u8, &str) -> bool>,
}

impl Filter {
    fn new(log: StderrLogger, filter_func: impl Fn(u8, &str) -> bool + 'static) -> Self {
        Self {
            logger: log,
            filter_func: Box::new(filter_func),
        }
    }
}

impl Logger for Filter {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.filter_func)(verbosity, message) {
            self.logger.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}