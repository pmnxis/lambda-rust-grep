# lambda_rust_grep

rust study.

## args
```
-F : include some string
-v : exclude some string
```

## example text
```
lambda
rust
restaurant
diva
buy doge coin
buy lambda coin
lambda should be include
coin should be not shown to us.
unhappy coin life
lambda ghost coder
lambda gazuaaaaa
doge is scam, because I am can breeder.
머선127                                 
머선 127 lambda included
머선 C O I N will not contain.
```

## Example Result
```
 master  cat ./test.txt | ./lambda-rust-grep -vF lambda -v coin
lambda
rust
restaurant
diva
lambda should be include
lambda ghost coder
lambda gazuaaaaa
doge is scam, because I am can breeder.
머선127                                 
머선 127 lambda included
머선 C O I N will not contain.
```