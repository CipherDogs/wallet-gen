#!/usr/bin/env python3
import sys

if __name__ == '__main__':
    with open(sys.argv[1], 'r') as fh:
        for line in fh.readlines():
            groups = [group.strip() for group in line.split('|')]

            try:
                symbol, index = groups[0], groups[1]
                wif_version, p2pkh_version, p2sh_version = groups[2], groups[3], groups[4]
                ext_secret, url = groups[5], groups[6]
            except IndexError:
                continue

            if symbol == '???':
                continue

            print(f"Coin::{symbol} => ({wif_version}, {p2pkh_version}, {p2sh_version}),")

            # Do something with variables
