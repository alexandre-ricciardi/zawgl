use super::records::*;
use super::store::*;

pub struct DynamicStore {
    dyn_records_store: Store,
}

impl DynamicStore {
    pub fn new(file: &str) -> Self {
        DynamicStore {dyn_records_store: Store::new(file, 129)}
    }
    pub fn save(&mut self, dr: &DynamicStoreRecord) -> u64 {
        self.dyn_records_store.save(&dr_to_bytes(dr))
    }
    pub fn save_data(&mut self, data: &[u8]) -> u64 {
        let mut count = data.len() / 120;
        let rest = data.len() % 120;
        let mut next = 0u64;
        let mut has_next = false;
        let mut end = rest + count * 120;
        loop {
            let mut dr = DynamicStoreRecord {
                in_use: true,
                has_next: has_next,
                data: [0u8; 120],
                next: next
            };
            let len = end - count * 120;
            dr.data[0..len].copy_from_slice(&data[count * 120..end]);
            next = self.save(&dr);
            end = count * 120;
            has_next = true;
            if count == 0 {
                break;
            } else {
                count -= 1;
            }
        }
        next
        
    }

    pub fn load_data(&mut self, id: u64) -> Box<[u8]> {
        let mut data = Vec::new();
        let mut next = id;
        let mut has_next = true;
        while has_next {
            let dr = self.load(next);
            data.extend_from_slice(&dr.data);
            has_next = dr.has_next;
            next = dr.next;
        }
        data.into_boxed_slice()
    }

    pub fn load(&mut self, dr_id: u64) -> DynamicStoreRecord {
        let mut data: [u8; 129] = [0; 129];
        self.dyn_records_store.load(dr_id, &mut data);
        dr_from_bytes(data)
    }
}

#[cfg(test)]
mod test_dyn_store {
    use super::*;
    fn clean(file: &str) {
        std::fs::remove_file(file);
    }
    #[test]
    fn test_dyn_short() {
        clean("C:\\Temp\\dyn.db");
        let mut ds = DynamicStore::new("C:\\Temp\\dyn.db");
        let short = b"qsdfqsdfqsdf";
        let id = ds.save_data(short);
        assert_eq!(id, 0);
        let data = ds.load_data(id);
        assert_eq!(&data[0..12], short);
    }

    #[test]
    fn test_dyn_long() {
        clean("C:\\Temp\\dyn_long.db");
        let mut ds = DynamicStore::new("C:\\Temp\\dyn_long.db");
        let long = b"qsdfqsdfqsdlkqshdfhljbqlcznzelfnqelincqzlnfqzlnec
        qfqsdfqsdfqsdlkqshdfhljbqlcznzelfnqelincqzlnfqzlnecqfqsdfqsdfqsdlkqsh
        dfhljbqlcznzelfnqelincqzlnfqzlnecqfqsdfqsdfqsdlkqshdfhljbqlcznzelfnqel";
        let id = ds.save_data(long);
        assert_eq!(id, 1);
        let data = ds.load_data(id);
        let mut count = long.len() / 32;
        let rest = long.len() % 32;
        let mut low = count * 32;
        let mut high = low + rest;
        loop {
            assert_eq!(&data[low..high], &long[low..high], "bounds {} -> {}", low, high);
            high = count * 32;
            if count > 0 {
                count -= 1;
            } else {
                break;
            }
            low = count * 32;
        }
        
    }
}