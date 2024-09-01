# Шаблон для сборки мультиязычного проекта

Шаблон для релиза библиотеки под python через pyo3 обвязку и для релиза в веб через webassembly

TODO list:

- [x] js и python библиотеки разнесены по файлам и собираются отдельно
- [ ] туториал установки
- [ ] nodejs example (под вопросом, это другой таргет, отличный от браузера)
- [ ] наборы тестов на python и js (перенести тесты на headless)
- [ ] команды для запуска тестов (желательно всех вместе ну или хотя бы по очереди)
- [ ] Docker и docker-compose для сборки + запуск тестов внутри (под каждую платформу)
- [ ] 
  Заполнить ` Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended`
- [ ] вынос кода обвязок в отдельные файлы под флагами
- [ ] создание полноценного шаблона (с генерацией имени библиотеки и другими плюшками)
- [ ] небольшой тутор по написанию функций, выдаваемых обвязками

## Установка

TODO

## Сборка

### python

- `maturin develop --features python`
- `maturin develop --features python --release` (для релиз сборки)

### js

- `wasm-pack build --dev --features js`
- `wasm-pack build --release --features js`
