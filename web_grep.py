import sys
import asyncio
from argparse import ArgumentParser

from aiohttp import ClientSession, ClientError, ClientTimeout


parser = ArgumentParser()
parser.add_argument('--word', help='word to find')
parser.add_argument('--limit', default=16, type=int, help='concurrency limit')
parser.add_argument('--timeout', default=60, type=int, help='timeout in seconds')
args = parser.parse_args()

semaphore = asyncio.Semaphore(args.limit)
timeout = ClientTimeout(total=args.timeout)


async def process_url(url, session, word='Python'):
    try:
        async with session.get(url, timeout=timeout) as response:
            async with semaphore:
                if response.status != 200:
                    print(f"Can't fetch {url}")
                    return
                text = await response.text()
                count = text.count(word)
                print(f"Count for {url}: {count}")
    except ClientError as e:
        print(f"Can't fetch {url}: {e}")


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
