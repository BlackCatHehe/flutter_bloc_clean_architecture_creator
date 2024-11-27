use heck::{ToSnakeCase, ToUpperCamelCase};

pub fn create_page_template(name: &str, as_router: bool) -> String {
    let name_snake_case = name.to_snake_case();
    let name_camel_case = name.to_upper_camel_case();

    let router_import = if as_router {
        "import 'package:auto_route/auto_route.dart';\n"
    } else {
        "\n"
    };
    let router_annotation = if as_router { "@RoutePage()" } else { "\n" };

    format!(
        r#"{router_import}import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../../../core/injection/injector.dart';
import '../bloc/{name_snake_case}/{name_snake_case}_bloc.dart';
{router_annotation}
class {name_camel_case}Page extends StatelessWidget {{
  const {name_camel_case}Page({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return BlocProvider(
      create: (context) => getIt<{name_camel_case}Bloc>(),
      child: const {name_camel_case}PageView(),
    );
  }}
}}

class {name_camel_case}PageView extends StatelessWidget {{
  const {name_camel_case}PageView({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return Scaffold(
      appBar: AppBar(title: const Text('{name_camel_case}')),
      body: Center(child: Text('example.helloWorld'.tr())),
    );
  }}
}}
"#
    )
}
