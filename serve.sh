#!/usr/bin/env bash

# sfz -b 0.0.0.0 / & #> ./logs/static_server_trade.txt

../gohfs -host 0.0.0.0 -port 4000 -dir / 
# Files served on http://0.0.0.0:5000
# a copy of binary server in ./debug
#https://github.com/finzzz/gohfs/releases