use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!(
    pub static ref LOOKUP: HashMap<&'static str, &'static str> = {
        let l = HashMap::from([
            ("google.com", "8.8.8.8")
        ]);
        l
    };  
);
