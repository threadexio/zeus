macro_rules! config_path {
	// Path resolving
	(@resolve $config:expr, $component:tt . $($path:tt)*) => {
		config_path!(@resolve $config.and_then(|x| x.get(stringify!($component))).and_then(|x| x.as_table()), $($path)*)
	};
	(@resolve $config:expr, $component:tt) => {
		$config.and_then(|x| x.get(stringify!($component)))
	};
	// Primitive type handlers
	(@primitive_type $value:expr, str) => {
		$value.as_str()
	};
	(@primitive_type $value:expr, bool) => {
		$value.as_bool()
	};
	(@primitive_type $value:expr, float) => {
		$value.as_float()
	};
	(@primitive_type $value:expr, integer) => {
		$value.as_integer()
	};
	// Type handlers
	// primitive types are represented with only one token
	(@type $value:expr, array<$inner:tt>) => {
		$value.as_array().map(|x| x.iter().filter_map(|x| config_path!(@primitive_type x, $inner)).collect::<Vec<_>>())
	};
	(@type $value:expr, $($type:tt)*) => {
		config_path!(@primitive_type $value, $($type)*)
	};
	// Main
	($config:expr => $($path:tt).* as $($type:tt)+) => {
		config_path!(@resolve Some(&$config), $($path).*).and_then(|x| config_path!(@type x, $($type)+))
	};
}
pub(crate) use config_path;

#[cfg(test)]
mod tests {
	#[test]
	fn config_path_macro() {
		let config: toml::Value = toml::from_str(
			r#"
root_property = 1234

[table1]
field1_str = "string"
field1_int = 42
field1_float = 42.0
field1_float_array = [42.0]
field1_str_array = ["string1", "string2"]

[table1.table2]
field2 = false
"#,
		)
		.unwrap();

		assert_eq!(
			config_path!(config => root_property as integer),
			Some(1234)
		);
		assert_eq!(
			config_path!(config => root_property as str),
			None
		);
		assert_eq!(
			config_path!(config => table1.field1_str as str),
			Some("string")
		);
		assert_eq!(
			config_path!(config => table1.field1_int as integer),
			Some(42)
		);
		assert_eq!(
			config_path!(config => table1.field1_float as float),
			Some(42.0)
		);
		assert_eq!(
			config_path!(config => table1.field1_float_array as array<float>),
			Some(vec![42.0])
		);
		assert_eq!(
			config_path!(config => table1.field1_str_array as array<str>),
			Some(vec!["string1", "string2"])
		);
		assert_eq!(
			config_path!(config => table1.table2.field2 as bool),
			Some(false)
		);
	}
}
