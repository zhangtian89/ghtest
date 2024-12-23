#!/bin/bash
set -e

mkdir -p "./target"
        BRANCH="$(echo "$(git branch --show-current)" | sed 's/[]:<>|?"*+\/\\[]/_/g')"
if [[ "$REUSE_RESULT" == "off" ]]; then 
    ACHIEVE_NAME="criterion-result-${CURRENT_OS}-${BRANCH}-$(date '+%s')"
    echo Result download disabled
else
    if [[ "$REUSE_RESULT" == "per_branch" ]]; then
        ACHIEVE_NAME="criterion-result-${CURRENT_OS}-${BRANCH}"
    elif [[ "$REUSE_RESULT" == "per_os" ]]; then
        ACHIEVE_NAME="criterion-result-${CURRENT_OS}"
    else
        echo Invalid REUSE_RESULT: $REUSE_RESULT
        exit 1
    fi
    DOWNLOADED=true
    URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${RELEASE_TAG}/$ACHIEVE_NAME.zip"
    echo Download last result from $URL
    curl -fL \
        -o "./target/$ACHIEVE_NAME.zip" \
        "$URL" \
        || DOWNLOADED=false
    if [[ "$DOWNLOADED" == "true" ]]; then
        cd ./target
        rm -r criterion || _A=1
        unzip $ACHIEVE_NAME.zip
        mv $ACHIEVE_NAME criterion
        cd ..
        echo Downloaded last result
    else
        echo No result downloaded
    fi
fi

cargo bench || exit 1

if ! [ -z "$(ls -A ./target/criterion 2>> /dev/null)" ]; then
    mv ./target/criterion ./target/upload/$ACHIEVE_NAME
fi
