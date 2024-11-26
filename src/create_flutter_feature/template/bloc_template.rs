use heck::*;
pub fn get_bloc_template(name: &str, use_injectable: bool) -> String {
    let pascal_bloc_name = name.to_upper_camel_case();
    let snake_case_bloc_name = name.to_snake_case();
    let event_name = format!("{}Event", name.to_upper_camel_case());
    let state_name = format!("{}State", name.to_upper_camel_case());

    let injectable_import = if use_injectable {
        "import 'package:injectable/injectable.dart';\n"
    } else {
        ""
    };
    let injectable_annotation = if use_injectable { "@injectable" } else { "" };

    format!(
        r##"import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
{injectable_import}
part '{snake_case_bloc_name}_bloc.freezed.dart';
part '{snake_case_bloc_name}_event.dart';
part '{snake_case_bloc_name}_state.dart';
{injectable_annotation}
class {pascal_bloc_name}Bloc extends Bloc<{event_name}, {state_name}> {{
  {pascal_bloc_name}Bloc() : super(const {state_name}()) {{
    on<{event_name}>((event, emit) {{}});
  }}
}}
"##,
    )
}
