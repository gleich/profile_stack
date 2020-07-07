import config
import os


def test_load_config():
    """Test for the load_config function
    """
    os.environ['INPUT_PATH'] = '../tests/example_config.yml'
    result = config.load_config('.')
    print(result)
    assert result == [{'name': 'Dart', 'logo': 'dart', 'url': 'https://dart.dev/', 'color': '52C0F2', 'logoColor': 'white', 'projects': ['https://github.com/fluttercommunity/import_sorter', 'https://github.com/Matt-Gleich/Personal-Site', {'url': 'https://github.com/Matt-Gleich/auralite-mobile',
                                                                                                                                                                                                                                               'wip': True}]}, {'name': 'Flutter', 'logo': 'flutter', 'url': 'https://flutter.dev/', 'color': '52C0F2', 'logoColor': 'white', 'projects': ['https://github.com/Matt-Gleich/Personal-Site', 'https://github.com/Matt-Gleich/auralite-mobile']}]
