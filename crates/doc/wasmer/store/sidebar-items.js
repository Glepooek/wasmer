window.SIDEBAR_ITEMS = {"enum":[["OnCalledAction","After the stack is unwound via asyncify what should the call loop do next"]],"struct":[["Store","The store represents all global state that can be manipulated by WebAssembly programs. It consists of the runtime representation of all instances of functions, tables, memories, and globals that have been allocated during the lifetime of the abstract machine."],["StoreHandle","Handle to an object managed by a context."],["StoreId","Unique ID to identify a context."],["StoreInner","We require the context to have a fixed memory address for its lifetime since various bits of the VM have raw pointers that point back to it. Hence we wrap the actual context in a box."],["StoreMut","A temporary handle to a [`Store`]."],["StoreObjects","Set of objects managed by a context."],["StoreRef","A temporary handle to a [`Store`]."]],"trait":[["AsStoreMut","Helper trait for a value that is convertible to a [`StoreMut`]."],["AsStoreRef","Helper trait for a value that is convertible to a [`StoreRef`]."],["Tunables","An engine delegates the creation of memories, tables, and globals to a foreign implementor of this trait."]],"type":[["OnCalledHandler","Call handler for a store."],["TrapHandlerFn","Function which may handle custom signals while processing traps."]]};