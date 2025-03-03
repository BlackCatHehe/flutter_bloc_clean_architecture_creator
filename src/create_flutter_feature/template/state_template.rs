use heck::{ToSnakeCase, ToUpperCamelCase};

pub fn get_state_template(name: &str) -> String {
    let pascal_name = name.to_upper_camel_case();
    let snake_case_name = name.to_snake_case();

    format!(
        r##"part of '{snake_case_name}_bloc.dart';

@freezed
class {pascal_name}State with _${pascal_name}State {{
  const factory {pascal_name}State() = _{pascal_name}State;
}}
"##,
    )
}
