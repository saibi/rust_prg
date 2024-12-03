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

pub fn start(log_level: &str, log_file: &str, to_stderr: bool) {
    let log_dir = std::path::Path::new(log_file).parent().unwrap();
    let log_filename = std::path::Path::new(log_file).file_stem().unwrap();
    let log_extension = std::path::Path::new(log_file).extension().unwrap();
    if !log_dir.exists() || log_filename == "" {
        panic!("invalid log filename {}", log_file);
    }

    let mut logger = flexi_logger::Logger::try_with_str(log_level)
        .unwrap()
        .log_to_file(
            flexi_logger::FileSpec::default()
                .suppress_timestamp()
                .directory(log_dir)
                .basename(log_filename.to_str().unwrap())
                .suffix(log_extension.to_str().unwrap()),
        )
        .append()
        .write_mode(flexi_logger::WriteMode::BufferAndFlush) // WriteMode::Direct
        .format(log_format) // format for console
        .format_for_files(log_format); // format for file

    if to_stderr {
        logger = logger.duplicate_to_stderr(flexi_logger::Duplicate::All);
    }

    logger.start().unwrap();
}
