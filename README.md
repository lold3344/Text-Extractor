# Text Extractor

Консольный инструмент для сбора русского текста с веб-сайтов и записи в базу данных ARIA.

---

## Что делает

- Принимает до 500 ссылок за раз
- Скачивает страницы параллельно
- Извлекает только русский текст (кириллица, фильтр >50%)
- Ищет текст в параграфах, блоках, таблицах и списках
- Убирает сноски вида [1], [2] и мусор
- Сохраняет в базу данных ARIA, а если ARIA не установлена - создаёт `DataBase.txt` на рабочем столе

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
cd C:\Users\USER\Documents\GitHub\Text-Extractor
cargo run
```

---

## Использование

Вставляй ссылки по одной, когда закончил - пиши `go`:

```
=== Text Extractor for ARIA ===
Output: C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt

Paste up to 500 URLs (one per line).
When done - type 'go' and press Enter.

> https://ru.wikipedia.org/wiki/Искусственный_интеллект
Added [1/500]: https://ru.wikipedia.org/wiki/Искусственный_интеллект
> https://ru.wikipedia.org/wiki/Нейронная_сеть
Added [2/500]: https://ru.wikipedia.org/wiki/Нейронная_сеть
> go
Processing 2 URLs...
Output: C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt

Fetching: https://ru.wikipedia.org/wiki/Искусственный_интеллект
Fetching: https://ru.wikipedia.org/wiki/Нейронная_сеть
[OK] 12453 chars - https://ru.wikipedia.org/wiki/Искусственный_интеллект
[OK] 8721 chars - https://ru.wikipedia.org/wiki/Нейронная_сеть

Done: 2/2 pages saved to DataBase.txt
```

---

## Куда сохраняется текст

Если ARIA установлена:
```
C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt
```

Если ARIA не найдена - файл создаётся на рабочем столе:
```
C:\Users\lold\Desktop\DataBase.txt
```

Программа сообщит куда сохраняет при каждом запуске.

---

## Примечания

- Английские и другие неруссkие сайты будут пропущены с пометкой `[SKIP]`
- Таймаут на загрузку страницы - 60 секунд
- Все ссылки в батче загружаются параллельно
