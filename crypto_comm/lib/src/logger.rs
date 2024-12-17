use flexi_logger::LoggerHandle;

const LOG_TIMESTAMP_FORMAT: &str = "%Y-%m-%dT%H:%M:%S %:z";

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    if record.level() == log::Level::Info {
        write!(w, "{} {}", now.format(LOG_TIMESTAMP_FORMAT), &record.args())
    } else if record.level() == log::Level::Debug {
        write!(
            w,
            "{} {} [{}:{}] {}",
            now.format(LOG_TIMESTAMP_FORMAT),
            record.level(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            &record.args()
        )
    } else {
        write!(
            w,
            "{} {} {}",
            now.format(LOG_TIMESTAMP_FORMAT),
            record.level(),
            &record.args()
        )
    }
}

pub fn start(
    log_level: &str,
    log_file: &str,
    to_stderr: bool,
) -> Result<LoggerHandle, flexi_logger::FlexiLoggerError> {
    if log_file == "" && !to_stderr {
        panic!("invalid log_file and to_stderr is not set");
    }

    let mut logger;

    if let Some(log_path_info) = LogPathInfo::new(log_file) {
        let file_spec = flexi_logger::FileSpec::default()
            .suppress_timestamp()
            .directory(log_path_info.dir)
            .basename(log_path_info.filename)
            .suffix(log_path_info.extension);

        logger = flexi_logger::Logger::try_with_str(log_level)
            .unwrap()
            .format(log_format)
            .log_to_file(file_spec)
            .append()
            .write_mode(flexi_logger::WriteMode::BufferAndFlush); // WriteMode::Direct

        if to_stderr {
            logger = logger.duplicate_to_stderr(flexi_logger::Duplicate::All);
        }
    } else {
        if to_stderr {
            logger = flexi_logger::Logger::try_with_str(log_level)
                .unwrap()
                .format(log_format)
                .log_to_stderr();
        } else {
            panic!("log_file is invalid");
        }
    }

    logger.start()
}

#[derive(Debug)]
struct LogPathInfo {
    dir: std::path::PathBuf,
    filename: String,
    extension: String,
}

impl LogPathInfo {
    fn new(log_file: &str) -> Option<Self> {
        let path = std::path::Path::new(log_file);
        let log_dir = path.parent()?;
        let log_filename = path.file_stem()?;
        let log_extension = path.extension();

        if !log_dir.is_dir() {
            return None;
        }

        Some(Self {
            dir: log_dir.to_path_buf(),
            filename: log_filename.to_str().unwrap().into(),
            extension: if let Some(extension) = log_extension {
                extension.to_str().unwrap().into()
            } else {
                // default extension
                "log".into()
            },
        })
    }
}
