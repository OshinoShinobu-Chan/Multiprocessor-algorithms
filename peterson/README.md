# Peterson's Algorithm
This the a well known algorithm valid for **two** threads mutual exclusion.

## Main Part
There is a flag and a victim label as the main part of this algorithm. Any thread who
wants to get into the critical section must raise its flag and write its id to the victim
label. 

Thread only enter the critical section if another thread do not want to get into the 
critical section (flag isn't raised) or it is not on victim label.

## Correctness
Peterson's Algorithm satisfies mutual exclusion and is starvation-free and deadlock-free.

I won't prove the correctness here, but I would like to give me own understanding of the 
correctness of the algorithm here. The label here in fact gives an order of the the two
threads and the thread who reach the label first can enter the critical seciton. Later in
Bakery algorithm you can see it is also using a method to give an order to all of the threads
who want to enter the critical section.

## More about Peterson's Algorithm
By generalizing peterson's algorithm we can get filter lock which is valid to ambitary number 
of threads. 

Filter lock use `n-1` 'waiting room', where `n` is the number of threads. A peterson-like 
algorithm is used to pick **one** thread out of the next 'waiting room'. Only the one whe get 
through all the 'waiting room' is allowed to enter critial seciton.