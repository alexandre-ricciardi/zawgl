use super::store::*;
use super::super::model::*;

pub struct PropertiesRespository {
    prop_store: properties_store::PropertiesStore,
    dyn_store: dynamic_store::DynamicStore,
}

fn compute_prop_name_size(prop: &Property) -> usize {
    let mut size = 0usize;
    prop.name.as_ref().map(|name| size += name.len());
    size
}

fn compute_prop_size(prop: &Property) -> usize {
    let mut size = 0usize;
    size += compute_prop_name_size(prop);
    prop.value.as_ref().map(|value| match value {
        PropertyValue::PString(sval) => size += sval.len(),
        PropertyValue::PInteger(_) => size += std::mem::size_of::<i64>(),
        PropertyValue::PFloat(_) => size += std::mem::size_of::<f64>(),
        PropertyValue::PBool(_) => size += std::mem::size_of::<bool>(),
    });
    size
}

impl PropertiesRespository {
    pub fn new(props_file: &str, dyn_file: &str) -> Self {
        PropertiesRespository {prop_store: properties_store::PropertiesStore::new(props_file), dyn_store: dynamic_store::DynamicStore::new(dyn_file)}
    }

    pub fn save(&mut self, prop: Property) {
        if (compute_prop_size(prop) < 24) {

        } else if (compute_prop_name_size(prop) < 24) {

        } else {

        }
        //self.dyn_store.save(dr)
        //self.prop_store.save(pr: PropertyRecord)
    }



    pub fn load(&mut self, prop_id: u64) {

    }
}