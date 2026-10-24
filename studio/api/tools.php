<?php
// studio/api/tools.php — что помощник AI-Corel знает о себе и какие действия ему разрешены.
//
// Инструменты выполняет БРАУЗЕР (studio.js): только он видит запущенный
// редактор. Сервер лишь объявляет модели, какие инструменты есть, и решает,
// какие из них доступны в текущем режиме.

/** Инструменты, которые ничего не меняют в документе. Доступны в обоих режимах. */
function studio_read_tools(): array
{
    return [
        [
            'type' => 'function',
            'function' => [
                'name' => 'find_commands',
                'description' => 'Найти команды редактора по смыслу (по-русски или по-английски). '
                    . 'Возвращает id команды, русское название, путь в меню, доступна ли она сейчас и описание параметров. '
                    . 'Вызывай ВСЕГДА перед run_command или open_command: id нельзя угадывать.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'query' => ['type' => 'string', 'description' => 'Что нужно сделать, 1–4 слова: «прямоугольник», «выровнять по центру», «объединить фигуры», «градиент», «export svg».'],
                    ],
                    'required' => ['query'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'get_state',
                'description' => 'Текущее состояние редактора: открытый документ, монтажные области (координаты и размеры в пунктах), '
                    . 'слои и объекты (id, тип, границы, заливка, обводка), выделение, последние шаги, выбранный инструмент, открытый диалог и его поля.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'look_at_screen',
                'description' => 'Сделать снимок экрана редактора и посмотреть на него. Используй, чтобы проверить результат своих действий или увидеть рисунок.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
    ];
}

/** Инструменты, которые меняют документ или интерфейс. Только в режиме «Сделай». */
function studio_act_tools(): array
{
    return [
        [
            'type' => 'function',
            'function' => [
                'name' => 'run_command',
                'description' => 'Выполнить команду редактора сразу, без диалога, с заданными параметрами. '
                    . 'Параметры бери из описания params, которое вернул find_commands. Действие попадает в Историю и отменяется.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'id' => ['type' => 'string', 'description' => 'id команды из find_commands, например shape.rectangle или object.align'],
                        'params' => ['type' => 'object', 'description' => 'Параметры команды (JSON-объект). Можно не передавать — тогда значения по умолчанию.'],
                    ],
                    'required' => ['id'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'open_command',
                'description' => 'Открыть команду так, как будто пользователь выбрал её в меню: если у команды есть диалог, он откроется, '
                    . 'и пользователь сможет сам настроить значения. Используй, когда пользователь хочет настроить сам или когда параметры неочевидны.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'id' => ['type' => 'string', 'description' => 'id команды из find_commands'],
                    ],
                    'required' => ['id'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'set_dialog_field',
                'description' => 'Изменить поле в открытом диалоге. Имена полей видны в get_state → dialog.fields.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'field' => ['type' => 'string'],
                        'value' => ['description' => 'Новое значение: число, строка, true/false'],
                    ],
                    'required' => ['field', 'value'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'confirm_dialog',
                'description' => 'Нажать OK в открытом диалоге.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'cancel_dialog',
                'description' => 'Нажать «Отмена» в открытом диалоге.',
                'parameters' => ['type' => 'object', 'properties' => new stdClass()],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'select_tool',
                'description' => 'Выбрать инструмент на панели инструментов.',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'tool' => [
                            'type' => 'string',
                            'description' => 'Имя инструмента',
                            'enum' => studio_tool_names(),
                        ],
                    ],
                    'required' => ['tool'],
                ],
            ],
        ],
        [
            'type' => 'function',
            'function' => [
                'name' => 'undo',
                'description' => 'Отменить последние действия (как Ctrl+Z).',
                'parameters' => [
                    'type' => 'object',
                    'properties' => [
                        'steps' => ['type' => 'integer', 'description' => 'Сколько шагов отменить, по умолчанию 1', 'minimum' => 1, 'maximum' => 20],
                    ],
                ],
            ],
        ],
    ];
}

