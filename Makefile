PROJECT_NAME=gh-search

build:
	docker build -t ddazza/$(PROJECT_NAME) .

run:
	docker run -i --rm  -e GITHUB_API_TOKEN -e RUST_BACKTRACE=1  ddazza/$(PROJECT_NAME)
	# docker run -i --rm  -e RUST_BACKTRACE=1 -v `pwd`:"/workspace":ro ddazza/$(PROJECT_NAME)

deploy: build
	docker tag ddazza/$(PROJECT_NAME):latest ddazza/$(PROJECT_NAME):$(VERSION)
	# docker push ddazza/$(PROJECT_NAME):$(VERSION)
	# docker push ddazza/$(PROJECT_NAME):latest

develop:
	docker run --rm -it -w /app \
		-v "$(PWD)/Cargo.toml":/app/Cargo.toml:ro \
		-v "$(PWD)/src/":/app/src/:ro \
		rust:1.46 bash

		# -v "$(PWD)/Cargo.lock":/app/Cargo.lock \
