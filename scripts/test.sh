#!/bin/sh
if [ -f FAIL ]; then echo "a test failed"; exit 1; fi
echo tests ok
