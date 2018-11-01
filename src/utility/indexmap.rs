#[derive(Copy, Clone)]
pub struct IndexToken {
    index: u32,
    version: u32,
}

impl IndexToken {
    /// Creates an invalid token. If this token is used, a panic is thrown.
    pub fn invalid() -> IndexToken {
        IndexToken { index: 0, version: 0 }
    }
}

impl Default for IndexToken {
    fn default() -> IndexToken {
        IndexToken::invalid()
    }
}

struct Slot {
    to_data: usize,
    to_map: usize,
    version: u32,
}

pub struct IndexMap {
    table: Vec<Slot>,
    data_len: usize,
    free_len: usize,
}

impl IndexMap {
    const DEFAULT_CAPACITY: usize = 128;

    pub fn new() -> IndexMap {
        IndexMap {
            table: Vec::with_capacity(IndexMap::DEFAULT_CAPACITY),
            data_len: 0,
            free_len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data_len
    }

    pub fn is_empty(&self) -> bool {
        self.data_len == 0
    }

    pub fn clear(&mut self) {
        let mut counter = 0;
        for slot in self.table.iter_mut() {
            slot.version += 1;
            slot.to_data = counter;
            slot.to_map = counter;
            counter += 1;
        }
        self.data_len = 0;
        self.free_len = self.table.len();
    }

    pub fn add(&mut self) -> IndexToken {
        // Calculate the index that we're inserting the new index at
        let (index, version) = if self.free_len > 0 {
            // Use an existing table slot since one is free. Get the index of
            // the free slot, decease the free slot length.
            self.free_len -= 1;
            let index = self.table[self.data_len].to_map;
            // Swap in the new data
            self.table[index].to_data = self.data_len;
            self.table[self.data_len].to_map = index;
            (index, self.table[index].version)
        } else {
            // Expand the table and create a new slot.
            self.table.push(Slot {
                to_data: self.data_len,
                to_map: self.data_len,
                version: 1,
            });
            (self.data_len, 1)
        };
        // Increment the number of values stored in the map.
        self.data_len += 1;
        // Return the token
        IndexToken {
            index: index as u32,
            version: version,
        }
    }

    /// Returns the index that was removed.
    pub fn remove(&mut self, token: IndexToken) -> usize {
        let index = token.index as usize;
        // Token validation.
        if token.version != self.table[index].version {
            panic!("Unable to remove: token version does not match.");
        }
        // Update slot version to invalidate old tokens.
        self.table[index].version += 1;
        // Removal process.
        self.data_len -= 1;
        let data_index = self.table[index].to_data;
        if data_index < self.data_len {
            // If the index we're removing is at the end of the list, we're
            // done already. If we're not at the end, the shuffling is done
            // here.
            let map_index = self.table[self.data_len].to_map;
            self.table[data_index].to_map = map_index;
            self.table[map_index].to_data = data_index;
        }
        // Free slot book-keeping
        self.table[self.data_len].to_map = index;
        self.free_len += 1;
        // Return the removed index
        data_index
    }

    pub fn get(&self, token: IndexToken) -> usize {
        let index = token.index as usize;
        // Token validation
        if token.version != self.table[index].version {
            panic!("Unable to get: token version does not match.");
        }
        // Retrieval
        unsafe { self.table.get_unchecked(index).to_data }
    }
}
