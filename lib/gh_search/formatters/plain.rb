module GhSearch
  module Formatters
    class Plain < Base
      def write(match)
        text = [match[:repo_name], match[:path], match[:linenumber], match[:text], "\n"].join(' ')

        if output_file
          @file ||= File.open(output_file, "w")
          @file.write text
          print_and_flush('.')
        else
          puts text
        end
      end

      def write_file!
        @file.close if @file
      end
    end
  end
end
