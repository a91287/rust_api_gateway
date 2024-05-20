use serde::{Serialize, Deserialize};
use config::{Config, ConfigError, File, Environment};


// Define the main settings structure, including service_mapping as a vector of ServiceMapping
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    listening_address: String,
    services: Vec<SvcDetails>,
    logging: LoggingDetails, 
}

#[allow(dead_code)]
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Initialize a new configuration object
        let mut config = Config::new();

        // Merge the configuration from the "config/default.yaml" file
        config.merge(File::with_name("config/default.yaml"))?;

        // Merge the environment variables prefixed with "APP"
        config.merge(Environment::with_prefix("APP"))?;
        println!("Loading configuration...");
        // Try to deserialize the configuration into the Settings struct
        config.try_into()
        
    }

    pub fn get_service_mapping(self) -> Vec<SvcDetails>{
        self.services.clone()
    }

    pub fn get_listening_address(self) -> String{
        self.listening_address
    }

    pub fn get_logging(self) -> LoggingDetails{
        self.logging
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SvcDetails {
    #[serde(rename = "url_matching_expression")]
    url_matching_expression: String,
    
    #[serde(rename = "service_address")]
    service_address: String,

    #[serde(rename = "request_plugins")]
    request_plugins: Vec<RequestPlugins>,

    #[serde(rename = "backend_prefix_removal")]
    backend_prefix_removal: String,


    
}

#[allow(dead_code)]
impl SvcDetails {
    pub fn get_url_matching_expression(self) -> String{
        self.url_matching_expression.clone()
    }
    pub fn get_service_address(self) -> String{
        self.service_address.clone()
    }
    pub fn get_request_plugins(self) -> Vec<RequestPlugins>{
        self.request_plugins.clone()
    }

    pub fn get_backend_prefix_removal(self) -> String{
        self.backend_prefix_removal.clone()
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestPlugins{
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "parameters")]
    parameters: String
}


#[allow(dead_code)]
impl  RequestPlugins {
    pub fn get_name(self) -> String{
        self.name.clone()
    }

    pub fn get_parameters(self) -> String{
        self.parameters.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  LoggingDetails{
    #[serde(rename = "log_path")]
    log_path: String,
    
    #[serde(rename = "log_level")]
    log_level: String,

    #[serde(rename = "log_to_std_out")]
    log_to_std_out: String,

    #[serde(rename = "log_file_size_in_bytes")]
    log_file_size_in_bytes: String,

    #[serde(rename = "log_response_header_and_body")]
    log_response_header_and_body: String,

    #[serde(rename = "log_request_header_and_body")]
    log_request_header_and_body: String,

    

    
}
#[allow(dead_code)]
impl  LoggingDetails {

    pub fn get_log_path(self) -> String{
        self.log_path
    }

    pub fn get_log_level(self) -> String{
        self.log_level
    }

    pub fn get_std_out(self) -> String{
        self.log_to_std_out
    }
    
    pub fn get_log_file_size_in_bytes(self) -> String{
        self.log_file_size_in_bytes
    }
    
    pub fn get_log_request_header_and_body(self) -> String{
        self.log_request_header_and_body
    }

    pub fn get_log_response_header_and_body(self) -> String{
        self.log_response_header_and_body
    }
    
}