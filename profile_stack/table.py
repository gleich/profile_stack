import os
import badge
from loguru import logger


def generate_table(config):
    """Generate the table

    Args:
        config (dictionary): The config file

    Returns:
        list: All the lines for the table
    """
    technology_emoji = os.getenv('INPUT_TECHNOLOGY_EMOJI')
    projects_emoji = os.getenv('INPUT_PROJECT_EMOJI')
    rows = [
        f'| {technology_emoji} **Technology** | {projects_emoji} **Projects** |',
        '|-|-|'
    ]
    for technology in config:
        technology_badge = badge.technology(technology)
        project_badges = []
        for project in technology['projects']:
            if type(project) is dict:
                project_url = project["url"]
                project_wip = project.get("wip", False)
            elif type(project) is str:
                project_url = project
                project_wip = False

            project_badges.append(badge.project(project_url, project_wip))
        combined_project_badges = ' '.join(project_badges)
        rows.append(f'| {technology_badge} | {combined_project_badges} |')
    logger.success('Generated table')
    return rows