/** Инструменты редактора: имя для select_tool → как он называется по-русски. */
function studio_tools_ru(): array
{
    return [
        'selection' => 'Выделение (V)', 'directSelection' => 'Прямое выделение (A)', 'groupSelection' => 'Выделение группы',
        'magicWand' => 'Волшебная палочка (Y)', 'lasso' => 'Лассо (Q)', 'pen' => 'Перо (P)', 'curvature' => 'Кривизна',
        'addAnchor' => 'Добавить опорную точку (+)', 'deleteAnchor' => 'Удалить опорную точку (-)', 'anchorPoint' => 'Опорная точка (Shift+C)',
        'type' => 'Текст (T)', 'areaType' => 'Текст в области', 'typeOnPath' => 'Текст по контуру', 'verticalType' => 'Вертикальный текст',
        'lineSegment' => 'Отрезок', 'arc' => 'Дуга', 'spiral' => 'Спираль', 'rectangularGrid' => 'Прямоугольная сетка', 'polarGrid' => 'Полярная сетка',
        'rectangle' => 'Прямоугольник (M)', 'roundedRectangle' => 'Скруглённый прямоугольник', 'ellipse' => 'Эллипс (L)',
        'polygon' => 'Многоугольник', 'star' => 'Звезда', 'flare' => 'Блик', 'paintbrush' => 'Кисть (B)', 'blobBrush' => 'Кисть-клякса (Shift+B)',
        'shaper' => 'Формообразование (Shift+N)', 'pencil' => 'Карандаш (N)', 'smooth' => 'Сглаживание', 'pathEraser' => 'Ластик для контуров',
        'join' => 'Соединение', 'eraser' => 'Ластик (Shift+E)', 'scissors' => 'Ножницы (C)', 'knife' => 'Нож',
        'rotate' => 'Поворот (R)', 'reflect' => 'Отражение (O)', 'scale' => 'Масштаб (S)', 'shear' => 'Наклон', 'reshape' => 'Изменение формы',
        'width' => 'Ширина (Shift+W)', 'warp' => 'Деформация (Shift+R)', 'freeTransform' => 'Свободное трансформирование (E)',
        'puppetWarp' => 'Марионеточная деформация', 'shapeBuilder' => 'Создание фигур (Shift+M)', 'livePaintBucket' => 'Быстрая заливка (K)',
        'mesh' => 'Сетчатый градиент (U)', 'gradient' => 'Градиент (G)', 'eyedropper' => 'Пипетка (I)', 'measure' => 'Измерение',
        'blend' => 'Переход (W)', 'symbolSprayer' => 'Распылитель символов (Shift+S)', 'columnGraph' => 'Диаграмма (J)',
        'artboard' => 'Монтажная область (Shift+O)', 'slice' => 'Фрагмент (Shift+K)', 'hand' => 'Рука (H)', 'rotateView' => 'Поворот вида (Shift+H)',
        'zoom' => 'Лупа (Z)',
    ];
}

function studio_tool_names(): array
{
    return array_keys(studio_tools_ru());
}

