module GhSearch
  module Formatters
    class Base
      attr_reader :output_file
      def initialize(options={})
        @output_file = options[:output]
      end

      def write!(match)
        raise NotImplementedError
      end

      def write_file!
        # used for writing out a file
      end

      protected

      def print_and_flush(str)
        print str
        $stdout.flush
      end

    end
  end
end
