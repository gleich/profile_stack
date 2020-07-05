import os
import url


def generate_table(config):
    technology_emoji = os.getenv('INPUT_TECHNOLOGY_EMOJI')
    projects_emoji = os.getenv('INPUT_PROJECT_EMOJI')
    rows = [
        f'|{technology_emoji} Techonology |{projects_emoji} Projects|',
        '|-|-|'
    ]
    for technology in config:
        technology_url = url.technology(technology)
