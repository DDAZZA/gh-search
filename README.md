# GH-Search

A simple utility to search GitHub for alphanumeric strings

## Installation

    $ gem install gh-search

or add it to your application's Gemfile

For usage with private repositories the GITHUB_API_TOKEN env var will need to be set.

## Usage

*with ruby*

  $ gh-search --help

*with docker*

  $ docker run --rm -it ddazza/gh-search:ruby --help
  $ docker run --rm  -it -v `pwd`:/output -e GITHUB_API_TOKEN ddazza/gh-search:ruby "SEARCH TERM" -o /output/results.txt

## Development with docker

    $ make
    $ make run

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ddazza/gh-search. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
