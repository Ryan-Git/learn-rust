mod ch15_1_box;
mod ch15_4_rc;
mod ch15_5_ref_cell;

//Box<T>, for allocating values on the heap
//Rc<T>, a reference counted type so data can have multiple owners
//RefCell<T>, which isn't a smart pointer itself, but manages access to the smart pointers Ref and RefMut to enforce the borrowing rules at runtime instead of compile time