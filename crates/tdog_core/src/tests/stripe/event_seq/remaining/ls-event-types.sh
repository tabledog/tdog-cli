#!/usr/bin/env bash
real=$(realpath "$(dirname "$0")");
rg --no-filename --no-line-number -o "^\s+?written_from_event\(\"(.+?)\"" $real/../all -r '$1' | sort | uniq