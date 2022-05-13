#!/bin/sh

##
## This script continuously runs the tests on code changes.
##

cargo watch -q -c -w src/ -x 'test model_ -- --test-threads=1 --nocapture'

