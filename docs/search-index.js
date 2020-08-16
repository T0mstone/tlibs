var searchIndex = JSON.parse('{\
"bi_result":{"doc":"This crate provides the `BiResult` struct.","i":[[3,"BiResult","bi_result","A `Result`-like struct that always contains a value, and…",null,null],[12,"0","","",0,null],[12,"1","","",0,null],[8,"ResultExt","","Extensions to the `Result` type",null,null],[10,"into_bi_result","","Convert a `Result` to a `BiResult`",1,[[],["biresult",3]]],[10,"push_error","","push the error in an `Err(_)` onto a collection, returning…",1,[[],["option",4]]],[11,"push_error_or_default","","push the error in an `Err(_)` onto a collection, returning…",1,[[]]],[11,"ok","","Creates a new `BiResult` with `t` as value and the default…",0,[[]]],[11,"err","","Creates a new `BiResult` with `e` as errors and the…",0,[[]]],[11,"map","","Maps a `BiResult<T, I>` to `BiResult<U, I>` by applying a…",0,[[["fnonce",8]],["biresult",3]]],[11,"map_err","","Maps a `BiResult<T, I>` to `BiResult<T, V>` by applying a…",0,[[["fnonce",8]],[["biresult",3],["intoiterator",8]]]],[11,"map_each_err","","Maps a `BiResult<T, I>` to `BiResult<T, ...>` by applying…",0,[[["fnmut",8]],[["biresult",3],["map",3]]]],[11,"join","","Composes two `BiResult`s by applying `f` to unify their…",0,[[["biresult",3],["intoiterator",8],["fnonce",8]],[["biresult",3],["chain",3]]]],[11,"consume_err","","Converts a `Result` to an `Option` by appending any `Err`…",0,[[["result",4]],["option",4]]],[11,"and_then","","Composes the result of applying `f` onto the value of…",0,[[["fnonce",8]],[["biresult",3],["chain",3]]]],[11,"push_errs","","Extend `target` with all errors and return only the value.…",0,[[]]],[11,"push_errs_with","","Extend `target` with all errors (applying `Into::into`)…",0,[[["fnmut",8]]]],[11,"push_errs_with_into","","Extend `target` with all errors (applying `Into::into`)…",0,[[]]],[11,"expect","","returns `self.0`, panics with the provided message if…",0,[[]]],[11,"unwrap","","Like `expect`, but with a default error message",0,[[]]],[11,"from","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"eq","","",0,[[["biresult",3]]]],[11,"ne","","",0,[[["biresult",3]]]],[11,"hash","","",0,[[]]],[11,"from_iter","","",0,[[["intoiterator",8]]]],[11,"clone","","",0,[[],["biresult",3]]],[11,"default","","",0,[[],["biresult",3]]]],"p":[[3,"BiResult"],[8,"ResultExt"]]},\
"cargo_pkg_info":{"doc":"This crate provides some data structures for working with…","i":[[3,"CargoPackageInfo","cargo_pkg_info","Information about a Cargo package",null,null],[4,"CargoProfile","","A Cargo compilation profile",null,null],[13,"Debug","","Cargo\'s `debug` profile",0,null],[13,"Release","","Cargo\'s `release` profile",0,null],[5,"cargo_pkg_version","","Get the package version for your cargo package",null,[[],["version",3]]],[11,"current","","Get the info for your cargo package",1,[[]]],[11,"version","","",1,[[],["version",3]]],[11,"name","","",1,[[]]],[11,"authors","","",1,[[]]],[11,"description","","",1,[[]]],[11,"homepage","","",1,[[]]],[11,"repository","","",1,[[]]],[11,"current","","The profile you are compiling with",0,[[]]],[11,"from","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"from","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"eq","","",1,[[["cargopackageinfo",3]]]],[11,"ne","","",1,[[["cargopackageinfo",3]]]],[11,"eq","","",0,[[["cargoprofile",4]]]],[11,"hash","","",1,[[]]],[11,"hash","","",0,[[]]],[11,"clone","","",1,[[],["cargopackageinfo",3]]],[11,"clone","","",0,[[],["cargoprofile",4]]]],"p":[[4,"CargoProfile"],[3,"CargoPackageInfo"]]},\
"direction":{"doc":"","i":[[3,"Side","direction","",null,null],[3,"Height","","",null,null],[3,"Depth","","",null,null],[4,"Axis","","",null,null],[13,"Side","","",0,null],[13,"Height","","",0,null],[13,"Depth","","",0,null],[4,"BinaryDirection","","",null,null],[13,"Regular","","",1,null],[13,"Inverted","","",1,null],[0,"ternary","","",null,null],[3,"Side","direction::ternary","",null,null],[3,"Height","","",null,null],[3,"Depth","","",null,null],[4,"TernaryDirection","","",null,null],[13,"Regular","","",2,null],[13,"Middle","","",2,null],[13,"Inverted","","",2,null],[18,"Right","","",3,null],[18,"Center","","",3,null],[18,"Left","","",3,null],[18,"AXIS","","",3,null],[18,"Top","","",4,null],[18,"Center","","",4,null],[18,"Bottom","","",4,null],[18,"AXIS","","",4,null],[18,"Front","","",5,null],[18,"Center","","",5,null],[18,"Back","","",5,null],[18,"AXIS","","",5,null],[18,"Right","direction","",6,null],[18,"Left","","",6,null],[18,"AXIS","","",6,null],[18,"Top","","",7,null],[18,"Bottom","","",7,null],[18,"AXIS","","",7,null],[18,"Front","","",8,null],[18,"Back","","",8,null],[18,"AXIS","","",8,null],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","direction::ternary","",3,[[]]],[11,"into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]]],"p":[[4,"Axis"],[4,"BinaryDirection"],[4,"TernaryDirection"],[3,"Side"],[3,"Height"],[3,"Depth"],[3,"Side"],[3,"Height"],[3,"Depth"]]},\
"growable_iter":{"doc":"This crate provides the `GrowableIterator` struct.","i":[[3,"GrowableIterator","growable_iter","An iterator that can grow to both sides after creation",null,null],[8,"Growable","","A trait to create a `GrowableIterator` from a regular…",null,null],[11,"growable","","creates a `GrowableIterator` from `self`, `front` and `back`",0,[[],["growableiterator",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"into_iter","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"next_back","","",1,[[],["option",4]]],[11,"next","","",1,[[],["option",4]]],[11,"push_front","","",1,[[],["result",4]]],[11,"push_back","","",1,[[],["result",4]]],[11,"peek_front","","",1,[[],["option",4]]],[11,"peek_back","","",1,[[],["option",4]]]],"p":[[8,"Growable"],[3,"GrowableIterator"]]},\
"infinite_iterators":{"doc":"`infinite_iterators`: Iterators that never end","i":[[3,"FromFn","infinite_iterators","",null,null],[3,"Successors","","",null,null],[3,"Decaying","","",null,null],[5,"from_fn","","",null,[[["fnmut",8]],[["fromfn",3],["fnmut",8]]]],[5,"successors","","",null,[[["fnmut",8]],[["fnmut",8],["successors",3]]]],[5,"decaying","","",null,[[["default",8]],[["default",8],["decaying",3]]]],[5,"true_once","","",null,[[]]],[0,"iters","","",null,null],[3,"Chain","infinite_iterators::iters","",null,null],[3,"StepBy","","",null,null],[3,"Take","","",null,null],[3,"Inf","","A struct that converts between `Iterator` and…",null,null],[8,"PromiseInfinite","infinite_iterators","Promote an iterator to an infinite iterator",null,null],[11,"promise_infinite","","",0,[[],["inf",3]]],[8,"ChainInfinite","","",null,null],[11,"chain_infinite","","",1,[[["infiniteiterator",8]],[["chain",3],["infiniteiterator",8]]]],[8,"InfiniteIterator","","An iterator that never ends",null,null],[16,"Item","","",2,null],[10,"next","","",2,[[]]],[11,"nth","","",2,[[]]],[11,"step_by","","",2,[[],["stepby",3]]],[11,"take","","",2,[[],["take",3]]],[11,"iterator","","",2,[[],["inf",3]]],[11,"from","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"into","","",4,[[]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","infinite_iterators::iters","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into_iter","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"into","","",8,[[]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"into_iter","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"into","","",9,[[]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"next","","",6,[[]]],[11,"next","","",7,[[]]],[11,"next","","",9,[[]]],[11,"next","infinite_iterators","",3,[[]]],[11,"next","","",4,[[]]],[11,"next","","",5,[[]]],[11,"next","infinite_iterators::iters","",8,[[],["option",4]]],[11,"next","","",9,[[],["option",4]]]],"p":[[8,"PromiseInfinite"],[8,"ChainInfinite"],[8,"InfiniteIterator"],[3,"FromFn"],[3,"Successors"],[3,"Decaying"],[3,"Chain"],[3,"StepBy"],[3,"Take"],[3,"Inf"]]},\
"map_range":{"doc":"This crate provides the `MapRange` struct.","i":[[3,"MapRange","map_range","A struct for mapping ranges linearly",null,null],[11,"from_start_end","","Construct the map from two start-end pairs of values",0,[[]]],[11,"from_start_len","","Construct the map from two start-length pairs of values",0,[[]]],[11,"eval","","Evaluate the map for a point.",0,[[]]],[11,"into_eval","","Evaluate the map for a point.",0,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"clone","","",0,[[],["maprange",3]]],[11,"eq","","",0,[[["maprange",3]]]],[11,"ne","","",0,[[["maprange",3]]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"hash","","",0,[[]]]],"p":[[3,"MapRange"]]},\
"nonempty_vec":{"doc":"This crate provides a `Vec`-like struct that cannot be empty","i":[[3,"NonemptyVec","nonempty_vec","A `Vec` that always has at least one element",null,null],[4,"HeadFirst","","The head item is in front of the rest (at index `0`)",null,null],[4,"HeadLast","","The head item is after the rest (at index `len-1`)",null,null],[8,"HeadLocation","","Specifies the location the head item has in relation to…",null,null],[18,"HEAD_FIRST","","This constant can be used to write code generic over…",0,null],[11,"new","","Creates a new `NonemptyVec` with one element",1,[[]]],[11,"head","","The `head` is simply the element guaranteed to exist. It…",1,[[]]],[11,"head_mut","","Like `head` but mutable",1,[[]]],[11,"into_head","","Consumes `self` and returns the head",1,[[]]],[11,"tail","","The `tail` is simply all elements that are not guaranteed…",1,[[]]],[11,"tail_mut","","Like `tail` but mutable",1,[[],["vec",3]]],[11,"into_tail","","Consumes `self` and returns the tail",1,[[],["vec",3]]],[11,"into_head_tail","","Consumes `self` and returns the head and the tail",1,[[]]],[11,"len","","Returns the length of `self`",1,[[],["nonzerousize",3]]],[11,"push","","Pushes an element to the end of `self`",1,[[]]],[11,"pop","","Removes the last element from `self` (unless `self` has…",1,[[],["option",4]]],[11,"insert","","Inserts an element at the specified index",1,[[]]],[11,"remove","","Removes an element from the specified index; Returns…",1,[[],["option",4]]],[11,"into_vec","","Creates a `Vec` from all its elements",1,[[],["vec",3]]],[11,"first","","The first element",1,[[]]],[11,"first_mut","","The first element, mutable",1,[[]]],[11,"into_first","","Returns the current first element, consuming `self`",1,[[]]],[11,"last","","The last element",1,[[]]],[11,"last_mut","","The last element, mutable",1,[[]]],[11,"into_last","","Returns the current last element, consuming `self`",1,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"into_iter","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"into_iter","","",1,[[]]],[11,"clone","","",1,[[],["nonemptyvec",3]]],[11,"default","","",1,[[],["nonemptyvec",3]]],[11,"eq","","",1,[[["nonemptyvec",3]]]],[11,"ne","","",1,[[["nonemptyvec",3]]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]]],"p":[[8,"HeadLocation"],[3,"NonemptyVec"],[4,"HeadFirst"],[4,"HeadLast"]]},\
"some_macros":{"doc":"This crate provides some macros","i":[[14,"count_args","some_macros","Counts how many arguments it receives.",null,null],[14,"vec_deque","","Constructs a VecDeque, calling `into` on the keys and values",null,null],[14,"hash_map","","Constructs a HashMap, calling `into` on the keys and values",null,null],[14,"hash_set","","Constructs a HashSet, calling `into` on the values",null,null],[14,"dbgr","","A macro to print pretty looking and informative debug logs.",null,null],[14,"debug_lvl","","Returns a boolean that tells you whether a specific…",null,null],[14,"alt","","A macro that returns the first set of token trees it gets",null,null]],"p":[]},\
"span":{"doc":"This crate provides the `Span` struct.","i":[[3,"Span","span","A `Span` is basically like a `Range<usize>` but it has…",null,null],[11,"new","","Creates a new `Span` from a start position and a length",0,[[]]],[11,"from_range","","Creates a new `Span` from a start position and a length",0,[[["range",3]]]],[11,"start","","The start position",0,[[]]],[11,"len","","The length",0,[[]]],[11,"is_empty","","Determines wheter the `Span` is empty, i.e. of length 0",0,[[]]],[11,"end","","The end position (the one-after-end index)",0,[[]]],[11,"as_range","","Converts the `Span` to its equivalent `Range<usize>`",0,[[],["range",3]]],[11,"contains","","Returns whether `other` is contained within `self` (this…",0,[[]]],[11,"join","","Create a (the smallest) span that contains both `self` and…",0,[[]]],[11,"from","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",0,[[["range",3]]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"eq","","",0,[[["span",3]]]],[11,"ne","","",0,[[["span",3]]]],[11,"into","","",0,[[],["range",3]]],[11,"clone","","",0,[[],["span",3]]]],"p":[[3,"Span"]]},\
"type_list":{"doc":"Heterogeneous lists, with or without values (the latter…","i":[[0,"het_list","type_list","A heterogeneous list",null,null],[3,"Nil","type_list::het_list","The empty heterogeneous list",null,null],[3,"Cons","","The heterogeneous list with a head of type `T` and a tail…",null,null],[5,"cons","","a helper function to ease the creation of new…",null,[[["hetlist",8]],[["hetlist",8],["cons",3]]]],[8,"HetList","","The core trait that any heterogeneous list implements",null,null],[16,"Head","","The type of the first element",0,null],[16,"Tail","","The type of the rest of the list",0,null],[18,"LEN","","The length of the list",0,null],[10,"head","","The first element",0,[[]]],[10,"tail","","The rest of the list",0,[[]]],[10,"head_mut","","The first element mutable",0,[[]]],[10,"tail_mut","","The rest of the list mutable",0,[[]]],[10,"into_head","","The first element, consuming `self`",0,[[]]],[10,"into_tail","","The rest of the list, consuming `self`",0,[[]]],[10,"into_head_tail","","The first element and the rest of the list, consuming `self`",0,[[]]],[8,"AppendItem","","A function-trait. Its `Output` type is the list that…",null,null],[16,"Output","","The list that represents `[..., T]` where `...` are all…",1,null],[10,"append","","appends an element to the end of the list",1,[[]]],[0,"type_list","type_list","A list consisting purely of types",null,null],[3,"Cons","type_list::type_list","`Cons<T, ...>` represents the list `[T, ...]` where `...`…",null,null],[4,"Nil","","The empty type list",null,null],[6,"Append","","An alias to append a type to the end of a type list",null,null],[8,"TypeList","","The core trait that any type list implements",null,null],[16,"Head","","The first item of the list",2,null],[16,"Tail","","The rest of the list",2,null],[18,"LEN","","The length of the list",2,null],[8,"AppendType","","A function-trait. Its `Output` type is the list that…",null,null],[16,"Output","","The list that represents `[..., T]` where `...` are all…",3,null],[14,"het_list","type_list","A macro that creates a heterogeneous list. The syntax is…",null,null],[14,"type_list","","A macro that creates a type list. The syntax is like…",null,null],[11,"from","type_list::het_list","",4,[[]]],[11,"into","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","type_list::type_list","",6,[[]]],[11,"into","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"head","type_list::het_list","",4,[[]]],[11,"tail","","",4,[[]]],[11,"head_mut","","",4,[[]]],[11,"tail_mut","","",4,[[]]],[11,"into_head","","",4,[[]]],[11,"into_tail","","",4,[[]]],[11,"into_head_tail","","",4,[[]]],[11,"head","","",5,[[]]],[11,"tail","","",5,[[]]],[11,"head_mut","","",5,[[]]],[11,"tail_mut","","",5,[[]]],[11,"into_head","","",5,[[]]],[11,"into_tail","","",5,[[]]],[11,"into_head_tail","","",5,[[]]],[11,"append","","",4,[[]]],[11,"append","","",5,[[]]]],"p":[[8,"HetList"],[8,"AppendItem"],[8,"TypeList"],[8,"AppendType"],[3,"Nil"],[3,"Cons"],[3,"Cons"],[4,"Nil"]]},\
"vec_like":{"doc":"This crate provides some traits that capture behaviour…","i":[[8,"PushFront","vec_like","Add an element to the front of the list",null,null],[16,"Err","","The type of the failure value",0,null],[10,"push_front","","Try to add an element to the front of the list.",0,[[],["result",4]]],[8,"PushBack","","Add an element to the back of the list",null,null],[16,"Err","","The type of the failure value",1,null],[10,"push_back","","Try to add an element to the back of the list.",1,[[],["result",4]]],[8,"PopFront","","Remove an element from the front of the list",null,null],[10,"pop_front","","Try to remove an element from the front of the list.",2,[[],["option",4]]],[8,"PopBack","","Remove an element from the back of the list",null,null],[10,"pop_back","","Try to remove an element from the back of the list.",3,[[],["option",4]]],[8,"PeekFront","","Get the element at the front of the list",null,null],[10,"peek_front","","Try to get the element at the front of the list.",4,[[],["option",4]]],[8,"PeekBack","","Get the element at the back of the list",null,null],[10,"peek_back","","Try to get the element at the back of the list.",5,[[],["option",4]]]],"p":[[8,"PushFront"],[8,"PushBack"],[8,"PopFront"],[8,"PopBack"],[8,"PeekFront"],[8,"PeekBack"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);