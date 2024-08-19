use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

use crate::prelude::Value;

use crate::isotope_grouper_module_parameters::*;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IsotopeGrouper{
    method: String,

    parameter_version: u8,

    parameter: Vec<IsotopeGrouperParameters>
}

impl IsotopeGrouper{
    pub fn new() -> Self{
        IsotopeGrouper{
            method: "io.github.mzmine.modules.dataprocessing.filter_isotopegrouper.IsotopeGrouperModule".to_owned(),
            parameter_version: 1,
            parameter: Vec::new(),
        }
    }

    pub fn get_method(&self) -> &str{
        &self.method
    }

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameter.len()
    }

    pub fn add_parameter(&mut self, parameter: IsotopeGrouperParameters){
        self.parameter.push(parameter)
    } 

    pub fn get_parameter(&mut self, target: &str) -> &mut IsotopeGrouperParameters {
        for parameter in &mut self.parameter {
            match parameter {
                IsotopeGrouperParameters::FeatureLists(_) if target == "Feature lists" => return parameter,
                IsotopeGrouperParameters::NameSuffix(_) if target == "Name suffix" => return parameter,
                IsotopeGrouperParameters::MzToleranceIntraSample(_) if target == "Mz tolerance intra sample" => return parameter,
                IsotopeGrouperParameters::MobilityTolerance(_) if target == "Mobility tolerance" => return parameter,
                IsotopeGrouperParameters::MonotonicShape(_) if target == "Monotonic shape" => return parameter,
                IsotopeGrouperParameters::MaximumCharge(_) if target == "Maximum charge" => return parameter,
                IsotopeGrouperParameters::RepresentativeIsotope(_) if target == "Representative isotope" => return parameter,
                IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_) if target == "Never remove feature with MS2" => return parameter,
                IsotopeGrouperParameters::OriginalFeatureList(_) if target == "Original feature lists" => return parameter,
                IsotopeGrouperParameters::RetentionTimeTolerance(_) if target == "Retention time tolerance" => return parameter,
                _ => continue,
            }
        }
        panic!("Parameter '{}' not found", target)
    }
    

    pub fn write_element(&mut self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()>{
        // <batchstep method="io.github.mzmine.modules.dataprocessing.filter_isotopegrouper.IsotopeGrouperModule" parameter_version="1">
        //      <parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/>
        //      <parameter name="Name suffix">deiso</parameter>
        //      <parameter name="m/z tolerance (intra-sample)">
        //          <absolutetolerance>0.0015</absolutetolerance>
        //          <ppmtolerance>3.0</ppmtolerance>
        //      </parameter>
        //      <parameter name="Retention time tolerance" unit="MINUTES">0.04</parameter>
        //      <parameter name="Mobility tolerance" selected="false">1.0</parameter>
        //      <parameter name="Monotonic shape">true</parameter>
        //      <parameter name="Maximum charge">2</parameter>
        //      <parameter name="Representative isotope">Most intense</parameter>
        //      <parameter name="Never remove feature with MS2">true</parameter>
        //      <parameter name="Original feature list">KEEP</parameter>
        // </batchstep>
        let mut element = BytesStart::new("batchstep");

        element.push_attribute(("method", self.get_method())); 
        element.push_attribute(("parameter_version", self.get_parameter_version().to_string().as_str()));

        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        for parameter in &mut self.parameter{
            match parameter{
                IsotopeGrouperParameters::FeatureLists(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::NameSuffix(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::MzToleranceIntraSample(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::MobilityTolerance(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::MonotonicShape(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::MaximumCharge(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::RepresentativeIsotope(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::OriginalFeatureList(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => _f.write_element(writer)?,
                IsotopeGrouperParameters::RetentionTimeTolerance(_f) => _f.write_element(writer)?,
                _ => panic!("No matching parameter")
            }
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("batchstep")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        Ok(())
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum IsotopeGrouperParameters{
    FeatureLists(FeatureLists),
    NameSuffix(NameSuffix),
    MzToleranceIntraSample(MzToleranceIntraSample),
    MobilityTolerance(MobilityTolerance),
    MonotonicShape(MonotonicShape),
    MaximumCharge(MaximumCharge),
    RepresentativeIsotope(RepresentativeIsotope),
    OriginalFeatureList(OriginalFeatureList),
    NeverRemoveFeatureWithMs2(NeverRemoveFeatureWithMs2),
    RetentionTimeTolerance(RetentionTimeTolerance)
}

impl IsotopeGrouperParameters{

    pub fn get_name(&self) -> &str{
        match self{
            IsotopeGrouperParameters::FeatureLists(_f) => _f.get_name(),
            IsotopeGrouperParameters::NameSuffix(_f) => _f.get_name(),
            IsotopeGrouperParameters::MzToleranceIntraSample(_f) => _f.get_name(),
            IsotopeGrouperParameters::MobilityTolerance(_f) => _f.get_name(),
            IsotopeGrouperParameters::MonotonicShape(_f) => _f.get_name(),
            IsotopeGrouperParameters::MaximumCharge(_f) => _f.get_name(),
            IsotopeGrouperParameters::RepresentativeIsotope(_f) => _f.get_name(),
            IsotopeGrouperParameters::OriginalFeatureList(_f) => _f.get_name(),
            IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => _f.get_name(),
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => _f.get_name(),
            _ => panic!("No {} matching", self.get_name())
        }
    }

    pub fn get_value(&self) -> Value{
        match self{
            // IsotopeGrouperParameters::FeatureLists(_f) => _f.get(writer)?,
            IsotopeGrouperParameters::NameSuffix(_f) => Value::Str(_f.get_value()),
            //IsotopeGrouperParameters::MzToleranceIntraSample(_f) => Value::Float(_f.get_value()),
            IsotopeGrouperParameters::MobilityTolerance(_f) => Value::Float(_f.get_value()),
            IsotopeGrouperParameters::MonotonicShape(_f) => Value::Bool(_f.get_value()),
            IsotopeGrouperParameters::MaximumCharge(_f) => Value::Unsigned8(_f.get_value()),
            IsotopeGrouperParameters::RepresentativeIsotope(_f) => Value::Str(_f.get_value()),
            IsotopeGrouperParameters::OriginalFeatureList(_f) => Value::Str(_f.get_value()),
            IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => Value::Bool(_f.get_value()),
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => Value::Float(_f.get_value()),

            _ => panic!("No matching parameter")
        }
    }

    pub fn get_type(&self) -> Value{
        match self {
            IsotopeGrouperParameters::FeatureLists(_f) => Value::Str(_f.get_type()),
            _ => panic!("Not Feature list parameter")
        }
    }

    pub fn get_unit(&self) -> Value{
        match self {
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => Value::Str(_f.get_unit()),
            _ => panic!("Not Retention time tolerance parameter")
        }
    }

    pub fn get_selected(&self) -> Value{
        match self {
            IsotopeGrouperParameters::MobilityTolerance(_f) => Value::Bool(_f.is_selected()),
            _ => panic!("Not Mobility tolerance parameter")
        }
    }

    // TODO: implement method to set/get ppm/absolute tolerance di MzTolerance

    pub fn set_value(&mut self, value: Value){
        match self{
            IsotopeGrouperParameters::NameSuffix(_f) => _f.set_value(value.get_str_value()),
            //IsotopeGrouperParameters::MzToleranceIntraSample(_f) => _f.set_value(*value.get_float_value()),
            IsotopeGrouperParameters::MobilityTolerance(_f) => _f.set_value(*value.get_float_value()),
            IsotopeGrouperParameters::MonotonicShape(_f) => _f.set_value(*value.get_bool_value()),
            IsotopeGrouperParameters::MaximumCharge(_f) => _f.set_value(*value.get_u8_value()),
            IsotopeGrouperParameters::RepresentativeIsotope(_f) => _f.set_value(value.get_str_value()),
            IsotopeGrouperParameters::OriginalFeatureList(_f) => _f.set_value(value.get_str_value()),
            IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => _f.set_value(*value.get_bool_value()),
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => _f.set_value(*value.get_float_value()),

            _ => panic!("No matching parameter")
        }
    }
}