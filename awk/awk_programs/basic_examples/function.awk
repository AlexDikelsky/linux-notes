function max(n, m) {
    return (n>m) ? n : m
}

function fib(n) {
    return (n<2) ? n : (fib(n-2) + fib(n-1))
}

#{ print $1, $2, max($1,$2) }
{ print $1, fib($1) }
