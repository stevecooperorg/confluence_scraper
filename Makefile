all: FORCE
all: download
all: convert

download: target/scraped.json FORCE
convert: target/converted.txt FORCE

target/scraped.json:
	cargo run > target/scraped.json

target/converted.html: target/scraped.json FORCE
	cat target/scraped.json | jq -r '.[] | { title: .title, body: .body.view.value } | ("\n\n<h1>" + .title + "</h1>\n\n" +  .body + "\n\n")' > target/converted.html

target/converted.txt: target/converted.html FORCE
	cat target/converted.html | pandoc -f html -t plain > target/converted.txt


.PHONY: FORCE
FORCE: