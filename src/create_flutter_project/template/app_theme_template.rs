pub fn create_app_theme_file() -> String {
    r#"import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:injectable/injectable.dart';

@injectable
class AppThemeCubit extends Cubit<ThemeMode> {
  AppThemeCubit() : super(ThemeMode.system);

  void toggleTheme() {
    final isLight = state == ThemeMode.system
        ? WidgetsBinding.instance.platformDispatcher.platformBrightness ==
            Brightness.light
        : state == ThemeMode.light;
    emit(isLight ? ThemeMode.dark : ThemeMode.light);
  }
}
"#
    .to_string()
}
