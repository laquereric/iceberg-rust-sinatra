# Iceberg-Rust-Sinatra

A Ruby gem that combines the Sinatra web framework with Apache Iceberg-Rust for high-performance data lake operations. This gem provides a seamless integration between Ruby web applications and Rust-based data processing, with support for executable packaging via Tebako.

## Features

- **Sinatra Web Framework**: Lightweight and flexible web server for REST API endpoints
- **Apache Iceberg Integration**: Connect to Iceberg data lakes via Rust bindings
- **Ruby-Rust Interoperability**: Uses Rutie for seamless Ruby-Rust communication
- **Executable Packaging**: Create standalone executables with Tebako
- **Comprehensive Testing**: Full RSpec and Cucumber test coverage
- **Fallback Implementation**: Works with or without Rust extension

## Installation

### As a Gem

Add this line to your application's Gemfile:

```ruby
gem 'iceberg-rust-sinatra'
```

And then execute:

```bash
$ bundle install
```

Or install it yourself as:

```bash
$ gem install iceberg-rust-sinatra
```

### Prerequisites

For full functionality with Rust extension:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install build dependencies
sudo apt update
sudo apt install -y build-essential cmake
```

## Usage

### Basic Usage

```ruby
require 'iceberg/rust/sinatra'

# Create an Iceberg catalog
catalog = Iceberg::Rust::Sinatra::Catalog.new

# Create a namespace
catalog.create_namespace('analytics')

# Define a table schema
schema = {
  fields: [
    { name: 'id', type: 'int', required: true },
    { name: 'user_id', type: 'string', required: true },
    { name: 'event_type', type: 'string', required: false },
    { name: 'timestamp', type: 'timestamp', required: true }
  ]
}

# Create a table
table = catalog.create_table('events', schema, namespace: 'analytics')

# Start the web server
app = Iceberg::Rust::Sinatra::WebApp.new(catalog)
app.run!(host: '0.0.0.0', port: 4567)
```

### Command Line Interface

The gem provides a command-line interface:

```bash
# Start the server (default)
iceberg-rust-sinatra

# Start with custom options
iceberg-rust-sinatra --port 8080 --host 127.0.0.1

# Run integration tests
iceberg-rust-sinatra test

# Show version
iceberg-rust-sinatra --version
```

### REST API Endpoints

The web server provides the following REST API endpoints:

#### Health Check
```
GET /health
```
Returns server health status.

#### Namespace Operations
```
GET /namespaces
POST /namespaces
```
List all namespaces or create a new namespace.

#### Table Operations
```
GET /namespaces/:namespace/tables
POST /namespaces/:namespace/tables
GET /namespaces/:namespace/tables/:table
```
Manage tables within namespaces.

### Example API Usage

```bash
# Check server health
curl http://localhost:4567/health

# List namespaces
curl http://localhost:4567/namespaces

# Create a namespace
curl -X POST http://localhost:4567/namespaces \
  -H "Content-Type: application/json" \
  -d '{"namespace": "analytics"}'

# Create a table
curl -X POST http://localhost:4567/namespaces/analytics/tables \
  -H "Content-Type: application/json" \
  -d '{
    "name": "events",
    "schema": {
      "fields": [
        {"name": "id", "type": "int", "required": true},
        {"name": "event_type", "type": "string", "required": false}
      ]
    }
  }'

# Get table info
curl http://localhost:4567/namespaces/analytics/tables/events
```

## Architecture

### Components

1. **Catalog**: Manages Iceberg namespaces and tables
2. **Table**: Represents individual Iceberg tables
3. **WebApp**: Sinatra-based web server with REST API
4. **Rust Extension**: Optional native extension for performance

### Fallback Implementation

The gem includes a pure Ruby fallback implementation that provides the same API without requiring the Rust extension. This ensures the gem works in environments where Rust compilation is not possible.

## Development

After checking out the repo, run:

```bash
# Install dependencies
bundle install

# Build the Rust extension (optional)
cd ext/iceberg_rust_sinatra
cargo build --release
cd ../..

# Run tests
rspec
cucumber

# Run integration tests
./exe/iceberg-rust-sinatra test
```

### Building the Rust Extension

```bash
cd ext/iceberg_rust_sinatra
cargo build --release
```

The Rust extension provides enhanced performance but is optional. The gem will automatically fall back to the Ruby implementation if the extension is not available.

## Packaging with Tebako

Create a standalone executable:

```bash
# Install Tebako
gem install tebako

# Set up Tebako environment (one-time setup)
tebako setup

# Package the application
tebako press
```

This creates a self-contained executable that includes Ruby, all gems, and the application code.

## Testing

The gem includes comprehensive test coverage:

### RSpec Unit Tests
```bash
rspec --format documentation
```

### Cucumber Integration Tests
```bash
cucumber
```

### Manual Integration Test
```bash
./exe/iceberg-rust-sinatra test
```

## Configuration

### Catalog Configuration

```ruby
# Basic configuration
catalog = Iceberg::Rust::Sinatra::Catalog.new

# With custom configuration
config = {
  warehouse: '/path/to/warehouse',
  catalog_type: 'hadoop'
}
catalog = Iceberg::Rust::Sinatra::Catalog.new(config)
```

### Server Configuration

```ruby
# Basic server
app = Iceberg::Rust::Sinatra::WebApp.new(catalog)
app.run!

# With custom options
app.run!(
  host: '0.0.0.0',
  port: 8080,
  environment: 'production'
)
```

## Performance

- **Rust Extension**: Provides native performance for Iceberg operations
- **Fallback Mode**: Pure Ruby implementation for compatibility
- **Memory Efficient**: Minimal memory footprint
- **Concurrent**: Handles multiple requests efficiently

## Compatibility

- **Ruby**: >= 3.0.0
- **Rust**: >= 1.70 (for extension)
- **Platforms**: Linux, macOS, Windows (with appropriate build tools)

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/manus-ai/iceberg-rust-sinatra.

## License

The gem is available as open source under the [MIT License](https://opensource.org/licenses/MIT).

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.

## Support

For questions and support:

- GitHub Issues: https://github.com/manus-ai/iceberg-rust-sinatra/issues
- Documentation: https://github.com/manus-ai/iceberg-rust-sinatra/wiki

## Related Projects

- [Apache Iceberg](https://iceberg.apache.org/)
- [Iceberg-Rust](https://github.com/apache/iceberg-rust)
- [Sinatra](http://sinatrarb.com/)
- [Rutie](https://github.com/danielpclark/rutie)
- [Tebako](https://www.tebako.org/)
# iceberg-rust-sinatra
