require 'json'

module GhSearch
  module Formatters
    class Json < Base
      def write(match)
        @data ||= []
        @data << match
        print_and_flush('.') if output_file
      end

      def write_file!
        if output_file
          @file ||= File.open(output_file, "w")
          @file.write @data.to_json
          @file.close
        else
          puts @data
        end
      end
    end
  end
end
