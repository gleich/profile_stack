import badge
import os
import config


def test_technology():
    """Test for the technology function
    """
    os.environ["INPUT_PATH"] = "../tests/example_config.yml"
    dart_config = config.load_config(".")[0]

    os.environ["INPUT_BADGES"] = "true"
    badged_result = badge.technology(dart_config)
    assert (
        badged_result
        == "[![Dart](https://img.shields.io/static/v1?label=&message=Dart&color=52C0F2&logo=dart&logoColor=white)](https://dart.dev/)"
    )

    os.environ["INPUT_BADGES"] = "false"
    plain_result = badge.technology(dart_config)
    assert plain_result == "[Dart](https://dart.dev/)"


def test_project():
    """Test for the project function
    """
    os.environ["INPUT_PATH"] = "../tests/example_config.yml"
    project_url = config.load_config(".")[0]["projects"][0]
    wip_project_url = config.load_config(".")[0]["projects"][-1]

    os.environ["INPUT_BADGES"] = "true"
    badged_result = badge.project(project_url)
    wip_badged_result = badge.project(project_url)
    assert (
        badged_result
        == "[![import_sorter](https://img.shields.io/static/v1?label=&message=import_sorter&color=000605&logo=github&logoColor=white&labelColor=000605)](https://github.com/fluttercommunity/import_sorter)"
    )

    os.environ["INPUT_BADGES"] = "false"
    plain_result = badge.project(project_url)
    assert (
        plain_result
        == "[import_sorter](https://github.com/fluttercommunity/import_sorter)"
    )
