// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::buf_config::PROPERTY_BLOCK_SIZE;

use super::store::*;
use super::super::model::*;

pub struct PropertiesRespository {
    prop_store: properties_store::PropertiesStore,
    dyn_store: dynamic_store::DynamicStore,
}

fn compute_prop_name_size(prop: &Property) -> Option<usize> {
    Some(prop.get_name().len())
}

fn compute_prop_size(prop: &Property) -> Option<usize> {
    let vsize = match prop.get_value() {
        PropertyValue::PString(sval) => sval.len(),
        PropertyValue::PInteger(_) => std::mem::size_of::<i64>(),
        PropertyValue::PFloat(_) => std::mem::size_of::<f64>(),
        PropertyValue::PBool(_) => std::mem::size_of::<bool>(),
        PropertyValue::PUInteger(_) => std::mem::size_of::<u64>(),
    };
    compute_prop_name_size(prop).map(|nsize| nsize + vsize)
}

fn map_prop_type(prop: &Property) -> Option<u8> {
    Some(match prop.get_value() {
        PropertyValue::PString(_) => 0,
        PropertyValue::PInteger(_) => 1,
        PropertyValue::PFloat(_) => 2,
        PropertyValue::PBool(_) => 3,
        PropertyValue::PUInteger(_) => 4,
    })
}

fn is_full_inlined(prop: &Property) -> Option<bool> {
    compute_prop_size(prop).map(|psize| psize < PROPERTY_BLOCK_SIZE -1 -8)
}

fn is_key_inlined(prop: &Property) -> Option<bool> {
    compute_prop_name_size(prop).map(|psize| psize < PROPERTY_BLOCK_SIZE -1 -8)
}

