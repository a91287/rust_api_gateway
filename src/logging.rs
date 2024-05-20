/*
Log module to be used to log messages during the application execution.
This mod needs to be initialized on the main.rs file.

*/
pub mod logger{
    use std::error::Error;
    use flexi_logger::{Logger, FileSpec, DeferredNow, Record, WriteMode};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn extract_path_and_filename(input_path: &str) -> (String, String) {
        let path = Path::new(input_path);
        let dir = path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::new()).to_string_lossy().into_owned();

        let file = path.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap();
        (dir, file)
    }

    fn get_value_from_key(cfg:Vec<(&str,&str)>, find_key:&str, default_value:&str) -> String{
        cfg.iter()
        .find(|&&(key, _)| key == find_key)
        .map(|&(_, value)| value)
        .unwrap_or(&default_value).to_string()
    }

    //This function set the log file path and filename as per cfg
    fn set_log_file_path(cfg:Vec<(&str,&str)>) -> (String, String){
        // BEGIN Define log file path including name
        let log_file = get_value_from_key(cfg.clone(),
            "log_path",
            "output.log" );

        let (mut dir, file) = extract_path_and_filename(log_file.as_str());

        if dir.eq(""){
            dir = "./".to_string();
        }
        // BEGIN Define log file path including name
        (dir,file)
    }

    //This function sets the log leve str
    fn set_log_level(cfg:Vec<(&str,&str)>) -> String{
        //Define the log level
        let log_level_str = get_value_from_key(cfg,
            "log_level",
            "debug" );
        log_level_str
    }

    //This function configure if logs are printed to the stdout
    fn get_print_to_stdout(cfg:Vec<(&str,&str)>) -> String{
        let std_out = get_value_from_key(cfg.clone(),
            "std_out",
            "false" );
            std_out
    }

    //This function configure if logs are printed to the stdout
    fn get_log_file_size_in_bytes(cfg:Vec<(&str,&str)>) -> u64{
        let log_file_size_in_bytes = get_value_from_key(cfg.clone(),
            "log_file_size_in_bytes",
            "10000" );
        
        let f_size: u64 =  match log_file_size_in_bytes.parse::<u64>(){
            Ok(num) => num,
            Err(e) => {
                panic!("Unable to parse the log file size from configuration: {}", e);
            }
        };
        f_size
    }

    pub fn setup_logger(cfg:Vec<(&str,&str)>) -> Result<(), Box<dyn Error>>{
        //--- BEGIN Assigning required logging parameters ---//
        let (dir, file) = set_log_file_path(cfg.clone());
        //Define the log level
        let log_level_str =set_log_level(cfg.clone());
        //Define if logs will be printed to stdout
        let std_out = get_print_to_stdout(cfg.clone());
        //Define log file size in bytes
        let log_file_size_in_bytes = get_log_file_size_in_bytes(cfg.clone());
        //--- END Assigning required logging parameters ---//

        //--- BEGIN Log setup --//
        //Set the log level
        let lg =Logger::try_with_str(log_level_str).expect(
            "ERROR: The log level on the configuration is invalid !"
            ).format(custom_format);

        //Set the log file directory an file name
        let lg = lg.log_to_file(FileSpec::default()
            .directory(dir)//Log directory
            .basename(file)); //Log file name

        let lg = lg.write_mode(WriteMode::BufferAndFlush);
        let lg = if std_out.to_lowercase().eq("true"){
            lg.log_to_stdout()
        }else{
            lg.append()
        };
        let lg = lg.rotate(flexi_logger::Criterion::Size(log_file_size_in_bytes), flexi_logger::Naming::Timestamps, flexi_logger::Cleanup::KeepLogFiles(3));
        let _lg = lg.start();
        Ok(())
        //--- END Log setup --//
    }

    //Format how the log messages are written.
    fn custom_format(
        w: &mut dyn Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> std::io::Result<()> {
        write!(
            w,
            "{} [{}] {} [{}:{}] {}",
            now.format("%Y-%m-%d %H:%M:%S%.3f"), // Timestamp with milliseconds
            record.level(),
            record.target(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            &record.args()
        )
    }

}