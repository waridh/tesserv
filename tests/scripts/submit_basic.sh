#!/usr/bin/env bash

# build the tar in the tmp
filename=$(uuidgen)
tar_dest="/tmp/${filename}.tar"
script_dir=$(dirname -- "$(readlink -f -- "$BASH_SOURCE")")
tar_directory="${script_dir}/../resources/"
tar_target="group1"

tar -cvf "${tar_dest}" -C "${tar_directory}/" "${tar_target}"

# sending a curl request
curl -v --location --request POST 'http://localhost:3030/submit' \
    --header 'Content-Type: multipart/form-data' \
    --form "file=@${tar_dest}"
