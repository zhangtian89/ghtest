#!/bin/bash
set -e

mkdir -p "./target"
DOWNLOADED=true
URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${RELEASE_TAG}/criterion-result-${CURRENT_OS}.zip"
echo Download last result from $URL
curl -fL \
    -o "./target/criterion-result-${CURRENT_OS}.zip" \
    "$URL" \
    || DOWNLOADED=false
if [[ "$DOWNLOADED" == "true" ]]; then
    cd ./target
    unzip criterion-result-${CURRENT_OS}.zip
    mv criterion-result-${CURRENT_OS} criterion
    echo Downloaded last result
else
    echo No result downloaded
fi

cargo bench || exit 1

if ! [ -z "$(ls -A ./target/criterion 2>> /dev/null)" ]; then
    mv ./target/criterion ./upload/criterion-result-$CURRENT_OS
fi
