let k;

func factorial_recursion(n)
[
    if n < 2 [
        return 1;
    ] else [
        return n * factorial_recursion(n-1);
    ]
]

func factorial_loop(n)
[
    let p;
    p = n;
    while n > 0 [
        n = n - 1;
        p = p * n;
    ]
    return p;
]

func main()
[
    let n;
    n = 5;
    print factorial_loop(n);
    print factorial_recursion(n);
]