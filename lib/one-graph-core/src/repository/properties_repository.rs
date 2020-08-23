use super::store::*;
use super::super::model::*;

pub struct PropertiesRespository {
    prop_store: properties_store::PropertiesStore,
    dyn_store: dynamic_store::DynamicStore,
}

fn compute_prop_name_size(prop: &Property) -> Option<usize> {
    prop.get_name().as_ref().map(|name| name.len())
}

fn compute_prop_size(prop: &Property) -> Option<usize> {
    prop.get_value().as_ref().map(|value| match value {
        PropertyValue::PString(sval) => sval.len(),
        PropertyValue::PInteger(_) => std::mem::size_of::<i64>(),
        PropertyValue::PFloat(_) => std::mem::size_of::<f64>(),
        PropertyValue::PBool(_) => std::mem::size_of::<bool>(),
    }).and_then(|vsize| compute_prop_name_size(prop).map(|nsize| nsize + vsize))
}

fn map_prop_type(prop: &Property) -> Option<u8> {
    prop.get_value().as_ref().map(|value| match value {
        PropertyValue::PString(_) => 0,
        PropertyValue::PInteger(_) => 1,
        PropertyValue::PFloat(_) => 2,
        PropertyValue::PBool(_) => 3,
    })
}

fn is_full_inlined(prop: &Property) -> Option<bool> {
    compute_prop_size(prop).map(|psize| psize < 23)
}

fn is_key_inlined(prop: &Property) -> Option<bool> {
    compute_prop_name_size(prop).map(|psize| psize < 23)
}

fn make_full_inlined_record(prop: &Property) -> Option<records::PropertyRecord> {
    is_full_inlined(prop).and_then(|full| {
        if full {
            prop.get_name().as_ref().and_then(|name| {
                let mut block = [0u8; 24];
                let bytes = name.clone().into_bytes();
                block[0..bytes.len()].copy_from_slice(&bytes);
                let skip = name.len() + 1;
                prop.get_value().as_ref().and_then(|value| {
                    match value {
                        PropertyValue::PString(sval) => block[skip..skip + sval.len()].copy_from_slice(&sval.clone().into_bytes()),
                        PropertyValue::PInteger(ival) => block[skip..skip + std::mem::size_of::<i64>()].copy_from_slice(&ival.to_be_bytes()),
                        PropertyValue::PFloat(fval) => block[skip..std::mem::size_of::<f64>()].copy_from_slice(&fval.to_be_bytes()),
                        PropertyValue::PBool(bval) => block[skip + 2] = *bval as u8,
                    };
                    map_prop_type(prop).map(|ptype| 
                        records::PropertyRecord {
                            in_use: true,
                            key_inlined: false,
                            full_inlined: true,
                            has_next: false,
                            prop_type: ptype,
                            key_id: 0,
                            prop_block: block,
                            next_prop_id: 0,
                        })
                    })
            })
        } else {
            None
        }
    })
    
}

impl PropertiesRespository {
    pub fn new(props_file: &str, dyn_file: &str) -> Self {
        PropertiesRespository {prop_store: properties_store::PropertiesStore::new(props_file), dyn_store: dynamic_store::DynamicStore::new(dyn_file)}
    }

    pub fn create(&mut self, prop: &mut Property) -> Option<()> {
        let prop_id = make_full_inlined_record(prop)
            .or_else(|| self.make_key_inlined_record(prop))
            .or_else(|| self.make_record(prop)).as_mut().map(|r| self.prop_store.create(r))?;
        prop.set_id(prop_id);
        Some(())
    }

    pub fn create_list(&mut self, props: &Vec<Property>) -> Option<u64> {
        let mut vec_records = Vec::new();
        for prop in props {
            let prop_record =  make_full_inlined_record(prop)
            .or_else(|| self.make_key_inlined_record(prop))
            .or_else(|| self.make_record(prop))?;
            vec_records.push(prop_record);
        }
        vec_records.reverse();
        let mut curr_id = 0;
        for pr in &mut vec_records {
            pr.next_prop_id = curr_id;
            curr_id = self.prop_store.create(pr)?;

        }
        Some(curr_id)
    }

    pub fn retrieve_list(&mut self, prop_id: u64) -> Option<Vec<Property>> {
        let mut curr_id = prop_id;
        let mut res = Vec::new();
        while curr_id != 0 {
            let pr = self.prop_store.load(curr_id)?;
            let mut prop = self.make_property(&pr)?;
            prop.set_id(Some(curr_id));
            res.push(prop);
            curr_id = pr.next_prop_id;
        }
        Some(res)
    }

