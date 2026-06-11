# Text Extractor

Консольный инструмент для сбора текста с веб-сайтов и записи в базу данных ARIA.

---

## Что делает

- Принимает URL из консоли
- Скачивает страницу и извлекает текст из параграфов и заголовков
- Добавляет текст в файл `DataBase.txt` в папке проекта ARIA

Текст дописывается в конец файла, предыдущие данные не удаляются.

---

## Требования

| Компонент | Скачать |
|---|---|
| Rust + Cargo | https://rustup.rs |
| MinGW-w64 (для Windows) | [winlibs-x86_64-posix-seh-gcc-16.1.0-mingw-w64ucrt-14.0.0-r3.zip](http://winlibs-x86_64-posix-seh-gcc-16.1.0-mingw-w64ucrt-14.0.0-r3.zip/) |

---

## Запуск

```bash
cd C:\Users\lold\Documents\GitHub\Text-Extractor
cargo run
```

---

## Использование

```
=== Text Extractor for ARIA ===
Output: C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt

Enter URL (or 'exit' to quit):
> https://ru.wikipedia.org/wiki/Искусственный_интеллект
Fetching: https://ru.wikipedia.org/wiki/Искусственный_интеллект
Saved 12453 characters to DataBase.txt

> https://ru.wikipedia.org/wiki/Нейронная_сеть
Fetching: ...
Saved 8721 characters to DataBase.txt

> exit
Goodbye.
```

Вводи ссылки по одной, каждая добавляется в конец `DataBase.txt`. Работает с Wikipedia и большинством обычных сайтов.

---

## Куда сохраняется текст

```
C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt
```

Этот файл используется ARIA для обучения. После наполнения базы запусти ARIA - она автоматически построит словарь и проведёт предобучение на новых данных.
