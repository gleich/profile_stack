import os
import yaml
from loguru import logger


def load_config():
    os.chdir('/github/workspace')
    with open(os.getenv('INPUT_PATH')) as config_file:
        config = yaml.load(config_file)
    print(config)