fn make_full_inlined_record(prop: &Property) -> Option<records::PropertyRecord> {
    is_full_inlined(prop).and_then(|full| {
        if full {
            let name = prop.get_name();
            let mut block = [0u8; 24];
            let bytes = String::from(name).into_bytes();
            block[0..bytes.len()].copy_from_slice(&bytes);
            let skip = name.len() + 1;
          
            match prop.get_value() {
                PropertyValue::PString(sval) => block[skip..skip + sval.len()].copy_from_slice(&sval.clone().into_bytes()),
                PropertyValue::PInteger(ival) => block[skip..skip + std::mem::size_of::<i64>()].copy_from_slice(&ival.to_be_bytes()),
                PropertyValue::PUInteger(uval) => block[skip..skip + std::mem::size_of::<u64>()].copy_from_slice(&uval.to_be_bytes()),
                PropertyValue::PFloat(fval) => block[skip..skip + std::mem::size_of::<f64>()].copy_from_slice(&fval.to_be_bytes()),
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

    pub fn create_list(&mut self, props: &mut Vec<Property>) -> Option<u64> {
        let mut vec_records = Vec::new();
        for prop in props.iter() {
            let prop_record =  make_full_inlined_record(prop)
            .or_else(|| self.make_key_inlined_record(prop))
            .or_else(|| self.make_record(prop))?;
            vec_records.push(prop_record);
        }
        vec_records.reverse();
        let mut curr_id = 0;
        let mut ids = Vec::new();
        for pr in &mut vec_records {
            pr.next_prop_id = curr_id;
            curr_id = self.prop_store.create(pr)?;
            ids.push(curr_id);
        }
        ids.reverse();
        let mut index = 0;
        for prop in props {
            prop.set_id(ids.get(index).map(|id| *id));
            index += 1;
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
        let value_id = 
            match prop.get_value() {
                PropertyValue::PString(sval) => self.dyn_store.save_data(&sval.clone().into_bytes()),
                PropertyValue::PInteger(ival) => self.dyn_store.save_data(&ival.to_be_bytes()),
                PropertyValue::PUInteger(uval) => self.dyn_store.save_data(&uval.to_be_bytes()),
                PropertyValue::PFloat(fval) => self.dyn_store.save_data(&fval.to_be_bytes()),
                PropertyValue::PBool(bval) => self.dyn_store.save_data(&[*bval as u8]),
            };
        let key_id = self.dyn_store.save_data(&String::from(prop.get_name()).into_bytes());
        value_id.and_then(|v_id| {
            key_id.and_then(|key_id| {
                let mut block = [0u8; PROPERTY_BLOCK_SIZE];
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
                let value_id = match prop.get_value() {
                    PropertyValue::PString(sval) => self.dyn_store.save_data(&sval.clone().into_bytes()),
                    PropertyValue::PInteger(ival) => self.dyn_store.save_data(&ival.to_be_bytes()),
                    PropertyValue::PUInteger(uval) => self.dyn_store.save_data(&uval.to_be_bytes()),
                    PropertyValue::PFloat(fval) => self.dyn_store.save_data(&fval.to_be_bytes()),
                    PropertyValue::PBool(bval) => self.dyn_store.save_data(&[*bval as u8]),
                };

                value_id.and_then(|dr_id| {
                    let mut block = [0u8; PROPERTY_BLOCK_SIZE];
                    block[..prop.get_name().len()].copy_from_slice(&String::from(prop.get_name()).into_bytes());
                    let beg = prop.get_name().len() + 1;
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
            } else {
                None
            }
        })
        
    }
    
    fn make_property(&mut self, pr: &records::PropertyRecord) -> Option<Property> {
        if pr.full_inlined {
            let name_index = extract_string(&pr.prop_block)?;
            let key_end = name_index.0;
            Some(Property::new(name_index.1, extract_value(key_end, pr.prop_type, &pr.prop_block)?))
        } else if pr.key_inlined {
            let (id_index, name_index) = extract_string(&pr.prop_block)?;
            let value_id = extract_id_from(id_index, &pr.prop_block);
            let data = self.dyn_store.load_data(value_id)?;
            Some(Property::new_with_id(value_id, name_index, extract_value(0, pr.prop_type, &data)?))
        } else {
            let key = self.dyn_store.load_data(pr.key_id)?;
            let name = extract_string(&key)?.1;
            let value_id = extract_id(&pr.prop_block);
            let data = self.dyn_store.load_data(value_id)?;
            Some(Property::new_with_id(value_id, name, extract_value(0, pr.prop_type, &data)?))
        }
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
    pub fn clear(&mut self) {
        self.prop_store.clear();
        self.dyn_store.clear();
    }
}

fn extract_string(data: &[u8]) -> Option<(usize, String)> {
    let mut it = data.iter();
    let str_end = it.position(|&c| c == b'\0').unwrap_or(data.len());
    let mut string = Vec::with_capacity(str_end);
    string.extend_from_slice(&data[0..str_end]);
    Some((str_end+1,  String::from_utf8(string).ok()?))
}

fn extract_id(data: &[u8]) -> u64 {
    let mut bytes = [0u8; std::mem::size_of::<u64>()];
    bytes.copy_from_slice(&data[0..std::mem::size_of::<u64>()]);
    u64::from_be_bytes(bytes)
}

fn extract_id_from(start: usize, data: &[u8]) -> u64 {
    let mut bytes = [0u8; std::mem::size_of::<u64>()];
    bytes.copy_from_slice(&data[start..start+std::mem::size_of::<u64>()]);
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
        bytes.copy_from_slice(&data[skip..skip+std::mem::size_of::<i64>()]);
        Some(PropertyValue::PInteger(i64::from_be_bytes(bytes)))
    } else if prop_type == 2 {
        let mut bytes = [0u8; std::mem::size_of::<f64>()];
        bytes.copy_from_slice(&data[skip..skip+std::mem::size_of::<f64>()]);
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
        let mut prop = Property::new(String::from("qsfsqdf"), PropertyValue::PString(String::from("qgkfdgsdf")));
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
        let mut prop = Property::new(String::from("qsfsqdfqsdfq"), PropertyValue::PString(String::from("qgkfdgsdf")));
        pr.create(&mut prop);
        let load = pr.load(prop.get_id().unwrap()).unwrap();
        assert_eq!(load.get_name(), prop.get_name());
        assert_eq!(load.get_value(), prop.get_value());
    }

    #[test]
    fn test_save_full_inlined() {
        let dyn_file = build_file_path_and_rm_old("test_save_full_inlined_dyn", "dyn.db").unwrap();
        let props_file = build_file_path_and_rm_old("test_save_full_inlined_prop", "prop.db").unwrap();
        let mut pr = PropertiesRespository::new(&props_file, &dyn_file);
        let mut prop = Property::new(String::from("age"),
        PropertyValue::PInteger(19236));
        pr.create(&mut prop);
        let load = pr.load(prop.get_id().unwrap()).unwrap();
        assert_eq!(load.get_name(), prop.get_name());
        assert_eq!(load.get_value(), prop.get_value());
    }
    #[test]
    fn test_save_inlined_key() {
        let dyn_file = build_file_path_and_rm_old("test_save_inlined_key_dyn", "dyn.db").unwrap();
        let props_file = build_file_path_and_rm_old("test_save_inlined_key_prop", "prop.db").unwrap();
        let mut pr = PropertiesRespository::new(&props_file, &dyn_file);
        let mut prop = Property::new(String::from("qsfsqdfqsdfq"),
        PropertyValue::PString(String::from("qgkfdgsdfqerqzerqzerqzerqzerqzerqzerarthdtrsdqeqtrshsreqsgstreq")));
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
        let mut prop = Property::new(String::from("qsfsqdfqsdfqdhgfdhgdfhgdfhqzerqzerqzregdfqsfdqsfderhryjsrrefqzeqgdsfdfsdrrdsredfsqer"),
        PropertyValue::PString(String::from("qgkfdgsdfqerqzerqzerqzerqzerqzerqzerarthdtrsdqeqtrshsreqsgstreq")));
        pr.create(&mut prop);
        let load = pr.load(prop.get_id().unwrap()).unwrap();
        assert_eq!(load.get_name(), prop.get_name());
        assert_eq!(load.get_value(), prop.get_value());
    }
}