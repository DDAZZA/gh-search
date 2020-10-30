require "gh_search/version"
require "gh_search/github_client"
require "gh_search/formatters"

module GhSearch
  class Error < StandardError; end

  def self.run(search_text, options={})
    client = GithubClient.new(search_text, options)

    Formatters.with(options.fetch(:format, 'plain')) do |formatter|
      client.each do |match|
        formatter.write(match)
      end
    end
  end
end
