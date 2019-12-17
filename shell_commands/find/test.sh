#!/bin/bash
echo "file" > file
echo "./abc" > abc 
find . -type f -execdir grep {} {} \; 
