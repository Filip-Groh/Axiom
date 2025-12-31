use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum DataType {
    None,
    ToBeInferred,

    I32,
    Bool,
    Function(Vec<DataType>, Box<DataType>),
    Type(Box<DataType>)
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            DataType::None => write!(f, "None"),
            DataType::ToBeInferred => write!(f, "unknown"),
            DataType::I32 => write!(f, "i32"),
            DataType::Bool => write!(f, "bool"),
            DataType::Function(param_datatypes, output_datatype) => {
                write!(f, "fn(")?;

                let mut first = true;
                for param_datatype in param_datatypes {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param_datatype)?;
                    first = false;
                }

                write!(f, ") -> {}", output_datatype)
            },
            DataType::Type(data_type) => write!(f, "{data_type}"),
        }
    }
}