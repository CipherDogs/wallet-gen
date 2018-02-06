#!/usr/bin/env python3

import json
import re
import sys

COIN_ROW_REGEX = re.compile(r'(\d+)\s*\|\s*(0x[A-Fa-f0-9]+)\s*\| ([A-Z]*)\s*\| (.+)\s?')
COIN_NAME_REGEX = re.compile(r'\[([^\]]+)\]\(([^\)]+)\)')

def get_name_url(text):
    match = COIN_NAME_REGEX.match(text)
    if match is None:
        return text, None
    else:
        return match[1], match[2]

if __name__ == '__main__':
    with open(sys.argv[1], 'r') as fh:
        coins = []
        for line in fh.readlines():
            row = COIN_ROW_REGEX.match(line)
            if row is None:
                continue

            idx, hexa, symbol, name = row[1], row[2], row[3], row[4]
            name, url = get_name_url(name)

            if not symbol:
                continue

            # TODO do something with data

