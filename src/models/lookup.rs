use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!(
    pub static ref LOOKUP: HashMap<&'static str, (&'static str, &'static str)> = {
        let l = HashMap::from([
            ("google.com", ("142.251.36.110", "2a00:1450:4014:80f::200e"))
        ]);
        l
    };  
);