    fn make_record(&mut self, prop: &Property) -> Option<records::PropertyRecord> {
        let value_id = prop.get_value().as_ref().and_then(|val| {
            match val {
                PropertyValue::PString(sval) => self.dyn_store.save_data(&sval.clone().into_bytes()),
                PropertyValue::PInteger(ival) => self.dyn_store.save_data(&ival.to_be_bytes()),
                PropertyValue::PFloat(fval) => self.dyn_store.save_data(&fval.to_be_bytes()),
                PropertyValue::PBool(bval) => self.dyn_store.save_data(&[*bval as u8]),
            }
        });
        let key_id = prop.get_name().as_ref().and_then(|key| {
            self.dyn_store.save_data(&key.clone().into_bytes())
        });
        value_id.and_then(|v_id| {
            key_id.and_then(|key_id| {
                let mut block = [0u8; 24];
                let beg = 0;
                let end = beg + std::mem::size_of::<u64>();
                block[beg..end].copy_from_slice(&v_id.to_be_bytes());
                map_prop_type(prop).map(|ptype| 
                    records::PropertyRecord {
                        in_use: true,
                        key_inlined: false,
                        full_inlined: false,
                        has_next: false,
                        prop_type: ptype,
                        key_id: key_id,
                        prop_block: block,
                        next_prop_id: 0,
                    })
            })
        })
    }

    fn make_key_inlined_record(&mut self, prop: &Property) -> Option<records::PropertyRecord> {
        is_key_inlined(prop).and_then(|key| {
            if key {
                let value_id = prop.get_value().as_ref().and_then(|val| {
                    match val {
                        PropertyValue::PString(sval) => self.dyn_store.save_data(&sval.clone().into_bytes()),
                        PropertyValue::PInteger(ival) => self.dyn_store.save_data(&ival.to_be_bytes()),
                        PropertyValue::PFloat(fval) => self.dyn_store.save_data(&fval.to_be_bytes()),
                        PropertyValue::PBool(bval) => self.dyn_store.save_data(&[*bval as u8]),
                    }
                });

                value_id.and_then(|dr_id| {
                    prop.get_name().as_ref().and_then(|name| {
                        let mut block = [0u8; 24];
                        block[..name.len()].copy_from_slice(&name.clone().into_bytes());
                        let beg = name.len() + 1;
                        let end = beg + std::mem::size_of::<u64>();
                        block[beg..end].copy_from_slice(&dr_id.to_be_bytes());
                        map_prop_type(prop).map(|ptype| 
                            records::PropertyRecord {
                                in_use: true,
                                key_inlined: true,
                                full_inlined: false,
                                has_next: false,
                                prop_type: ptype,
                                key_id: 0,
                                prop_block: block,
                                next_prop_id: 0,
                            })
                    })
                })
            } else {
                None
            }
        })
        
    }
    
    fn make_property(&mut self, pr: &records::PropertyRecord) -> Option<Property> {
        let mut prop = Property::new();
        if pr.full_inlined {
            let name_index = extract_string(&pr.prop_block);
            prop.set_option_name(name_index.1);
            let key_end = name_index.0;
            prop.set_value(extract_value(key_end + 1, pr.prop_type, &pr.prop_block));
        } else if pr.key_inlined {
            let name_index = extract_string(&pr.prop_block);
            prop.set_option_name(name_index.1);
            let value_id = extract_id(name_index.0, &pr.prop_block);
            let data = self.dyn_store.load_data(value_id)?;
            prop.set_value(extract_value(0, pr.prop_type, &data));
        } else {
            let key = self.dyn_store.load_data(pr.key_id)?;
            prop.set_option_name(extract_string(&key).1);
            let value_id = extract_id(0, &pr.prop_block);
            let data = self.dyn_store.load_data(value_id)?;
            prop.set_value(extract_value(0, pr.prop_type, &data));
        }
        Some(prop)
    }

    pub fn load(&mut self, prop_id: u64) -> Option<Property> {
        let pr = self.prop_store.load(prop_id)?;
        let mut prop = self.make_property(&pr)?;
        prop.set_id(Some(prop_id));
        Some(prop)
    }
    
    pub fn sync(&mut self) {
        self.prop_store.sync();
        self.dyn_store.sync();
    }
}

