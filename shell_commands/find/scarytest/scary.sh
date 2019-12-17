#!/bin/bash 

storepath=$PATH

#Executes whatever "ls" is in your current directory
PATH=.
/bin/touch a b c d
echo '#/bin/bash' > ls
echo 'echo "This is executed"' >> ls
/bin/chmod 777 ls
/bin/find . -exec ls {} \;

PATH=$storepath
