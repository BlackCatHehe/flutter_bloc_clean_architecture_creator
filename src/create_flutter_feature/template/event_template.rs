use heck::{ToSnakeCase, ToUpperCamelCase};

pub fn get_event_template(name: &str) -> String {
    let pascal_name = name.to_upper_camel_case();
    let snake_case_name = name.to_snake_case();

    format!(
        r##"part of '{snake_case_name}_bloc.dart';

@freezed
class {pascal_name}Event with _${pascal_name}Event {{
  const factory {pascal_name}Event() = _{pascal_name}Event;
}}
"##,
    )
}