fn extract_string(data: &[u8]) -> (usize, Option<String>) {
    let mut it = data.iter();
    let str_end = it.position(|&c| c == b'\0').unwrap_or(data.len());
    let mut string = Vec::with_capacity(str_end);
    string.extend_from_slice(&data[0..str_end]);
    (str_end,  String::from_utf8(string).ok())
}

fn extract_id(skip: usize, data: &[u8]) -> u64 {
    let mut bytes = [0u8; std::mem::size_of::<u64>()];
    bytes.copy_from_slice(&data[0..std::mem::size_of::<f64>()]);
    u64::from_be_bytes(bytes)
}

fn extract_value(skip: usize, prop_type: u8, data: &[u8]) -> Option<PropertyValue> {
    if prop_type == 0 {
        let mut it = data.iter().skip(skip);
        let value_end = it.position(|&c| c == b'\0').unwrap_or(data.len()) + skip;
        let mut value = Vec::with_capacity(value_end - skip);
        value.extend_from_slice(&data[skip..value_end]);
        String::from_utf8(value).ok().map(|v|PropertyValue::PString(v))
    } else if prop_type == 1 {
        let mut bytes = [0u8; std::mem::size_of::<i64>()];
        bytes.copy_from_slice(&data[skip..std::mem::size_of::<i64>()]);
        Some(PropertyValue::PInteger(i64::from_be_bytes(bytes)))
    } else if prop_type == 2 {
        let mut bytes = [0u8; std::mem::size_of::<f64>()];
        bytes.copy_from_slice(&data[skip..std::mem::size_of::<f64>()]);
        Some(PropertyValue::PFloat(f64::from_be_bytes(bytes)))
    } else if prop_type == 3 {
        Some(PropertyValue::PBool(data[skip + 1] > 0))
    } else {
        None
    }
}


#[cfg(test)]
mod test_prop_repo {
    use super::*;
    use super::super::super::test_utils::*;
    #[test]
    fn test_save_load_0() {
        let dyn_file = build_file_path_and_rm_old("test_save_load_0", "dyn.db").unwrap();
        let prop_file = build_file_path_and_rm_old("test_save_load_0", "prop.db").unwrap();
        let mut pr = PropertiesRespository::new(&prop_file, &dyn_file);
        let mut prop = Property::new();
        prop.set_name("qsfsqdf");
        prop.set_value(Some(PropertyValue::PString(String::from("qgkfdgsdf"))));
        pr.create(&mut prop);
        let load = pr.load(prop.get_id().unwrap()).unwrap();
        assert_eq!(load.get_name(), prop.get_name());
        assert_eq!(load.get_value(), prop.get_value());
    }

    #[test]
    fn test_save_load_1() {
        let dyn_file = build_file_path_and_rm_old("test_save_load_1", "dyn.db").unwrap();
        let prop_file = build_file_path_and_rm_old("test_save_load_1", "prop.db").unwrap();
        let mut pr = PropertiesRespository::new(&prop_file, &dyn_file);
        let mut prop = Property::new();
        prop.set_name("qsfsqdfqsdfq");
        prop.set_value(Some(PropertyValue::PString(String::from("qgkfdgsdf"))));
        pr.create(&mut prop);
        let load = pr.load(prop.get_id().unwrap()).unwrap();
        assert_eq!(load.get_name(), prop.get_name());
        assert_eq!(load.get_value(), prop.get_value());
    }

    #[test]
    fn test_save_load_2() {
        let dyn_file = build_file_path_and_rm_old("test_save_load_2", "dyn.db").unwrap();
        let props_file = build_file_path_and_rm_old("test_save_load_2", "prop.db").unwrap();
        let mut pr = PropertiesRespository::new(&props_file, &dyn_file);
        let mut prop = Property::new();
        prop.set_name("qsfsqdfqsdfqdhgfdhgdfhgdfhqzerqzerqzregdfqsfdqsfderhryjsrrefqzeqgdsfdfsdrrdsredfsqer");
        prop.set_value(Some(PropertyValue::PString(String::from("qgkfdgsdfqerqzerqzerqzerqzerqzerqzerarthdtrsdqeqtrshsreqsgstreq"))));
        pr.create(&mut prop);
        let load = pr.load(prop.get_id().unwrap()).unwrap();
        assert_eq!(load.get_name(), prop.get_name());
        assert_eq!(load.get_value(), prop.get_value());
    }
}