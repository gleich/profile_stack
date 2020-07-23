import os
from loguru import logger


def run_commands(commands, error_message):
    """run_commands

    Args:
        commands (list of strings): Commands to run
        error_message ([type]): [description]
    """
    for command in commands:
        exit_code = os.system(command)
        if exit_code != 0:
            logger.error(error_message)
            exit(1)
