
# Rust API Gateway

This project is a fast and reliable API gateway written in Rust. It acts as a central hub for managing and routing API requests between clients and backend services.

## Features

- **High Performance**: Utilizes Rust's performance to efficiently route requests.
- **Configurable Routing**: Easily manage routes through configuration files.
- **Secure**: Implements various security features to protect your backend services, including rate limiting and authentication.
- **Scalable**: Designed to handle high traffic and can be scaled horizontally.
- **Pluggable Architecture**: Support for request plugins to customize and extend functionality.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/amauri912/rust_api_gateway.git
   cd rust_api_gateway
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the API gateway:
   ```sh
   cargo run --release
   ```

### Configuration

The API Gateway uses a YAML configuration file to set up its routes, services, and logging. Below is an example configuration file:

```yaml
listening_address: "127.0.0.1:3000"

services:
  - svc0:
    url_matching_expression: \/v1\/api\/$
    service_address: "http://localhost:2021/"
    backend_prefix_removal: /v1/api
    request_plugins:
    - name: test_plugin
      parameters: parameters
  - svc1:
    url_matching_expression: "(/v1/api/Documents/$)"
    service_address: "http://localhost:2021/////"
    backend_prefix_removal: /v1/api
    request_plugins:
    - name: test_plugin
      parameters: parameters
  - svc2:
    url_matching_expression: "(/target3)"
    service_address: "http://localhost:20232/target"
    backend_prefix_removal: /v1/api
    request_plugins:
    - name: test_plugin
      parameters: parameters

logging:
  log_path: logs/api_gtw_p.log
  log_level: debug
  log_to_std_out: false
  log_file_size_in_bytes: 100000
  log_response_header_and_body: false
  log_request_header_and_body: true
```

#### Configuration Details

- `listening_address`: Specifies the address and port on which the API Gateway will listen for incoming requests.
  
- `services`: Defines the backend services to which the gateway will route requests.
  - `url_matching_expression`: A regex pattern to match the incoming request URLs.
  - `service_address`: The address of the backend service.
  - `backend_prefix_removal`: The prefix to remove from the incoming request URL before forwarding it to the backend service.
  - `request_plugins`: List of plugins to apply to the request. Each plugin has a `name` and `parameters`.

- `logging`: Configures the logging behavior of the API Gateway.
  - `log_path`: The file path for storing log files.
  - `log_level`: The logging level (e.g., debug, info).
  - `log_to_std_out`: Whether to log to standard output.
  - `log_file_size_in_bytes`: Maximum size of the log file before rotation.
  - `log_response_header_and_body`: Whether to log response headers and body.
  - `log_request_header_and_body`: Whether to log request headers and body.

### Environment Variables

The following environment variables can be set to configure the gateway:

- `API_GATEWAY_PORT`: Port on which the gateway will run (default: 8080).
- `LOG_LEVEL`: Logging level (e.g., info, debug).

### Logging

The API gateway uses the `env_logger` crate for logging. You can configure the log level by setting the `RUST_LOG` environment variable. For example:
```sh
RUST_LOG=info cargo run --release
```

## Usage

Once the API gateway is running, it will listen for incoming API requests on the specified port and route them to the appropriate backend services based on the configuration.

### Example Requests

You can test the API gateway using tools like `curl` or Postman. Here are some example requests:

```sh
curl -X GET http://localhost:3000/v1/api/resource
```

## Testing

The project includes a suite of tests to ensure functionality. You can run the tests using Cargo:
```sh
cargo test
```

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- Inspired by various open-source API gateway projects.

## Contact

For any inquiries, please open an issue on the GitHub repository or contact the project maintainers directly.
