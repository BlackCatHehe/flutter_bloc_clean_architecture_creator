pub fn create_injector_file() -> String {
    r#"import 'package:get_it/get_it.dart';
import 'package:injectable/injectable.dart';

import 'injector.config.dart';

final getIt = GetIt.instance;

@InjectableInit(initializerName: 'initGetIt')
void configureDependencies() => getIt.initGetIt();
"#
    .to_string()
}
