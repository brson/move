#!/bin/sh

# run from the move directory

set -e

git diff upstream/sui-move language/move-core/types > diff-move-core-types.diff
