pub enum Value<'a> {
    Float(&'a Option<f32>),
    Str(&'a str),
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
}