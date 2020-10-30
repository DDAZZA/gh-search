require 'csv'

module GhSearch
  module Formatters
    class Csv < Base
      def write(match)

        if output_file
          @file ||= CSV.open(output_file, "w")
          unless @headers
            @headers = match.keys
            @file << @headers
          end

          @file << match.values
          print_and_flush('.')
        else
          unless @headers
            @headers = match.keys
            puts @headers.join(',')
          end
          puts match.values.join(',')
        end
      end

      def write_file!
        @file.close if @file
      end
    end
  end
end
