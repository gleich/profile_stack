import os
import distutils
import urllib.parse


def technology(config):
    """Generate a badge for the given technology

    Args:
        config (dictionary): Configuration for specific technology

    Returns:
        str: The badge
    """
    if eval(os.getenv('INPUT_BADGES').title()):
        return '[![{name}](https://img.shields.io/static/v1?label=&message={name}&color={color}&logo={logo}&logoColor={logoColor})]({url})'.format(
            name=config['name'],
            color=config['color'],
            logo=config['logo'],
            logoColor=config['logoColor'],
            url=config['url'],
        )
    return '[{name}]({url})'.format(name=config['name'], url=config['url'])


def project(url, wip=False):
    """Generate a badge for the given project url

    Args:
        url (str): Project url on github

    Returns:
        str: The badge
    """
    repo_name = url.split('/')[-1]
    if eval(os.getenv('INPUT_BADGES').title()):
        return f'[![{repo_name}](https://img.shields.io/static/v1?label={repo_name}{urllib.parse.quote(" (WIP)") if wip else ""}&message=%20&color=000605&logo=github&logoColor=white&labelColor=000605)]({url})'
    return '[{name}]({url})'.format(
        name=repo_name,
        url=url,
    )
