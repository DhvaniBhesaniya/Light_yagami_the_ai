#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_unsafe)]

use std::collections::{HashMap, HashSet};
use std::thread;

use lazy_static::lazy_static;
use log4rs::Config;
use log4rs::append::Append;
use log4rs::append::rolling_file::policy::compound::{CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger};
use log4rs::append::{console::ConsoleAppender, file::FileAppender, rolling_file::RollingFileAppender};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::Style;
use log4rs::encode::json::{JsonEncoder, JsonEncoderConfig};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::writer::simple::SimpleWriter;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::filter::{Filter, FilterConfig};

// use crate::configuration as config;
use log::{Level, LevelFilter, Record};

lazy_static! {
    //Most of these will define in config File.
    // static ref LOG_LINE_PATTERN_CONSOLE: &'static str = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";
    // static ref LOG_LINE_PATTERN_FILE: &'static str = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

    static ref TRIGGER_FILE_SIZE : u64 = 17408; // config::get::<u64>("logger.logFileSize");
    static ref ROLLER_FILEPATH_PATTERN: String  = "logs/LogsFiles/All-Logs_{}.log".to_string(); // config::get::<String>("logger.roller_filepath_pattern");
    static ref ROLLER_MAX_COUNT: u32 = 999; // config::get::<u32>("logger.roller_max_count") as u32;
    static ref ROLLER_BASE_START: u32 = 1; // config::get::<u32>("logger.roller_base_start") as u32;
    static ref ALL_LOG_FILE_PATH:String = "logs/step/All-Logs.log".to_string(); // config::get::<String>("logger.all_logs_common_file_path");
}

pub struct RollingFileAppenderComponent {
    pub trigger: Box<SizeTrigger>,
    pub roller: Box<FixedWindowRoller>,
    pub compound_policy: Box<CompoundPolicy>,
}
impl RollingFileAppenderComponent {
    pub fn new_policy() -> Box<CompoundPolicy> {
        //Set trigger, roller and compoundPolicy for "RollingFileAppender"
        let trigger: Box<SizeTrigger> = Box::new(SizeTrigger::new(*TRIGGER_FILE_SIZE));
        let roller: Box<FixedWindowRoller> = Box::new(FixedWindowRoller::builder().base(*ROLLER_BASE_START).build(&ROLLER_FILEPATH_PATTERN, *ROLLER_MAX_COUNT).unwrap());
        let compound_policy: Box<CompoundPolicy> = Box::new(CompoundPolicy::new(trigger.clone(), roller.clone()));
        compound_policy
    }
}

pub struct LoggerConfig {}
impl LoggerConfig {
    pub fn create_Global_logs_config() -> Config {
        //===== get compound Policy =====
        let compound_policy: Box<CompoundPolicy> = RollingFileAppenderComponent::new_policy();

        //===== set appanders for console and file =====
        let console_appender = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new("{l} - {m}{n}"))).build();

        // Pattern vise logs
        // let all_log_appender = RollingFileAppender::builder()
        // .encoder(Box::new(PatternEncoder::new(&LOG_LINE_PATTERN_FILE)))
        // .build(*ALL_LOG_FILE_PATH, compound_policy)
        // .unwrap();

        //json format logs
        let all_log_appender = RollingFileAppender::builder().encoder(Box::new(JsonEncoder::new())).build(ALL_LOG_FILE_PATH.as_str(), compound_policy).unwrap();

        //===== create config =====
        //ThresholdFilter is mendatory for set LogLevel on specific appenders
        //describe all appenders in config. and declare only that appanders name in Root which you want to use.
        //set your Max(Default) Log-level in Root.

        // TO-DO : replace level og log

        let Global_logs_config: Config = Config::builder()
            .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Info))).build("console_appender", Box::new(console_appender)))
            .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Info))).build("all_log_appender", Box::new(all_log_appender)))
            .logger(Logger::builder().appender("all_log_appender").additive(true).build("All-Logs", LevelFilter::Info))
            .logger(Logger::builder().appender("console_appender").additive(true).build("console", LevelFilter::Info))
            // .logger(Logger::builder().appender("console_appender").additive(true).build("inventyv_datalayer_lib", LevelFilter::Off))
            // .logger(Logger::builder().appender("console_appender").additive(true).build("couchbase", LevelFilter::Error))
            .build(Root::builder().appenders(["console_appender", "all_log_appender"]).build(LevelFilter::Info))
            .unwrap();
        Global_logs_config
    }
}

pub fn startLogger() {
    // You can use handle to change logger config at runtime
    // just call startLogger() in main.rs and you can use log4rs in all your Project-crate.
    let Global_logs_config = LoggerConfig::create_Global_logs_config();
    let handle = match log4rs::init_config(Global_logs_config) {
        Ok(handle) => handle,
        Err(e) => {
            println!("Logger initialization failed: {}", e);
            return;
        }
    };
}
