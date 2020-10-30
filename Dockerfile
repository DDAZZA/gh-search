FROM ruby:2.7.1-slim-buster
LABEL author="Dave Elliott"
LABEL homepage="https://github.com/DDAZZA/gh-search"

COPY gh-search-*.gem .
RUN gem install gh-search-*.gem
VOLUME ./output/

ENTRYPOINT ["gh-search"]
