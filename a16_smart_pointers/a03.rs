//-------------------------------------------------RefCell<T>-------------------------------------------------
//there can only be one mutable refrence or multiple immutatble refrence if you try to make multiple but not both 
// and these rules are enforced at the compile time 
// but refcell is a type that enforeces interior immutability pattern
//refcell type is only used when u r sure that you are following the borrow rules but the compiler is not sure abt this
//Rc<T> is only used for single-threaded scenarios and will panic if used for multithreaded scenario
//-------------------------------------------------RECAP-------------------------------------------------
// Rc<T> enables multiple owners of the same data,Box<T> and RefCell<T> have single owners
// Box<T> allows immutable and mutable borrows while Rc<T> allows only immutable borrows checked at compiletime
// because RefCell allows mutable borrows checked at runtime you can mutate the value inside RefCell<T> even when the RefCell<T> is immutable


fn main(){

}