#!/usr/bin/ruby
# Simple utilitiy that uses the GitHub API for search
#
#  Example endpoint it uses: https://api.github.com/search/code?q=HACK+in:file+org:Sage

module GhSearch
  require 'net/http'
  require "base64"
  require 'json'

  class GithubClient
    GITHUB_URL = "https://api.github.com"
    ENDPOINT = "/search/code"

    attr_reader :org, :search_text, :filename

    attr_accessor :pagenum

    def initialize(search_text, options={})
      @search_text = search_text
      @org = options[:org]
      @filename = options[:file]
      @pagenum = 1
    end

    def each
      has_results = true

      while has_results do
        json_response = make_request(uri)['items']

        if has_results = json_response.any?
          json_response.each do |item|
            each_match(URI(item['url'])) do |match|

              result = match.merge({
                repo_name: item['repository']['full_name'],
                html_url: item['html_url'],
              })

              yield result
            end
          end
        end

        @pagenum += 1
      end
    end

    private

    def make_request(request_uri)
      # puts uri # debug
      req = Net::HTTP::Get.new(request_uri)

      if ENV['GITHUB_API_TOKEN']
        req['AUTHORIZATION'] = "token #{github_token}"
      else
        puts "[WARN] unauthenticated"
      end

      http = Net::HTTP.new(request_uri.hostname, request_uri.port)
      http.use_ssl = true

      response = http.request(req)

      if response.code != "200"
        puts "[ERROR]: #{response.code}"
        puts "[DEBUG] failed to call #{request_uri}"
        puts response.body
        exit 1
      end

      JSON.parse(response.body)
    end

    def each_match(request_uri)
      response = make_request(request_uri)

      raise 'Error dont know how to decode content' if response['encoding'] != "base64"
      plain = Base64.decode64(response['content'])

      begin
        # TODO get line number
        plain.match(/(.*#{search_text}.*)/i).captures.each do |capture|
          match = {
            text: capture,
            path: response['path'],
            commit_sha: response['sha'],
          }
          yield match
        end
      rescue
        # could not find match
        return nil
      end
    end

    def github_token
      ENV.fetch('GITHUB_API_TOKEN')
    end

    def uri
      query = "q=#{search_text}"
      query += "+in:file"
      query += "+org:#{org}" if org
      query += "+filename:#{filename}" if filename

      url = [GITHUB_URL, ENDPOINT, "?page=#{@pagenum}&", query].join
      # puts "[DEBUG] url: '#{url}'"

      URI(url)
    end

  end
end
