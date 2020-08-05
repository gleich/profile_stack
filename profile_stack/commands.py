import os
import sys
from loguru import logger


def run_commands(commands, error_message):
    """run_commands

    Args:
        commands (list of strings): Commands to run
        error_message (string): Error message to be displayed to the user
    """
    for command in commands:
        exit_code = os.system(command)
        if exit_code == 256:
            print("Stopped early")
            sys.exit(0)
        elif exit_code != 0:
            logger.error(error_message)
            sys.exit(1)
