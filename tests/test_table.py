import table
import config
import os


def test_generate_table():
    os.environ['INPUT_TECHNOLOGY_EMOJI'] = '💻'
    os.environ['INPUT_PROJECT_EMOJI'] = '🚀'
    os.environ['INPUT_PATH'] = '../tests/example_config.yml'
    os.environ['INPUT_BADGES'] = 'true'
    config_contents = config.load_config('.')

    result = table.generate_table(config_contents)
    print(result)
    assert result == [
        '| 💻 **Technology** | 🚀 **Projects** |',
        '|-|-|',
        '| [![Dart](https://img.shields.io/static/v1?label=&message=Dart&color=52C0F2&logo=dart&logoColor=white)](https://dart.dev/) | [![import_sorter](https://img.shields.io/static/v1?label=import_sorter&message=%20&color=000605&logo=github&logoColor=white&labelColor=000605)](https://github.com/fluttercommunity/import_sorter) [![Personal-Site](https://img.shields.io/static/v1?label=Personal-Site&message=%20&color=000605&logo=github&logoColor=white&labelColor=000605)](https://github.com/Matt-Gleich/Personal-Site) [![auralite-mobile](https://img.shields.io/static/v1?label=auralite-mobile&message=%20&color=000605&logo=github&logoColor=white&labelColor=000605)](https://github.com/Matt-Gleich/auralite-mobile) |',
        '| [![Flutter](https://img.shields.io/static/v1?label=&message=Flutter&color=52C0F2&logo=flutter&logoColor=white)](https://flutter.dev/) | [![Personal-Site](https://img.shields.io/static/v1?label=Personal-Site&message=%20&color=000605&logo=github&logoColor=white&labelColor=000605)](https://github.com/Matt-Gleich/Personal-Site) [![auralite-mobile](https://img.shields.io/static/v1?label=auralite-mobile&message=%20&color=000605&logo=github&logoColor=white&labelColor=000605)](https://github.com/Matt-Gleich/auralite-mobile) |'
    ]
