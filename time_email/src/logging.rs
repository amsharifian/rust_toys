use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

pub fn log_setup() {
    // let level = log::LevelFilter::Info;

    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();


    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stderr)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .build(
            Root::builder()
            .appender("requests")
            //.appender("stdout")
            .build(LevelFilter::Trace)
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("Dandelion AWS control manager is booting up...");
}
