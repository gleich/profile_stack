import os
import distutils


def technology(config):
    """Generate a url for the given technology

    Args:
        config (dictionary): Configuration for specific technology

    Returns:
        str: url
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
