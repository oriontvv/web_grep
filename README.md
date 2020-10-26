# web_grep
A simple example that shows how to fetch urls from stdin and find specified word with python and aiohttp.

## usage:
1. install aiohttp with pip:
```bash
$ python3 -m pip install aiohttp
```

2. exec script
```bash
$ echo -e 'https://golang.org\nhttps://golang.org' | python3 web_grep.py --word Go
```
You can compare result with `curl https://golang.org | grep -o 'Go' | wc -l`