/** Системная инструкция. $mode: 'advice' (Совет) или 'act' (Сделай). */
function studio_system_prompt(string $mode): string
{
    $tools = [];
    foreach (studio_tools_ru() as $id => $ru) {
        $tools[] = "$id — $ru";
    }
    $toolList = implode('; ', $tools);

    $common = <<<TXT
Ты — встроенный помощник векторного графического редактора AI-Corel (аналог CorelDRAW и Adobe Illustrator: фигуры, контуры,
перо, заливки и обводки, градиенты, текст, слои, монтажные области, обработка контуров, эффекты; интерфейс на русском).
Пользователь работает в редакторе прямо сейчас. К его сообщению приложено состояние программы, а иногда снимок экрана.

Как отвечать:
- Всегда по-русски, коротко и по делу: 1–6 предложений или короткий нумерованный список шагов.
- Пункты меню пиши так: «Объект › Трансформирование › Поворот». Горячие клавиши — в скобках: (Ctrl+G).
- Названия команд и путь в меню бери из find_commands, а не по памяти: в AI-Corel они могут отличаться от CorelDRAW и Illustrator.
  Если пользователь привык к CorelDRAW, называй и привычное ему слово: «Обработка контуров (в CorelDRAW — Формирование)».
- Если не уверен, что видишь на снимке, — так и скажи. Не выдумывай функций, которых нет.
- Если документ не открыт, а задача требует документа, подскажи создать новый (Файл › Новый, Ctrl+N), открыть файл (Ctrl+O)
  или перетащить SVG, PDF, AI, EPS в окно. Файлы CorelDRAW (.cdr) редактор не открывает: их нужно сохранить из CorelDRAW в SVG, PDF или EPS.

Инструменты панели (имя для select_tool — русское название): {$toolList}.
TXT;

    if ($mode === 'act') {
        return $common . "\n\n" . <<<TXT
Режим «Сделай»: ты сам рисуешь и меняешь объекты в редакторе с помощью инструментов.
Это главное правило: не объясняй пользователю, как сделать вручную, — сделай сам. Если раньше в разговоре
ты был в режиме «Совет» и писал, что не можешь ничего менять, — это больше не так: сейчас режим «Сделай».
Каждое сообщение пользователя начинается с пометки текущего режима — верь ей.
Порядок работы:
1. Пойми задачу. Если она неоднозначна и ошибка дорого стоит — задай один уточняющий вопрос.
2. get_state — посмотри монтажные области (координаты и размеры в пунктах, pt), объекты и выделение.
3. find_commands — найди нужную команду; проверь, что она enabled. Если why_disabled говорит, что нет документа или выделения, —
   сначала устрани причину (создай документ, выдели объекты через select.set {ids}) или объясни пользователю.
4. Выполни run_command с параметрами из params. Частые случаи:
   - фигуры: shape.rectangle {x, y, width, height, radius?}, shape.ellipse {x, y, width, height}, shape.polygon {cx, cy, radius, sides},
     shape.star {cx, cy, radius1, radius2, points}, shape.line, text.create {x, y, text, size?, font?, color?};
     координаты в пунктах внутри монтажной области; новая фигура становится выделенной;
   - цвет выделенного: paint.setFill {color: "#rrggbb"}, paint.setStroke {color, ...}; убрать заливку или обводку — {none: true};
   - выравнивание: object.align {horizontal?, vertical?, to?}, object.distribute; порядок: object.arrange.*; группы: object.group / object.ungroup;
   - обработка контуров: object.pathfinder.unite / minusFront / intersect / exclude / divide (нужно выделить 2+ объекта);
   - трансформирование: object.move {dx, dy}, object.rotate, object.scale, object.reflect.
   Когда пользователь хочет настроить сам или видеть диалог — open_command, затем set_dialog_field и confirm_dialog.
5. После заметных изменений посмотри на результат (look_at_screen) и коротко скажи, что сделал и как отменить (Ctrl+Z).
Правила безопасности:
- Не закрывай документы, не сохраняй поверх файлов и не удаляй объекты и слои, если пользователь прямо об этом не просил.
- Делай не больше 8 действий подряд; если задача больше — сделай часть и спроси, продолжать ли.
- Если команда вернула ошибку — не повторяй вслепую: прочитай ошибку и params_doc, поправь параметры или объясни, что мешает.
TXT;
    }

    return $common . "\n\n" . <<<TXT
Режим «Совет»: ты только объясняешь и подсказываешь, ничего не меняешь сам.
Можно искать команды (find_commands), смотреть состояние (get_state) и экран (look_at_screen).
Объясняй, куда нажать и что выставить, с конкретными числами, если они уместны.
Если пользователь просит нарисовать или сделать что-то за него — скажи, что для этого нужно переключиться в режим «Сделай» (кнопка вверху панели).
TXT;
}
