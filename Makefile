build:
	docker run --rm -it -v $(PWD):/workspace ruby:2.7.1-slim-buster bash -c "cd /workspace && gem build gh_search.gemspec"
	docker build -t ddazza/gh-search:ruby .

run:
	# ruby -I lib bin/gh-search lib/ $*
	docker run -it ddazza/gh-search:ruby --help

deploy:
	git push origin "v$(TAG)"
	gem push renogen-$(TAG).gem
	docker push ddazza/gh-search:latest
	docker push ddazza/gh-search:$(TAG)
