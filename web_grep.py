import sys
import asyncio
from argparse import ArgumentParser
from aiohttp import ClientSession


parser = ArgumentParser()
parser.add_argument('--word', help='word to find')
args = parser.parse_args()


async def process_url(url, session, word='Python'):
    async with session.get(url) as response:
        if response.status != 200:
            print(f"Can't fetch {url}")
            return
        text = await response.text()
        count = text.count(word)
        print(f"Count for {url}: {count}")


async def run():
    tasks = []
    async with ClientSession() as session:
        for url in sys.stdin:
            future = process_url(url.strip(), session, word=args.word)
            task = asyncio.ensure_future(future)
            tasks.append(task)

        responses = await asyncio.gather(*tasks)


loop = asyncio.get_event_loop()
future = asyncio.ensure_future(run())
loop.run_until_complete(future)
