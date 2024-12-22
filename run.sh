#!/bin/bash
set -e

cargo bench || exit 1

if ! [ -z "$(ls -A ./target/criterion 2>> /dev/null)" ]; then
    mv ./target/criterion ./upload/criterion-result-$CURRENT_OS
fi
