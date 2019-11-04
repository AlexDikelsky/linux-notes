    ld zero     #Start stuff at 0
    st sum   #Start stuff at 0
loop    get
    jz END
    add sum
    st sum
    j loop
done    ld sum
    put 
    halt
zero   const 0
sum    const
