## Tokenomicon

---

### Описание

Данный проект создавался для целей знакомства с языком Rust и фреймворком Axum.
Использование токенизации в качестве логики сервиса - просто забава. Такие вещи лучше выносить в крейт.

Так же проект является первым шагом на пути к полноценной реализации `LLM` на архитектуре `Transformer`.
Часть логики из этого сервиса (а именно `Byte-level BPE`) будет в дальнейшем использована там.

Проект представляет собой веб-сервер, предназначенный для токенизации текста с использованием языка Rust и фреймворка Axum.

Добавлена поддержка следующих функций:
- Токенизация по словам (words)
- Токенизация по символам (chars)
- Токенизация по методу BPE (без обучения)
- Токенизация по методу byte-level BPE (c обучением)

Для метода BPE (без обучения) в качестве `vocab` используется предварительно обученный словарь токенов.
Вы можете почитать подробнее тут [BPEmb](https://github.com/bheinzerling/bpemb).

### Некоторые детали реализации BPE

Функция `tokenize`:
Токенизирует (сегментирует) полученный текст.
Делает это путем разбивки его на предложения и затем на слова, обрабатывая их, и собирая в итоговый результат.

Функция `split_into_sentences`:
Разбивает текст на предложения и применяет к каждому предложению функцию маркировки.

Функция `tokenize_sentence_with_markers`:
Добавляет маркеры начала/конца к каждому предложению, а затем токенизирует отдельные слова в предложении.

Функция `tokenize_word`:
1. Проверяем пришедшее слово (текст) на пустоту. Если истина, возвращаем пустой вектор, иначе продолжим обработку.
2. Далее слово (текст) преобразуем в вектор символов.
3. После этого циклом проходим по всем возможным длинам подслов от самой большой до самой маленькой. Для каждой длины подслова ищем максимальное совпадение в словаре.
4. Если нашли совпадение, то слово разделяем на три части: левую, правую, и найденное подслово (кандидат). Каждую из этих частей рекурсивно токенизируем.
5. Если ни одно подслово не найдено, возвращаем токен неизвестного слова.

### Некоторые детали реализации Byte-level BPE

- vocab: Словарь, который сопоставляет токены (в виде байтовых последовательностей) с их идентификаторами.
- reverse_vocab: Обратный словарь, который сопоставляет идентификаторы с токенами.
- merges: Список пар токенов, которые были смержены в процессе обучения.
- unk_id: Идентификатор для неизвестных токенов, используемый при кодировании и декодировании.

Функция `train`:
Строит словарь токенов и список слияний, чтобы достичь заданного размера словаря (vocab_size).
Процесс обучения включает:
1. Преобразуем входной текст в байты и инициализируем токены как отдельные байты.
2. Копируем текущий словарь vocab и обратный словарь reverse_vocab. Определяем следующий доступный ID для новых токенов.
3. Все уникальные байты собираются и добавляются в словарь, если они еще не присутствуют в нем.
4. Пока размер словаря меньше заданного vocab_size, находим наиболее частую пару токенов и объединяем их в новый токен.
5. Обновляем токены, заменяя найденные пары на новый токен. Если больше нет пар для слияния, цикл прерываем.
6. Сохраняем обновленный словарь и обратный словарь в структуре.

Функция `encode`:
1. Преобразуем входной текст в байты и инициализируем токены как отдельные байты.
2. Применяем правила слияния для объединения токенов в более длинные последовательности.
3. Преобразуем каждый токен в его идентификатор из словаря vocab. Если токен отсутствует в словаре, используем unk_id.
4. Возвращаем вектор идентификаторов токенов.

Функция `decode`:
1. Инициализируем пустой вектор байтов.
2. Для каждого идентификатора в последовательности находим соответствующий токен в обратном словаре reverse_vocab. Если токен не найден, используем UNKNOWN_TOKEN.
3. Добавляем байты токена в результирующий вектор.
4. Преобразуем вектор байтов в строку и возвращаем её.

### Глоссарий

- Токенизация (сегментация) — это процесс разбиения текста на отдельные части (слова, символы, и т.п.)
- BPE (Byte Pair Encoding) – это алгоритм, используемый в обработке естественного языка (NLP) для токенизации (сегментирования текста на более мелкие единицы).
- Byte-level BPE - это подтип BPE, который в качестве основного компонента токена использует байты вместо символов.

### API

#### Конечная точка: `/api/v1/tokenize/simple`

- **Метод:** `POST`
- **Описание:** Роут для простой токенизации текста по словам или символам.

#### Тело Запроса (с описанием полей):

- `text` (обязательный): Текст который нужно токенизировать.
- `method` (обязательный): Метод, которым будет выполнена токенизация (доступны `chars` и `words`).

```Json
{
    "method": "words",
    "text": "Hello, world! This is a test."
}
```

#### Ожидаемый успешный ответ:

```Json
{
   "tokens": ["Hello,", "world!", "This", "is", "a", "test."]
}
```

---

#### Конечная точка: `/api/v1/tokenize/standard-bpe`

- **Метод:** `POST`
- **Описание:** Роут для токенизации по методу BPE (с использованием готового словаря).

#### Тело Запроса (с описанием полей):

- `text` (обязательный): Текст который нужно токенизировать.

```Json
{
    "text": "Hello, world! This is a test."
}
```

#### Ожидаемый успешный ответ:

```Json
{
   "tokens": ["<s>", "▁hello", "▁world", "</s>", "<s>", "▁this", "▁is", "▁a", "▁test", "</s>"]
}
```

---

#### Конечная точка: `/api/v1/tokenize/byte-level-bpe/train`

- **Метод:** `POST`
- **Описание:** Роут для обучения словаря по методу Byte-level BPE.

#### Тело Запроса (с описанием полей):

- `size` (обязательный): Предельный размер словаря.
- `text` (обязательный): Корпус текста, на котором нужно обучать.

```Json
{
    "size": 30,
    "text": "Hello, world! This is a test."
}
```

#### Ожидаемый успешный ответ:

```Json
{
   "vocab_size": 30,
   "vocab": {"t": 16, "H": 5, "e": 17, "o": 13, "a": 1, "T": 6, "! ": 21, "r": 15, "o,": 22, "i": 10, "!": 8, "st.": 29, "h": 12, "is": 18, ".": 3, "l": 2, "is is ": 25, "d": 9, " ": 11, "<unk>": 0, "orl": 23, "w": 14, "o, ": 27, "a ": 24, "s": 7, "t.": 28, "ll": 26, "is ": 19, "or": 20, ",": 4}
}
```

---

#### Конечная точка: `/api/v1/tokenize/byte-level-bpe/encode`

- **Метод:** `POST`
- **Описание:** Роут для токенизации (векторное представление) по методу Byte-level BPE.

#### Тело Запроса (с описанием полей):

- `text` (обязательный): Текст который нужно токенизировать.

```Json
{
    "text": "This is a test!"
}
```

#### Ожидаемый успешный ответ:

```Json
{
   "tokens": [6, 12, 25, 24, 16, 17, 7, 16, 8]
}
```

---

#### Конечная точка: `/api/v1/tokenize/byte-level-bpe/decode`

- **Метод:** `POST`
- **Описание:** Роут для преобразования токенов (векторное представление) обратно в текст.

#### Тело Запроса (с описанием полей):

- `tokens` (обязательный): Векторное представление токенов.

```Json
{
   "tokens": [6, 12, 25, 24, 16, 17, 7, 16, 8]
}
```

#### Ожидаемый успешный ответ:

```Json
{
   "text": "This is a test!"
}
```

---

### Локальный запуск

1) Для установки `Rust` на unix подобные системы (MacOS, Linux, ...) - запускаем в терминале команду.
   По окончании загрузки вы получите последнюю стабильную версию Rust для вашей платформы, а так же последнюю версию Cargo.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2) Для проверки выполните следующую команду в терминале.

```shell
cargo --version
```

3) Открываем проект, и запускаем команды.

Проверяет код на возможность компиляции (без запуска).
```shell
cargo check
```

Сборка + запуск проекта (в режиме релиза с оптимизациями).
```shell
cargo run --release
```

UDP: Если вдруг у вас Windows, посмотрите [Инструкцию тут](https://forge.rust-lang.org/infra/other-installation-methods.html)
