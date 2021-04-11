#!/bin/bash

set -e

cd $(dirname $0)

./build-book.sh

mdbook serve ../book --port 11000 --open
