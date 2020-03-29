use super::store::*;
use super::super::model::*;

pub struct PropertiesRespository {
    prop_store: properties_store::PropertiesStore,
    dyn_store: dynamic_store::DynamicStore,
}

fn compute_prop_name_size(prop: &Property) -> Option<usize> {
    prop.name.as_ref().map(|name| name.len())
}

fn compute_prop_size(prop: &Property) -> Option<usize> {
    prop.value.as_ref().map(|value| match value {
        PropertyValue::PString(sval) => sval.len(),
        PropertyValue::PInteger(_) => std::mem::size_of::<i64>(),
        PropertyValue::PFloat(_) => std::mem::size_of::<f64>(),
        PropertyValue::PBool(_) => std::mem::size_of::<bool>(),
    }).and_then(|vsize| compute_prop_name_size(prop).map(|nsize| nsize + vsize))
}

fn map_prop_type(prop: &Property) -> Option<u8> {
    prop.value.as_ref().map(|value| match value {
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
            prop.name.as_ref().and_then(|name| {
                let mut block = [0u8; 24];
                block.copy_from_slice(&name.clone().into_bytes());
                prop.value.as_ref().and_then(|value| {
                    match value {
                        PropertyValue::PString(sval) => block[(name.len() + 1)..].copy_from_slice(&sval.clone().into_bytes()),
                        PropertyValue::PInteger(ival) => block[(name.len() + 1)..].copy_from_slice(&ival.to_be_bytes()),
                        PropertyValue::PFloat(fval) => block[(name.len() + 1)..].copy_from_slice(&fval.to_be_bytes()),
                        PropertyValue::PBool(bval) => block[(name.len() + 1)..].copy_from_slice(&[*bval as u8]),
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

fn make_key_inlined_record(prop: &Property) -> Option<records::PropertyRecord> {
    is_key_inlined(prop).and_then(|key| {
        if key {
            prop.name.as_ref().and_then(|name| {
                let mut block = [0u8; 24];
                block.copy_from_slice(&name.clone().into_bytes());
                
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
        } else {
            None
        }
    })
    
}


impl PropertiesRespository {
    pub fn new(props_file: &str, dyn_file: &str) -> Self {
        PropertiesRespository {prop_store: properties_store::PropertiesStore::new(props_file), dyn_store: dynamic_store::DynamicStore::new(dyn_file)}
    }

    pub fn save(&mut self, prop: &mut Property) {
        let prop_id = make_full_inlined_record(prop).as_mut()
            .or(make_key_inlined_record(prop).as_mut()
            .and_then(|r| {
                prop.value.as_ref().map(|val| {
                    match val {
                        PropertyValue::PString(sval) => self.dyn_store.save_data(&sval.clone().into_bytes()),
                        PropertyValue::PInteger(ival) => self.dyn_store.save_data(&ival.to_be_bytes()),
                        PropertyValue::PFloat(fval) => self.dyn_store.save_data(&fval.to_be_bytes()),
                        PropertyValue::PBool(bval) => self.dyn_store.save_data(&[*bval as u8]),
                    }
                }).map(|dr_id| {r.prop_block.copy_from_slice(&dr_id.to_be_bytes()); r})
            })).as_deref().map(|r| self.prop_store.save(r));
            prop.id = prop_id;
    }


    pub fn load(&mut self, prop_id: u64) {

    }
}