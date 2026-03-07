#!/usr/bin/env bash

# This version includes building the system
make >/dev/null
if [ -e ./hello ]; then
    ./hello | diff <(echo "hello world") - >/dev/null
    retcode=$?
    if [ $retcode -eq 0 ]; then
        echo "1/1"
    else
        echo "0/1"
    fi
else
    echo "0/0"
fi
