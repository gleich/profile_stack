import url
import os
import config


def test_technology():
    """Test for the technology function
    """
    os.environ['INPUT_PATH'] = '../tests/example_config.yml'
    dart_config = config.load_config('.')[0]

    os.environ['INPUT_BADGES'] = 'true'
    badged_result = url.technology(dart_config)
    assert badged_result == '[![Dart](https://img.shields.io/static/v1?label=&message=Dart&color=52C0F2&logo=dart&logoColor=white)](https://dart.dev/)'

    os.environ['INPUT_BADGES'] = 'false'
    plain_result = url.technology(dart_config)
    assert plain_result == '[Dart](https://dart.dev/)'
