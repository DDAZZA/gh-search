lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "gh_search/version"

Gem::Specification.new do |spec|
  spec.name          = "gh-search"
  spec.version       = GhSearch::VERSION
  spec.authors       = ["Dave Elliott"]
  spec.email         = ["ddazza@gmail.com"]

  spec.summary       = %q{Tool to search GitHub for matching text}
  spec.description   = %q{Uses the GitHub API to perform searches for given string}
  spec.homepage      = "https://github.com/DDAZZA/gh-search/blob/master/README.md"
  spec.license       = "MIT"

  # Prevent pushing this gem to RubyGems.org. To allow pushes either set the 'allowed_push_host'
  # to allow pushing to a single host or delete this section to allow pushing to any host.
  if spec.respond_to?(:metadata)
    spec.metadata["homepage_uri"] = spec.homepage
    spec.metadata["source_code_uri"] = "https://github.com/DDAZZA/gh-search"
    spec.metadata["changelog_uri"] = "https://github.com/DDAZZA/gh-search/blob/master/CHANGE_LOG.md"
  else
    raise "RubyGems 2.0 or newer is required to protect against " \
      "public gem pushes."
  end

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    Dir.glob('{bin,lib}/**/**/**') + %w(LICENSE.txt)
  end
  spec.bindir        = "bin"
  spec.executables   << 'gh-search'
  spec.require_paths = ["lib"]

  spec.add_development_dependency "bundler", "~> 1.17"
  spec.add_development_dependency "pry", "~> 0.14"
  spec.add_development_dependency "rake", "~> 10.0"
  spec.add_development_dependency "rspec", "~> 3.0"
end

