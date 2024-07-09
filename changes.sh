#!/bin/bash
/home/jopitim/mzmine-batch-mode-sandbox/test_1
dir1="/home/jopitim/mzmine-batch-mode-sandbox/test_1"
dir2="/home/jopitim/mzmine-batch-mode-sandbox/data/mzml"

for file in "$dir1"/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "Checking: $dir1/$filename"
        if [ -f "$dir2/$filename" ]; then
            echo -e "\t-> Against: $dir2/$filename"
            diff "$file" "$dir2/$filename" > changes_log.txt
        else
            echo "$filename not found in $dir2"
        fi
    fi
done