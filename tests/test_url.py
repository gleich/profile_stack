import url
import os


def test_technology():
    """[summary]
    """
    dart_config = {'name': 'Dart', 'logo': 'dart', 'url': 'https://dart.dev/', 'color': '52C0F2', 'logoColor': 'white', 'projects': [
        'https://github.com/fluttercommunity/import_sorter', 'https://github.com/Matt-Gleich/Personal-Site', 'https://github.com/Matt-Gleich/auralite-mobile']}

    os.environ['INPUT_BADGES'] = 'true'
    badged_result = url.technology(dart_config)
    assert badged_result == '[![Dart](https://img.shields.io/static/v1?label=&message=Dart&color=52C0F2&logo=dart&logoColor=white)](https://dart.dev/)'

    os.environ['INPUT_BADGES'] = 'false'
    plain_result = url.technology(dart_config)
    assert plain_result == '[Dart](https://dart.dev/)'
