# frozen_string_literal: true

require_relative "sinatra/version"
require "rutie"

module Iceberg
  module Rust
    module Sinatra
      class Error < StandardError; end
      
      Rutie.new(:iceberg_rust_sinatra).init "Init_iceberg_rust_sinatra", __dir__
    end
  end
end
