#!/bin/sh

# Populate the random files directory
rm -rf random
mkdir random
base64 /dev/urandom | head -c 1M > random/m1.txt
base64 /dev/urandom | head -c 2M > random/m1.txt
base64 /dev/urandom | head -c 10M > random/m10.txt
base64 /dev/urandom | head -c 20M > random/m20.txt
base64 /dev/urandom | head -c 100M > random/m100.txt
base64 /dev/urandom | head -c 200M > random/m200.txt