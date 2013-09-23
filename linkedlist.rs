struct Node { 
	value: uint, 
	/*make sure that the list can terminate*/
	next: Option<~Node> 
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
     *  before giving them out into the loop body
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
}