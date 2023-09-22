#!/usr/bin/env bash
# 
# test.sh: run some stashr commands

echo "Starting tests in $(pwd)"
find .

stashr a
if [[ -f a ]]; then
    echo "Could not push a file"
fi

stashr
if [[ ! -f a ]]; then
    echo "Could not pop a file"
fi

stashr subdir
if [[ -d subdir ]]; then
    echo "Could not push a dir"
fi

stashr
if [[ ! -d subdir ]]; then
    echo "Could not pop a dir"
fi

stashr a subdir
if [ `ls -1 2> /dev/null | wc -l` -gt 0 ]; then
    echo "Could not push all"
fi

stashr
if [ `ls -1 2> /dev/null | wc -l` -lt 2 ]; then
    echo "Could not pop all"
fi

echo "Finished..."

