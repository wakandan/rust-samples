struct Node { 
	value: uint, 
	/*make sure that the list can terminate*/
	next: Option<~Node> 
}

/*print a linked list, recursively. Simple example of pattern matching*/
fn print_linkedlist(head: Option<~Node>){
    match head {
        Some(~x) => {
            print(fmt!("%? --> ", x.value));
            print_linkedlist(x.next);
        },
        None => {
            println("*");
        }
    }
}

/*Reverse a linked-list in-place. Notice that after this operation, the 
* head pointer is _moved_*/
fn reverse_linkedlist(head: Option<~Node>) -> Option<~Node> {       
    let mut current_head = head;
    let mut return_head = None;
    
    loop {
        /*if we just use current_head here, the below won't work
        * as it will be marked as immutable. take() basically return 
        * the `node` inside the Option<node> and assign `None` to that
        * variable. Here's we're matching that value*/
        match current_head.take() {
            Some(node) => {
                /*make the node mutable, it seemed weird to me at first*/
                let mut node = node;
                /*advance the temporary head pointer. take() here
                 * is required. If next() is not used, current_head 
                 * will be assigned a copy of node.next(), but the 
                 * type of node.next is not copy-able (as showned 
                 * in the compile message). Here we assigning 
                 * the _value_ of the pointer*/
                current_head = node.next.take();
                /*make the `next` pointer point backward*/
                node.next = return_head;

                return_head = Some(node);
            }
            None => {
                break; // we're done
            }
        }
    }
    //return the result over here, notice it's just the variable name 
    return_head
}


fn main() {	
    /*define a vector of numbers*/
	let v = [1u,2,3,4]; 

	/*For every type T, and value of type Option<T> has 
     * two possible values: None or Some(T). head's type 
     * is forward-inferred from (2) by compiler. To be more 
     * explicit: 
     *      let mut head: Option<~Node> = None::<~Node>; 
     *
     * CARE: None is not the same as null in java/javascript. It's 
     * actually an enum member
     * */
	let mut head = None; 

    /* .iter() to make the array iterable by a for loop
     *
     * .invert() to iterate from the back. The list is not 
     *  actually reverse, the for loop just iterate backward
     * 
     * .map(|x| *x) - the for loop will chunk out references of variables
     *  in the original list. This is just to dereference each of them
     *  before giving them out into the loop body. The dereference here 
     *  only works with implicitly copyable types, but let's just keep 
     *  it simple for now
     * */
	for e in v.iter().invert().map(|x| *x) { 
        /* take a minute and think about it, how are you going to 
         * initialize the head of this linked list?
         *
         * I was clueless because I didn't know what to 
         * assign for an initial reference if there's no "null" value 
         * to check on.
         *
         * .take() will return the value of the Option, and replace 
         * it with an None. If it's already None to being with, 
         * that's not an problem, it just returns None
         *  */
		let old_head = head.take(); //(1) IMPORTANT

        /* this part should be straighforward, this code is creating 
         * a node and _prepending_ the new node in front of the 
         * previous node. (remembe we _revert_ the list in the for loop?)
         *
         * head should be "Some" so that it could have "None" 
         * as an inital value 
         * */
		head = Some(~Node{ value: e, next: old_head }); //(2)
	}

    print_linkedlist(head);
}
