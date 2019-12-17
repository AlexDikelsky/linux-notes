#!/bin/bash

python3 -c 'import base_calc; base_calc.base_b(4, 8, True)'  > regex.txt
python3 -c 'import base_calc; base_calc.base_b(2, 8, False)' > nums.txt
./grep.sh > file

