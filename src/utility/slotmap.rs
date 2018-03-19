pub struct IndexToken {
    index: usize,
}

impl IndexToken {
    /// Creates an invalid token. If this token is used, a panic is thrown.
    pub fn invalid() -> IndexToken {
        IndexToken { index: !0 }
    }

    fn validate(&self) {
        if self.index == !0 {
            panic!("Attempted to use an invalid token.");
        }
    }
}

pub struct Slot {
    to_data: usize,
    to_map: usize,
}

pub struct IndexMap {
    table: Vec<Slot>,
    data_len: usize,
    free: Vec<usize>,
}

impl IndexMap {
    const DEFAULT_CAPACITY: usize = 128;

    pub fn new() -> IndexMap {
        IndexMap {
            table: Vec::with_capacity(IndexMap::DEFAULT_CAPACITY),
            data_len: 0,
            free: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data_len
    }

    pub fn add(&mut self) -> IndexToken {
        let index = match self.free.pop() {
            Some(index) => {
                self.table[index].to_data = self.data_len;
                self.table[self.data_len].to_map = index;
                index
            },
            None => {
                self.table.push(Slot {
                    to_data: self.data_len,
                    to_map: self.data_len,
                });
                self.data_len
            },
        };
        self.data_len += 1;
        IndexToken { index: index }
    }

    pub fn remove(&mut self, token: IndexToken) -> usize {
        token.validate();
        self.free.push(token.index);
        let data_index = self.table[token.index].to_data;
        self.data_len -= 1;
        if data_index < self.data_len {
            let map_index = self.table[self.data_len].to_map;
            self.table[data_index].to_map = map_index;
            self.table[map_index].to_data = data_index;
        }
        data_index
    }

    pub fn get(&self, token: &IndexToken) -> usize {
        token.validate();
        unsafe { self.table.get_unchecked(token.index).to_data as usize }
    }
}

pub struct SlotMap<T> {
    table: Vec<Slot>,
    data: Vec<T>,
    free: Vec<usize>,
}

impl<T> SlotMap<T> {
    const DEFAULT_CAPACITY: usize = 128;

    pub fn new() -> SlotMap<T> {
        SlotMap {
            table: Vec::with_capacity(SlotMap::<T>::DEFAULT_CAPACITY),
            data: Vec::with_capacity(SlotMap::<T>::DEFAULT_CAPACITY),
            free: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn add(&mut self, value: T) -> IndexToken {
        let index = match self.free.pop() {
            Some(index) => {
                self.table[index].to_data = self.data.len();
                self.table[self.data.len()].to_map = index;
                index
            },
            None => {
                self.table.push(Slot {
                    to_data: self.data.len(),
                    to_map: self.data.len(),
                });
                self.data.len()
            },
        };
        self.data.push(value);
        IndexToken { index: index }
    }

    pub fn remove(&mut self, token: IndexToken) -> T {
        token.validate();
        self.free.push(token.index);
        let data_index = self.table[token.index].to_data;
        let data_length = self.data.len() - 1;
        if data_index < data_length {
            let map_index = self.table[data_length].to_map;
            self.table[data_index].to_map = map_index;
            self.table[map_index].to_data = data_index;
        }
        self.data.swap_remove(data_index)
    }

    pub fn get(&self, token: &IndexToken) -> &T {
        token.validate();
        unsafe {
            &self.data
                .get_unchecked(self.table.get_unchecked(token.index).to_data)
        }
    }
}
