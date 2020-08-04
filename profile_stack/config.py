import os
import yaml
from loguru import logger


def load_config(dir_path):
    """Load the config file

    Args:
        dir_path (str): Path to the config file

    Returns:
        dictionary: The config file
    """
    os.chdir(dir_path)
    with open(os.getenv("INPUT_PATH")) as config_file:
        config = yaml.load(config_file, Loader=yaml.Loader)
    logger.success("Loaded config file")
    return config
