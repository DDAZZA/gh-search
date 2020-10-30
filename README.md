# gh-search
Tool to search GitHub and match a regex

*Note*
- Not all results are currently displayed (WIP)
- You will need a GitHub token to query private repos

*Examples*
```
# Show help
docker run --rm ddazza/gh-search --help
```

```
# Find all versions of a dependency for an organisation
docker run -i --rm -e GITHUB_API_TOKEN  ddazza/gh-search --org [Org] --file Gemfile.lock "rake \(\d"
```

```
# Find where CSRF protection has been turned off in ruby applications that for an organisation.
docker run -i --rm -e GITHUB_API_TOKEN  ddazza/gh-search --org Sage --lang ruby "skip_before_action.*verify_authenticity_token"
```

## Development

```
$ make develop
> cargo run
```

