#!/bin/bash

set -e

cd $(dirname $0)

mdbook build ../book
