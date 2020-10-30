require 'gh_search'
require 'getoptlong'

module GhSearch
  class CLI
    def self.start
      opts = GetoptLong.new(
        [ '--help', '-h', GetoptLong::NO_ARGUMENT ],
        [ '--version', '-v', GetoptLong::NO_ARGUMENT ],
        [ '--org', GetoptLong::REQUIRED_ARGUMENT ],
        [ '--format', '-f', GetoptLong::REQUIRED_ARGUMENT ],
        [ '--file', GetoptLong::REQUIRED_ARGUMENT ],
        [ '--lang', GetoptLong::REQUIRED_ARGUMENT ],
        [ '--output', '-o', GetoptLong::REQUIRED_ARGUMENT ],
      )


      options = {}

      begin
        opts.each do |opt, arg|
          case opt
          when '--help'
            print_help
            exit 0
          when '--version'
            puts "gh-search #{GhSearch::VERSION}"
            exit 0
          when '--org'
            options[:org] = arg
          when '--lang'
            options[:lang] = arg
          when '--file'
            options[:file] = arg
          when '--format'
            options[:format ]= arg
          when '--output'
            options[:output] = arg
          end
        end
      rescue GetoptLong::Error => e
        print_help
        exit 1
      end
      if ARGV.length != 1
        puts "Missing SEARCH_TEXT argument (try --help)"
        puts
        print_help
        exit 0
      end

      search_text = ARGV.shift

      GhSearch.run(search_text, options)
    end


    private

    def self.print_usage
      puts "Usage: gh-search SEARCH_TEXT [OPTIONS]"
      puts
    end

    def self.print_help
      print_usage
      puts <<-EOF
SEARCH_TEXT:      The alphanumeric text to find matches of.

Search Options:
  --org           GitHub Organisation
  --file          Filename to search (e.g. package.json)
  --lang          Language type to search (e.g. yml)

Output Options:
  --output        Destination to write file
  --format        Output format (e.g. json, csv)

Misc Options:
  -v, --version   Prints version
  -h, --help      Prints this message

For searching private repositories you will need to set the GITHUB_API_TOKEN environment variable.
The GitHub documentation provides steps for this (https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token).
      EOF
    end
  end
end

