#!/usr/bin/env bash
real=$(realpath "$(dirname "$0")");
rg --no-filename --no-line-number -o "^\s+?inserted_from_dl\(\"(.+?)\"" $real/../all -r '$1' | sort | uniq