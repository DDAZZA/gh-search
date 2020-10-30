module GhSearch
  module Formatters
    require "gh_search/formatters/base"
    require "gh_search/formatters/plain"
    require "gh_search/formatters/json"
    require "gh_search/formatters/csv"

    def self.with(type, options={})
      formatter_class = case type
                  when 'plain'
                    Formatters::Plain
                  when 'csv'
                    Formatters::Csv
                  when 'json'
                    Formatters::Json
                  when 'stdout'
                    Formatters::Stdout
                  else
                    raise 'unknown format type'
                  end

      # puts "[DEBUG] Loaded #{formatter_class.name}"
      formatter = formatter_class.new(options)
      yield formatter

      formatter.write_file!
      puts
    end
  end
end
