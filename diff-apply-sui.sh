#!/bin/bash

# run from the sui directory

set -e

APPLY=${APPLY:true}
CLEAN=${CLEAN:true}
COMMI=${COMMI:true}

$APPLY && git apply ../move/diff-move-core-types.diff \
    -p4 --directory external-crates/move/crates/move-core-types --reject || true
$CLEAN && git clean -f external-crates/move/crates/move-core-types
$COMMI && git commit -am "apply diff-move-core-types.diff"
