import os
import badge


def generate_table(config):
    technology_emoji = os.getenv('INPUT_TECHNOLOGY_EMOJI')
    projects_emoji = os.getenv('INPUT_PROJECT_EMOJI')
    rows = [
        f'|{technology_emoji} Techonology |{projects_emoji} Projects|',
        '|-|-|'
    ]
    for technology in config:
        technology_badge = badge.technology(technology)
        project_badges = []
        for project_url in technology['projects']:
            project_badges.append(badge.project(project_url))
