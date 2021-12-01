#!/usr/bin/env python3

import argparse
import os
from http import client

parser = argparse.ArgumentParser()
parser.add_argument("day_num", type=int)
args = parser.parse_args()

directory = os.path.dirname(__file__)
sess_token_file = os.path.join(directory, 'session_token')

with open(sess_token_file, 'r') as f:
    sesstoken = f.read().strip()

conn = client.HTTPSConnection("adventofcode.com")
conn.request('GET', f'/2021/day/{args.day_num}/input', headers={'Cookie': f'session={sesstoken}'})

res = conn.getresponse()

print(res.status, res.reason)
if res.status == 200:
    with open(os.path.join(directory, f'src/bin/inputs/{args.day_num:02d}.txt'), 'wb') as f:
        data = res.read()
        f.write(data)
        print(f.name)
