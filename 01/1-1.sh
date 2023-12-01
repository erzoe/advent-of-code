paste -d '' <(sed -E 's/[^0-9]*([0-9]).*/\1/' puzzle) <(sed -E 's/.*([0-9])[^0-9]*$/\1/' puzzle)  | paste -sd '+' | bc
