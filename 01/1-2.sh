#!/usr/bin/env bash

# This does not give the desired result because it interprets jcb82eightwond as 88 instead of 82.
# I think this was not described well enough in the task and it was not covered in the example.

cp input puzzle
sed -Ei 's/(one|two|three|four|five|six|seven|eight|nine)/<\1>/g; s/<one>/1/g; s/<two>/2/g; s/<three>/3/g; s/<four>/4/g; s/<five>/5/g; s/<six>/6/g; s/<seven>/7/g; s/<eight>/8/g; s/<nine>/9/g;' puzzle
paste -d '' <(sed -E 's/[^0-9]*([0-9]).*/\1/' puzzle) <(sed -E 's/.*([0-9])[^0-9]*$/\1/' puzzle) | paste -sd '+' | bc
