func add(a,b) [
    return a + b;
]

func main(argc) [
    let x;
    x = 5 + 4 + 2;
    while x < 25 [
        x = x + 1;
        print "so true: " + x;
    ]
    x = add(x, 1500);
    print "x just got 1500 new friends x: " + x;

    let z;

   z = 5.0 + 4.2;
   print z;

   while z <= 10.1 [
        z = z + 0.1;
        print z + " has grown!";
   ]

   if z > 11.1 [
    print "big";
   ] else [
    print "not so tough after all";
   ]
]