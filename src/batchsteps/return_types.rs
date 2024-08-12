pub enum Value<'a> {
    Float(&'a Option<f32>),
    Str(&'a str),
    Bool(&'a bool),
    Unsigned8(&'a Option<u8>)
}

impl<'a> Value<'a> {
    pub fn get_float_value(&self) -> &Option<f32> {
        match self {
            Value::Float(val) => val,
            _ => panic!("Not &Option<f32> returning parameter"),
        }
    }

    pub fn get_str_value(&self) -> &str{
        match self{
            Value::Str(val) => val,
        _=> panic!("Not a &str returning parameter"),
        }
    }

    pub fn get_bool_value(&self) -> &bool{
        match self {
            Value::Bool(val) => val,
            _ => panic!("Not a &bool returning parameter"),
        }
    }

    pub fn get_u8_value(&self) -> &Option<u8>{
        match self {
            Value::Unsigned8(val) => val,
            _ => panic!("Not a &u8 returning parameter"),
        }
    }
}