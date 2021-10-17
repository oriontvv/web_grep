# web_grep
A simple example that shows how to fetch urls from stdin and find specified word with rust and tokio + reqwest.

## usage:
1. install [reqwest's requirements](https://github.com/seanmonstar/reqwest#requirements)

2. clone this repo
```bash 
$ git clone https://github.com/oriontvv/web_grep
$ cd web_grep/web_grep
```

2. exec script
```bash
$ echo -e 'https://golang.org\nhttps://golang.org' | cargo run -- --word Go
```
You can compare result with `curl https://golang.org | grep -o 'Go' | wc -l`
